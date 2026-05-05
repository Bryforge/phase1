use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{Duration, Instant};

pub const VERSION: &str = "3.6.0";
const MAX_PROCESSES: usize = 64;
const AUDIT_LIMIT: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProcessState {
    Running,
    RunningBg,
    Terminated,
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            ProcessState::Running => "run",
            ProcessState::RunningBg => "bg",
            ProcessState::Terminated => "term",
        };
        f.write_str(label)
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
    pub cpu_ticks: u64,
    pub background: bool,
    pub started: Instant,
    pub cr3: u64,
}

pub struct Scheduler {
    processes: Vec<SimProcess>,
    next_pid: u32,
    pub current_user: String,
    pub current_uid: u32,
    pub current_cr3: u64,
    pub cr4_pcide: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        let mut scheduler = Self {
            processes: Vec::with_capacity(MAX_PROCESSES),
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

    pub fn spawn(
        &mut self,
        name: &str,
        ppid: u32,
        cmdline: &str,
        mem_kb: u64,
        background: bool,
        priority: i32,
    ) -> Option<u32> {
        self.reap_terminated();
        if self.processes.len() >= MAX_PROCESSES {
            return None;
        }

        let pid = self.next_pid;
        self.next_pid = self.next_pid.saturating_add(1);
        self.processes.push(SimProcess {
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
            priority: priority.clamp(-20, 19),
            cpu_ticks: 0,
            background,
            started: Instant::now(),
            cr3: 0x10000 + (pid as u64 * 0x1000),
        });
        Some(pid)
    }

    pub fn ps(&self) -> String {
        let mut out = String::from("PID   PPID  USER   PRI STATE MEMKB TICKS AGE CR3       NAME CMD\n");
        for process in self.live_processes() {
            out.push_str(&format!(
                "{:<5} {:<5} {:<6} {:>3} {:<5} {:>5} {:>5} {:>3} 0x{:06x} {:<10} {}\n",
                process.pid,
                process.ppid,
                self.current_user,
                process.priority,
                process.state,
                process.mem_kb,
                process.cpu_ticks,
                process.started.elapsed().as_secs(),
                process.cr3,
                process.name,
                process.cmdline
            ));
        }
        out
    }

    pub fn top(&self) -> String {
        let mut out = format!("phase1 top v{}\n", VERSION);
        out.push_str(&self.ps());
        out
    }

    pub fn jobs(&self) -> String {
        let mut out = String::new();
        for process in self.live_processes().filter(|p| p.background) {
            out.push_str(&format!("[{}] {:<5} {}\n", process.pid, process.state, process.cmdline));
        }
        if out.is_empty() {
            "no background jobs\n".to_string()
        } else {
            out
        }
    }

    pub fn kill(&mut self, raw_pid: Option<&str>) -> String {
        let Some(pid) = raw_pid.and_then(|value| value.parse::<u32>().ok()) else {
            return "usage: kill <pid>".to_string();
        };
        if pid == 1000 {
            return "kill: refusing to terminate init".to_string();
        }
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = ProcessState::Terminated;
            return format!("process {} terminated", pid);
        }
        format!("kill: no such process: {}", pid)
    }

