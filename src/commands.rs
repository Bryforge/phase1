use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{self, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::browser::Browser;
use crate::kernel::{Kernel, VfsNode, VERSION};
use crate::man;
use crate::ned;
use crate::network::NetworkStack;

pub struct Phase1Shell {
    pub kernel: Kernel,
    pub network: NetworkStack,
    pub start_time: Instant,
    pub history: VecDeque<String>,
    pub plugins_dir: PathBuf,
    pub env: HashMap<String, String>,
}

impl Phase1Shell {
    pub fn new() -> Self {
        let mut shell = Self {
            kernel: Kernel::new(),
            network: NetworkStack::new(),
            start_time: Instant::now(),
            history: VecDeque::with_capacity(300),
            plugins_dir: PathBuf::from("plugins"),
            env: HashMap::with_capacity(8),
        };

        shell.env.insert("PATH".to_string(), "/bin:/usr/bin:/plugins".to_string());
        shell.env.insert("USER".to_string(), "root".to_string());
        shell.env.insert("HOME".to_string(), "/home".to_string());
        shell.env.insert("SHELL".to_string(), "phase1".to_string());
        shell.env.insert("TERM".to_string(), "xterm-256color".to_string());

        if let Err(err) = fs::create_dir_all(&shell.plugins_dir) {
            eprintln!("plugin directory warning: {}", err);
        }

        shell
    }

    pub fn print_boot() {
        println!("================================================================================");
        println!(" phase1 v{} — Terminal OS Simulator", VERSION);
        println!(" VFS + Scheduler + Editor + Python + C + Networking + Browser + Man Pages");
        println!("================================================================================");
        println!("[ 0.000000] phase1 kernel booted");
        println!("[ 0.012345] in-memory VFS and proc mounted");
        println!("[ 0.034567] simulated scheduler ready");
        println!("[ 0.067890] host integrations guarded by timeout/safety checks");
        println!("[ 0.178901] boot complete — type 'help'");
        println!();
    }

    pub fn user(&self) -> &str {
        &self.kernel.scheduler.current_user
    }

    pub fn push_history(&mut self, line: &str) {
        self.history.push_back(line.to_string());
        if self.history.len() > 300 {
            self.history.pop_front();
        }
    }

    pub fn expand_env(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] != '$' {
                out.push(chars[i]);
                i += 1;
                continue;
            }

            i += 1;
            let mut name = String::new();
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                name.push(chars[i]);
                i += 1;
            }

            if name.is_empty() {
                out.push('$');
            } else if let Some(value) = self.env.get(&name) {
                out.push_str(value);
            }
        }

        out
    }

    pub fn cmd_cd(&mut self, path: Option<&str>) {
        let target = path.unwrap_or_else(|| self.env.get("HOME").map(String::as_str).unwrap_or("/home"));
        let resolved = self.kernel.vfs.resolve_path(target);

        match self.kernel.vfs.get_node(&resolved) {
            Some(VfsNode::Dir { .. }) => self.kernel.vfs.cwd = resolved,
            Some(_) => println!("cd: not a directory"),
            None => println!("cd: no such directory"),
        }
    }

    fn list_plugins(&self) -> String {
        let mut names = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.plugins_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("py") {
                    if let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) {
                        names.push(name.to_string());
                    }
                }
            }
        }
        names.sort();
        if names.is_empty() {
            "No plugins found in ./plugins\n".to_string()
        } else {
            format!("{}\n", names.join("\n"))
        }
    }

    fn run_plugin(&self, name: &str) -> bool {
        if !is_safe_command_name(name) {
            return false;
        }

        let path = self.plugins_dir.join(format!("{}.py", name));
        if !path.exists() {
            return false;
        }

        let input = format!(
            "COMMAND={}\nUSER={}\nCWD={}\nVERSION={}\n",
            name,
            self.user(),
            self.kernel.vfs.cwd.display(),
            VERSION
        );

        let mut cmd = Command::new("python3");
        cmd.arg(path);
        match run_with_input(cmd, &input, Duration::from_secs(5)) {
            Ok(output) if output.status.success() => {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Ok(output) => {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }
            Err(err) => eprintln!("plugin failed: {}", err),
        }

        true
    }
}

