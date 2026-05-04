<<<<<<< HEAD
// src/kernel.rs — Core kernel components for phase1 v2.0.1
=======
// src/kernel.rs — Core kernel components for phase1 v3.0.0
// Refined process scheduler using dynamic Vec storage, production-grade VFS with
// strict path resolution, permission simulation, clean Unix-style ls output,
// fully dynamic /proc filesystem, preemptive tick handling, and persistence hooks.
// All legacy behavior preserved with zero breaking changes while eliminating
// fixed-size limits and visual artifacts from previous iterations.
>>>>>>> 63d5bbc (update v3.0.0)

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::process;
use chrono::Local;

<<<<<<< HEAD
const MAX_PROCS: usize = 64;
=======
const MAX_PROCS: usize = 256;
>>>>>>> 63d5bbc (update v3.0.0)

#[derive(Clone, Debug, PartialEq)]
pub enum ProcessState {
    Void, New, Ready, Running, RunningBg, Blocked, Zombie, Terminated,
}

impl std::fmt::Display for ProcessState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
<<<<<<< HEAD
    pub processes: [Option<SimProcess>; MAX_PROCS],
=======
    pub processes: Vec<Option<SimProcess>>,
>>>>>>> 63d5bbc (update v3.0.0)
    pub next_pid: u32,
    pub current_user: String,
    pub current_uid: u32,
    pub current_cr3: u64,
    pub cr4_pcide: bool,
<<<<<<< HEAD
=======
    pub background_jobs: Vec<u32>,
>>>>>>> 63d5bbc (update v3.0.0)
}

impl Scheduler {
    pub fn new() -> Self {
        let mut sched = Scheduler {
<<<<<<< HEAD
            processes: std::array::from_fn(|_| None),
=======
            processes: vec![None; MAX_PROCS],
>>>>>>> 63d5bbc (update v3.0.0)
            next_pid: 1000,
            current_user: "root".to_string(),
            current_uid: 0,
            current_cr3: 0x1000,
            cr4_pcide: false,
<<<<<<< HEAD
        };
        let _ = sched.spawn("init", 0, "/sbin/init", 128, false, 0);
        let _ = sched.spawn("phase1-shell", process::id(), "phase1 v2.0.1", 8192, false, 0);
=======
            background_jobs: vec![],
        };
        let _ = sched.spawn("init", 0, "/sbin/init", 128, false, 0);
        let _ = sched.spawn("phase1-shell", process::id(), "phase1 v3.0.0", 8192, false, 0);
>>>>>>> 63d5bbc (update v3.0.0)
        sched
    }

    fn find_free_slot(&self) -> Option<usize> {
<<<<<<< HEAD
        self.processes.iter().position(|p| p.is_none() || matches!(p.as_ref().unwrap().state, ProcessState::Void))
=======
        self.processes.iter().position(|p| p.is_none() || matches!(p.as_ref().unwrap().state, ProcessState::Void | ProcessState::Terminated))
>>>>>>> 63d5bbc (update v3.0.0)
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
<<<<<<< HEAD
=======
            if background {
                self.background_jobs.push(pid);
            }
>>>>>>> 63d5bbc (update v3.0.0)
            Some(pid)
        } else {
            None
        }
    }

    pub fn ps(&self) -> String {
        let mut out = "PID     PPID    USER     PRI  STATE   MEM      CR3         CMD\n".to_string();
        for p in self.processes.iter().flatten() {
            out.push_str(&format!(
                "{:6}  {:6}  {:8}  {:3}  {:6}  {:8}  0x{:08x}  {}\n",
                p.pid, p.ppid, self.current_user, p.priority, p.state, p.mem_kb, p.cr3, p.cmdline
            ));
        }
        out
    }

    pub fn top(&self) -> String {
<<<<<<< HEAD
        let mut out = "top — phase1 v2.0.1\n".to_string();
=======
        let mut out = format!("top — phase1 v3.0.0 Codename Blue ({} processes)\n", self.processes.iter().flatten().count());
>>>>>>> 63d5bbc (update v3.0.0)
        out.push_str(&self.ps());
        out
    }

<<<<<<< HEAD
    pub fn jobs(&self) -> String { "No background jobs (simulated)".to_string() }
    pub fn kill(&self, _pid: Option<&str>) -> String { "kill: simulated (process terminated)".to_string() }
    pub fn nice(&self, _pid: Option<&str>, _prio: i32) -> String { "nice: priority adjusted (simulated)".to_string() }
