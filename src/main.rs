// phase1 v0.4.0 - Educational Embedded Operating System Simulator
// Major improvements over v0.3:
// - Dynamic process spawning (`spawn <name>`)
// - Simulated disk usage (`df`)
// - User identity simulation (`whoami`, `id`)
// - Enhanced `top` command with fake CPU% visualization
// - Updated boot banner, help text, and version sync
// - More realistic educational OS feel
// - Zero new dependencies

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command};
use std::time::Instant;

const VERSION: &str = "0.4.0";

const ANSI_CLEAR: &str = "\x1b[2J\x1b[H";
const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_BLUE: &str = "\x1b[34m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_CYAN: &str = "\x1b[36m";

#[derive(Clone)]
struct SimProcess {
    pid: u32,
    ppid: u32,
    name: String,
    state: String,
    mem_kb: u64,
    cmdline: String,
}

struct Phase1OS {
    start_time: Instant,
    processes: Vec<SimProcess>,
    memory_total_mb: u64,
}

impl SimProcess {
    fn new(pid: u32, ppid: u32, name: &str, state: &str, mem_kb: u64, cmdline: &str) -> Self {
        Self {
            pid,
            ppid,
            name: name.to_string(),
            state: state.to_string(),
            mem_kb,
            cmdline: cmdline.to_string(),
        }
    }
}

impl Phase1OS {
    fn new() -> Self {
        let mut os = Phase1OS {
            start_time: Instant::now(),
            processes: Vec::new(),
            memory_total_mb: 1024,
        };
        os.initialize_processes();
        os
    }

    fn initialize_processes(&mut self) {
        let current_pid = process::id();
        self.processes.push(SimProcess::new(1, 0, "init", "running", 128, "/sbin/init"));
        self.processes.push(SimProcess::new(42, 1, "python-plugin", "running", 2048, "python3 plugin example"));
        self.processes.push(SimProcess::new(current_pid, 1, "phase1", "running", 1024, &format!("phase1 v{} (current shell)", VERSION)));
        self.processes.push(SimProcess::new(100, 1, "shell", "running", 512, "user shell"));
    }

    fn get_uptime(&self) -> String {
        let elapsed = self.start_time.elapsed();
        let total_secs = elapsed.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        format!("{:02}:{:02}:{:02} up {} min,  1 user", hours, minutes, seconds, hours * 60 + minutes)
    }

    fn ps(&self) {
        println!("{}  PID  PPID  NAME             STATE      MEM(KB)   CMD{}", ANSI_BOLD, ANSI_RESET);
        for p in &self.processes {
            println!(
                "{:>5} {:>5}  {:<16} {:<10} {:>8}   {}",
                p.pid, p.ppid, p.name, p.state, p.mem_kb, p.cmdline
            );
        }
        println!("\n{}({} simulated processes active){}", ANSI_YELLOW, self.processes.len(), ANSI_RESET);
    }

    fn top(&self) {
        println!("{}=== phase1 top (v{}) ==={}", ANSI_BOLD, VERSION, ANSI_RESET);
        println!("  PID  PPID  NAME             STATE      MEM(KB)  CPU%   CMD");
        for p in &self.processes {
            let cpu = match p.pid {
                42 => "12.5",
                p if p == process::id() => "45.8",
                _ => "0.0",
            };
            println!(
                "{:>5} {:>5}  {:<16} {:<10} {:>8}  {:>5}   {}",
                p.pid, p.ppid, p.name, p.state, p.mem_kb, cpu, p.cmdline
            );
        }
        println!("\n{}Educational CPU simulation (real-time in future versions){}", ANSI_YELLOW, ANSI_RESET);
    }

    fn free(&self) {
        let used_mb: u64 = self.processes.iter().map(|p| p.mem_kb / 1024).sum();
        let free_mb = self.memory_total_mb.saturating_sub(used_mb);
        println!("              total        used        free      shared  buff/cache   available");
        println!("Mem:     {:>8}M   {:>8}M   {:>8}M          0M         16M   {:>8}M", self.memory_total_mb, used_mb, free_mb, free_mb);
        println!("Swap:           0M          0M          0M");
        println!("{} (Dynamic simulated memory){}", ANSI_YELLOW, ANSI_RESET);
    }

    fn kill(&mut self, pid_str: &str) {
        match pid_str.parse::<u32>() {
            Ok(pid) => {
                if pid == 1 {
                    println!("{}kill: cannot terminate init (PID 1){}", ANSI_YELLOW, ANSI_RESET);
                } else if let Some(pos) = self.processes.iter().position(|p| p.pid == pid) {
                    let removed = self.processes.remove(pos);
                    println!("{}kill: terminated PID {} ({})", ANSI_GREEN, pid, removed.name);
                } else {
                    println!("kill: no such process: {}", pid);
                }
            }
            Err(_) => println!("kill: invalid PID"),
        }
    }

    fn spawn(&mut self, name: &str) {
        let max_pid = self.processes.iter().map(|p| p.pid).max().unwrap_or(100) + 1;
        self.processes.push(SimProcess::new(
            max_pid,
            1,
            name,
            "running",
            512,
            &format!("spawned by user: {}", name),
        ));
        println!("{}spawn: ✓ created process '{}' (PID {})", ANSI_GREEN, name, max_pid);
    }

    fn df(&self) {
        println!("{}Filesystem            1K-blocks     Used Available Use% Mounted on{}", ANSI_BOLD, ANSI_RESET);
        println!("phase1-simfs             1048576     262144    786432  25% /");
        println!("tmpfs                     524288          0    524288   0% /tmp");
        println!("\n{}Simulated educational disk (no real I/O){}", ANSI_YELLOW, ANSI_RESET);
    }

    fn whoami(&self) {
        println!("user");
    }
}

// ─────────────────────────────────────────────────────────────
// Helper functions (unchanged from v0.3)
fn clear_screen() {
    print!("{}", ANSI_CLEAR);
    let _ = io::stdout().flush();
}

fn print_boot() {
    clear_screen();
    println!("{}{}phase1 v{} - Educational Embedded Operating System{}", ANSI_BOLD, ANSI_GREEN, VERSION, ANSI_RESET);
    println!("Kernel: Rust userspace simulation");
    println!("New in v0.4: spawn • df • whoami/id • top with CPU%");
    println!();
    if let Ok(output) = Command::new("date").arg("+%A, %Y-%m-%d %H:%M:%S %Z").output() {
        if let Ok(s) = String::from_utf8(output.stdout) {
            println!("Boot time: {}", s.trim());
        }
    }
    println!("Host OS: {}", env::consts::OS);
    println!("\nType 'help' for commands.\n");
}

fn list_directory() {
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if let Ok(meta) = entry.metadata() {
                    if meta.is_dir() {
                        println!("{}{}/{}", ANSI_BLUE, name, ANSI_RESET);
                    } else {
                        println!("{}", name);
                    }
                } else {
                    println!("{}", name);
                }
            }
        }
        Err(e) => eprintln!("ls: {}", e),
    }
}

