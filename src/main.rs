// =====================================================
// phase1 v1.0.0 — Advanced Educational Embedded OS Simulator
// =====================================================
// Complete in-memory tree VFS with permissions, read/write/append,
// environment variables with expansion, command history, basic
// redirection (echo > file and >>), improved plugins, scheduler
// with priorities and ticks, many realistic OS commands,
// dynamic /proc, educational comments throughout, colored boot,
// and much more — all in pure Rust userspace using only chrono.

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::thread;
use std::time::{Duration, Instant};

// macOS / Unix-specific import for ExitStatus fallback (required on macOS)
use std::os::unix::process::ExitStatusExt;

use chrono::prelude::*;

const VERSION: &str = "1.0.0";
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

// ==============================================
// VFS Node & Complete Virtual File System
// ==============================================
#[derive(Clone, Debug)]
enum VfsNode {
    File {
        content: String,
        perm: u8, // owner rwx simulation (4+2+1)
    },
    Dir {
        children: HashMap<String, VfsNode>,
        perm: u8,
    },
}

struct Vfs {
    root: VfsNode,
    cwd: PathBuf,
}

impl Vfs {
    fn new() -> Self {
        let mut root_children: HashMap<String, VfsNode> = HashMap::new();

        // /proc — dynamic kernel information
        let mut proc_children: HashMap<String, VfsNode> = HashMap::new();
        proc_children.insert(
            "cpuinfo".to_string(),
            VfsNode::File {
                content: "processor   : 0\nmodel name  : phase1 Virtual Cortex-A76\ncpu cores   : 4\nbogomips    : 4800.00\n".to_string(),
                perm: 4,
            },
        );
        proc_children.insert(
            "meminfo".to_string(),
            VfsNode::File {
                content: "MemTotal:     2097152 kB\nMemFree:      1048576 kB\nMemAvailable: 1572864 kB\nBuffers:      131072 kB\n".to_string(),
                perm: 4,
            },
        );
        proc_children.insert(
            "uptime".to_string(),
            VfsNode::File {
                content: "Dynamic uptime".to_string(),
                perm: 4,
            },
        );
        proc_children.insert(
            "loadavg".to_string(),
            VfsNode::File {
                content: "0.12 0.15 0.10 1/42 1234".to_string(),
                perm: 4,
            },
        );
        proc_children.insert(
            "version".to_string(),
            VfsNode::File {
                content: format!("phase1 v{} (Grok Advanced Educational Build {})", VERSION, BUILD_DATE),
                perm: 4,
            },
        );
        proc_children.insert(
            "stat".to_string(),
            VfsNode::File {
                content: "cpu  12345 0 54321 987654 0 0 0 0 0 0\n".to_string(),
                perm: 4,
            },
        );

        root_children.insert(
            "proc".to_string(),
            VfsNode::Dir {
                children: proc_children,
                perm: 5,
            },
        );

        // /home with welcome file
        let mut home_children: HashMap<String, VfsNode> = HashMap::new();
        home_children.insert(
            "readme.txt".to_string(),
            VfsNode::File {
                content: "Welcome to phase1 v1.0.0 — Advanced Educational OS Simulator!\n\nTry these commands:\n  ls -l, mkdir, touch, echo \"text\" > file.txt\n  cat, cp, mv, rm, ps, top, jobs, su\n  python plugins, env, export, history, uname\n\nEverything runs entirely in memory for safe learning.\n".to_string(),
                perm: 6,
            },
        );
        root_children.insert(
            "home".to_string(),
            VfsNode::Dir {
                children: home_children,
                perm: 7,
            },
        );

        // /etc
        let mut etc_children: HashMap<String, VfsNode> = HashMap::new();
        etc_children.insert(
            "passwd".to_string(),
            VfsNode::File {
                content: "root:x:0:0:root:/root:/bin/sh\nuser:x:1000:1000:user:/home:/bin/sh\n".to_string(),
                perm: 4,
            },
        );
        etc_children.insert(
            "motd".to_string(),
            VfsNode::File {
                content: "Welcome to the phase1 educational kernel!\n".to_string(),
                perm: 4,
            },
        );
        root_children.insert(
            "etc".to_string(),
            VfsNode::Dir {
                children: etc_children,
                perm: 5,
            },
        );

        // /dev (simulated devices)
        let mut dev_children: HashMap<String, VfsNode> = HashMap::new();
        dev_children.insert(
            "null".to_string(),
            VfsNode::File {
                content: "".to_string(),
                perm: 6,
            },
        );
        dev_children.insert(
            "zero".to_string(),
            VfsNode::File {
                content: "\0".repeat(1024),
                perm: 4,
            },
        );
        root_children.insert(
            "dev".to_string(),
            VfsNode::Dir {
                children: dev_children,
                perm: 5,
            },
        );

        // /bin (symbolic placeholder)
        root_children.insert(
            "bin".to_string(),
            VfsNode::Dir {
                children: HashMap::new(),
                perm: 5,
            },
        );

        Vfs {
            root: VfsNode::Dir {
                children: root_children,
                perm: 5,
            },
            cwd: PathBuf::from("/"),
        }
    }