    pub fn nice(&mut self, raw_pid: Option<&str>, priority: Option<i32>) -> String {
        let Some(pid) = raw_pid.and_then(|value| value.parse::<u32>().ok()) else {
            return "usage: nice <pid> <priority>".to_string();
        };
        let Some(priority) = priority else {
            return "usage: nice <pid> <priority>".to_string();
        };
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.priority = priority.clamp(-20, 19);
            return format!("process {} priority set to {}", pid, process.priority);
        }
        format!("nice: no such process: {}", pid)
    }

    pub fn set_background(&mut self, raw_pid: Option<&str>, background: bool) -> String {
        let Some(pid) = raw_pid.and_then(|value| value.parse::<u32>().ok()) else {
            return if background { "usage: bg <pid>" } else { "usage: fg <pid>" }.to_string();
        };
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.background = background;
            process.state = if background { ProcessState::RunningBg } else { ProcessState::Running };
            return format!("process {} moved to {}", pid, if background { "background" } else { "foreground" });
        }
        format!("no such process: {}", pid)
    }

    pub fn get_cr3(&self) -> u64 {
        self.current_cr3
    }

    pub fn load_cr3(&mut self, value: u64) -> Result<(), String> {
        if !self.cr4_pcide && value % 4096 != 0 {
            return Err("CR3 must be 4KiB aligned unless PCIDE is enabled".to_string());
        }
        self.current_cr3 = value;
        Ok(())
    }

    pub fn cr4(&self) -> String {
        format!("CR4: PCIDE={}", if self.cr4_pcide { "on" } else { "off" })
    }

    pub fn set_pcide(&mut self, enabled: bool) {
        self.cr4_pcide = enabled;
    }

    pub fn tick(&mut self) {
        for process in &mut self.processes {
            if matches!(process.state, ProcessState::Running | ProcessState::RunningBg) {
                process.cpu_ticks = process.cpu_ticks.saturating_add(1);
            }
        }
    }

    fn live_processes(&self) -> impl Iterator<Item = &SimProcess> {
        self.processes.iter().filter(|p| p.state != ProcessState::Terminated)
    }

    fn reap_terminated(&mut self) {
        self.processes.retain(|p| p.state != ProcessState::Terminated);
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

    fn len(&self) -> usize {
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
        let mut root = HashMap::new();
        root.insert("bin".to_string(), dir(HashMap::new(), 0o555));
        root.insert("dev".to_string(), devfs());
        root.insert("etc".to_string(), etcfs());
        root.insert("home".to_string(), homefs());
        root.insert("proc".to_string(), procfs(0));
        root.insert("tmp".to_string(), dir(HashMap::new(), 0o777));
        root.insert("var".to_string(), varfs());
        Self { root: dir(root, 0o755), cwd: PathBuf::from("/") }
    }

    pub fn resolve_path(&self, raw: &str) -> PathBuf {
        let mut stack = if raw.starts_with('/') { Vec::new() } else { path_parts(&self.cwd) };
        for part in raw.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    stack.pop();
                }
                value => stack.push(value.to_string()),
            }
        }
        let mut path = PathBuf::from("/");
        for part in stack {
            path.push(part);
        }
        path
    }

    pub fn get_node(&self, path: &Path) -> Option<&VfsNode> {
        let mut current = &self.root;
        for part in path_parts(path) {
            current = match current {
                VfsNode::Dir { children, .. } => children.get(&part)?,
                VfsNode::File { .. } => return None,
            };
        }
        Some(current)
    }

    pub fn get_node_mut(&mut self, path: &Path) -> Option<&mut VfsNode> {
        let mut current = &mut self.root;
        for part in path_parts(path) {
            current = match current {
                VfsNode::Dir { children, .. } => children.get_mut(&part)?,
                VfsNode::File { .. } => return None,
            };
        }
        Some(current)
    }

    pub fn mkdir(&mut self, path: &str) -> Result<(), String> {
        self.insert(path, dir(HashMap::new(), 0o755), false)
    }

    pub fn touch(&mut self, path: &str) -> Result<(), String> {
        let resolved = self.resolve_path(path);
        if self.get_node(&resolved).is_some() {
            return Ok(());
        }
        self.insert(path, file(String::new(), 0o644), false)
    }

    pub fn cat(&self, path: &str) -> Result<String, String> {
        let resolved = self.resolve_path(path);
        match self.get_node(&resolved) {
            Some(VfsNode::File { content, .. }) => Ok(content.clone()),
            Some(VfsNode::Dir { .. }) => Err(format!("{} is a directory", resolved.display())),
            None => Err(format!("no such file: {}", path)),
        }
    }

    pub fn write_file(&mut self, path: &str, content: &str, append: bool) -> Result<(), String> {
        let resolved = self.resolve_path(path);
        if let Some(node) = self.get_node_mut(&resolved) {
            match node {
                VfsNode::File { content: existing, perm } => {
                    if *perm & 0o200 == 0 {
                        return Err("permission denied".to_string());
                    }
                    if append {
                        existing.push_str(content);
                    } else {
                        *existing = content.to_string();
                    }
                    return Ok(());
                }
                VfsNode::Dir { .. } => return Err("cannot write to directory".to_string()),
            }
        }
        self.insert(path, file(content.to_string(), 0o644), false)
    }

    pub fn ls(&self, path: Option<&str>, long: bool) -> String {
        let target = path.unwrap_or(".");
        let resolved = self.resolve_path(target);
        match self.get_node(&resolved) {
            Some(node @ VfsNode::File { .. }) => {
                let name = resolved.file_name().and_then(|s| s.to_str()).unwrap_or(target);
                if long { format_entry(name, node) } else { format!("{}\n", name) }
            }
            Some(VfsNode::Dir { children, .. }) => {
                let mut names: Vec<_> = children.keys().cloned().collect();
                names.sort();
                let mut out = if long { "total 0\n".to_string() } else { String::new() };
                for name in names {
                    let node = &children[&name];
                    if long {
                        out.push_str(&format_entry(&name, node));
                    } else {
                        out.push_str(&name);
                        out.push('\n');
                    }
                }
                out
            }
            None => format!("ls: cannot access '{}': no such file or directory\n", target),
        }
    }

    pub fn rm(&mut self, path: &str) -> Result<(), String> {
        let (parent, name) = parent_and_name(&self.resolve_path(path))?;
        match self.get_node_mut(&parent) {
            Some(VfsNode::Dir { children, perm }) => {
                if *perm & 0o200 == 0 {
                    return Err("permission denied".to_string());
                }
                children.remove(&name).map(|_| ()).ok_or_else(|| "no such file".to_string())
            }
            _ => Err("parent is not a directory".to_string()),
        }
    }

    pub fn cp(&mut self, src: &str, dst: &str) -> Result<(), String> {
        let src_path = self.resolve_path(src);
        let src_node = self.get_node(&src_path).cloned().ok_or_else(|| "source not found".to_string())?;
        if src_node.is_dir() {
            return Err("cp currently supports files only".to_string());
        }
        let dst_path = self.destination_path(dst, &src_path)?;
        self.insert_at(dst_path, src_node, true)
    }

    pub fn mv(&mut self, src: &str, dst: &str) -> Result<(), String> {
        let src_path = self.resolve_path(src);
        let node = self.get_node(&src_path).cloned().ok_or_else(|| "source not found".to_string())?;
        let dst_path = self.destination_path(dst, &src_path)?;
        self.insert_at(dst_path, node, true)?;
        self.rm(src)
    }

    pub fn tree(&self) -> String {
        let mut out = String::from("/\n");
        draw_tree(&self.root, "", &mut out);
        out
    }

    pub fn update_proc(&mut self, uptime_secs: u64) {
        if let Some(VfsNode::Dir { children, .. }) = self.get_node_mut(Path::new("/proc")) {
            children.insert("uptime".to_string(), file(format!("{} seconds\n", uptime_secs), 0o444));
            children.insert("version".to_string(), file(format!("phase1 {}\n", VERSION), 0o444));
        }
    }

    fn destination_path(&self, dst: &str, src_path: &Path) -> Result<PathBuf, String> {
        let dst_path = self.resolve_path(dst);
        match self.get_node(&dst_path) {
            Some(VfsNode::Dir { .. }) => {
                let name = src_path.file_name().and_then(|s| s.to_str()).ok_or_else(|| "invalid source".to_string())?;
                Ok(dst_path.join(name))
            }
            _ => Ok(dst_path),
        }
    }

    fn insert(&mut self, path: &str, node: VfsNode, overwrite: bool) -> Result<(), String> {
        self.insert_at(self.resolve_path(path), node, overwrite)
    }

    fn insert_at(&mut self, path: PathBuf, node: VfsNode, overwrite: bool) -> Result<(), String> {
        let (parent, name) = parent_and_name(&path)?;
        match self.get_node_mut(&parent) {
            Some(VfsNode::Dir { children, perm }) => {
                if *perm & 0o200 == 0 {
                    return Err("permission denied".to_string());
                }
                if !overwrite && children.contains_key(&name) {
                    return Err("file exists".to_string());
                }
                children.insert(name, node);
                Ok(())
            }
            Some(VfsNode::File { .. }) => Err("parent is not a directory".to_string()),
            None => Err("parent not found".to_string()),
        }
    }
}

