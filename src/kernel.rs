use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

pub const VERSION: &str = "3.3.2";
const MAX_PROCS: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    RunningBg,
    Blocked,
    Zombie,
    Terminated,
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self {
            ProcessState::Ready => "READY",
            ProcessState::Running => "R",
            ProcessState::RunningBg => "R(bg)",
            ProcessState::Blocked => "S",
            ProcessState::Zombie => "Z",
            ProcessState::Terminated => "T",
        };
        f.write_str(state)
    }
}

#[derive(Clone, Debug)]
pub struct SimProcess {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub state: ProcessState,
    pub mem_kb: u64,
    pub cmdline: String,
    pub priority: i32,
    pub cpu_time: u64,
    pub background: bool,
    pub start_time: Instant,
    pub cr3: u64,
}

pub struct Scheduler {
    pub processes: [Option<SimProcess>; MAX_PROCS],
    pub next_pid: u32,
    pub current_user: String,
    pub current_uid: u32,
    pub current_cr3: u64,
    pub cr4_pcide: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        let mut scheduler = Self {
            processes: std::array::from_fn(|_| None),
            next_pid: 1000,
            current_user: "root".to_string(),
            current_uid: 0,
            current_cr3: 0x1000,
            cr4_pcide: false,
        };

        let _ = scheduler.spawn("init", 0, "/sbin/init", 128, false, 0);
        let _ = scheduler.spawn("phase1-shell", process::id(), "phase1 shell", 8192, false, 0);
        scheduler
    }

    fn find_free_slot(&self) -> Option<usize> {
        self.processes
            .iter()
            .position(|proc| proc.as_ref().is_none_or(|p| p.state == ProcessState::Terminated))
    }

    pub fn spawn(
        &mut self,
        name: &str,
        ppid: u32,
        cmdline: &str,
        mem_kb: u64,
        background: bool,
        priority: i32,
    ) -> Option<u32> {
        let idx = self.find_free_slot()?;
        let pid = self.next_pid;
        self.next_pid = self.next_pid.saturating_add(1);

        self.processes[idx] = Some(SimProcess {
            pid,
            ppid,
            name: name.to_string(),
            state: if background {
                ProcessState::RunningBg
            } else {
                ProcessState::Running
            },
            mem_kb,
            cmdline: cmdline.to_string(),
            priority,
            cpu_time: 0,
            background,
            start_time: Instant::now(),
            cr3: 0x10000 + (pid as u64) * 0x1000,
        });

        Some(pid)
    }

    pub fn ps(&self) -> String {
        let mut out = "PID    PPID   USER      PRI STATE  MEM(KB) CPU  AGE(s) CR3        CMD\n".to_string();
        for p in self.live_processes() {
            out.push_str(&format!(
                "{:<6} {:<6} {:<9} {:>3} {:<6} {:>7} {:>4} {:>6} 0x{:08x} {}\n",
                p.pid,
                p.ppid,
                self.current_user,
                p.priority,
                p.state,
                p.mem_kb,
                p.cpu_time,
                p.start_time.elapsed().as_secs(),
                p.cr3,
                p.cmdline
            ));
        }
        out
    }

    pub fn top(&self) -> String {
        format!("top — phase1 {}\n{}", VERSION, self.ps())
    }

    pub fn jobs(&self) -> String {
        let mut out = String::new();
        for p in self.live_processes().filter(|p| p.background) {
            out.push_str(&format!("[{}] {:<8} {}\n", p.pid, p.state, p.cmdline));
        }
        if out.is_empty() {
            "No background jobs\n".to_string()
        } else {
            out
        }
    }

    pub fn kill(&mut self, pid: Option<&str>) -> String {
        let Some(pid) = pid.and_then(|raw| raw.parse::<u32>().ok()) else {
            return "Usage: kill <pid>".to_string();
        };

        if pid == 1000 {
            return "kill: refusing to terminate init".to_string();
        }

        for p in self.processes.iter_mut().flatten() {
            if p.pid == pid && p.state != ProcessState::Terminated {
                p.state = ProcessState::Terminated;
                return format!("kill: process {} terminated", pid);
            }
        }

        format!("kill: no such process: {}", pid)
    }

    pub fn nice(&mut self, pid: Option<&str>, prio: Option<i32>) -> String {
        let Some(pid) = pid.and_then(|raw| raw.parse::<u32>().ok()) else {
            return "Usage: nice <pid> <priority>".to_string();
        };
        let Some(prio) = prio else {
            return "Usage: nice <pid> <priority>".to_string();
        };

        for p in self.processes.iter_mut().flatten() {
            if p.pid == pid && p.state != ProcessState::Terminated {
                p.priority = prio.clamp(-20, 19);
                return format!("nice: process {} priority set to {}", pid, p.priority);
            }
        }

        format!("nice: no such process: {}", pid)
    }

    pub fn set_background(&mut self, pid: Option<&str>, background: bool) -> String {
        let Some(pid) = pid.and_then(|raw| raw.parse::<u32>().ok()) else {
            return if background {
                "Usage: bg <pid>".to_string()
            } else {
                "Usage: fg <pid>".to_string()
            };
        };

        for p in self.processes.iter_mut().flatten() {
            if p.pid == pid && p.state != ProcessState::Terminated {
                p.background = background;
                p.state = if background {
                    ProcessState::RunningBg
                } else {
                    ProcessState::Running
                };
                return format!(
                    "{}: process {} moved to {}",
                    if background { "bg" } else { "fg" },
                    pid,
                    if background { "background" } else { "foreground" }
                );
            }
        }

        format!("{}: no such process: {}", if background { "bg" } else { "fg" }, pid)
    }

    pub fn get_cr3(&self) -> u64 {
        self.current_cr3
    }

    pub fn load_cr3(&mut self, val: u64) -> Result<(), String> {
        if !self.cr4_pcide && val % 4096 != 0 {
            return Err("CR3 must be 4KiB-aligned unless PCIDE is enabled".to_string());
        }
        self.current_cr3 = val;
        Ok(())
    }

    pub fn cr4(&self) -> String {
        format!(
            "CR4: PCIDE={}",
            if self.cr4_pcide { "enabled" } else { "disabled" }
        )
    }

    pub fn set_pcide(&mut self, enabled: bool) {
        self.cr4_pcide = enabled;
    }

    pub fn tick(&mut self, _uptime: u64) {
        for p in self.processes.iter_mut().flatten() {
            if matches!(p.state, ProcessState::Running | ProcessState::RunningBg) {
                p.cpu_time = p.cpu_time.saturating_add(1);
            }
        }
    }

    fn live_processes(&self) -> impl Iterator<Item = &SimProcess> {
        self.processes
            .iter()
            .flatten()
            .filter(|p| p.state != ProcessState::Terminated)
    }
}