fn run_python_code(code: &str) {
    let status = Command::new("python3").arg("-c").arg(code).status();
    match status {
        Ok(s) if !s.success() => {
            if let Some(code) = s.code() { eprintln!("python exited with code {}", code); }
        }
        Err(e) => eprintln!("python: failed to execute: {}", e),
        _ => {}
    }
}

fn run_plugin_or_host(command: &str, args: &[&str]) {
    let plugin_path = format!("plugins/{}.py", command);
    if Path::new(&plugin_path).exists() {
        let status = Command::new("python3").arg(&plugin_path).args(args).status();
        if let Err(e) = status {
            eprintln!("{} (plugin): {}", command, e);
        }
    } else {
        let status = Command::new(command).args(args).status();
        match status {
            Ok(s) if !s.success() => {
                if let Some(code) = s.code() { eprintln!("{} exited with code {}", command, code); }
            }
            Err(e) => eprintln!("{}: command not found or failed: {}", command, e),
            _ => {}
        }
    }
}

fn list_plugins() {
    println!("{}Available Python plugins (in plugins/ directory):{}", ANSI_BOLD, ANSI_RESET);
    match fs::read_dir("plugins") {
        Ok(entries) => {
            let mut found = false;
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.ends_with(".py") {
                    println!("  • {}", name.trim_end_matches(".py"));
                    found = true;
                }
            }
            if !found { println!("  (no .py plugins found yet)"); }
        }
        Err(_) => {
            println!("  plugins/ directory not found.");
            if let Err(e) = fs::create_dir_all("plugins") {
                eprintln!("Failed to create plugins/: {}", e);
            } else {
                println!("{}✓ Created plugins/ directory!{}", ANSI_GREEN, ANSI_RESET);
            }
        }
    }
}

