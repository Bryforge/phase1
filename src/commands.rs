use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::browser::Browser;
use crate::kernel::{Kernel, VfsNode, VERSION};
use crate::man;
use crate::ned;
use crate::network::NetworkStack;
use crate::registry;

pub struct Phase1Shell {
    pub kernel: Kernel,
    pub network: NetworkStack,
    pub history: VecDeque<String>,
    pub plugins_dir: PathBuf,
    pub env: HashMap<String, String>,
}

impl Phase1Shell {
    pub fn new() -> Self {
        let mut shell = Self {
            kernel: Kernel::new(),
            network: NetworkStack::new(),
            history: VecDeque::with_capacity(512),
            plugins_dir: PathBuf::from("plugins"),
            env: HashMap::with_capacity(16),
        };

        shell
            .env
            .insert("PATH".to_string(), "/bin:/usr/bin:/plugins".to_string());
        shell.env.insert("USER".to_string(), "root".to_string());
        shell.env.insert("HOME".to_string(), "/home".to_string());
        shell
            .env
            .insert("SHELL".to_string(), "phase1".to_string());
        shell
            .env
            .insert("TERM".to_string(), "xterm-256color".to_string());

        if let Err(err) = fs::create_dir_all(&shell.plugins_dir) {
            eprintln!("plugin directory warning: {}", err);
        }
        shell.ensure_example_plugins();
        shell
    }

    pub fn user(&self) -> &str {
        &self.kernel.scheduler.current_user
    }

    pub fn push_history(&mut self, line: &str) {
        self.history.push_back(line.to_string());
        if self.history.len() > 512 {
            self.history.pop_front();
        }
    }

    pub fn expand_env(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch != '$' {
                out.push(ch);
                continue;
            }

            if chars.peek() == Some(&'{') {
                chars.next();
                let mut key = String::new();
                for next in chars.by_ref() {
                    if next == '}' {
                        break;
                    }
                    key.push(next);
                }
                if let Some(value) = self.env.get(&key) {
                    out.push_str(value);
                }
                continue;
            }

            let mut key = String::new();
            while let Some(&next) = chars.peek() {
                if next.is_ascii_alphanumeric() || next == '_' {
                    key.push(next);
                    chars.next();
                } else {
                    break;
                }
            }

            if key.is_empty() {
                out.push('$');
            } else if let Some(value) = self.env.get(&key) {
                out.push_str(value);
            }
        }

        out
    }

    pub fn cmd_cd(&mut self, dir: Option<&str>) {
        let target = dir.unwrap_or_else(|| self.env.get("HOME").map(String::as_str).unwrap_or("/home"));
        let path = self.kernel.vfs.resolve_path(target);
        match self.kernel.vfs.get_node(&path) {
            Some(VfsNode::Dir { .. }) => self.kernel.vfs.cwd = path,
            Some(_) => println!("cd: not a directory"),
            None => println!("cd: no such directory"),
        }
    }

    fn ensure_example_plugins(&self) {
        let example = r#"import sys

data = {}
for line in sys.stdin:
    if "=" in line:
        key, value = line.strip().split("=", 1)
        data[key] = value

print(f"plugin={data.get('COMMAND', 'unknown')}")
print(f"user={data.get('USER', 'unknown')}")
print(f"cwd={data.get('CWD', '/')}")
print("status=ok")
"#;
        for file in ["hello.py", "demo.py"] {
            let path = self.plugins_dir.join(file);
            if !path.exists() {
                let _ = fs::write(path, example);
            }
        }
    }

    fn list_plugins(&self) -> String {
        let mut plugins = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.plugins_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("py") {
                    if let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) {
                        if is_safe_name(name) {
                            plugins.push(name.to_string());
                        }
                    }
                }
            }
        }
        plugins.sort();
        if plugins.is_empty() {
            "no plugins found\n".to_string()
        } else {
            format!("{}\n", plugins.join("\n"))
        }
    }

    fn try_plugin(&mut self, name: &str, args: &[String]) -> bool {
        if !is_safe_name(name) {
            return false;
        }
        let path = self.plugins_dir.join(format!("{}.py", name));
        if !path.exists() {
            return false;
        }
        if host_tools_blocked() {
            println!("{}", crate::policy::host_denial_message("plugin"));
            return true;
        }

        self.kernel
            .audit
            .record(format!("plugin.exec name={} argc={}", name, args.len()));
        let context = format!(
            "COMMAND={}\nARGS={}\nUSER={}\nCWD={}\nVERSION={}\n",
            name,
            args.join(" "),
            self.user(),
            self.kernel.vfs.cwd.display(),
            VERSION
        );

        let mut cmd = Command::new("python3");
        cmd.arg(path);
        match run_with_input(cmd, &context, Duration::from_secs(5)) {
            Ok(output) => print_output(output),
            Err(err) => eprintln!("plugin failed: {}", err),
        }
        true
    }
}

