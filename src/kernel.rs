// src/kernel.rs — Core OS primitives for phase1 v2.0.0
// VFS, scheduler, PCIe, CR3/CR4/PCID. Optimized and practical.
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use chrono::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ProcessState {
    Void,
    New,
    Ready,
    Running,
    RunningBg,
    Blocked,
    Zombie,
    Terminated,
}

impl std::fmt::Display for ProcessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessState::Void => write!(f, "VOID"),
            ProcessState::New => write!(f, "NEW"),
            ProcessState::Ready => write!(f, "READY"),
            ProcessState::Running => write!(f, "R"),
            ProcessState::RunningBg => write!(f, "R(bg)"),
            ProcessState::Blocked => write!(f, "S"),
            ProcessState::Zombie => write!(f, "Z"),
            ProcessState::Terminated => write!(f, "T"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum VfsNode {
    File { content: String, perm: u8 },
    Dir { children: HashMap<String, VfsNode>, perm: u8 },
}

pub struct Vfs {
    root: VfsNode,
    pub cwd: PathBuf,
}

impl Vfs {
    pub fn new() -> Self {
        let mut root_children: HashMap<String, VfsNode> = HashMap::new();
        let mut proc_children: HashMap<String, VfsNode> = HashMap::new();
        proc_children.insert("cpuinfo".to_string(), VfsNode::File { content: "processor   : 0\nmodel name  : phase1 Virtual Cortex-A76\ncpu cores   : 4\nbogomips    : 4800.00\n".to_string(), perm: 4 });
        proc_children.insert("meminfo".to_string(), VfsNode::File { content: "MemTotal:     2097152 kB\nMemFree:      1048576 kB\nMemAvailable: 1572864 kB\nBuffers:      131072 kB\n".to_string(), perm: 4 });
        proc_children.insert("uptime".to_string(), VfsNode::File { content: "Dynamic uptime".to_string(), perm: 4 });
        proc_children.insert("loadavg".to_string(), VfsNode::File { content: "0.12 0.15 0.10 1/42 1234".to_string(), perm: 4 });
        proc_children.insert("version".to_string(), VfsNode::File { content: "phase1 v2.0.0 (Terminal OS Build 2026-05-04)".to_string(), perm: 4 });
        proc_children.insert("stat".to_string(), VfsNode::File { content: "cpu  12345 0 54321 987654 0 0 0 0 0 0\n".to_string(), perm: 4 });
        root_children.insert("proc".to_string(), VfsNode::Dir { children: proc_children, perm: 5 });
        let mut home_children: HashMap<String, VfsNode> = HashMap::new();
        home_children.insert("readme.txt".to_string(), VfsNode::File { content: "Welcome to phase1 v2.0.0\n\nFull terminal OS with editors, Python, C compilation.\nCommands: ls, cat, nano, vi, gcc, python, ifconfig, ping, etc.\n".to_string(), perm: 6 });
        root_children.insert("home".to_string(), VfsNode::Dir { children: home_children, perm: 7 });
        let mut etc_children: HashMap<String, VfsNode> = HashMap::new();
        etc_children.insert("passwd".to_string(), VfsNode::File { content: "root:x:0:0:root:/root:/bin/sh\nuser:x:1000:1000:user:/home:/bin/sh\n".to_string(), perm: 4 });
        etc_children.insert("motd".to_string(), VfsNode::File { content: "phase1 v2.0.0 ready.\n".to_string(), perm: 4 });
        root_children.insert("etc".to_string(), VfsNode::Dir { children: etc_children, perm: 5 });
        let mut dev_children: HashMap<String, VfsNode> = HashMap::new();
        dev_children.insert("null".to_string(), VfsNode::File { content: "".to_string(), perm: 6 });
        dev_children.insert("zero".to_string(), VfsNode::File { content: "\0".repeat(1024), perm: 4 });
        root_children.insert("dev".to_string(), VfsNode::Dir { children: dev_children, perm: 5 });
        root_children.insert("bin".to_string(), VfsNode::Dir { children: HashMap::new(), perm: 5 });
        Vfs {
            root: VfsNode::Dir { children: root_children, perm: 7 },
            cwd: PathBuf::from("/"),
        }
    }

    pub fn resolve_path(&self, path: &str) -> PathBuf {
        let mut p = if path.starts_with('/') { PathBuf::from("/") } else { self.cwd.clone() };
        for part in Path::new(path).components() {
            match part.as_os_str().to_str().unwrap_or("") {
                "" | "." => {}
                ".." => { let _ = p.pop(); }
                seg => p.push(seg),
            }
        }
        p
    }

    pub fn get_node<'a>(&'a self, path: &Path) -> Option<&'a VfsNode> {
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

    pub fn get_node_mut<'a>(&'a mut self, path: &Path) -> Option<&'a mut VfsNode> {
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

    pub fn mkdir(&mut self, path_str: &str) -> Result<(), String> {
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

    pub fn touch(&mut self, path_str: &str) -> Result<(), String> {
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

    pub fn cat(&self, path_str: &str) -> Result<String, String> {
        let path = self.resolve_path(path_str);
        if let Some(VfsNode::File { content, .. }) = self.get_node(&path) {
            Ok(content.clone())
        } else {
            Err(format!("No such file or directory: {}", path_str))
        }
    }

    pub fn write_file(&mut self, path_str: &str, content: &str, append: bool) -> Result<(), String> {
        let path = self.resolve_path(path_str);
        let parent = path.parent().unwrap_or(Path::new("/"));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or("Invalid filename")?.to_string();
        let parent_node = self.get_node_mut(parent).ok_or("Parent directory not found")?;
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied (write)".to_string()); }
            if let Some(VfsNode::File { content: existing, .. }) = children.get_mut(&name) {
                if append { existing.push_str(content); } else { *existing = content.to_string(); }
            } else {
                children.insert(name, VfsNode::File { content: content.to_string(), perm: 6 });
            }
            Ok(())
        } else {
            Err("Parent is not a directory".to_string())
        }
    }

    pub fn ls(&self, path_str: Option<&str>, long: bool) -> String {
        let path = self.resolve_path(path_str.unwrap_or("."));
        if let Some(VfsNode::Dir { children, .. }) = self.get_node(&path) {
            let mut out = String::new();
            if long { out.push_str("total 0\n"); }
            for (name, node) in children.iter() {
                match node {
                    VfsNode::Dir { .. } => {
                        if long {
                            out.push_str(&format!("drwxr-xr-x  2 root root 4096 {} {}\n", Local::now().format("%b %d %H:%M"), name));
                        } else {
                            out.push_str(&format!("{}/\n", name));
                        }
                    }
                    VfsNode::File { .. } => {
                        if long {
                            out.push_str(&format!("-rw-r--r--  1 root root  123 {} {}\n", Local::now().format("%b %d %H:%M"), name));
                        } else {
                            out.push_str(&format!("{}\n", name));
                        }
                    }
                }
            }
            out
        } else {
            "Not a directory".to_string()
        }
    }

    pub fn rm(&mut self, path_str: &str) -> Result<(), String> {
        let path = self.resolve_path(path_str);
        let parent = path.parent().unwrap_or(Path::new("/"));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or("Invalid name")?.to_string();
        let parent_node = self.get_node_mut(parent).ok_or("Parent directory not found")?;
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied".to_string()); }
            if children.remove(&name).is_some() { Ok(()) } else { Err("No such file or directory".to_string()) }
        } else {
            Err("Not a directory".to_string())
        }
    }

    pub fn cp(&mut self, src: &str, dst: &str) -> Result<(), String> {
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

    pub fn mv(&mut self, src: &str, dst: &str) -> Result<(), String> {
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

    pub fn update_proc_uptime(&mut self, uptime: String) {
        let proc_path = PathBuf::from("/proc");
        if let Some(VfsNode::Dir { children, .. }) = self.get_node_mut(&proc_path) {
            if let Some(VfsNode::File { content, .. }) = children.get_mut("uptime") {
                *content = uptime;
            }
        }
    }
}

#[derive(Clone)]
pub struct PcieDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub name: String,
    pub driver: Option<String>,
}