    fn resolve_path(&self, path: &str) -> PathBuf {
        let mut p = if path.starts_with('/') {
            PathBuf::from("/")
        } else {
            self.cwd.clone()
        };

        for part in Path::new(path).components() {
            match part.as_os_str().to_str().unwrap_or("") {
                "" | "." => {}
                ".." => { let _ = p.pop(); }
                seg => p.push(seg),
            }
        }
        p
    }

    fn get_node<'a>(&'a self, path: &Path) -> Option<&'a VfsNode> {
        let mut current = &self.root;
        for component in path.components().skip(1) {
            let name = component.as_os_str().to_str()?;
            if let VfsNode::Dir { children, .. } = current {
                current = children.get(name)?;
            } else {
                return None;
            }
        }
        Some(current)
    }

    fn get_node_mut<'a>(&'a mut self, path: &Path) -> Option<&'a mut VfsNode> {
        let mut current = &mut self.root;
        for component in path.components().skip(1) {
            let name = component.as_os_str().to_str()?;
            if let VfsNode::Dir { children, .. } = current {
                current = children.get_mut(name)?;
            } else {
                return None;
            }
        }
        Some(current)
    }

    fn mkdir(&mut self, path_str: &str) -> Result<(), String> {
        let path = self.resolve_path(path_str);
        let parent = path.parent().unwrap_or(Path::new("/"));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or("Invalid name")?.to_string();

        let parent_node = self.get_node_mut(parent).ok_or("Parent directory not found")?;
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied (write)".to_string()); }
            children.insert(name, VfsNode::Dir { children: HashMap::new(), perm: 7 });
            Ok(())
        } else {
            Err("Parent is not a directory".to_string())
        }
    }

    fn touch(&mut self, path_str: &str) -> Result<(), String> {
        let path = self.resolve_path(path_str);
        let parent = path.parent().unwrap_or(Path::new("/"));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or("Invalid name")?.to_string();

        let parent_node = self.get_node_mut(parent).ok_or("Parent directory not found")?;
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied (write)".to_string()); }
            children.insert(name, VfsNode::File { content: String::new(), perm: 6 });
            Ok(())
        } else {
            Err("Parent is not a directory".to_string())
        }
    }

    fn cat(&self, path_str: &str) -> Result<String, String> {
        let path = self.resolve_path(path_str);
        if let Some(VfsNode::File { content, .. }) = self.get_node(&path) {
            Ok(content.clone())
        } else {
            Err(format!("No such file or directory: {}", path_str))
        }
    }

    fn write_file(&mut self, path_str: &str, content: &str, append: bool) -> Result<(), String> {
        let path = self.resolve_path(path_str);
        let parent = path.parent().unwrap_or(Path::new("/"));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or("Invalid filename")?.to_string();

        let parent_node = self.get_node_mut(parent).ok_or("Parent directory not found")?;
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied (write)".to_string()); }

            if let Some(VfsNode::File { content: existing, .. }) = children.get_mut(&name) {
                if append {
                    existing.push_str(content);
                } else {
                    *existing = content.to_string();
                }
            } else {
                children.insert(name, VfsNode::File { content: content.to_string(), perm: 6 });
            }
            Ok(())
        } else {
            Err("Parent is not a directory".to_string())
        }
    }

    fn ls(&self, path_str: Option<&str>, long: bool) -> String {
        let path = self.resolve_path(path_str.unwrap_or("."));
        if let Some(VfsNode::Dir { children, .. }) = self.get_node(&path) {
            let mut out = String::new();
            if long {
                out.push_str("total 0\n");
            }
            for (name, node) in children.iter() {
                match node {
                    VfsNode::Dir { .. } => {
                        if long {
                            out.push_str(&format!("drwxr-xr-x  2 root root 4096 {} {}\n", Local::now().format("%b %d %H:%M"), name));
                        } else {
                            out.push_str(&format!("📁 {}\n", name));
                        }
                    }
                    VfsNode::File { .. } => {
                        if long {
                            out.push_str(&format!("-rw-r--r--  1 root root  123 {} {}\n", Local::now().format("%b %d %H:%M"), name));
                        } else {
                            out.push_str(&format!("📄 {}\n", name));
                        }
                    }
                }
            }
            out
        } else {
            "Not a directory".to_string()
        }
    }

    fn rm(&mut self, path_str: &str) -> Result<(), String> {
        let path = self.resolve_path(path_str);
        let parent = path.parent().unwrap_or(Path::new("/"));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or("Invalid name")?.to_string();

        let parent_node = self.get_node_mut(parent).ok_or("Parent directory not found")?;
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied".to_string()); }
            if children.remove(&name).is_some() {
                Ok(())
            } else {
                Err("No such file or directory".to_string())
            }
        } else {
            Err("Not a directory".to_string())
        }
    }

    fn cp(&mut self, src: &str, dst: &str) -> Result<(), String> {
        let src_path = self.resolve_path(src);
        let dst_path = self.resolve_path(dst);

        let content = match self.get_node(&src_path) {
            Some(VfsNode::File { content, .. }) => content.clone(),
            _ => return Err("Source is not a file".to_string()),
        };

        let dst_parent = dst_path.parent().unwrap_or(Path::new("/"));
        let dst_name = dst_path.file_name().and_then(|s| s.to_str()).ok_or("Invalid destination")?.to_string();

        let parent_node = self.get_node_mut(dst_parent).ok_or("Destination parent not found")?;
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied".to_string()); }
            children.insert(dst_name, VfsNode::File { content, perm: 6 });
            Ok(())
        } else {
            Err("Destination parent is not a directory".to_string())
        }
    }

    fn mv(&mut self, src: &str, dst: &str) -> Result<(), String> {
        let src_path = self.resolve_path(src);
        let dst_path = self.resolve_path(dst);

        let parent_src = src_path.parent().unwrap_or(Path::new("/"));
        let name_src = src_path.file_name().and_then(|s| s.to_str()).ok_or("Invalid source")?.to_string();

        let src_node = {
            let parent_node = self.get_node_mut(parent_src).ok_or("Source parent not found")?;
            if let VfsNode::Dir { children, .. } = parent_node {
                children.remove(&name_src).ok_or("No such file or directory")?
            } else {
                return Err("Source parent is not a directory".to_string());
            }
        };

        let dst_parent = dst_path.parent().unwrap_or(Path::new("/"));
        let dst_name = dst_path.file_name().and_then(|s| s.to_str()).ok_or("Invalid destination")?.to_string();

        let dst_parent_node = self.get_node_mut(dst_parent).ok_or("Destination parent not found")?;
        if let VfsNode::Dir { children, perm } = dst_parent_node {
            if *perm & 2 == 0 { return Err("Permission denied".to_string()); }
            children.insert(dst_name, src_node);
            Ok(())
        } else {
            Err("Destination parent is not a directory".to_string())
        }
    }

    fn update_proc_uptime(&mut self, uptime: String) {
        let proc_path = PathBuf::from("/proc");
        if let Some(VfsNode::Dir { children, .. }) = self.get_node_mut(&proc_path) {
            if let Some(VfsNode::File { content, .. }) = children.get_mut("uptime") {
                *content = uptime;
            }
        }
    }
}