impl Default for Phase1Shell {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_line(line: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();
    let mut quote: Option<char> = None;

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
            None if ch == '#' && current.is_empty() => break,
            None if ch == '\\' => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
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
    let command = registry::canonical_name(cmd).unwrap_or(cmd);

    match command {
        "help" => print!("{}", registry::command_map()),
        "complete" => print_completions(args.first().map(String::as_str)),
        "capabilities" => print!("{}", registry::capabilities_report()),
        "dash" => print!("{}", dashboard(shell, args)),
        "version" => println!("phase1 {}", VERSION),
        "clear" => print!("\x1b[2J\x1b[H"),
        "exit" => {
            println!("shutdown: phase1 {}", VERSION);
            std::process::exit(0);
        }
        "pwd" => println!("{}", shell.kernel.vfs.cwd.display()),
        "cd" => shell.cmd_cd(args.first().map(String::as_str)),
        "ls" => {
            let long = args
                .iter()
                .any(|arg| matches!(arg.as_str(), "-l" | "-la" | "-al"));
            let path = args
                .iter()
                .find(|arg| !arg.starts_with('-'))
                .map(String::as_str);
            print!("{}", shell.kernel.vfs.ls(path, long));
        }
        "cat" => match args.first() {
            Some(path) => match shell.kernel.sys_read(path) {
                Ok(content) => print!("{}", content),
                Err(err) => println!("cat: {}", err),
            },
            None => println!("usage: cat <file>"),
        },
        "mkdir" => one_arg(args, "mkdir <dir>", |path| {
            report(shell.kernel.vfs.mkdir(path));
        }),
        "touch" => one_arg(args, "touch <file>", |path| {
            report(shell.kernel.vfs.touch(path));
        }),
        "rm" => one_arg(args, "rm <path>", |path| {
            report(shell.kernel.vfs.rm(path));
        }),
        "cp" => two_args(args, "cp <src> <dst>", |src, dst| {
            report(shell.kernel.vfs.cp(src, dst));
        }),
        "mv" => two_args(args, "mv <src> <dst>", |src, dst| {
            report(shell.kernel.vfs.mv(src, dst));
        }),
        "tree" => print!("{}", shell.kernel.vfs.tree()),
        "echo" => handle_echo(shell, args),
        "ps" => print!("{}", shell.kernel.scheduler.ps()),
        "top" => print!("{}", shell.kernel.scheduler.top()),
        "jobs" => print!("{}", shell.kernel.scheduler.jobs()),
        "spawn" => spawn(shell, args),
        "kill" => println!("{}", shell.kernel.sys_kill(args.first().map(String::as_str))),
        "nice" => {
            let priority = args.get(1).and_then(|p| p.parse::<i32>().ok());
            println!(
                "{}",
                shell
                    .kernel
                    .scheduler
                    .nice(args.first().map(String::as_str), priority)
            );
        }
        "fg" => println!(
            "{}",
            shell
                .kernel
                .scheduler
                .set_background(args.first().map(String::as_str), false)
        ),
        "bg" => println!(
            "{}",
            shell
                .kernel
                .scheduler
                .set_background(args.first().map(String::as_str), true)
        ),
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
            if args.is_empty() && !host_tools_blocked() {
                println!("usage: wifi-connect <ssid> [password]");
            } else {
                println!(
                    "{}",
                    shell.network.wifi_connect(
                        args.first().map(String::as_str).unwrap_or(""),
                        args.get(1).map(String::as_str)
                    )
                );
            }
        }
        "ping" => one_arg(args, "ping <host>", |host| {
            print!("{}", shell.network.ping(host));
        }),
        "nmcli" => print!("{}", shell.network.nmcli()),
        "browser" => {
            if host_tools_blocked() {
                println!("{}", crate::policy::host_denial_message("browser"));
            } else {
                println!(
                    "{}",
                    Browser::new().browse(args.first().map(String::as_str).unwrap_or("about"))
                );
            }
        }
        "python" => run_python(shell, args),
        "gcc" => run_c(shell, args),
        "plugins" => print!("{}", shell.list_plugins()),
        "ned" => one_arg(args, "ned <file>", |path| {
            ned::edit(&mut shell.kernel.vfs, path);
        }),
        "lspci" => print!("{}", shell.kernel.pcie.lspci()),
        "pcie" => print!("{}", shell.kernel.pcie.pcie_info()),
        "cr3" => println!("CR3=0x{:x}", shell.kernel.scheduler.get_cr3()),
        "loadcr3" => match args.first().and_then(|value| parse_u64(value)) {
            Some(value) => match shell.kernel.scheduler.load_cr3(value) {
                Ok(()) => println!("CR3 loaded: 0x{:x}", value),
                Err(err) => println!("loadcr3: {}", err),
            },
            None => println!("usage: loadcr3 <hex|decimal>"),
        },
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
            _ => println!("usage: pcide on|off"),
        },
        "free" => println!("MemTotal: 4194304 kB\nMemFree: 2097152 kB\nMemUsed: 2097152 kB"),
        "df" => println!("Filesystem 1K-blocks Used Available Mounted\nphase1fs   1048576   4    1048572   /"),
        "dmesg" => println!(
            "[0.000000] phase1 {} boot\n[0.012345] vfs mounted\n[0.034567] scheduler online",
            VERSION
        ),
        "vmstat" => println!("procs memory system\nr=1 b=0 free=2097152 in=10 cs=25"),
        "uname" => println!("phase1 {} terminal-os-sim rust", VERSION),
        "date" => println!("{}", now_unix()),
        "uptime" => println!("up {} seconds", shell.kernel.uptime().as_secs()),
        "hostname" => println!("phase1"),
        "audit" => print!("{}", shell.kernel.audit.dump()),
        "env" => print_env(shell),
        "export" => export_env(shell, args),
        "unset" => one_arg(args, "unset VAR", |key| {
            shell.env.remove(key);
        }),
        "whoami" => println!("{}", shell.user()),
        "id" => println!(
            "uid={}({}) gid={}({})",
            shell.kernel.scheduler.current_uid,
            shell.user(),
            shell.kernel.scheduler.current_uid,
            shell.user()
        ),
        "su" => {
            let user = args.first().map(String::as_str).unwrap_or("root");
            shell.kernel.scheduler.current_user = user.to_string();
            shell.kernel.scheduler.current_uid = if user == "root" { 0 } else { 1000 };
            shell.env.insert("USER".to_string(), user.to_string());
        }
        "history" => {
            for (idx, line) in shell.history.iter().enumerate() {
                println!("{:>4} {}", idx + 1, line);
            }
        }
        "sandbox" => println!("sandbox: VFS/processes are simulated; host commands are guarded by validation, timeouts, safe mode, and PHASE1_ALLOW_HOST_TOOLS=1."),
        "man" => match args.first() {
            Some(topic) => match man::get_man_page(topic) {
                Some(page) => println!("{}", page),
                None => println!("no manual entry for {}", topic),
            },
            None => println!("usage: man <command>"),
        },
        other => {
            if !shell.try_plugin(other, args) {
                println!("command not found: {}", other);
            }
        }
    }
    let _ = io::stdout().flush();
}