pub struct PcieManager {
    devices: Vec<PcieDevice>,
}

impl PcieManager {
    pub fn new() -> Self {
        let mut mgr = PcieManager { devices: vec![] };
        mgr.devices.extend_from_slice(&[
            PcieDevice { bus: 0, device: 0, function: 0, vendor_id: 0x8086, device_id: 0x1237, class: 0x06, subclass: 0x00, name: "Intel 440FX - 82441FX PMC [Natoma]".to_string(), driver: Some("host".to_string()) },
            PcieDevice { bus: 0, device: 1, function: 0, vendor_id: 0x10de, device_id: 0x1e07, class: 0x03, subclass: 0x00, name: "NVIDIA TU104 [GeForce RTX 2080] (virtual)".to_string(), driver: Some("nvidia".to_string()) },
            PcieDevice { bus: 0, device: 3, function: 0, vendor_id: 0x8086, device_id: 0x100e, class: 0x02, subclass: 0x00, name: "Intel 82540EM Gigabit Ethernet".to_string(), driver: Some("e1000".to_string()) },
            PcieDevice { bus: 0, device: 4, function: 0, vendor_id: 0x1b36, device_id: 0x000d, class: 0x01, subclass: 0x08, name: "QEMU NVM Express Controller".to_string(), driver: Some("nvme".to_string()) },
            PcieDevice { bus: 0, device: 5, function: 0, vendor_id: 0x1b36, device_id: 0x0001, class: 0x0c, subclass: 0x03, name: "QEMU USB Controller".to_string(), driver: Some("xhci".to_string()) },
        ]);
        mgr
    }

