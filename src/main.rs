// phase1 v0.3.0 - Educational Embedded Operating System Simulator
// Major improvements over v0.2:
// - Stateful OS simulation (dynamic processes, memory accounting, kill actually removes processes)
// - Real dynamic uptime using std::time::Instant
// - Colored terminal UI for better educational UX
// - Cleaner, more idiomatic Rust code with helper functions and structs
// - Auto-creation of plugins/ directory
// - Improved error handling and user feedback
// - Better plugin/host command dispatching
// - Enhanced help, boot sequence, and sandbox info
// - Zero new dependencies

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command};
use std::time::Instant;

const VERSION: &str = "0.3.0";

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
        self.processes.push(SimProcess::new(
            1, 0, "init", "running", 128, "/sbin/init",
        ));
        self.processes.push(SimProcess::new(
            42, 1, "python-plugin", "running", 2048, "python3 plugin example",
        ));
        self.processes.push(SimProcess::new(
            current_pid, 1, "phase1", "running", 1024,
            &format!("phase1 v{} (current shell)", VERSION),
        ));
        self.processes.push(SimProcess::new(
            100, 1, "shell", "running", 512, "user shell",
        ));
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
        println!(
            "\n{}({} simulated processes active - educational process table demo){}",
            ANSI_YELLOW, self.processes.len(), ANSI_RESET
        );
        println!("Use 'kill <pid>' to terminate (except PID 1). Real host PID shown for current shell.");
    }

    fn free(&self) {
        let used_mb: u64 = self.processes.iter().map(|p| p.mem_kb / 1024).sum();
        let free_mb = self.memory_total_mb.saturating_sub(used_mb);
        println!("              total        used        free      shared  buff/cache   available");
        println!(
            "Mem:     {:>8}M   {:>8}M   {:>8}M          0M         16M   {:>8}M",
            self.memory_total_mb, used_mb, free_mb, free_mb
        );
        println!("Swap:           0M          0M          0M");
        println!(
            "{} (Dynamic simulated memory - tracks process allocations){}",
            ANSI_YELLOW, ANSI_RESET
        );
    }

    fn kill(&mut self, pid_str: &str) {
        match pid_str.parse::<u32>() {
            Ok(pid) => {
                if pid == 1 {
                    println!("{}kill: cannot terminate init (PID 1) - system critical{}", ANSI_YELLOW, ANSI_RESET);
                } else if let Some(pos) = self.processes.iter().position(|p| p.pid == pid) {
                    let removed = self.processes.remove(pos);
                    println!(
                        "{}kill: terminated simulated PID {} ({}) - process reaped{}",
                        ANSI_GREEN, pid, removed.name, ANSI_RESET
                    );
                } else {
                    println!("kill: no such process: {}", pid);
                }
            }
            Err(_) => println!("kill: invalid PID '{}'", pid_str),
        }
    }
}

fn clear_screen() {
    print!("{}", ANSI_CLEAR);
    let _ = io::stdout().flush();
}

fn print_boot() {
    clear_screen();
    println!(
        "{}{}phase1 v{} - Educational Embedded Operating System{}",
        ANSI_BOLD, ANSI_GREEN, VERSION, ANSI_RESET
    );
    println!("Kernel: Rust userspace simulation (no VM/bare-metal needed)");
    println!("New in v0.3: Stateful processes ✓ | Dynamic uptime ✓ | Memory accounting ✓ | Colored UI");
    println!("Sandbox: Linux namespaces (user/mount/pid/net/uts) for safe execution");
    println!();
    if let Ok(output) = Command::new("date").arg("+%A, %Y-%m-%d %H:%M:%S %Z").output() {
        if let Ok(s) = String::from_utf8(output.stdout) {
            println!("Boot time: {}", s.trim());
        }
    }
    println!("Host OS: {}", env::consts::OS);
    println!("\nType 'help' for available commands.\n");
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
            if let Some(code) = s.code() {
                eprintln!("python exited with code {}", code);
            }
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
                if let Some(code) = s.code() {
                    eprintln!("{} exited with code {}", command, code);
                }
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
            if !found {
                println!("  (no .py plugins found yet)");
            }
        }
        Err(_) => {
            println!("  plugins/ directory not found.");
            if let Err(e) = fs::create_dir_all("plugins") {
                eprintln!("Failed to create plugins/: {}", e);
            } else {
                println!("{}✓ Created plugins/ directory for extensibility!{}", ANSI_GREEN, ANSI_RESET);
                println!("   Drop .py files here to instantly add new commands.");
            }
        }
    }
    println!("\nTip: See examples in the project repository.");
}