fn dashboard(shell: &mut Phase1Shell, args: &[String]) -> String {
    shell.network.refresh();
    let compact = args.iter().any(|arg| arg == "--compact");
    let uptime = shell.kernel.uptime().as_secs();
    let cwd = shell.kernel.vfs.cwd.display();
    let ps_output = shell.kernel.scheduler.ps();
    let process_count = ps_output.lines().skip(1).count();
    let jobs_output = shell.kernel.scheduler.jobs();
    let job_count = if jobs_output.trim() == "no background jobs" {
        0
    } else {
        jobs_output.lines().count()
    };
    let iface_count = shell
        .network
        .ifconfig()
        .lines()
        .filter(|line| line.contains(": flags=<"))
        .count();
    let audit_tail = shell
        .kernel
        .audit
        .dump()
        .lines()
        .last()
        .unwrap_or("audit log empty")
        .to_string();
    let pcie_count = shell.kernel.pcie.lspci().lines().count();
    let cr4 = shell.kernel.scheduler.cr4();

    if compact {
        format!(
            "PHASE1 DASHBOARD v{}\nCORE  user={} uptime={}s mode=operator\nPROC  tasks={} bg={}\nVFS   cwd={} mounts=/,/proc,/dev,/tmp,/var/log\nNET   interfaces={} safety={}\nHW    cr3=0x{:x} {} pcie={}\nAUDIT latest={}\n",
            VERSION,
            shell.user(),
            uptime,
            process_count,
            job_count,
            cwd,
            iface_count,
            if host_tools_blocked() {
                "safe-mode"
            } else {
                "host-enabled"
            },
            shell.kernel.scheduler.get_cr3(),
            cr4,
            pcie_count,
            audit_tail
        )
    } else {
        format!(
            "PHASE1 // OPERATOR DASHBOARD v{}\n\nCORE\n  user      {}\n  uptime    {}s\n  mode      operator\n\nPROC\n  tasks     {}\n  bg jobs   {}\n\nVFS\n  cwd       {}\n  mounts    / /proc /dev /tmp /var/log\n\nNET\n  ifaces    {}\n  safety    {}\n\nHW\n  cr3       0x{:x}\n  cr4       {}\n  pcie      {} devices\n\nAUDIT\n  latest    {}\n",
            VERSION,
            shell.user(),
            uptime,
            process_count,
            job_count,
            cwd,
            iface_count,
            if host_tools_blocked() {
                "safe-mode"
            } else {
                "host-enabled"
            },
            shell.kernel.scheduler.get_cr3(),
            cr4,
            pcie_count,
            audit_tail
        )
    }
}