pub struct AuditLog {
    events: VecDeque<String>,
}

impl AuditLog {
    pub fn new() -> Self {
        Self { events: VecDeque::with_capacity(AUDIT_LIMIT) }
    }

    pub fn record(&mut self, event: impl Into<String>) {
        if self.events.len() == AUDIT_LIMIT {
            self.events.pop_front();
        }
        self.events.push_back(event.into());
    }

    pub fn dump(&self) -> String {
        if self.events.is_empty() {
            return "audit log empty\n".to_string();
        }
        self.events.iter().enumerate().map(|(idx, event)| format!("{:04} {}\n", idx, event)).collect()
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
    devices: Vec<PcieDevice>,
}

impl PcieManager {
    pub fn new() -> Self {
        Self {
            devices: vec![
                PcieDevice { bus: 0, device: 0, function: 0, vendor_id: 0x8086, device_id: 0x1237, name: "Intel 440FX host bridge".to_string() },
                PcieDevice { bus: 0, device: 3, function: 0, vendor_id: 0x8086, device_id: 0x100e, name: "Intel 82540EM network adapter".to_string() },
            ],
        }
    }

    pub fn lspci(&self) -> String {
        let mut out = String::new();
        for dev in &self.devices {
            out.push_str(&format!("{:02x}:{:02x}.{} {} [{:04x}:{:04x}]\n", dev.bus, dev.device, dev.function, dev.name, dev.vendor_id, dev.device_id));
        }
        out
    }