#[derive(Clone, Debug)]
pub enum VfsNode {
    File { content: String, perm: u16 },
    Dir { children: HashMap<String, VfsNode>, perm: u16 },
}

impl VfsNode {
    fn is_dir(&self) -> bool {
        matches!(self, VfsNode::Dir { .. })
    }

    fn file_len(&self) -> usize {
        match self {
            VfsNode::File { content, .. } => content.len(),
            VfsNode::Dir { children, .. } => children.len(),
        }
    }

    fn perm(&self) -> u16 {
        match self {
            VfsNode::File { perm, .. } | VfsNode::Dir { perm, .. } => *perm,
        }
    }
}

pub struct Vfs {
    pub root: VfsNode,
    pub cwd: PathBuf,
}

impl Vfs {
    pub fn new() -> Self {
        let mut root_children = HashMap::new();

        let mut proc_children = HashMap::new();
        proc_children.insert(
            "cpuinfo".to_string(),
            VfsNode::File {
                content: "processor\t: 0\nmodel name\t: phase1 Virtual Cortex-A76\n".to_string(),
                perm: 0o444,
            },
        );
        proc_children.insert(
            "meminfo".to_string(),
            VfsNode::File {
                content: "MemTotal: 4194304 kB\nMemFree: 2097152 kB\n".to_string(),
                perm: 0o444,
            },
        );
        proc_children.insert(
            "uptime".to_string(),
            VfsNode::File {
                content: "0 seconds".to_string(),
                perm: 0o444,
            },
        );
        proc_children.insert(
            "version".to_string(),
            VfsNode::File {
                content: format!("phase1 {}\n", VERSION),
                perm: 0o444,
            },
        );
        root_children.insert(
            "proc".to_string(),
            VfsNode::Dir {
                children: proc_children,
                perm: 0o555,
            },
        );

        let mut home_children = HashMap::new();
        home_children.insert(
            "readme.txt".to_string(),
            VfsNode::File {
                content: format!("Welcome to phase1 {}\nType `help` to begin.\n", VERSION),
                perm: 0o644,
            },
        );
        root_children.insert(
            "home".to_string(),
            VfsNode::Dir {
                children: home_children,
                perm: 0o777,
            },
        );

        let mut etc_children = HashMap::new();
        etc_children.insert(
            "passwd".to_string(),
            VfsNode::File {
                content: "root:x:0:0:root:/root:/bin/sh\nuser:x:1000:1000:user:/home:/bin/sh\n"
                    .to_string(),
                perm: 0o444,
            },
        );
        root_children.insert(
            "etc".to_string(),
            VfsNode::Dir {
                children: etc_children,
                perm: 0o555,
            },
        );

        let mut dev_children = HashMap::new();
        dev_children.insert(
            "null".to_string(),
            VfsNode::File {
                content: String::new(),
                perm: 0o666,
            },
        );
        root_children.insert(
            "dev".to_string(),
            VfsNode::Dir {
                children: dev_children,
                perm: 0o555,
            },
        );

        root_children.insert(
            "bin".to_string(),
            VfsNode::Dir {
                children: HashMap::new(),
                perm: 0o555,
            },
        );

        Self {
            root: VfsNode::Dir {
                children: root_children,
                perm: 0o755,
            },
            cwd: PathBuf::from("/"),
        }
    }