fn handle_echo(shell: &mut Phase1Shell, args: &[String]) {
    if let Some(idx) = args.iter().position(|arg| arg == ">" || arg == ">>") {
        if idx + 1 >= args.len() {
            println!("echo: missing redirect target");
            return;
        }
        let append = args[idx] == ">>";
        let mut text = args[..idx].join(" ");
        text.push('\n');
        if let Err(err) = shell.kernel.sys_write(&args[idx + 1], &text, append) {
            println!("echo: {}", err);
        }
    } else {
        println!("{}", args.join(" "));
    }
}

fn spawn(shell: &mut Phase1Shell, args: &[String]) {
    if args.is_empty() {
        println!("usage: spawn <name> [args...] [--background]");
        return;
    }
    let background = args.iter().any(|arg| arg == "--background" || arg == "&");
    let filtered: Vec<_> = args
        .iter()
        .filter(|arg| *arg != "--background" && *arg != "&")
        .cloned()
        .collect();
    let name = filtered
        .first()
        .cloned()
        .unwrap_or_else(|| "process".to_string());
    let cmdline = filtered.join(" ");
    match shell.kernel.sys_spawn(&name, &cmdline, background) {
        Ok(pid) => println!("spawned pid {}", pid),
        Err(err) => println!("spawn: {}", err),
    }
}

fn run_python(shell: &mut Phase1Shell, args: &[String]) {
    if host_tools_blocked() {
        println!("{}", crate::policy::host_denial_message("python"));
        return;
    }
    if args.is_empty() || (args[0] == "-c" && args.len() < 2) {
        println!("usage: python <file.py> | python -c <code>");
        return;
    }
    shell.kernel.audit.record("host.python".to_string());
    let code = if args[0] == "-c" {
        args[1..].join(" ")
    } else {
        match shell.kernel.sys_read(&args[0]) {
            Ok(content) => content,
            Err(err) => {
                println!("python: {}", err);
                return;
            }
        }
    };
    let mut cmd = Command::new("python3");
    cmd.arg("-c").arg(code);
    match run_command(cmd, Duration::from_secs(5)) {
        Ok(output) => print_output(output),
        Err(err) => println!("python: {}", err),
    }
}