    pub fn pcie_info(&self) -> String {
        format!("{} PCIe devices\n{}", self.devices.len(), self.lspci())
    }
}

pub struct Kernel {
    pub vfs: Vfs,
    pub scheduler: Scheduler,
    pub pcie: PcieManager,
    pub audit: AuditLog,
    booted: Instant,
}

impl Kernel {
    pub fn new() -> Self {
        let mut kernel = Self { vfs: Vfs::new(), scheduler: Scheduler::new(), pcie: PcieManager::new(), audit: AuditLog::new(), booted: Instant::now() };
        kernel.audit.record("kernel.boot version=3.6.0");
        kernel
    }

    pub fn tick(&mut self) {
        self.scheduler.tick();
        self.vfs.update_proc(self.uptime().as_secs());
    }

    pub fn uptime(&self) -> Duration {
        self.booted.elapsed()
    }

    pub fn sys_read(&mut self, path: &str) -> Result<String, String> {
        self.audit.record(format!("sys.read path={}", path));
        self.vfs.cat(path)
    }

    pub fn sys_write(&mut self, path: &str, content: &str, append: bool) -> Result<(), String> {
        self.audit.record(format!("sys.write path={} append={}", path, append));
        self.vfs.write_file(path, content, append)
    }

    pub fn sys_spawn(&mut self, name: &str, cmdline: &str, background: bool) -> Result<u32, String> {
        self.audit.record(format!("sys.spawn name={} bg={}", name, background));
        self.scheduler.spawn(name, process::id(), cmdline, 4096, background, 0).ok_or_else(|| "process table full".to_string())
    }