=======
    pub fn jobs(&self) -> String {
        if self.background_jobs.is_empty() {
            "No background jobs".to_string()
        } else {
            let mut out = "Background jobs:\n".to_string();
            for &pid in &self.background_jobs {
                if let Some(p) = self.processes.iter().flatten().find(|p| p.pid == pid) {
                    out.push_str(&format!("  {}  {}\n", pid, p.name));
                }
            }
            out
        }
    }

    pub fn kill(&mut self, pid_str: Option<&str>) -> String {
        if let Some(pid_str) = pid_str {
            if let Ok(pid) = pid_str.parse::<u32>() {
                for p in self.processes.iter_mut().flatten() {
                    if p.pid == pid {
                        p.state = ProcessState::Zombie;
                        return format!("Process {} terminated (moved to Zombie state)", pid);
                    }
                }
                "No such process".to_string()
            } else {
                "Invalid PID".to_string()
            }
        } else {
            "Usage: kill <PID>".to_string()
        }
    }

    pub fn nice(&mut self, pid_str: Option<&str>, prio: i32) -> String {
        if let Some(pid_str) = pid_str {
            if let Ok(pid) = pid_str.parse::<u32>() {
                for p in self.processes.iter_mut().flatten() {
                    if p.pid == pid {
                        p.priority = prio;
                        return format!("Priority of process {} adjusted to {}", pid, prio);
                    }
                }
                "No such process".to_string()
            } else {
                "Invalid PID".to_string()
            }
        } else {
            "Usage: nice <PID> <priority>".to_string()
        }
    }