fn print_help() {
    println!("{}Built-in commands (new in v0.4 highlighted):{}", ANSI_BOLD, ANSI_RESET);
    println!("  spawn <name>        ## Create new simulated process");
    println!("  df                  ## Show simulated disk usage");
    println!("  whoami / id         ## Show user identity");
    println!("  top                 ## Enhanced process monitor with CPU%");
    println!("  ps / kill / free / uptime / uname   (existing)");
    println!("  help, ls, pwd, cd, cat, echo, clear, python/py, plugins, sandbox");
    println!("\n{}Any other command checked for plugin first, then passed to host.{}", ANSI_YELLOW, ANSI_RESET);
}

fn print_sandbox_info() {
    println!("{}=== phase1 Sandbox & Isolation Information ==={}", ANSI_BOLD, ANSI_RESET);
    let is_sandboxed = env::var("PHASE1_SANDBOXED").is_ok();
    println!("Status: {}", if is_sandboxed { "ENABLED ✓ (namespaces)" } else { "DISABLED ⚠" });
    println!("Layers: User • Mount • PID • Network • UTS");
    println!("\nEducational note: Safe userspace OS simulation.");
}

fn main() {
    print_boot();
    let mut os = Phase1OS::new();
    let mut running = true;

    while running {
        let cwd = env::current_dir().map(|p| p.display().to_string()).unwrap_or_else(|_| "/".to_string());
        print!("{}user@phase1{}:{}$ {}", ANSI_BOLD, ANSI_RESET, cwd, ANSI_GREEN);
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { continue; }
        let input = input.trim();
        if input.is_empty() { continue; }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args: Vec<&str> = parts[1..].to_vec();

        match command {
            "help" => print_help(),
            "ls" => list_directory(),
            "pwd" => println!("{}", cwd),
            "cd" => {
                if let Some(dir) = args.first() {
                    if let Err(e) = env::set_current_dir(dir) { eprintln!("cd: {}: {}", dir, e); }
                } else { println!("Usage: cd <directory>"); }
            }
            "cat" => {
                if let Some(file) = args.first() {
                    match fs::read_to_string(file) {
                        Ok(content) => print!("{}", content),
                        Err(e) => eprintln!("cat: {}: {}", file, e),
                    }
                } else { println!("Usage: cat <filename>"); }
            }
            "echo" => println!("{}", args.join(" ")),
            "clear" => clear_screen(),
            "python" | "py" => {
                if args.is_empty() {
                    println!("Usage: python <code>");
                } else {
                    run_python_code(&args.join(" "));
                }
            }
            "plugins" | "plugin" => list_plugins(),
            "ps" | "proc" => os.ps(),
            "top" => os.top(),
            "free" | "mem" => os.free(),
            "uptime" => println!("{}", os.get_uptime()),
            "uname" | "kernel" => {
                println!("phase1 {} #1 SMP {} x86_64 phase1-embedded (userspace simulator)", VERSION, chrono::Local::now().format("%a %b %d %H:%M:%S %Z %Y"));
                let _ = Command::new("uname").arg("-a").status();
            }
            "kill" => {
                if let Some(pid) = args.first() { os.kill(pid); } else { println!("Usage: kill <pid>"); }
            }
            "spawn" => {
                if let Some(name) = args.first() {
                    os.spawn(name);
                } else {
                    println!("Usage: spawn <name>");
                }
            }
            "df" => os.df(),
            "whoami" | "id" => os.whoami(),
            "sandbox" | "nsinfo" => print_sandbox_info(),
            "exit" | "quit" | "shutdown" => {
                println!("Shutting down phase1...");
                running = false;
            }
            _ => run_plugin_or_host(command, &args),
        }
    }
    println!("{}System halted. Goodbye!{}", ANSI_RESET, ANSI_BOLD);
}