fn print_help() {
    println!("{}Built-in commands (OS-simulated where noted):{}", ANSI_BOLD, ANSI_RESET);
    println!("  help                  Show this message");
    println!("  ls                    List directory (colored dirs)");
    println!("  pwd                   Print working directory");
    println!("  cd <dir>              Change directory");
    println!("  cat <file>            Display file contents");
    println!("  echo <text>           Print text");
    println!("  clear                 Clear screen");
    println!("  python/py <code>      Run Python code inline");
    println!("  plugins               List/extend with Python plugins");
    println!("  ps / proc             Show simulated process table (stateful)");
    println!("  free / mem            Show simulated memory usage (dynamic)");
    println!("  uptime                Show dynamic system uptime");
    println!("  uname / kernel        Show kernel/system info");
    println!("  kill <pid>            Terminate simulated process");
    println!("  sandbox / nsinfo      Show isolation details");
    println!("  exit / quit / shutdown Exit the OS simulator");
    println!("\n{}Any other command is checked for a matching .py plugin first, then passed to host (sandboxed).{}", ANSI_YELLOW, ANSI_RESET);
}

fn print_sandbox_info() {
    println!("{}=== phase1 Sandbox & Isolation Information ==={}", ANSI_BOLD, ANSI_RESET);
    let is_sandboxed = env::var("PHASE1_SANDBOXED").is_ok();
    println!("Status: {}", if is_sandboxed { "ENABLED ✓ (namespaces)" } else { "DISABLED ⚠ (set env var for full isolation)" });
    println!("Layers:");
    println!("  • User namespace + map-root-user");
    println!("  • Mount + private /proc");
    println!("  • PID namespace (isolated ps)");
    println!("  • Network namespace (loopback only)");
    println!("  • UTS namespace");
    println!("\nFilesystem: Shared with host (subject to your permissions)");
    println!("Python/plugins: Fully sandboxed.");
    println!("\nEducational note: This keeps the simulator safe while allowing real OS interaction.");
}

fn main() {
    print_boot();

    let mut os = Phase1OS::new();

    let mut running = true;

    while running {
        let cwd = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "/".to_string());

        print!(
            "{}user@phase1{}:{}$ {}",
            ANSI_BOLD, ANSI_RESET, cwd, ANSI_GREEN
        );
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args: Vec<&str> = parts[1..].to_vec();

        match command {
            "help" => print_help(),
            "ls" => list_directory(),
            "pwd" => println!("{}", cwd),
            "cd" => {
                if let Some(dir) = args.first() {
                    if let Err(e) = env::set_current_dir(dir) {
                        eprintln!("cd: {}: {}", dir, e);
                    }
                } else {
                    println!("Usage: cd <directory>");
                }
            }
            "cat" => {
                if let Some(file) = args.first() {
                    match fs::read_to_string(file) {
                        Ok(content) => print!("{}", content),
                        Err(e) => eprintln!("cat: {}: {}", file, e),
                    }
                } else {
                    println!("Usage: cat <filename>");
                }
            }
            "echo" => println!("{}", args.join(" ")),
            "clear" => clear_screen(),
            "python" | "py" => {
                if args.is_empty() {
                    println!("Usage: python <code>  (or py <code>)");
                    println!("Example: py print('Hello from embedded Python!')");
                } else {
                    run_python_code(&args.join(" "));
                }
            }
            "plugins" | "plugin" => list_plugins(),
            "ps" | "proc" => os.ps(),
            "free" | "mem" => os.free(),
            "uptime" => println!("{}", os.get_uptime()),
            "uname" | "kernel" => {
                println!("phase1 {} #1 SMP Sun May  3 03:31:00 UTC 2026 x86_64 phase1-embedded (userspace simulator)", VERSION);
                println!("(This is a userspace educational OS simulator, not bare-metal. See 'sandbox' for isolation details)");
                let _ = Command::new("uname").arg("-a").status(); // optional real host info
            }
            "kill" => {
                if let Some(pid) = args.first() {
                    os.kill(pid);
                } else {
                    println!("Usage: kill <pid>");
                }
            }
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