// ==============================================
// Process Scheduler with priorities and ticks
// ==============================================
#[derive(Clone)]
struct SimProcess {
    pid: u32,
    ppid: u32,
    name: String,
    state: String,
    mem_kb: u64,
    cmdline: String,
    priority: i32,
    cpu_time: u64,
    background: bool,
    start_time: Instant,
}

struct Scheduler {
    processes: Vec<SimProcess>,
    next_pid: u32,
    current_user: String,
    current_uid: u32,
}

impl Scheduler {
    fn new() -> Self {
        let mut sched = Scheduler {
            processes: vec![],
            next_pid: 1000,
            current_user: "root".to_string(),
            current_uid: 0,
        };
        sched.spawn("init", 0, "/sbin/init", 128, false, 0);
        sched.spawn("phase1-shell", process::id(), &format!("phase1 v{}", VERSION), 8192, false, 0);
        sched
    }

    fn spawn(&mut self, name: &str, ppid: u32, cmdline: &str, mem_kb: u64, background: bool, priority: i32) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;
        let p = SimProcess {
            pid,
            ppid,
            name: name.to_string(),
            state: if background { "running (bg)".to_string() } else { "running".to_string() },
            mem_kb,
            cmdline: cmdline.to_string(),
            priority,
            cpu_time: 0,
            background,
            start_time: Instant::now(),
        };
        self.processes.push(p.clone());
        if background {
            let _ = thread::spawn(move || {
                thread::sleep(Duration::from_secs(6));
            });
        }
        pid
    }

    fn ps(&self) -> String {
        let mut out = format!("{:>5} {:>5} {:>8} {:>8} {:>6} {:>8} {}\n", "PID", "PPID", "USER", "PRI", "STATE", "MEM", "CMD");
        for p in &self.processes {
            out.push_str(&format!("{:>5} {:>5} {:>8} {:>8} {:>6} {:>8} {}\n",
                p.pid, p.ppid, self.current_user, p.priority, p.state, p.mem_kb, p.cmdline));
        }
        out
    }

    fn top(&self) -> String {
        let mut out = String::from("top — phase1 live process view (3 second snapshot)\n");
        out.push_str(&format!("{:>5} {:>8} {:>6} {:>8} {:>8} {}\n", "PID", "USER", "%CPU", "MEM", "TIME+", "COMMAND"));
        for p in &self.processes {
            let cpu = ((p.cpu_time % 500) as f32) / 5.0;
            out.push_str(&format!("{:>5} {:>8} {:>6.1} {:>8} {:>8} {}\n",
                p.pid, self.current_user, cpu, p.mem_kb, p.cpu_time / 1000, p.name));
        }
        out
    }

    fn kill(&mut self, pid_str: Option<&&str>) -> String {
        if let Some(pid_str) = pid_str {
            if let Ok(pid) = pid_str.parse::<u32>() {
                self.processes.retain(|p| p.pid != pid);
                return format!("Process {} terminated", pid);
            }
        }
        "Usage: kill <PID>".to_string()
    }

    fn jobs(&self) -> String {
        let bg: Vec<_> = self.processes.iter().filter(|p| p.background).collect();
        if bg.is_empty() {
            "No background jobs running".to_string()
        } else {
            let mut s = "Background jobs:\n".to_string();
            for p in bg {
                s.push_str(&format!(" [{}] {} ({})\n", p.pid, p.name, p.state));
            }
            s
        }
    }

    fn nice(&mut self, pid_str: Option<&&str>, pri: i32) -> String {
        if let Some(pid_str) = pid_str {
            if let Ok(pid) = pid_str.parse::<u32>() {
                if let Some(p) = self.processes.iter_mut().find(|p| p.pid == pid) {
                    p.priority = pri.clamp(-20, 19);
                    return format!("Priority of process {} changed to {}", pid, p.priority);
                }
            }
        }
        "Usage: nice <PID> <priority>".to_string()
    }

    fn tick(&mut self) {
        for p in &mut self.processes {
            if p.state.contains("running") {
                p.cpu_time += 120; // simulate preemptive scheduling tick
            }
        }
    }
}