    pub fn sys_kill(&mut self, pid: Option<&str>) -> String {
        self.audit.record(format!("sys.kill pid={}", pid.unwrap_or("?")));
        self.scheduler.kill(pid)
    }
}

fn file(content: String, perm: u16) -> VfsNode {
    VfsNode::File { content, perm }
}

fn dir(children: HashMap<String, VfsNode>, perm: u16) -> VfsNode {
    VfsNode::Dir { children, perm }
}

fn procfs(uptime: u64) -> VfsNode {
    let mut children = HashMap::new();
    children.insert("cpuinfo".to_string(), file("processor: 0\nmodel: phase1 virtual cpu\n".to_string(), 0o444));
    children.insert("meminfo".to_string(), file("MemTotal: 4194304 kB\nMemFree: 2097152 kB\n".to_string(), 0o444));
    children.insert("uptime".to_string(), file(format!("{} seconds\n", uptime), 0o444));
    children.insert("version".to_string(), file(format!("phase1 {}\n", VERSION), 0o444));
    dir(children, 0o555)
}

fn homefs() -> VfsNode {
    let mut children = HashMap::new();
    children.insert("readme.txt".to_string(), file(format!("phase1 {}\nType help to begin.\n", VERSION), 0o644));
    dir(children, 0o777)
}

fn etcfs() -> VfsNode {
    let mut children = HashMap::new();
    children.insert("passwd".to_string(), file("root:x:0:0:root:/root:/bin/sh\nuser:x:1000:1000:user:/home:/bin/sh\n".to_string(), 0o444));
    dir(children, 0o555)
}

fn devfs() -> VfsNode {
    let mut children = HashMap::new();
    children.insert("null".to_string(), file(String::new(), 0o666));
    children.insert("tty".to_string(), file("phase1 tty0\n".to_string(), 0o666));
    dir(children, 0o555)
}

fn varfs() -> VfsNode {
    let mut log = HashMap::new();
    log.insert("boot.log".to_string(), file("phase1 boot nominal\n".to_string(), 0o644));
    let mut var = HashMap::new();
    var.insert("log".to_string(), dir(log, 0o755));
    dir(var, 0o755)
}

fn path_parts(path: &Path) -> Vec<String> {
    path.iter().filter_map(|part| part.to_str()).filter(|part| !part.is_empty() && *part != "/").map(ToOwned::to_owned).collect()
}

fn parent_and_name(path: &Path) -> Result<(PathBuf, String), String> {
    let parent = path.parent().unwrap_or_else(|| Path::new("/")).to_path_buf();
    let name = path.file_name().and_then(|part| part.to_str()).ok_or_else(|| "invalid path".to_string())?.to_string();
    Ok((parent, name))
}

fn format_entry(name: &str, node: &VfsNode) -> String {
    let kind = if node.is_dir() { 'd' } else { '-' };
    format!("{}{} root root {:>5} {}\n", kind, perm_string(node.perm()), node.len(), name)
}

fn perm_string(perm: u16) -> String {
    let bits = [(0o400, 'r'), (0o200, 'w'), (0o100, 'x'), (0o040, 'r'), (0o020, 'w'), (0o010, 'x'), (0o004, 'r'), (0o002, 'w'), (0o001, 'x')];
    bits.iter().map(|(bit, ch)| if perm & bit != 0 { *ch } else { '-' }).collect()
}

fn draw_tree(node: &VfsNode, prefix: &str, out: &mut String) {
    if let VfsNode::Dir { children, .. } = node {
        let mut names: Vec<_> = children.keys().cloned().collect();
        names.sort();
        for (idx, name) in names.iter().enumerate() {
            let last = idx + 1 == names.len();
            out.push_str(prefix);
            out.push_str(if last { "`-- " } else { "|-- " });
            out.push_str(name);
            out.push('\n');
            let next = format!("{}{}", prefix, if last { "    " } else { "|   " });
            draw_tree(&children[name], &next, out);
        }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PcieManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Kernel, Vfs, VERSION};

    #[test]
    fn vfs_write_and_read_round_trip() {
        let mut vfs = Vfs::new();
        vfs.write_file("/home/test.txt", "ok", false).unwrap();
        assert_eq!(vfs.cat("/home/test.txt").unwrap(), "ok");
    }

    #[test]
    fn syscalls_record_audit_events() {
        let mut kernel = Kernel::new();
        kernel.sys_write("/home/a.txt", "hello", false).unwrap();
        let audit = kernel.audit.dump();
        assert!(audit.contains("sys.write"));
    }

    #[test]
    fn release_version_is_current() {
        assert_eq!(VERSION, "3.6.0");
    }
}