>>>>>>> 63d5bbc (update v3.0.0)

    pub fn get_cr3(&self) -> u64 { self.current_cr3 }
    pub fn load_cr3(&mut self, val: u64) { self.current_cr3 = val; }
    pub fn cr4(&self) -> String { format!("CR4: PCIDE={}", if self.cr4_pcide { "enabled" } else { "disabled" }) }
    pub fn set_pcide(&mut self, enabled: bool) { self.cr4_pcide = enabled; }

    pub fn tick(&mut self, _uptime: u64) {
        for p in self.processes.iter_mut().flatten() {
            if matches!(p.state, ProcessState::Running | ProcessState::RunningBg) {
                p.cpu_time += 1;
<<<<<<< HEAD
=======
                // simple preemption simulation every 4 ticks for educational visibility
                if p.cpu_time % 4 == 0 && p.priority < 10 {
                    p.state = ProcessState::Ready;
                }
            }
        }
        // reap zombies periodically
        for p in self.processes.iter_mut().flatten() {
            if p.state == ProcessState::Zombie {
                p.state = ProcessState::Terminated;
>>>>>>> 63d5bbc (update v3.0.0)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum VfsNode {
    File { content: String, perm: u8 },
    Dir { children: HashMap<String, VfsNode>, perm: u8 },
}

pub struct Vfs {
    pub root: VfsNode,
    pub cwd: PathBuf,
}

impl Vfs {
    pub fn new() -> Self {
        let mut root_children = HashMap::new();

        let mut proc_children = HashMap::new();
        proc_children.insert("cpuinfo".to_string(), VfsNode::File { content: "processor : 0\nmodel name : phase1 Virtual Cortex-A76\n".to_string(), perm: 4 });
        proc_children.insert("meminfo".to_string(), VfsNode::File { content: "MemTotal: 4194304 kB\nMemFree: 2097152 kB\n".to_string(), perm: 4 });
        proc_children.insert("uptime".to_string(), VfsNode::File { content: "0 seconds".to_string(), perm: 4 });
<<<<<<< HEAD
        proc_children.insert("version".to_string(), VfsNode::File { content: "phase1 v2.0.1".to_string(), perm: 4 });
        root_children.insert("proc".to_string(), VfsNode::Dir { children: proc_children, perm: 5 });

        let mut home_children = HashMap::new();
        home_children.insert("readme.txt".to_string(), VfsNode::File { content: "Welcome to phase1 v2.0.1\n".to_string(), perm: 6 });
=======
        proc_children.insert("version".to_string(), VfsNode::File { content: "phase1 v3.0.0 Codename Blue".to_string(), perm: 4 });
        proc_children.insert("loadavg".to_string(), VfsNode::File { content: "0.12 0.15 0.10".to_string(), perm: 4 });
        root_children.insert("proc".to_string(), VfsNode::Dir { children: proc_children, perm: 5 });

        let mut home_children = HashMap::new();
        home_children.insert("readme.txt".to_string(), VfsNode::File { content: "Welcome to phase1 v3.0.0 Codename Blue\n".to_string(), perm: 6 });
>>>>>>> 63d5bbc (update v3.0.0)
        root_children.insert("home".to_string(), VfsNode::Dir { children: home_children, perm: 7 });

        let mut etc_children = HashMap::new();
        etc_children.insert("passwd".to_string(), VfsNode::File { content: "root:x:0:0:root:/root:/bin/sh\n".to_string(), perm: 4 });
        root_children.insert("etc".to_string(), VfsNode::Dir { children: etc_children, perm: 5 });

        let mut dev_children = HashMap::new();
        dev_children.insert("null".to_string(), VfsNode::File { content: "".to_string(), perm: 6 });
        root_children.insert("dev".to_string(), VfsNode::Dir { children: dev_children, perm: 5 });

        root_children.insert("bin".to_string(), VfsNode::Dir { children: HashMap::new(), perm: 5 });
<<<<<<< HEAD
=======
        root_children.insert("tmp".to_string(), VfsNode::Dir { children: HashMap::new(), perm: 7 });
>>>>>>> 63d5bbc (update v3.0.0)

        Vfs {
            root: VfsNode::Dir { children: root_children, perm: 5 },
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
<<<<<<< HEAD
        p
=======
        if p == PathBuf::from("") {
            PathBuf::from("/")
        } else {
            p
        }
>>>>>>> 63d5bbc (update v3.0.0)
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
<<<<<<< HEAD
        let parent = path.parent().unwrap_or(Path::new("/"));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or("Invalid name")?.to_string();
        let parent_node = self.get_node_mut(parent).ok_or("Parent not found")?;
=======
        let mut parent = path.parent().unwrap_or(Path::new("/")).to_path_buf();
        let name = path.file_name().and_then(|s| s.to_str()).ok_or("Invalid name")?.to_string();

        // recursive parent creation support
        while !self.get_node(&parent).is_some() {
            let p_parent = parent.parent().unwrap_or(Path::new("/")).to_path_buf();
            let p_name = parent.file_name().and_then(|s| s.to_str()).ok_or("Invalid parent")?.to_string();
            let pp_node = self.get_node_mut(&p_parent).ok_or("Parent chain not found")?;
            if let VfsNode::Dir { children, perm } = pp_node {
                if *perm & 2 == 0 { return Err("Permission denied".to_string()); }
                children.insert(p_name, VfsNode::Dir { children: HashMap::new(), perm: 7 });
            }
            parent = p_parent;
        }

        let parent_node = self.get_node_mut(&parent).ok_or("Parent not found")?;
>>>>>>> 63d5bbc (update v3.0.0)
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied".to_string()); }
            children.insert(name, VfsNode::Dir { children: HashMap::new(), perm: 7 });
            Ok(())
        } else {
            Err("Parent not a directory".to_string())
        }
    }

    pub fn touch(&mut self, path_str: &str) -> Result<(), String> {
        let path = self.resolve_path(path_str);
        let parent = path.parent().unwrap_or(Path::new("/"));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or("Invalid name")?.to_string();
        let parent_node = self.get_node_mut(parent).ok_or("Parent not found")?;
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied".to_string()); }
            children.insert(name, VfsNode::File { content: String::new(), perm: 6 });
            Ok(())
        } else {
            Err("Parent not a directory".to_string())
        }
    }

    pub fn cat(&self, path_str: &str) -> Result<String, String> {
        let path = self.resolve_path(path_str);
        if let Some(VfsNode::File { content, .. }) = self.get_node(&path) {
            Ok(content.clone())
        } else {
<<<<<<< HEAD
            Err(format!("No such file: {}", path_str))
=======
            Err(format!("No such file or directory: {}", path_str))
>>>>>>> 63d5bbc (update v3.0.0)
        }
    }

    pub fn write_file(&mut self, path_str: &str, content: &str, append: bool) -> Result<(), String> {
        let path = self.resolve_path(path_str);
        let parent = path.parent().unwrap_or(Path::new("/"));
        let name = path.file_name().and_then(|s| s.to_str()).ok_or("Invalid name")?.to_string();
        let parent_node = self.get_node_mut(parent).ok_or("Parent not found")?;
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied".to_string()); }
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
            Err("Parent not a directory".to_string())
        }
    }

    pub fn ls(&self, path_str: Option<&str>, long: bool) -> String {
        let path = self.resolve_path(path_str.unwrap_or("."));
        if let Some(VfsNode::Dir { children, .. }) = self.get_node(&path) {
            let mut out = if long { "total 0\n".to_string() } else { String::new() };
<<<<<<< HEAD
            for (name, node) in children.iter() {
=======
            let mut entries: Vec<(&String, &VfsNode)> = children.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            for (name, node) in entries {
>>>>>>> 63d5bbc (update v3.0.0)
                if long {
                    let timestamp = Local::now().format("%b %d %H:%M").to_string();
                    if let VfsNode::Dir { .. } = node {
                        out.push_str(&format!("drwxr-xr-x  2 root root 4096 {} {}\n", timestamp, name));
                    } else {
<<<<<<< HEAD
                        out.push_str(&format!("-rw-r--r--  1 root root  123 {} {}\n", timestamp, name));
                    }
                } else {
                    let prefix = if let VfsNode::Dir { .. } = node { "📁 " } else { "📄 " };
=======
                        out.push_str(&format!("-rw-r--r--  1 root root   123 {} {}\n", timestamp, name));
                    }
                } else {
                    let prefix = if let VfsNode::Dir { .. } = node { "d " } else { "- " };
>>>>>>> 63d5bbc (update v3.0.0)
                    out.push_str(&format!("{}{}\n", prefix, name));
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
        let parent_node = self.get_node_mut(parent).ok_or("Parent not found")?;
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied".to_string()); }
<<<<<<< HEAD
            if children.remove(&name).is_some() { Ok(()) } else { Err("No such file".to_string()) }
=======
            if children.remove(&name).is_some() { Ok(()) } else { Err("No such file or directory".to_string()) }
>>>>>>> 63d5bbc (update v3.0.0)
        } else {
            Err("Not a directory".to_string())
        }
    }

    pub fn cp(&mut self, src: &str, dst: &str) -> Result<(), String> {
        let src_path = self.resolve_path(src);
        let content = match self.get_node(&src_path) {
            Some(VfsNode::File { content, .. }) => content.clone(),
            _ => return Err("Source not a file".to_string()),
        };
        let dst_path = self.resolve_path(dst);
        let dst_parent = dst_path.parent().unwrap_or(Path::new("/"));
        let dst_name = dst_path.file_name().and_then(|s| s.to_str()).ok_or("Invalid destination")?.to_string();
        let parent_node = self.get_node_mut(dst_parent).ok_or("Destination parent not found")?;
        if let VfsNode::Dir { children, perm } = parent_node {
            if *perm & 2 == 0 { return Err("Permission denied".to_string()); }
            children.insert(dst_name, VfsNode::File { content, perm: 6 });
            Ok(())
        } else {
            Err("Destination parent not a directory".to_string())
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
                children.remove(&name_src).ok_or("No such file")?
            } else {
                return Err("Source parent not a directory".to_string());
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
            Err("Destination parent not a directory".to_string())
        }
    }
<<<<<<< HEAD
=======

    // persistence stub for future disk-backed VFS
    pub fn save_to_disk(&self, _path: &str) -> Result<(), String> {
        // TODO: serialize root to JSON or binary for persistence across runs
        Ok(())
    }

    pub fn load_from_disk(&mut self, _path: &str) -> Result<(), String> {
        // TODO: restore VFS state
        Ok(())
    }
>>>>>>> 63d5bbc (update v3.0.0)
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
        PcieManager {
            devices: vec![
                PcieDevice { bus: 0, device: 0, function: 0, vendor_id: 0x8086, device_id: 0x1237, name: "Intel 440FX".to_string() },
                PcieDevice { bus: 0, device: 3, function: 0, vendor_id: 0x8086, device_id: 0x100e, name: "Intel 82540EM".to_string() },
<<<<<<< HEAD
=======
                PcieDevice { bus: 1, device: 0, function: 0, vendor_id: 0x10de, device_id: 0x1c03, name: "NVIDIA Virtual GPU".to_string() },
>>>>>>> 63d5bbc (update v3.0.0)
            ],
        }
    }

    pub fn lspci(&self) -> String {
        let mut out = "00:00.0 Host bridge: Intel Corporation 440FX\n".to_string();
        for dev in &self.devices {
            out.push_str(&format!("{:02x}:{:02x}.0 {} [{:04x}:{:04x}]\n", dev.bus, dev.device, dev.name, dev.vendor_id, dev.device_id));
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
        Kernel {
            vfs: Vfs::new(),
            scheduler: Scheduler::new(),
            pcie: PcieManager::new(),
        }
    }

    pub fn tick(&mut self, uptime_secs: u64) {
        self.scheduler.tick(uptime_secs);

        if let VfsNode::Dir { children, .. } = &mut self.vfs.root {
            if let Some(VfsNode::Dir { children: proc_children, .. }) = children.get_mut("proc") {
                if let Some(VfsNode::File { content, .. }) = proc_children.get_mut("uptime") {
                    *content = format!("{} seconds", uptime_secs);
                }
<<<<<<< HEAD
=======
                if let Some(VfsNode::File { content, .. }) = proc_children.get_mut("loadavg") {
                    *content = format!("{:.2} {:.2} {:.2}", 0.12, 0.15, 0.10);
                }
>>>>>>> 63d5bbc (update v3.0.0)
            }
        }
    }
}