    pub fn lspci(&self) -> String {
        let mut out = format!("00:00.0 Host bridge: Intel Corporation 440FX - 82441FX PMC [Natoma] (rev 02)\n");
        for dev in &self.devices {
            out.push_str(&format!(
                "{:02x}:{:02x}.{:x} {} [{}:{:04x}] {}\n",
                dev.bus, dev.device, dev.function,
                if dev.driver.is_some() { format!("[{}] ", dev.driver.as_ref().unwrap()) } else { "".to_string() },
                format!("{:04x}", dev.vendor_id),
                dev.device_id,
                dev.name
            ));
        }
        out
    }

    pub fn pcie_info(&self) -> String {
        let mut out = "PCIe bus enumeration complete (simulated ECAM + MCFG table)\n".to_string();
        out.push_str(&format!("Found {} PCIe devices\n\n", self.devices.len()));
        for dev in &self.devices {
            out.push_str(&format!("  [{:02x}:{:02x}.{:x}] {} (class {:02x}{:02x})\n",
                dev.bus, dev.device, dev.function, dev.name, dev.class, dev.subclass));
        }
        out
    }
}

#[derive(Clone)]
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
    processes: [Option<SimProcess>; 64],
    next_pid: u32,
    pub current_user: String,
    pub current_uid: u32,
    pub current_cr3: u64,
    pub cr4_pcide: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        let mut sched = Scheduler {
            processes: std::array::from_fn(|_| None),
            next_pid: 1000,
            current_user: "root".to_string(),
            current_uid: 0,
            current_cr3: 0x1000,
            cr4_pcide: false,
        };
        let _ = sched.spawn("init", 0, "/sbin/init", 128, false, 0);
        let _ = sched.spawn("phase1-shell", std::process::id(), "phase1 v2.0.0", 8192, false, 0);
        sched
    }

    fn find_free_slot(&self) -> Option<usize> {
        self.processes.iter().position(|s| s.is_none() || matches!(s.as_ref().unwrap().state, ProcessState::Void))
    }

    pub fn spawn(&mut self, name: &str, ppid: u32, cmdline: &str, mem_kb: u64, background: bool, priority: i32) -> Option<u32> {
        if let Some(idx) = self.find_free_slot() {
            let pid = self.next_pid;
            self.next_pid += 1;
            let p = SimProcess {
                pid,
                ppid,
                name: name.to_string(),
                state: if background { ProcessState::RunningBg } else { ProcessState::Running },
                mem_kb,
                cmdline: cmdline.to_string(),
                priority,
                cpu_time: 0,
                background,
                start_time: Instant::now(),
                cr3: 0x10000 + (pid as u64) * 0x1000,
            };
            self.processes[idx] = Some(p);
            Some(pid)
        } else {
            None
        }
    }

    fn reap_zombies(&mut self) {
        for slot in &mut self.processes {
            if let Some(p) = slot {
                if p.state == ProcessState::Zombie {
                    p.state = ProcessState::Void;
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.reap_zombies();
        for p in self.processes.iter_mut().flatten() {
            if p.state == ProcessState::Running || p.state == ProcessState::RunningBg {
                p.cpu_time += 1;
            }
        }
    }

    pub fn ps(&self) -> String {
        let mut out = format!("PID     PPID    USER     PRI  STATE   MEM      CR3         CMD\n");
        for p in self.processes.iter().flatten() {
            out.push_str(&format!(
                "{:6}  {:6}  {:8}  {:3}  {:6}  {:8}  0x{:08x}  {}\n",
                p.pid, p.ppid, self.current_user, p.priority, p.state, p.mem_kb, p.cr3, p.cmdline
            ));
        }
        out
    }

    pub fn top(&self) -> String {
        format!("top — phase1 v2.0.0 ({} processes active)\n", self.processes.iter().flatten().count())
    }

    pub fn kill(&mut self, pid_str: Option<&str>) -> String {
        if let Some(pid_str) = pid_str {
            if let Ok(pid) = pid_str.parse::<u32>() {
                for p in &mut self.processes {
                    if let Some(proc) = p {
                        if proc.pid == pid {
                            proc.state = ProcessState::Zombie;
                            return format!("Killed process {}", pid);
                        }
                    }
                }
                "No such process".to_string()
            } else {
                "Invalid PID".to_string()
            }
        } else {
            "Usage: kill <pid>".to_string()
        }
    }

    pub fn nice(&mut self, pid_str: Option<&str>, prio: i32) -> String {
        if let Some(pid_str) = pid_str {
            if let Ok(pid) = pid_str.parse::<u32>() {
                for p in &mut self.processes {
                    if let Some(proc) = p {
                        if proc.pid == pid {
                            proc.priority = prio;
                            return format!("Nice level set for {}", pid);
                        }
                    }
                }
            }
        }
        "Usage: nice <pid> <priority>".to_string()
    }

    pub fn jobs(&self) -> String {
        "Background job control active. Use fg/bg as needed.".to_string()
    }

    pub fn get_cr3(&self) -> u64 {
        self.current_cr3
    }

    pub fn load_cr3(&mut self, val: u64) {
        self.current_cr3 = val;
    }

    pub fn cr4(&self) -> String {
        format!("CR4: 0x000000000000{} (PCIDE: {})", if self.cr4_pcide { "1000" } else { "0000" }, if self.cr4_pcide { "enabled" } else { "disabled" })
    }

    pub fn set_pcide(&mut self, enable: bool) {
        self.cr4_pcide = enable;
    }
}

pub struct Kernel {
    pub vfs: Vfs,
    pub scheduler: Scheduler,
    pub pcie: PcieManager,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel {
            vfs: Vfs::new(),
            scheduler: Scheduler::new(),
            pcie: PcieManager::new(),
        }
    }

    pub fn tick(&mut self, uptime_secs: u64) {
        self.vfs.update_proc_uptime(format!("{} seconds", uptime_secs));
        self.scheduler.tick();
    }
}