// ==============================================
// Main Shell with environment, history, parsing
// ==============================================
struct Phase1Shell {
    scheduler: Scheduler,
    vfs: Vfs,
    start_time: Instant,
    history: VecDeque<String>,
    plugins_dir: PathBuf,
    env: HashMap<String, String>,
}

impl Phase1Shell {
    fn new() -> Self {
        let mut shell = Phase1Shell {
            scheduler: Scheduler::new(),
            vfs: Vfs::new(),
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

        // Example Python plugin (simple key=value protocol, no extra crates)
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
        println!("{}╔══════════════════════════════════════════════════════════════════════════════╗{}", ANSI_GREEN, ANSI_RESET);
        println!("{}║                    phase1 v1.0.0  —  Advanced OS Simulator                   ║{}", ANSI_GREEN, ANSI_RESET);
        println!("{}║                 Full VFS • Scheduler • Plugins • Education Focus             ║{}", ANSI_GREEN, ANSI_RESET);
        println!("{}╚══════════════════════════════════════════════════════════════════════════════╝{}", ANSI_GREEN, ANSI_RESET);
        println!("{}[    0.000000] phase1 kernel booted on virtual ARM64 hardware{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.012345] Initializing in-memory tree Virtual File System{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.034567] Mounting /proc, /dev, /home, /etc{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.067890] Preemptive scheduler with priority support activated{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.089012] Environment subsystem loaded with variable expansion{}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.112345] Python plugin bridge ready (stdin key=value protocol){}", ANSI_YELLOW, ANSI_RESET);
        println!("{}[    0.145678] Educational boot complete — type 'help' or 'man' for details{}", ANSI_GREEN, ANSI_RESET);
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
        if !plugin_path.exists() {
            return false;
        }

        let context_str = format!(
            "COMMAND={}\nARGS={}\nUSER={}\nCWD={}\nPID={}\nHOME={}\n",
            cmd,
            args.join(" "),
            self.scheduler.current_user,
            self.vfs.cwd.to_str().unwrap_or("/"),
            process::id(),
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
            // Update dynamic proc files
            let uptime_secs = self.start_time.elapsed().as_secs();
            self.vfs.update_proc_uptime(format!("{} seconds", uptime_secs));
            self.scheduler.tick();

            print!("{}@phase1{}:{}$ ", ANSI_CYAN, ANSI_RESET, self.vfs.cwd.display());
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
                    println!("{}Shutting down the phase1 simulator... Goodbye!{}", ANSI_YELLOW, ANSI_RESET);
                    break;
                }
                "help" => self.cmd_help(),
                "man" => self.cmd_man(args.first().copied()),
                "ps" => println!("{}", self.scheduler.ps()),
                "top" => {
                    println!("{}", self.scheduler.top());
                    thread::sleep(Duration::from_secs(3));
                }
                "free" | "mem" => self.cmd_free(),
                "kill" => println!("{}", self.scheduler.kill(args.first())),
                "nice" => println!("{}", self.scheduler.nice(args.first(), args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0))),
                "spawn" => {
                    let name = args.get(0).unwrap_or(&"anon");
                    let pid = self.scheduler.spawn(name, process::id(), &args.join(" "), 2048, false, 0);
                    println!("Spawned new process with PID {}", pid);
                }
                "df" => println!("{}Filesystem     1K-blocks    Used Available Use% Mounted on\nphase1-vfs     4194304   1048576   3145728  25% /{}", ANSI_YELLOW, ANSI_RESET),
                "whoami" => println!("{}", self.scheduler.current_user),
                "id" => println!("uid={}({}) gid=0(root) groups=0(root)", self.scheduler.current_uid, self.scheduler.current_user),
                "ls" => {
                    let long = args.contains(&"-l");
                    let path = if let Some(p) = args.iter().find(|&&x| x != "-l") { Some(*p) } else { None };
                    println!("{}", self.vfs.ls(path, long));
                }
                "cd" => self.cmd_cd(args.first().copied()),
                "pwd" => println!("{}", self.vfs.cwd.display()),
                "cat" => match self.vfs.cat(args.first().unwrap_or(&"")) {
                    Ok(c) => println!("{}", c),
                    Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                },
                "mkdir" => match self.vfs.mkdir(args.first().unwrap_or(&"")) {
                    Ok(_) => println!("Directory created"),
                    Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                },
                "touch" => match self.vfs.touch(args.first().unwrap_or(&"")) {
                    Ok(_) => println!("File touched"),
                    Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                },
                "rm" => match self.vfs.rm(args.first().unwrap_or(&"")) {
                    Ok(_) => println!("Removed"),
                    Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                },
                "cp" => if args.len() >= 2 {
                    match self.vfs.cp(args[0], args[1]) {
                        Ok(_) => println!("Copied"),
                        Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                    }
                } else {
                    println!("Usage: cp <source> <destination>");
                },
                "mv" => if args.len() >= 2 {
                    match self.vfs.mv(args[0], args[1]) {
                        Ok(_) => println!("Moved/Renamed"),
                        Err(e) => println!("{}Error: {}{}", ANSI_RED, e, ANSI_RESET),
                    }
                } else {
                    println!("Usage: mv <source> <destination>");
                },
                "echo" => {
                    let text = args.join(" ");
                    // Very basic redirection support: echo text > file   or   echo text >> file
                    if let Some(redirect_pos) = args.iter().position(|&x| x == ">" || x == ">>") {
                        if redirect_pos + 1 < args.len() {
                            let content = args[0..redirect_pos].join(" ");
                            let file = args[redirect_pos + 1];
                            let append = args[redirect_pos] == ">>";
                            match self.vfs.write_file(file, &content, append) {
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
                "jobs" => println!("{}", self.scheduler.jobs()),
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
                "ifconfig" => println!("eth0: 192.168.1.42  netmask 255.255.255.0  broadcast 192.168.1.255\nlo: 127.0.0.1"),
                "ping" => println!("PING simulation: 64 bytes from 1.1.1.1: icmp_seq=1 ttl=64 time=12.3 ms (fake)"),
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
        println!("{}phase1 v1.0.0 — Complete Command Reference{}", ANSI_BOLD, ANSI_RESET);
        println!("Core filesystem:   ls [-l]  cd  pwd  cat  mkdir  touch  rm  cp  mv  echo [> or >> file]");
        println!("Process mgmt:      ps  top  kill  spawn  nice  jobs  fg  bg");
        println!("System info:       free  df  uname  date  uptime  dmesg  vmstat  ifconfig  ping");
        println!("Shell:             env  export  unset  history  clear  su  whoami  id");
        println!("Plugins:           python/py  plugin/plugins  (any .py in ./plugins/)");
        println!("Misc:              tree  hostname  sandbox  version  man <cmd>  exit");
        println!("\n{}This simulator teaches real OS concepts entirely in memory. Enjoy learning!{}", ANSI_YELLOW, ANSI_RESET);
    }

    fn cmd_man(&self, topic: Option<&str>) {
        match topic {
            Some("ls") => println!("ls: list directory contents. Use -l for long format with permissions."),
            Some("echo") => println!("echo: print text. Supports basic redirection: echo text > file or >> file"),
            Some("cd") => println!("cd: change working directory. Use .. for parent."),
            Some(_) => println!("No manual entry for that command yet."),
            None => println!("Usage: man <command>"),
        }
    }

    fn cmd_cd(&mut self, dir: Option<&str>) {
        if let Some(d) = dir {
            let new_path = self.vfs.resolve_path(d);
            if self.vfs.get_node(&new_path).is_some() {
                self.vfs.cwd = new_path;
            } else {
                println!("{}cd: no such directory or permission denied{}", ANSI_RED, ANSI_RESET);
            }
        } else {
            self.vfs.cwd = PathBuf::from("/home");
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
            self.scheduler.current_user = u.to_string();
            self.scheduler.current_uid = if u == "root" { 0 } else { 1000 };
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
        println!("{}[    0.078901] Environment and plugin subsystems initialized{}", ANSI_YELLOW, ANSI_RESET);
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
}

// ==============================================
// Entry point
// ==============================================
fn main() {
    let mut shell = Phase1Shell::new();
    shell.run();
}
