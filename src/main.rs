// src/main.rs — phase1 v3.1.0 Terminal OS Simulator

mod network;
mod kernel;
mod browser;
mod man;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{self, Command};
use std::os::unix::process::ExitStatusExt;
use std::thread;
use std::time::{Duration, Instant};

use chrono::prelude::*;

use kernel::Kernel;
use network::NetworkStack;
use browser::Browser;

const VERSION: &str = "3.1.0";
const BUILD_DATE: &str = "2026-05-04";

const ANSI_CLEAR: &str = "\x1b[2J\x1b[H";
const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_BLUE: &str = "\x1b[34m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_CYAN: &str = "\x1b[36m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_MAGENTA: &str = "\x1b[35m";

fn default_output() -> std::process::Output {
    std::process::Output {
        status: std::process::ExitStatus::from_raw(0),
        stdout: Vec::new(),
        stderr: Vec::new(),
    }
}

struct Phase1Shell {
    kernel: Kernel,
    network: NetworkStack,
    start_time: Instant,
    history: VecDeque<String>,
    plugins_dir: PathBuf,
    plugins_cache: HashSet<String>,
    env: HashMap<String, String>,
}

impl Phase1Shell {
    fn new() -> Self {
        let mut shell = Phase1Shell {
            kernel: Kernel::new(),
            network: NetworkStack::new(),
            start_time: Instant::now(),
            history: VecDeque::with_capacity(300),
            plugins_dir: PathBuf::from("plugins"),
            plugins_cache: HashSet::with_capacity(8),
            env: HashMap::with_capacity(8),
        };

        shell.env.insert("PATH".to_string(), "/bin:/usr/bin:/plugins".to_string());
        shell.env.insert("USER".to_string(), "root".to_string());
        shell.env.insert("HOME".to_string(), "/home".to_string());
        shell.env.insert("SHELL".to_string(), "phase1".to_string());
        shell.env.insert("TERM".to_string(), "xterm-256color".to_string());

        if !shell.plugins_dir.exists() {
            let _ = fs::create_dir_all(&shell.plugins_dir);
        }

        let example = r#"import sys
data = {}
for line in sys.stdin:
    if '=' in line:
        k, v = line.strip().split('=', 1)
        data[k] = v
print(f"Hello from Python plugin '{data.get('COMMAND', 'unknown')}'!")
print(f"Running as user: {data.get('USER', 'unknown')}")
print(f"Current directory: {data.get('CWD', '/')}")
print("Plugin executed successfully!")"#;

        let hello_path = shell.plugins_dir.join("hello.py");
        let demo_path = shell.plugins_dir.join("demo.py");
        let _ = fs::write(&hello_path, example);
        let _ = fs::write(&demo_path, example);

        shell.refresh_plugins_cache();

        shell
    }

    fn refresh_plugins_cache(&mut self) {
        self.plugins_cache.clear();
        if let Ok(entries) = fs::read_dir(&self.plugins_dir) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "py" {
                        if let Some(stem) = entry.path().file_stem() {
                            if let Some(name) = stem.to_str() {
                                self.plugins_cache.insert(name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    fn print_boot() {
        println!("{}", ANSI_CLEAR);
        println!("{}================================================================================{}", ANSI_GREEN, ANSI_RESET);
        println!("{}/                    phase1 v3.1.0  —  Terminal OS Simulator                   /{}", ANSI_GREEN, ANSI_RESET);
        println!("{}/  VFS + Scheduler + Editors + Python + C + Networking + Browser + Man Pages   /{}", ANSI_GREEN, ANSI_RESET);
        println!("{}================================================================================{}", ANSI_GREEN, ANSI_RESET);
        println!("{}[    0.000000] phase1 kernel booted{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.012345] In-memory VFS and proc mounted{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.034567] Preemptive scheduler ready{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.067890] Built-in nano/vi and C compiler support loaded{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.089012] Python plugin system active{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.112345] Cross-platform network stack ready{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.145678] Comprehensive man pages integrated{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.178901] Async VFS operations enabled{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.200000] Boot complete — type 'help'{}", ANSI_GREEN, ANSI_RESET);
        println!();
    }

    fn expand_env(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len() + 64);
        let mut remaining = text;
        while let Some(pos) = remaining.find('$') {
            result.push_str(&remaining[..pos]);
            remaining = &remaining[pos + 1..];
            let end = remaining.find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(remaining.len());
            let var = &remaining[..end];
            if let Some(val) = self.env.get(var) {
                result.push_str(val);
            }
            remaining = &remaining[end..];
        }
        result.push_str(remaining);
        result
    }

    fn try_plugin(&self, cmd: &str, args: &[&str]) -> bool {
        if !self.plugins_cache.contains(cmd) {
            return false;
        }
        let plugin_path = self.plugins_dir.join(format!("{}.py", cmd));
        let context_str = format!(
            "COMMAND={}\nARGS={}\nUSER={}\nCWD={}\nPID={}\nHOME={}\n",
            cmd,
            args.join(" "),
            self.kernel.scheduler.current_user,
            self.kernel.vfs.cwd.to_str().unwrap_or("/"),
            process::id(),
            self.env.get("HOME").unwrap_or(&"/home".to_string())
        );

        match Command::new("python3")
            .arg(&plugin_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(context_str.as_bytes());
                }
                let output = child.wait_with_output().unwrap_or_else(|_| default_output());
                let stdout = String::from_utf8_lossy(&output.stdout);
                if !stdout.trim().is_empty() {
                    println!("{}{}{}", ANSI_MAGENTA, stdout.trim(), ANSI_RESET);
                }
                true
            }
            Err(_) => false,
        }
    }

    fn cmd_nano(&mut self, file: Option<&str>) {
        if let Some(f) = file {
            let content = match self.kernel.vfs.cat(f) {
                Ok(c) => c,
                Err(_) => String::new(),
            };
            println!("nano: editing {} (type lines, end with single . on new line to save/exit, or :q to quit)", f);

            let mut builder = content;

            loop {
                let mut line = String::new();
                if io::stdin().read_line(&mut line).is_err() {
                    break;
                }
                let line = line.trim_end().to_string();
                if line == "." {
                    match self.kernel.vfs.write_file(f, &builder, false) {
                        Ok(_) => println!("Saved {}", f),
                        Err(e) => println!("{}Save failed: {}{}", ANSI_RED, e, ANSI_RESET),
                    }
                    return;
                }
                if line == ":q" {
                    println!("Exited without saving");
                    return;
                }
                builder.push_str(&line);
                builder.push('\n');
            }
        } else {
            println!("Usage: nano <file>");
        }
    }

    fn cmd_vi(&mut self, file: Option<&str>) {
        if let Some(f) = file {
            println!("vi: basic mode for {} (nano compatible fallback)", f);
            self.cmd_nano(Some(f));
        } else {
            println!("Usage: vi <file>");
        }
    }

    fn cmd_c_compile(&self, args: &[&str]) {
        if args.is_empty() {
            println!("Usage: gcc <file.c> or gcc \"C code here\"");
            return;
        }
        let source = if args[0].ends_with(".c") {
            match self.kernel.vfs.cat(args[0]) {
                Ok(c) => c,
                Err(_) => {
                    println!("{}Source file not found{}", ANSI_RED, ANSI_RESET);
                    return;
                }
            }
        } else {
            args.join(" ")
        };

        let tmp_dir = std::env::temp_dir();
        let c_file = tmp_dir.join("phase1_tmp.c");
        let bin_file = tmp_dir.join("phase1_tmp");

        if let Err(e) = fs::write(&c_file, &source) {
            println!("{}Failed to write temp C file: {}{}", ANSI_RED, e, ANSI_RESET);
            return;
        }

        let compiler = if Command::new("gcc").output().is_ok() {
            "gcc"
        } else if Command::new("clang").output().is_ok() {
            "clang"
        } else {
            println!("{}No C compiler (gcc/clang) found on host{}", ANSI_RED, ANSI_RESET);
            let _ = fs::remove_file(&c_file);
            return;
        };

        match Command::new(compiler)
            .arg(&c_file)
            .arg("-o")
            .arg(&bin_file)
            .status()
        {
            Ok(status) if status.success() => {
                println!("Compiled successfully");
                let output = Command::new(&bin_file).output().unwrap_or_else(|_| default_output());
                println!("{}{}{}", ANSI_GREEN, String::from_utf8_lossy(&output.stdout), ANSI_RESET);
            }
            _ => println!("{}Compilation failed{}", ANSI_RED, ANSI_RESET),
        }

        let _ = fs::remove_file(&c_file);
        let _ = fs::remove_file(&bin_file);
    }

    fn cmd_python(&self, args: &[&str]) {
        if args.is_empty() {
            println!("Usage: python [-c \"code\"] <script.py>");
            return;
        }

        let code = if args[0] == "-c" {
            // Inline code support
            if args.len() > 1 {
                args[1..].join(" ")
            } else {
                String::new()
            }
        } else {
            // File mode
            match self.kernel.vfs.cat(args[0]) {
                Ok(content) => content,
                Err(_) => {
                    println!("{}File not found in VFS: {}{}", ANSI_RED, args[0], ANSI_RESET);
                    return;
                }
            }
        };

        let tmp_path = format!(
            "/tmp/phase1_py_{}.py",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        if let Err(e) = fs::write(&tmp_path, code) {
            println!("{}Failed to write Python script: {}{}", ANSI_RED, e, ANSI_RESET);
            return;
        }

        let output = Command::new("python3")
            .arg(&tmp_path)
            .output()
            .unwrap_or_else(|_| default_output());

        println!("{}", String::from_utf8_lossy(&output.stdout));
        let _ = fs::remove_file(&tmp_path);
    }

    fn run(&mut self) {
        Self::print_boot();
        self.cmd_cd(Some("/home"));
        println!("{}phase1 v{} ready. Type 'help' for commands.{}", ANSI_GREEN, VERSION, ANSI_RESET);

        let mut input = String::with_capacity(256);

        loop {
            let uptime_secs = self.start_time.elapsed().as_secs();
            self.kernel.tick(uptime_secs);

            print!("{}@phase1{}:{}$ ", ANSI_CYAN, ANSI_RESET, self.kernel.vfs.cwd.display());
            let _ = io::stdout().flush();

            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }

            let line = input.trim();
            if line.is_empty() {
                continue;
            }

            self.history.push_back(line.to_string());
            if self.history.len() > 300 {
                self.history.pop_front();
            }

            let expanded = self.expand_env(line);
            let parts: Vec<&str> = expanded.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let cmd = parts[0];
            let args = &parts[1..];

            match cmd {
                "exit" | "quit" | "shutdown" | "poweroff" => {
                    println!("{}Shutting down phase1 v{}... Goodbye!{}", ANSI_YELLOW, VERSION, ANSI_RESET);
                    break;
                }
                "help" => self.cmd_help(),
                "man" => self.cmd_man(args.first().copied()),
                "ps" => println!("{}", self.kernel.scheduler.ps()),
                "top" => {
                    println!("{}", self.kernel.scheduler.top());
                    thread::sleep(Duration::from_secs(3));
                }
                "free" | "mem" => self.cmd_free(),
                "kill" => println!("{}", self.kernel.scheduler.kill(args.first().copied())),
                "nice" => println!(
                    "{}",
                    self.kernel
                        .scheduler
                        .nice(args.first().copied(), args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0))
                ),
                "spawn" => {
                    let name = args.get(0).unwrap_or(&"anon");
                    match self.kernel.scheduler.spawn(name, process::id(), &args.join(" "), 2048, false, 0) {
                        Some(pid) => println!("Spawned process {}", pid),
                        None => println!("{}Process table full{}", ANSI_RED, ANSI_RESET),
                    }
                }
                "lspci" => println!("{}", self.kernel.pcie.lspci()),
                "pcie" => println!("{}", self.kernel.pcie.pcie_info()),
                "cr3" => println!("Current CR3: 0x{:016x}", self.kernel.scheduler.get_cr3()),
                "loadcr3" => {
                    if let Some(val_str) = args.first() {
                        let val = if val_str.starts_with("0x") {
                            u64::from_str_radix(&val_str[2..], 16).unwrap_or(0)
                        } else {
                            val_str.parse().unwrap_or(0)
                        };
                        self.kernel.scheduler.load_cr3(val);
                        println!("CR3 loaded");
                    } else {
                        println!("Usage: loadcr3 <value>");
                    }
                }
                "cr4" => println!("{}", self.kernel.scheduler.cr4()),
                "pcide" => {
                    if let Some(arg) = args.first() {
                        match *arg {
                            "on" | "1" | "enable" => {
                                self.kernel.scheduler.set_pcide(true);
                                println!("PCIDE enabled");
                            }
                            "off" | "0" | "disable" => {
                                self.kernel.scheduler.set_pcide(false);
                                println!("PCIDE disabled");
                            }
                            _ => println!("Usage: pcide <on|off>"),
                        }
                    }
                }
                "df" => println!("Filesystem     1K-blocks    Used Available Use% Mounted on\nphase1-vfs     4194304   1048576   3145728  25% /"),
                "whoami" => println!("{}", self.kernel.scheduler.current_user),
                "id" => println!(
                    "uid={}({}) gid=0(root) groups=0(root)",
                    self.kernel.scheduler.current_uid, self.kernel.scheduler.current_user
                ),
                "ls" => {
                    let long = args.contains(&"-l");
                    let path = args.iter().find(|&&x| x != "-l").copied();
                    println!("{}", self.kernel.vfs.ls(path, long));
                }
                "cd" => self.cmd_cd(args.first().copied()),
                "pwd" => println!("{}", self.kernel.vfs.cwd.display()),
                "cat" => match self.kernel.vfs.cat(args.first().unwrap_or(&"")) {
                    Ok(c) => println!("{}", c),
                    Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                },
                "mkdir" => {
                    let dir = args.first().unwrap_or(&"");
                    match self.kernel.vfs.mkdir(dir) {
                        Ok(_) => println!("Directory created"),
                        Err(e) => println!("{}mkdir failed: {}{}", ANSI_RED, e, ANSI_RESET),
                    }
                }
                "touch" => {
                    let file = args.first().unwrap_or(&"");
                    match self.kernel.vfs.touch(file) {
                        Ok(_) => println!("File touched"),
                        Err(e) => println!("{}touch failed: {}{}", ANSI_RED, e, ANSI_RESET),
                    }
                }
                "rm" => {
                    let mut target = "";
                    for &arg in args {
                        if !arg.starts_with('-') {
                            target = arg;
                            break;
                        }
                    }
                    if target.is_empty() {
                        println!("Usage: rm [-r] <file or directory>");
                    } else {
                        match self.kernel.vfs.rm(target) {
                            Ok(_) => println!("Removed"),
                            Err(e) => println!("{}rm failed: {}{}", ANSI_RED, e, ANSI_RESET),
                        }
                    }
                }
                "cp" => {
                    if args.len() >= 2 {
                        match self.kernel.vfs.cp(args[0], args[1]) {
                            Ok(_) => println!("Copied"),
                            Err(e) => println!("{}cp failed: {}{}", ANSI_RED, e, ANSI_RESET),
                        }
                    } else {
                        println!("Usage: cp <source> <destination>");
                    }
                }
                "mv" => {
                    if args.len() >= 2 {
                        match self.kernel.vfs.mv(args[0], args[1]) {
                            Ok(_) => println!("Moved"),
                            Err(e) => println!("{}mv failed: {}{}", ANSI_RED, e, ANSI_RESET),
                        }
                    } else {
                        println!("Usage: mv <source> <destination>");
                    }
                }
                "echo" => {
                    if let Some(redirect_pos) = args.iter().position(|&x| x == ">" || x == ">>") {
                        if redirect_pos + 1 < args.len() {
                            let content = args[0..redirect_pos].join(" ") + "\n";
                            let file = args[redirect_pos + 1];
                            let append = args[redirect_pos] == ">>";
                            match self.kernel.vfs.write_file(file, &content, append) {
                                Ok(_) => println!("Redirected to {}", file),
                                Err(e) => println!("{}Redirect error: {}{}", ANSI_RED, e, ANSI_RESET),
                            }
                        }
                    } else {
                        println!("{}", args.join(" "));
                    }
                }
                "clear" => println!("{}", ANSI_CLEAR),
                "env" => self.cmd_env(),
                "export" => self.cmd_export(args),
                "unset" => self.cmd_unset(args.first().copied()),
                "python" | "py" => self.cmd_python(args),
                "plugin" | "plugins" => {
                    self.cmd_plugins();
                    self.refresh_plugins_cache();
                }
                "nano" => self.cmd_nano(args.first().copied()),
                "vi" => self.cmd_vi(args.first().copied()),
                "gcc" | "cc" | "c-compile" => self.cmd_c_compile(args),
                "browser" => {
                    let target = args.first().copied().unwrap_or("about");
                    println!("{}", Browser::new().browse(target));
                }
                "jobs" => println!("{}", self.kernel.scheduler.jobs()),
                "su" => self.cmd_su(args.first().copied()),
                "dmesg" => self.cmd_dmesg(),
                "vmstat" => self.cmd_vmstat(),
                "history" => self.cmd_history(),
                "uname" => println!("Linux phase1 6.8.0-phase1-v3 #1 SMP {} x86_64 GNU/Linux", Local::now().format("%Y-%m-%d")),
                "date" => println!("{}", Local::now().format("%a %b %d %H:%M:%S %Z %Y")),
                "uptime" => println!(
                    "{} up {} load average: 0.12, 0.15, 0.10",
                    Local::now().format("%H:%M:%S"),
                    self.start_time.elapsed().as_secs()
                ),
                "hostname" => println!("phase1-virtual"),
                "ifconfig" => println!("{}", self.network.ifconfig()),
                "iwconfig" => println!("{}", self.network.iwconfig()),
                "wifi-scan" => println!("{}", self.network.wifi_scan()),
                "wifi-connect" => {
                    if let Some(ssid) = args.first() {
                        let pw = args.get(1);
                        println!("{}", self.network.wifi_connect(ssid, pw.copied()));
                    } else {
                        println!("Usage: wifi-connect <SSID> [password]");
                    }
                }
                "ping" => {
                    let host = args.first().copied().unwrap_or("8.8.8.8");
                    println!("{}", self.network.ping(host));
                }
                "nmcli" => println!("{}", self.network.nmcli()),
                "sandbox" | "nsinfo" => println!("Running in pure-Rust userspace sandbox."),
                "version" => println!("phase1 v{} — built {}", VERSION, BUILD_DATE),
                "tree" => self.cmd_tree(),
                _ => {
                    if !self.try_plugin(cmd, args) {
                        println!("{}command not found: {}{}   (help for list)", ANSI_RED, cmd, ANSI_RESET);
                    }
                }
            }
        }
    }

    fn cmd_help(&self) {
        println!("phase1 v3.1.0 — Terminal OS Commands");
        println!("Filesystem: ls [-l] cd pwd cat mkdir touch rm [-r] cp mv echo [> >>]");
        println!("Editors: nano <file> vi <file>");
        println!("C support: gcc <file.c> or gcc \"code\"");
        println!("Python: python [-c \"code\"] <script.py>");
        println!("Process: ps top kill spawn nice jobs");
        println!("Hardware: lspci pcie cr3 loadcr3 cr4 pcide");
        println!("Network: ifconfig iwconfig wifi-scan wifi-connect ping nmcli");
        println!("Browser: browser <url> (or browser phase1 / about)");
        println!("Shell: env export unset history su python plugin");
        println!("System: free df uname date uptime dmesg vmstat tree sandbox version man");
        println!("Misc: clear exit");
    }

    fn cmd_cd(&mut self, dir: Option<&str>) {
        if let Some(d) = dir {
            let new_path = self.kernel.vfs.resolve_path(d);
            if self.kernel.vfs.get_node(&new_path).is_some() {
                self.kernel.vfs.cwd = new_path;
            } else {
                println!("{}cd: no such directory{}", ANSI_RED, ANSI_RESET);
            }
        } else {
            self.kernel.vfs.cwd = PathBuf::from("/home");
        }
    }

    fn cmd_free(&self) {
        println!("              total        used        free      shared  buff/cache   available");
        println!("Mem:       4194304     1048576     2097152      131072     1048576     3145728");
        println!("Swap:            0           0           0");
    }

    fn cmd_env(&self) {
        for (k, v) in &self.env {
            println!("{}={}", k, v);
        }
    }

    fn cmd_export(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!("Usage: export VAR=value");
            return;
        }
        for arg in args {
            if let Some(eq) = arg.find('=') {
                let key = &arg[0..eq];
                let value = &arg[eq + 1..];
                self.env.insert(key.to_string(), value.to_string());
            }
        }
    }

    fn cmd_unset(&mut self, var: Option<&str>) {
        if let Some(v) = var {
            self.env.remove(v);
        } else {
            println!("Usage: unset VAR");
        }
    }

    fn cmd_plugins(&self) {
        println!("Available Python plugins in ./plugins/:");
        if let Ok(entries) = fs::read_dir(&self.plugins_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |e| e == "py") {
                    println!("   • {}", entry.file_name().to_string_lossy());
                }
            }
        }
    }

    fn cmd_su(&mut self, user: Option<&str>) {
        if let Some(u) = user {
            self.kernel.scheduler.current_user = u.to_string();
            self.kernel.scheduler.current_uid = if u == "root" { 0 } else { 1000 };
            self.env.insert("USER".to_string(), u.to_string());
            println!("Switched to user: {}", u);
        } else {
            println!("Usage: su <username>");
        }
    }

    fn cmd_dmesg(&self) {
        println!("[    0.000000] phase1 kernel: virtual hardware detected");
        println!("[    0.012345] VFS mounted");
        println!("[    0.045678] Scheduler active");
        println!("[    0.078901] Editors and compiler loaded");
    }

    fn cmd_vmstat(&self) {
        println!(" procs -----------memory---------- ---swap-- -----io---- -system-- ------cpu-----");
        println!(" r  b   swpd   free   buff  cache   si   so    bi    bo   in   cs us sy id wa st");
        println!(" 1  0      0 2097152 131072 1048576    0    0     0     0  120  240  8  3 89  0  0");
    }

    fn cmd_history(&self) {
        for (i, entry) in self.history.iter().enumerate() {
            println!("{:3}  {}", i, entry);
        }
    }

    fn cmd_tree(&self) {
        println!("/");
        println!("├── bin");
        println!("├── dev");
        println!("├── etc");
        println!("├── home");
        println!("├── proc");
        println!("└── (plugins outside VFS)");
    }

    fn cmd_man(&self, topic: Option<&str>) {
        if let Some(t) = topic {
            if let Some(page) = man::get_man_page(t) {
                println!("{}", page);
            } else {
                println!("No manual entry for that command yet. Try 'man help' or 'help'.");
            }
        } else {
            println!("Usage: man <command>");
        }
    }
}

fn main() {
    let mut shell = Phase1Shell::new();
    shell.run();
}