pub fn parse_line(input: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match quote {
            Some(q) if ch == q => quote = None,
            Some(_) if ch == '\\' => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            Some(_) => current.push(ch),
            None if ch == '\'' || ch == '"' => quote = Some(ch),
            None if ch.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            None if ch == '>' => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(">>".to_string());
                } else {
                    tokens.push(">".to_string());
                }
            }
            None if ch == '\\' => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            None => current.push(ch),
        }
    }

    if let Some(q) = quote {
        return Err(format!("unterminated quote {}", q));
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

pub fn dispatch(shell: &mut Phase1Shell, cmd: &str, args: &[String]) {
    if shell.run_plugin(cmd) {
        return;
    }

    match cmd {
        "help" => print_help(),
        "version" => println!("phase1 {}", VERSION),
        "exit" | "quit" | "shutdown" | "poweroff" => {
            println!("Shutting down phase1 v{}... Goodbye!", VERSION);
            std::process::exit(0);
        }
        "clear" => print!("{}", "\n".repeat(40)),
        "pwd" => println!("{}", shell.kernel.vfs.cwd.display()),
        "cd" => shell.cmd_cd(args.first().map(String::as_str)),
        "ls" => {
            let long = args.iter().any(|a| a == "-l" || a == "-la" || a == "-al");
            let path = args.iter().find(|a| !a.starts_with('-')).map(String::as_str);
            print!("{}", shell.kernel.vfs.ls(path, long));
        }
        "cat" => {
            if let Some(path) = args.first() {
                match shell.kernel.vfs.cat(path) {
                    Ok(content) => print!("{}", content),
                    Err(err) => println!("{}", err),
                }
            } else {
                println!("Usage: cat <file>");
            }
        }
        "mkdir" => one_arg(args, "mkdir <dir>", |path| {
            if let Err(err) = shell.kernel.vfs.mkdir(path) {
                println!("mkdir failed: {}", err);
            }
        }),
        "touch" => one_arg(args, "touch <file>", |path| {
            if let Err(err) = shell.kernel.vfs.touch(path) {
                println!("touch failed: {}", err);
            }
        }),
        "rm" => one_arg(args, "rm <path>", |path| {
            if let Err(err) = shell.kernel.vfs.rm(path) {
                println!("rm failed: {}", err);
            }
        }),
        "cp" => {
            if args.len() < 2 {
                println!("Usage: cp <src> <dst>");
            } else if let Err(err) = shell.kernel.vfs.cp(&args[0], &args[1]) {
                println!("cp failed: {}", err);
            }
        }
        "mv" => {
            if args.len() < 2 {
                println!("Usage: mv <src> <dst>");
            } else if let Err(err) = shell.kernel.vfs.mv(&args[0], &args[1]) {
                println!("mv failed: {}", err);
            }
        }
        "tree" => print!("{}", shell.kernel.vfs.tree()),
        "echo" => handle_echo(shell, args),
        "history" => {
            for (idx, line) in shell.history.iter().enumerate() {
                println!("{:>4}  {}", idx + 1, line);
            }
        }
        "env" => {
            let mut keys: Vec<_> = shell.env.keys().cloned().collect();
            keys.sort();
            for key in keys {
                println!("{}={}", key, shell.env[&key]);
            }
        }
        "export" => {
            if let Some(raw) = args.first() {
                if let Some((key, value)) = raw.split_once('=') {
                    shell.env.insert(key.to_string(), value.to_string());
                } else {
                    println!("Usage: export KEY=value");
                }
            } else {
                println!("Usage: export KEY=value");
            }
        }
        "unset" => one_arg(args, "unset KEY", |key| {
            shell.env.remove(key);
        }),
        "whoami" => println!("{}", shell.user()),
        "id" => println!("uid={}({}) gid={}({})", shell.kernel.scheduler.current_uid, shell.user(), shell.kernel.scheduler.current_uid, shell.user()),
        "su" => {
            let user = args.first().map(String::as_str).unwrap_or("root");
            shell.kernel.scheduler.current_user = user.to_string();
            shell.kernel.scheduler.current_uid = if user == "root" { 0 } else { 1000 };
            shell.env.insert("USER".to_string(), user.to_string());
        }
        "ps" => print!("{}", shell.kernel.scheduler.ps()),
        "top" => print!("{}", shell.kernel.scheduler.top()),
        "jobs" => print!("{}", shell.kernel.scheduler.jobs()),
        "spawn" => {
            if args.is_empty() {
                println!("Usage: spawn <name> [args...] [--background]");
            } else {
                let background = args.iter().any(|a| a == "--background" || a == "&");
                let filtered: Vec<_> = args.iter().filter(|a| *a != "--background" && *a != "&").cloned().collect();
                let name = filtered.first().cloned().unwrap_or_else(|| "process".to_string());
                let cmdline = filtered.join(" ");
                match shell.kernel.scheduler.spawn(&name, process::id(), &cmdline, 4096, background, 0) {
                    Some(pid) => println!("spawned pid {}", pid),
                    None => println!("process table full"),
                }
            }
        }
        "kill" => println!("{}", shell.kernel.scheduler.kill(args.first().map(String::as_str))),
        "nice" => {
            let prio = args.get(1).and_then(|p| p.parse::<i32>().ok());
            println!("{}", shell.kernel.scheduler.nice(args.first().map(String::as_str), prio));
        }
        "fg" => println!("{}", shell.kernel.scheduler.set_background(args.first().map(String::as_str), false)),
        "bg" => println!("{}", shell.kernel.scheduler.set_background(args.first().map(String::as_str), true)),
        "browser" => {
            let url = args.first().map(String::as_str).unwrap_or("about");
            println!("{}", Browser::new().browse(url));
        }
        "ifconfig" => {
            shell.network.refresh();
            print!("{}", shell.network.ifconfig());
        }
        "iwconfig" => {
            shell.network.refresh();
            print!("{}", shell.network.iwconfig());
        }
        "wifi-scan" => print!("{}", shell.network.wifi_scan()),
        "wifi-connect" => {
            if args.is_empty() {
                println!("Usage: wifi-connect <ssid> [password]");
            } else {
                println!("{}", shell.network.wifi_connect(&args[0], args.get(1).map(String::as_str)));
            }
        }
        "ping" => one_arg(args, "ping <host>", |host| print!("{}", shell.network.ping(host))),
        "nmcli" => print!("{}", shell.network.nmcli()),
        "lspci" => print!("{}", shell.kernel.pcie.lspci()),
        "pcie" => print!("{}", shell.kernel.pcie.pcie_info()),
        "cr3" => println!("CR3=0x{:x}", shell.kernel.scheduler.get_cr3()),
        "loadcr3" => {
            if let Some(value) = args.first().and_then(|v| parse_u64(v)) {
                match shell.kernel.scheduler.load_cr3(value) {
                    Ok(_) => println!("CR3 loaded: 0x{:x}", value),
                    Err(err) => println!("{}", err),
                }
            } else {
                println!("Usage: loadcr3 <hex|decimal>");
            }
        }
        "cr4" => println!("{}", shell.kernel.scheduler.cr4()),
        "pcide" => match args.first().map(String::as_str) {
            Some("on") => {
                shell.kernel.scheduler.set_pcide(true);
                println!("PCIDE enabled");
            }
            Some("off") => {
                shell.kernel.scheduler.set_pcide(false);
                println!("PCIDE disabled");
            }
            _ => println!("Usage: pcide on|off"),
        },
        "free" | "mem" => println!("MemTotal: 4194304 kB\nMemFree: 2097152 kB\nMemUsed: 2097152 kB"),
        "df" => println!("Filesystem     1K-blocks   Used Available Mounted on\nphase1fs         1048576      4   1048572 /"),
        "dmesg" => println!("[0.000000] phase1 {}\n[0.012345] VFS mounted\n[0.034567] scheduler online", VERSION),
        "vmstat" => println!("procs -----------memory---------- ---system--\nr  b   free   buff  cache   in   cs\n1  0 2097152  1024  4096   10   25"),
        "uname" => println!("phase1 3.3.2 terminal-os-sim rust"),
        "date" => println!("{}", now_unix()),
        "uptime" => println!("up {} seconds", shell.start_time.elapsed().as_secs()),
        "hostname" => println!("phase1"),
        "sandbox" | "nsinfo" => println!("phase1 uses an in-memory VFS and simulated processes. Host tools are guarded with validation and timeouts."),
        "man" => {
            if let Some(topic) = args.first() {
                match man::get_man_page(topic) {
                    Some(page) => println!("{}", page),
                    None => println!("No manual entry for {}", topic),
                }
            } else {
                println!("Usage: man <command>");
            }
        }
        "ned" | "nano" | "vi" => one_arg(args, "ned <file>", |path| ned::edit(&mut shell.kernel.vfs, path)),
        "plugins" | "plugin" => print!("{}", shell.list_plugins()),
        "python" | "py" => run_python(shell, args),
        "gcc" | "cc" => run_c(shell, args),
        other => println!("command not found: {} (type help)", other),
    }

    let _ = io::stdout().flush();
}

fn print_help() {
    println!("phase1 commands:");
    println!("  help man version clear exit");
    println!("  ls cd pwd cat mkdir touch rm cp mv tree echo history");
    println!("  env export unset whoami id su");
    println!("  ps top spawn jobs fg bg kill nice");
    println!("  cr3 loadcr3 cr4 pcide lspci pcie free df dmesg vmstat");
    println!("  browser ifconfig iwconfig wifi-scan wifi-connect ping nmcli");
    println!("  ned python gcc plugins sandbox uptime date hostname uname");
}

fn handle_echo(shell: &mut Phase1Shell, args: &[String]) {
    if let Some(idx) = args.iter().position(|a| a == ">" || a == ">>") {
        if idx + 1 >= args.len() {
            println!("echo: missing redirect target");
            return;
        }
        let append = args[idx] == ">>";
        let mut text = args[..idx].join(" ");
        text.push('\n');
        if let Err(err) = shell.kernel.vfs.write_file(&args[idx + 1], &text, append) {
            println!("Redirect error: {}", err);
        }
    } else {
        println!("{}", args.join(" "));
    }
}

fn run_python(shell: &mut Phase1Shell, args: &[String]) {
    if args.is_empty() {
        println!("Usage: python <file.py> | python -c \"code\"");
        return;
    }

    let code = if args[0] == "-c" {
        args[1..].join(" ")
    } else {
        match shell.kernel.vfs.cat(&args[0]) {
            Ok(content) => content,
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
    };

    let path = std::env::temp_dir().join(format!("phase1_{}_{}.py", process::id(), now_unix()));
    if let Err(err) = fs::write(&path, code) {
        println!("Failed to write Python script: {}", err);
        return;
    }

    let mut cmd = Command::new("python3");
    cmd.arg(&path);
    match run_with_timeout(cmd, Duration::from_secs(5)) {
        Ok(output) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Err(err) => println!("Python failed: {}", err),
    }

    let _ = fs::remove_file(path);
}

fn run_c(shell: &mut Phase1Shell, args: &[String]) {
    if args.is_empty() {
        println!("Usage: gcc <file.c> | gcc \"code\"");
        return;
    }

    let code = if args.len() == 1 && args[0].ends_with(".c") {
        match shell.kernel.vfs.cat(&args[0]) {
            Ok(content) => content,
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
    } else {
        args.join(" ")
    };

    let stem = format!("phase1_{}_{}", process::id(), now_unix());
    let src = std::env::temp_dir().join(format!("{}.c", stem));
    let bin = std::env::temp_dir().join(stem);

    if let Err(err) = fs::write(&src, code) {
        println!("Failed to write temp C file: {}", err);
        return;
    }

    let compiler = if command_exists("cc") { "cc" } else { "gcc" };
    let mut compile = Command::new(compiler);
    compile.arg(&src).arg("-O2").arg("-o").arg(&bin);

    match run_with_timeout(compile, Duration::from_secs(8)) {
        Ok(output) if output.status.success() => {
            let run = Command::new(&bin);
            match run_with_timeout(run, Duration::from_secs(5)) {
                Ok(output) => {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                }
                Err(err) => println!("Execution failed: {}", err),
            }
        }
        Ok(output) => {
            println!("Compilation failed");
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Err(err) => println!("Compilation failed: {}", err),
    }

    let _ = fs::remove_file(src);
    let _ = fs::remove_file(bin);
}

fn one_arg<F>(args: &[String], usage: &str, mut f: F)
where
    F: FnMut(&str),
{
    if let Some(value) = args.first() {
        f(value);
    } else {
        println!("Usage: {}", usage);
    }
}

fn parse_u64(raw: &str) -> Option<u64> {
    if let Some(hex) = raw.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).ok()
    } else {
        raw.parse().ok()
    }
}

fn is_safe_command_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
}

fn command_exists(name: &str) -> bool {
    Command::new(name)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn run_with_input(mut cmd: Command, input: &str, timeout: Duration) -> io::Result<Output> {
    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input.as_bytes())?;
    }

    wait_with_timeout(child, timeout)
}

fn run_with_timeout(mut cmd: Command, timeout: Duration) -> io::Result<Output> {
    let child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    wait_with_timeout(child, timeout)
}

fn wait_with_timeout(mut child: std::process::Child, timeout: Duration) -> io::Result<Output> {
    let started = Instant::now();

    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output();
        }

        if started.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::new(io::ErrorKind::TimedOut, "command timed out"));
        }

        thread::sleep(Duration::from_millis(25));
    }
}
