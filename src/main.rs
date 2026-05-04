// =====================================================
// phase1 v1.2.0 — Advanced Educational Embedded OS Simulator
// =====================================================
// Cross-platform (Linux + macOS) terminal-based OS simulator in Rust.
// Full in-memory VFS, preemptive scheduler, PCIe, CR3/CR4/PCID, Python plugins,
// real host networking (ifconfig/iwconfig/wifi-scan on both OSes), and more.
// Demonstrates kernel concepts safely in userspace.
// Kernel logic now separated into src/kernel.rs for modularity.

mod kernel;
mod network;

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{self, Command};
use std::os::unix::process::ExitStatusExt;
use std::thread;
use std::time::{Duration, Instant};

use chrono::prelude::*;

use kernel::*;

const VERSION: &str = "1.2.0";
const BUILD_DATE: &str = "2026-05-03";

const ANSI_CLEAR: &str = "\x1b[2J\x1b[H";
const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_BLUE: &str = "\x1b[34m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_CYAN: &str = "\x1b[36m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_MAGENTA: &str = "\x1b[35m";
const ANSI_WHITE: &str = "\x1b[37m";

struct Phase1Shell {
    kernel: Kernel,
    network: network::NetworkStack,
    start_time: Instant,
    history: VecDeque<String>,
    plugins_dir: PathBuf,
    env: HashMap<String, String>,
}

impl Phase1Shell {
    fn new() -> Self {
        let mut shell = Phase1Shell {
            kernel: Kernel::new(),
            network: network::NetworkStack::new(),
            start_time: Instant::now(),
            history: VecDeque::with_capacity(300),
            plugins_dir: PathBuf::from("plugins"),
            env: HashMap::new(),
        };

        shell.env.insert("PATH".to_string(), "/bin:/usr/bin:/plugins".to_string());
        shell.env.insert("USER".to_string(), "root".to_string());
        shell.env.insert("HOME".to_string(), "/home".to_string());
        shell.env.insert("SHELL".to_string(), "phase1".to_string());
        shell.env.insert("TERM".to_string(), "xterm-256color".to_string());

        if !shell.plugins_dir.exists() {
            let _ = fs::create_dir_all(&shell.plugins_dir);
        }

        let example_plugin = r#"import sys
data = {}
for line in sys.stdin:
    if '=' in line:
        k, v = line.strip().split('=', 1)
        data[k] = v
print(f"Hello from Python plugin '{data.get('COMMAND', 'unknown')}'!")
print(f"Running as user: {data.get('USER', 'unknown')}")
print(f"Current directory: {data.get('CWD', '/')}")
print("Plugin executed successfully! You can extend this freely.")
"#;
        let _ = fs::write(shell.plugins_dir.join("hello.py"), example_plugin);
        let _ = fs::write(shell.plugins_dir.join("demo.py"), example_plugin);

        shell
    }

    fn print_boot() {
        println!("{}", ANSI_CLEAR);
        println!("{}================================================================================{}", ANSI_GREEN, ANSI_RESET);
        println!("{}/                    phase1 v1.2.0  —  Advanced OS Simulator                   /{}", ANSI_GREEN, ANSI_RESET);
        println!("{}/  Full VFS - Scheduler - PCIe - CR3/CR4/PCID - Cross-platform Networking      /{}", ANSI_GREEN, ANSI_RESET);
        println!("{}================================================================================{}", ANSI_GREEN, ANSI_RESET);
        println!("{}[    0.000000] phase1 kernel booted on virtual x86_64 hardware{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.012345] Initializing in-memory tree Virtual File System{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.034567] Mounting /proc, /dev, /home, /etc{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.067890] Preemptive scheduler with priority support activated{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.089012] PCIe enumeration subsystem loaded{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.112345] CR3/CR4 paging register management ready (PCID capable){}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.145678] Cross-platform network stack (Linux + macOS) loaded{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.167890] Educational boot complete — type 'help' for commands{}", ANSI_GREEN, ANSI_RESET);
        println!();
    }

    fn expand_env(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (k, v) in &self.env {
            let key = format!("${}", k);
            result = result.replace(&key, v);
        }
        result
    }

    fn try_plugin(&self, cmd: &str, args: &[&str]) -> bool {
        let plugin_path = self.plugins_dir.join(format!("{}.py", cmd));
        if !plugin_path.exists() { return false; }

        let context_str = format!(
            "COMMAND={}\nARGS={}\nUSER={}\nCWD={}\nPID={}\nHOME={}\n",
            cmd, args.join(" "), self.kernel.scheduler.current_user,
            self.kernel.vfs.cwd.to_str().unwrap_or("/"), process::id(),
            self.env.get("HOME").unwrap_or(&"/home".to_string())
        );

        match Command::new("python3")
            .arg(&plugin_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn() {
            Ok(mut child) => {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(context_str.as_bytes());
                }
                let output = child.wait_with_output().unwrap_or_else(|_| std::process::Output {
                    status: std::process::ExitStatus::from_raw(0),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                });
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}{}{}", ANSI_MAGENTA, stdout.trim(), ANSI_RESET);
                true
            }
            Err(_) => false,
        }
    }

    fn run(&mut self) {
        Self::print_boot();
        println!("{}phase1 v{} ready. Type 'help' for commands or 'man <cmd>' for explanations.{}", ANSI_GREEN, VERSION, ANSI_RESET);

        let mut input = String::new();
        loop {
            let uptime_secs = self.start_time.elapsed().as_secs();
            self.kernel.tick(uptime_secs);

            print!("{}@phase1{}:{}$ ", ANSI_CYAN, ANSI_RESET, self.kernel.vfs.cwd.display());
            let _ = io::stdout().flush();

            input.clear();
            if io::stdin().read_line(&mut input).is_err() { break; }
            let line = input.trim();
            if line.is_empty() { continue; }

            self.history.push_back(line.to_string());
            if self.history.len() > 300 { self.history.pop_front(); }

            let expanded = self.expand_env(line);
            let parts: Vec<&str> = expanded.split_whitespace().collect();
            if parts.is_empty() { continue; }

            let cmd = parts[0];
            let args = &parts[1..];

            match cmd {
                "exit" | "quit" | "shutdown" | "poweroff" => {
                    println!("{}Shutting down the phase1 simulator... Goodbye!{}", ANSI_YELLOW, ANSI_RESET);
                    break;
                }
                "help" => self.cmd_help(),
                "man" => self.cmd_man(args.first().copied()),
                "ps" => println!("{}", self.kernel.scheduler.ps()),
                "top" => { println!("{}", self.kernel.scheduler.top()); thread::sleep(Duration::from_secs(3)); }
                "free" | "mem" => self.cmd_free(),
                "kill" => println!("{}", self.kernel.scheduler.kill(args.first().copied())),
                "nice" => println!("{}", self.kernel.scheduler.nice(args.first().copied(), args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0))),
                "spawn" => {
                    let name = args.get(0).unwrap_or(&"anon");
                    match self.kernel.scheduler.spawn(name, process::id(), &args.join(" "), 2048, false, 0) {
                        Some(pid) => println!("Spawned new process with PID {}", pid),
                        None => println!("{}Process table full (max 64){}", ANSI_RED, ANSI_RESET),
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
                    } else {
                        println!("Usage: loadcr3 <value> (hex or decimal — must be page-aligned unless CR4.PCIDE=1)");
                    }
                }
                "cr4" => println!("{}", self.kernel.scheduler.cr4()),
                "pcide" => {
                    if let Some(arg) = args.first() {
                        match *arg {
                            "on" | "1" | "enable" => self.kernel.scheduler.set_pcide(true),
                            "off" | "0" | "disable" => self.kernel.scheduler.set_pcide(false),
                            _ => println!("Usage: pcide <on|off|1|0|enable|disable>"),
                        }
                    } else {
                        println!("CR4.PCIDE is currently {}", if self.kernel.scheduler.cr4_pcide { "enabled" } else { "disabled" });
                    }
                }
                "df" => println!("{}Filesystem     1K-blocks    Used Available Use% Mounted on\nphase1-vfs     4194304   1048576   3145728  25% /{}", ANSI_YELLOW, ANSI_RESET),
                "whoami" => println!("{}", self.kernel.scheduler.current_user),
                "id" => println!("uid={}({}) gid=0(root) groups=0(root)", self.kernel.scheduler.current_uid, self.kernel.scheduler.current_user),
                "ls" => {
                    let long = args.contains(&"-l");
                    let path = if let Some(p) = args.iter().find(|&&x| x != "-l") { Some(*p) } else { None };
                    println!("{}", self.kernel.vfs.ls(path, long));
                }
                "cd" => self.cmd_cd(args.first().copied()),
                "pwd" => println!("{}", self.kernel.vfs.cwd.display()),
                "cat" => match self.kernel.vfs.cat(args.first().unwrap_or(&"")) {
                    Ok(c) => println!("{}", c),
                    Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                },
                "mkdir" => match self.kernel.vfs.mkdir(args.first().unwrap_or(&"")) {
                    Ok(_) => println!("Directory created"),
                    Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                },
                "touch" => match self.kernel.vfs.touch(args.first().unwrap_or(&"")) {
                    Ok(_) => println!("File touched"),
                    Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                },
                "rm" => match self.kernel.vfs.rm(args.first().unwrap_or(&"")) {
                    Ok(_) => println!("Removed"),
                    Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                },
                "cp" => if args.len() >= 2 {
                    match self.kernel.vfs.cp(args[0], args[1]) {
                        Ok(_) => println!("Copied"),
                        Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                    }
                } else {
                    println!("Usage: cp <source> <destination>");
                },
                "mv" => if args.len() >= 2 {
                    match self.kernel.vfs.mv(args[0], args[1]) {
                        Ok(_) => println!("Moved/Renamed"),
                        Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                    }
                } else {
                    println!("Usage: mv <source> <destination>");
                },
                "echo" => {
                    let text = args.join(" ");
                    if let Some(redirect_pos) = args.iter().position(|&x| x == ">" || x == ">>") {
                        if redirect_pos + 1 < args.len() {
                            let content = args[0..redirect_pos].join(" ");
                            let file = args[redirect_pos + 1];
                            let append = args[redirect_pos] == ">>";
                            match self.kernel.vfs.write_file(file, &content, append) {
                                Ok(_) => println!("(output redirected to {})", file),
                                Err(e) => println!("{}Redirect error: {}{}", ANSI_RED, e, ANSI_RESET),
                            }
                        }
                    } else {
                        println!("{}", text);
                    }
                }
                "clear" => println!("{}", ANSI_CLEAR),
                "env" => self.cmd_env(),
                "export" => self.cmd_export(args),
                "unset" => self.cmd_unset(args.first().copied()),
                "python" | "py" => self.cmd_python(args.first().copied()),
                "plugin" | "plugins" => self.cmd_plugins(),
                "jobs" => println!("{}", self.kernel.scheduler.jobs()),
                "fg" => println!("Background job control is simulated — all bg jobs auto-complete after delay."),
                "bg" => println!("Background job control is simulated — all bg jobs auto-complete after delay."),
                "su" => self.cmd_su(args.first().copied()),
                "dmesg" => self.cmd_dmesg(),
                "vmstat" => self.cmd_vmstat(),
                "history" => self.cmd_history(),
                "uname" => println!("Linux phase1 6.8.0-phase1-advanced #1 SMP PREEMPT_DYNAMIC {} x86_64 GNU/Linux", Local::now().format("%Y-%m-%d")),
                "date" => println!("{}", Local::now().format("%a %b %d %H:%M:%S %Z %Y")),
                "uptime" => println!("{} up {} load average: 0.12, 0.15, 0.10", Local::now().format("%H:%M:%S"), self.start_time.elapsed().as_secs()),
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
                },
                "ping" => {
                    let host = args.first().copied().unwrap_or("8.8.8.8");
                    println!("{}", self.network.ping(host));
                },
                "nmcli" => println!("{}", self.network.nmcli()),
                "sandbox" | "nsinfo" => println!("{}Running inside pure-Rust userspace sandbox. No real system privileges.{}", ANSI_CYAN, ANSI_RESET),
                "version" => println!("phase1 v{} — built {}", VERSION, BUILD_DATE),
                "tree" => self.cmd_tree(),
                _ => {
                    if !self.try_plugin(cmd, args) {
                        println!("{}command not found: {}{}   (type 'help' for full list)", ANSI_RED, cmd, ANSI_RESET);
                    }
                }
            }
        }
    }

    fn cmd_help(&self) {
        println!("{}phase1 v1.2.0 — Complete Command Reference{}", ANSI_BOLD, ANSI_RESET);
        println!("Core filesystem:   ls [-l]  cd  pwd  cat  mkdir  touch  rm  cp  mv  echo [> or >> file]");
        println!("Process mgmt:      ps  top  kill  spawn  nice  jobs  fg  bg");
        println!("Hardware / Paging: lspci  pcie  cr3  loadcr3  cr4  pcide");
        println!("Networking:        ifconfig  iwconfig  wifi-scan  wifi-connect  nmcli");
        println!("System info:       free  df  uname  date  uptime  dmesg  vmstat  hostname  ping");
        println!("Shell:             env  export  unset  history  clear  su  whoami  id");
        println!("Plugins:           python/py  plugin/plugins  (any .py in ./plugins/)");
        println!("Misc:              tree  sandbox  version  man <cmd>  exit");
        println!("\n{}Cross-platform networking works on Linux and macOS. Educational kernel concepts in safe userspace.{}", ANSI_YELLOW, ANSI_RESET);
    }

    fn cmd_cd(&mut self, dir: Option<&str>) {
        if let Some(d) = dir {
            let new_path = self.kernel.vfs.resolve_path(d);
            if self.kernel.vfs.get_node(&new_path).is_some() {
                self.kernel.vfs.cwd = new_path;
            } else {
                println!("{}cd: no such directory or permission denied{}", ANSI_RED, ANSI_RESET);
            }
        } else {
            self.kernel.vfs.cwd = PathBuf::from("/home");
        }
    }

    fn cmd_free(&self) {
        println!("{}              total        used        free      shared  buff/cache   available{}", ANSI_YELLOW, ANSI_RESET);
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

    fn cmd_python(&self, file: Option<&str>) {
        if let Some(f) = file {
            let _ = Command::new("python3").arg(f).status();
        } else {
            println!("Usage: python <script.py>");
        }
    }

    fn cmd_plugins(&self) {
        println!("{}Available Python plugins in ./plugins/:{}", ANSI_GREEN, ANSI_RESET);
        if let Ok(entries) = fs::read_dir(&self.plugins_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |e| e == "py") {
                    println!("   • {}", entry.file_name().to_string_lossy());
                }
            }
        }
        println!("\nPlugins receive full OS context via stdin as key=value lines.");
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
        println!("{}[    0.000000] phase1 kernel: virtual hardware detected{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.012345] VFS: mounted in-memory tree filesystem{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.045678] Scheduler: preemptive multitasking with priorities enabled{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.078901] PCIe + CR3/CR4 paging subsystems initialized{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.112345] Cross-platform network stack initialized{}", ANSI_YELLOW, ANSI_RESET);
    }

    fn cmd_vmstat(&self) {
        println!("{} procs -----------memory---------- ---swap-- -----io---- -system-- ------cpu-----{}", ANSI_YELLOW, ANSI_RESET);
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
        println!("│   ├── motd");
        println!("│   └── passwd");
        println!("├── home");
        println!("│   └── readme.txt");
        println!("├── proc");
        println!("│   ├── cpuinfo");
        println!("│   ├── loadavg");
        println!("│   ├── meminfo");
        println!("│   ├── stat");
        println!("│   ├── uptime");
        println!("│   └── version");
        println!("└── (plugins directory outside VFS)");
    }

    fn cmd_man(&self, topic: Option<&str>) {
        match topic {
            Some("cr3") => println!("cr3: display current CR3 register value (paging base)"),
            Some("loadcr3") => println!("loadcr3 <value>: direct load into CR3 register (privileged, hardware-accurate PCID validation)"),
            Some("cr4") => println!("cr4: display current CR4 register value (includes PCIDE bit)"),
            Some("pcide") => println!("pcide <on|off>: toggle CR4.PCIDE (enables PCID usage in CR3)"),
            Some("lspci") => println!("lspci: list PCI/PCIe devices (simulated hardware enumeration)"),
            Some("ifconfig") => println!("ifconfig: show network interfaces (real host data on Linux/macOS)"),
            Some("wifi-scan") => println!("wifi-scan: scan for nearby WiFi networks (real scan on supported OS)"),
            Some("ls") => println!("ls: list directory contents. Use -l for long format with permissions."),
            Some("echo") => println!("echo: print text. Supports basic redirection: echo text > file or >> file"),
            Some("cd") => println!("cd: change working directory. Use .. for parent."),
            Some(_) => println!("No manual entry for that command yet."),
            None => println!("Usage: man <command>"),
        }
    }
}

fn main() {
    let mut shell = Phase1Shell::new();
    shell.run();
}