    pub fn resolve_path(&self, path: &str) -> PathBuf {
        let mut parts: Vec<String> = if path.starts_with('/') {
            Vec::new()
        } else {
            Self::path_parts(&self.cwd)
        };

        for seg in path.split('/') {
            match seg {
                "" | "." => {}
                ".." => {
                    parts.pop();
                }
                other => parts.push(other.to_string()),
            }
        }

        let mut resolved = PathBuf::from("/");
        for part in parts {
            resolved.push(part);
        }
        resolved
    }

    pub fn get_node(&self, path: &Path) -> Option<&VfsNode> {
        let mut current = &self.root;
        for name in Self::path_parts(path) {
            current = match current {
                VfsNode::Dir { children, .. } => children.get(&name)?,
                VfsNode::File { .. } => return None,
            };
        }
        Some(current)
    }

    pub fn get_node_mut(&mut self, path: &Path) -> Option<&mut VfsNode> {
        let mut current = &mut self.root;
        for name in Self::path_parts(path) {
            current = match current {
                VfsNode::Dir { children, .. } => children.get_mut(&name)?,
                VfsNode::File { .. } => return None,
            };
        }
        Some(current)
    }

    pub fn mkdir(&mut self, path_str: &str) -> Result<(), String> {
        self.insert_node(path_str, VfsNode::Dir { children: HashMap::new(), perm: 0o755 }, false)
    }

    pub fn touch(&mut self, path_str: &str) -> Result<(), String> {
        let path = self.resolve_path(path_str);
        if self.get_node(&path).is_some() {
            return Ok(());
        }
        self.insert_node(
            path_str,
            VfsNode::File {
                content: String::new(),
                perm: 0o644,
            },
            false,
        )
    }

    pub fn cat(&self, path_str: &str) -> Result<String, String> {
        let path = self.resolve_path(path_str);
        match self.get_node(&path) {
            Some(VfsNode::File { content, .. }) => Ok(content.clone()),
            Some(VfsNode::Dir { .. }) => Err(format!("{} is a directory", path.display())),
            None => Err(format!("No such file: {}", path_str)),
        }
    }

    pub fn write_file(&mut self, path_str: &str, content: &str, append: bool) -> Result<(), String> {
        let path = self.resolve_path(path_str);

        if let Some(node) = self.get_node_mut(&path) {
            match node {
                VfsNode::File {
                    content: existing,
                    perm,
                } => {
                    if *perm & 0o200 == 0 {
                        return Err("Permission denied".to_string());
                    }
                    if append {
                        existing.push_str(content);
                    } else {
                        *existing = content.to_string();
                    }
                    return Ok(());
                }
                VfsNode::Dir { .. } => return Err("Cannot write to a directory".to_string()),
            }
        }

        self.insert_node(
            path_str,
            VfsNode::File {
                content: content.to_string(),
                perm: 0o644,
            },
            false,
        )
    }