fn run_c(shell: &mut Phase1Shell, args: &[String]) {
    if host_tools_blocked() {
        println!("{}", crate::policy::host_denial_message("gcc"));
        return;
    }
    if args.is_empty() {
        println!("usage: gcc <file.c> | gcc <code>");
        return;
    }
    shell.kernel.audit.record("host.c".to_string());
    let code = if args.len() == 1 && args[0].ends_with(".c") {
        match shell.kernel.sys_read(&args[0]) {
            Ok(content) => content,
            Err(err) => {
                println!("gcc: {}", err);
                return;
            }
        }
    } else {
        args.join(" ")
    };
    let Some(compiler) = find_compiler() else {
        println!("gcc: no cc/gcc/clang found on host");
        return;
    };
    let nonce = unique_nonce();
    let source = std::env::temp_dir().join(format!("phase1_{}.c", nonce));
    let binary = std::env::temp_dir().join(format!("phase1_{}", nonce));
    if let Err(err) = fs::write(&source, code) {
        println!("gcc: {}", err);
        return;
    }
    let mut compile = Command::new(compiler);
    compile
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-O0")
        .arg(&source)
        .arg("-o")
        .arg(&binary);
    match run_command(compile, Duration::from_secs(10)) {
        Ok(output) if output.status.success() => {
            match run_command(Command::new(&binary), Duration::from_secs(5)) {
                Ok(output) => print_output(output),
                Err(err) => println!("run: {}", err),
            }
        }
        Ok(output) => print_output(output),
        Err(err) => println!("gcc: {}", err),
    }
    let _ = fs::remove_file(source);
    let _ = fs::remove_file(binary);
}

fn print_completions(prefix: Option<&str>) {
    let prefix = prefix.unwrap_or("");
    let matches = registry::completions(prefix);
    if matches.is_empty() {
        println!("complete: no matches for '{}'", prefix);
    } else {
        println!("{}", matches.join(" "));
    }
}

fn print_env(shell: &Phase1Shell) {
    let mut keys: Vec<_> = shell.env.keys().cloned().collect();
    keys.sort();
    for key in keys {
        println!("{}={}", key, shell.env[&key]);
    }
}

fn export_env(shell: &mut Phase1Shell, args: &[String]) {
    if args.is_empty() {
        println!("usage: export VAR=value");
        return;
    }
    for raw in args {
        if let Some((key, value)) = raw.split_once('=') {
            shell.env.insert(key.to_string(), value.to_string());
        } else {
            println!("export: expected VAR=value");
        }
    }
}

fn one_arg<F>(args: &[String], usage: &str, mut action: F)
where
    F: FnMut(&str),
{
    match args.first() {
        Some(value) => action(value),
        None => println!("usage: {}", usage),
    }
}

fn two_args<F>(args: &[String], usage: &str, mut action: F)
where
    F: FnMut(&str, &str),
{
    if args.len() < 2 {
        println!("usage: {}", usage);
    } else {
        action(&args[0], &args[1]);
    }
}

fn report(result: Result<(), String>) {
    if let Err(err) = result {
        println!("error: {}", err);
    }
}

fn parse_u64(raw: &str) -> Option<u64> {
    raw.strip_prefix("0x")
        .and_then(|hex| u64::from_str_radix(hex, 16).ok())
        .or_else(|| raw.parse().ok())
}

fn is_safe_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-'))
}

fn host_tools_blocked() -> bool {
    !crate::policy::host_tools_allowed()
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn unique_nonce() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0)
}

fn find_compiler() -> Option<&'static str> {
    ["cc", "gcc", "clang"].into_iter().find(|name| {
        Command::new(name)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
    })
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
    drop(child.stdin.take());
    wait_child(child, timeout)
}

fn run_command(mut cmd: Command, timeout: Duration) -> io::Result<Output> {
    let child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    wait_child(child, timeout)
}

fn wait_child(mut child: std::process::Child, timeout: Duration) -> io::Result<Output> {
    let start = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output();
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::new(io::ErrorKind::TimedOut, "command timed out"));
        }
        thread::sleep(Duration::from_millis(25));
    }
}

fn print_output(output: Output) {
    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
}

#[cfg(test)]
mod tests {
    use super::parse_line;

    #[test]
    fn parses_quotes_and_redirect() {
        let expected = vec![
            "echo".to_string(),
            "hello world".to_string(),
            ">".to_string(),
            "out".to_string(),
        ];
        assert_eq!(parse_line("echo 'hello world' > out").unwrap(), expected);
    }
}