    pub fn ls(&self, path_str: Option<&str>, long: bool) -> String {
        let path = self.resolve_path(path_str.unwrap_or("."));
        match self.get_node(&path) {
            Some(VfsNode::File { .. }) if long => self.format_entry(
                path.file_name().and_then(|s| s.to_str()).unwrap_or(""),
                self.get_node(&path).unwrap(),
            ),
            Some(VfsNode::File { .. }) => format!(
                "{}\n",
                path.file_name().and_then(|s| s.to_str()).unwrap_or("")
            ),
            Some(VfsNode::Dir { children, .. }) => {
                let mut names: Vec<_> = children.keys().cloned().collect();
                names.sort();

                let mut out = if long { "total 0\n".to_string() } else { String::new() };
                for name in names {
                    let node = &children[&name];
                    if long {
                        out.push_str(&self.format_entry(&name, node));
                    } else {
                        out.push_str(&format!("{}\n", name));
                    }
                }
                out
            }
            None => format!("ls: cannot access '{}': No such file or directory", path_str.unwrap_or(".")),
        }
    }

    pub fn rm(&mut self, path_str: &str) -> Result<(), String> {
        let (parent, name) = self.parent_and_name(path_str)?;
        if name.is_empty() {
            return Err("Refusing to remove root".to_string());
        }

        match self.get_node_mut(&parent) {
            Some(VfsNode::Dir { children, perm }) => {
                if *perm & 0o200 == 0 {
                    return Err("Permission denied".to_string());
                }
                children
                    .remove(&name)
                    .map(|_| ())
                    .ok_or_else(|| "No such file or directory".to_string())
            }
            _ => Err("Parent is not a directory".to_string()),
        }
    }

    pub fn cp(&mut self, src: &str, dst: &str) -> Result<(), String> {
        let src_path = self.resolve_path(src);
        let cloned = match self.get_node(&src_path) {
            Some(VfsNode::File { content, perm }) => VfsNode::File {
                content: content.clone(),
                perm: *perm,
            },
            Some(VfsNode::Dir { .. }) => return Err("cp currently supports files only".to_string()),
            None => return Err("Source not found".to_string()),
        };

        let dst_path = self.resolve_path(dst);
        let final_dst = match self.get_node(&dst_path) {
            Some(VfsNode::Dir { .. }) => {
                let name = src_path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| "Invalid source".to_string())?;
                dst_path.join(name)
            }
            _ => dst_path,
        };

        self.insert_node_at(final_dst, cloned, true)
    }

    pub fn mv(&mut self, src: &str, dst: &str) -> Result<(), String> {
        let (src_parent, src_name) = self.parent_and_name(src)?;
        let node = match self.get_node_mut(&src_parent) {
            Some(VfsNode::Dir { children, perm }) => {
                if *perm & 0o200 == 0 {
                    return Err("Permission denied".to_string());
                }
                children
                    .remove(&src_name)
                    .ok_or_else(|| "Source not found".to_string())?
            }
            _ => return Err("Source parent is not a directory".to_string()),
        };

        let dst_path = self.resolve_path(dst);
        let final_dst = match self.get_node(&dst_path) {
            Some(VfsNode::Dir { .. }) => dst_path.join(src_name),
            _ => dst_path,
        };

        if let Err(err) = self.insert_node_at(final_dst, node.clone(), true) {
            let _ = self.insert_node_at(self.resolve_path(src), node, true);
            return Err(err);
        }

        Ok(())
    }

    pub fn tree(&self) -> String {
        let mut out = "/\n".to_string();
        self.tree_node(&self.root, "", &mut out);
        out
    }

    fn tree_node(&self, node: &VfsNode, prefix: &str, out: &mut String) {
        if let VfsNode::Dir { children, .. } = node {
            let mut names: Vec<_> = children.keys().cloned().collect();
            names.sort();
            for (idx, name) in names.iter().enumerate() {
                let last = idx + 1 == names.len();
                let connector = if last { "└── " } else { "├── " };
                out.push_str(prefix);
                out.push_str(connector);
                out.push_str(name);
                out.push('\n');

                let next_prefix = format!("{}{}", prefix, if last { "    " } else { "│   " });
                self.tree_node(&children[name], &next_prefix, out);
            }
        }
    }

    fn insert_node(&mut self, path_str: &str, node: VfsNode, overwrite: bool) -> Result<(), String> {
        self.insert_node_at(self.resolve_path(path_str), node, overwrite)
    }

    fn insert_node_at(&mut self, path: PathBuf, node: VfsNode, overwrite: bool) -> Result<(), String> {
        let parent = path.parent().unwrap_or_else(|| Path::new("/")).to_path_buf();
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| "Invalid name".to_string())?
            .to_string();

        if name.is_empty() {
            return Err("Invalid name".to_string());
        }

        match self.get_node_mut(&parent) {
            Some(VfsNode::Dir { children, perm }) => {
                if *perm & 0o200 == 0 {
                    return Err("Permission denied".to_string());
                }
                if !overwrite && children.contains_key(&name) {
                    return Err("File exists".to_string());
                }
                children.insert(name, node);
                Ok(())
            }
            Some(VfsNode::File { .. }) => Err("Parent is not a directory".to_string()),
            None => Err("Parent not found".to_string()),
        }
    }

    fn parent_and_name(&self, path_str: &str) -> Result<(PathBuf, String), String> {
        let path = self.resolve_path(path_str);
        let parent = path.parent().unwrap_or_else(|| Path::new("/")).to_path_buf();
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| "Invalid path".to_string())?
            .to_string();
        Ok((parent, name))
    }

    fn path_parts(path: &Path) -> Vec<String> {
        path.iter()
            .filter_map(|part| part.to_str())
            .filter(|part| !part.is_empty() && *part != "/")
            .map(ToOwned::to_owned)
            .collect()
    }

    fn format_entry(&self, name: &str, node: &VfsNode) -> String {
        let kind = if node.is_dir() { 'd' } else { '-' };
        let mode = Self::perm_string(node.perm());
        format!(
            "{}{} 1 root root {:>6} now {}\n",
            kind,
            mode,
            node.file_len(),
            name
        )
    }

    fn perm_string(perm: u16) -> String {
        let flags = [
            (0o400, 'r'),
            (0o200, 'w'),
            (0o100, 'x'),
            (0o040, 'r'),
            (0o020, 'w'),
            (0o010, 'x'),
            (0o004, 'r'),
            (0o002, 'w'),
            (0o001, 'x'),
        ];

        flags
            .iter()
            .map(|(bit, ch)| if perm & bit != 0 { *ch } else { '-' })
            .collect()
    }
}

#[derive(Clone)]
pub struct PcieDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub name: String,
}

pub struct PcieManager {
    pub devices: Vec<PcieDevice>,
}

impl PcieManager {
    pub fn new() -> Self {
        Self {
            devices: vec![
                PcieDevice {
                    bus: 0,
                    device: 0,
                    function: 0,
                    vendor_id: 0x8086,
                    device_id: 0x1237,
                    name: "Intel 440FX host bridge".to_string(),
                },
                PcieDevice {
                    bus: 0,
                    device: 3,
                    function: 0,
                    vendor_id: 0x8086,
                    device_id: 0x100e,
                    name: "Intel 82540EM network adapter".to_string(),
                },
            ],
        }
    }

    pub fn lspci(&self) -> String {
        let mut out = String::new();
        for dev in &self.devices {
            out.push_str(&format!(
                "{:02x}:{:02x}.{} {} [{:04x}:{:04x}]\n",
                dev.bus, dev.device, dev.function, dev.name, dev.vendor_id, dev.device_id
            ));
        }
        out
    }

    pub fn pcie_info(&self) -> String {
        format!("Found {} PCIe devices\n{}", self.devices.len(), self.lspci())
    }
}

pub struct Kernel {
    pub vfs: Vfs,
    pub scheduler: Scheduler,
    pub pcie: PcieManager,
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            vfs: Vfs::new(),
            scheduler: Scheduler::new(),
            pcie: PcieManager::new(),
        }
    }

    pub fn tick(&mut self, uptime_secs: u64) {
        self.scheduler.tick(uptime_secs);

        if let VfsNode::Dir { children, .. } = &mut self.vfs.root {
            if let Some(VfsNode::Dir {
                children: proc_children,
                ..
            }) = children.get_mut("proc")
            {
                if let Some(VfsNode::File { content, .. }) = proc_children.get_mut("uptime") {
                    *content = format!("{} seconds", uptime_secs);
                }
            }
        }
    }
}

trait OptionExt<T> {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T> OptionExt<T> for Option<T> {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            None => true,
            Some(value) => f(value),
        }
    }
}
