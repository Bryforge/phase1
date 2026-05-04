// man.rs — Comprehensive manual pages for phase1 educational OS simulator
// Terse, exact documentation for every command. Each entry details usage syntax,
// the demonstrated kernel/OS concept, and integration within the in-memory system.

use std::collections::HashMap;

pub fn get_man_page(cmd: &str) -> Option<String> {
    let mut pages: HashMap<&str, &str> = HashMap::new();

    pages.insert("help", "help: List all available commands.\n\nProvides categorized overview of core filesystem, process management, hardware simulation, networking and shell features.\nConcept: Shell command dispatcher.\nUsage: help");

    pages.insert("man", "man: Display manual page for a command.\n\nShows detailed usage, kernel concept and integration.\nUsage: man <command>\nExample: man cr3");

    pages.insert("ps", "ps: Report current process status.\n\nDisplays PID, PPID, user, priority, state, memory usage in KB, CR3 register value and command line.\nKernel concept: Process table and scheduler snapshot of all PCBs.\nUsage: ps");

    pages.insert("top", "top: Display dynamic process information.\n\nLive view of processes with CPU usage simulation via scheduler ticks.\nConcept: Real-time scheduler monitoring.\nUsage: top");

    pages.insert("free", "free: Display memory usage.\n\nShows total, used, free, shared, buffers/cache and available memory in the simulated VFS.\nConcept: Kernel memory accounting (/proc/meminfo).\nUsage: free or mem");

    pages.insert("kill", "kill: Terminate a process by PID.\n\nSends termination signal to process, moving it to Zombie then reaped by scheduler.\nConcept: Process lifecycle management and signaling.\nUsage: kill <PID>");

    pages.insert("nice", "nice: Adjust process priority.\n\nChanges scheduling priority of a process.\nConcept: Preemptive priority-based scheduler.\nUsage: nice <PID> <priority>");

    pages.insert("spawn", "spawn: Create a new process.\n\nInstantiates a simulated process in the scheduler with given name and resources.\nConcept: Process creation (fork/exec equivalent).\nUsage: spawn <name> [args]");

    pages.insert("lspci", "lspci: List PCI/PCIe devices.\n\nEnumerates simulated hardware devices on PCIe bus.\nConcept: Hardware enumeration and driver binding.\nUsage: lspci");

    pages.insert("pcie", "pcie: Show detailed PCIe subsystem information.\n\nReports bus enumeration, ECAM and device details.\nConcept: PCIe configuration space and device discovery.\nUsage: pcie");

    pages.insert("cr3", "cr3: Display current CR3 register value.\n\nShows base address of top-level page directory for current address space.\nKernel concept: x86-64 paging and process isolation.\nUsage: cr3");

    pages.insert("loadcr3", "loadcr3: Load new value into CR3 register.\n\nUpdates paging base (PML4 address). Validates alignment unless PCID enabled.\nConcept: Context switching and address space switching.\nUsage: loadcr3 <value> (hex or decimal)");

    pages.insert("cr4", "cr4: Display CR4 control register.\n\nShows flags including PCIDE bit for Process Context Identifiers.\nConcept: CPU control registers for paging features.\nUsage: cr4");

    pages.insert("pcide", "pcide: Toggle CR4.PCIDE bit.\n\nEnables or disables Process Context ID support for TLB efficiency.\nConcept: Advanced paging optimizations.\nUsage: pcide <on|off|enable|disable>");

    pages.insert("df", "df: Report filesystem disk space usage.\n\nShows size, used and available space for the in-memory VFS.\nConcept: Filesystem metadata and block accounting.\nUsage: df");

    pages.insert("whoami", "whoami: Print current user name.\n\nDisplays effective user from scheduler context.\nConcept: User identity and privilege separation.\nUsage: whoami");

    pages.insert("id", "id: Print user and group information.\n\nShows UID, GID and groups for current user.\nConcept: Unix-style credentials.\nUsage: id");

    pages.insert("ls", "ls: List directory contents.\n\nDisplays files and directories in current or specified path.\nSupports -l for long format with permissions.\nConcept: VFS directory traversal.\nUsage: ls [-l] [path]");

    pages.insert("cd", "cd: Change working directory.\n\nUpdates shell CWD in VFS.\nUsage: cd [directory]\nUse .. for parent directory.");

    pages.insert("pwd", "pwd: Print working directory.\n\nOutputs absolute path of current VFS location.\nConcept: Process filesystem context.\nUsage: pwd");

    pages.insert("cat", "cat: Concatenate and display file content.\n\nReads and prints file from in-memory VFS.\nUsage: cat <file>");

    pages.insert("mkdir", "mkdir: Create new directory.\n\nAdds directory node to VFS with default permissions.\nConcept: Filesystem hierarchy management.\nUsage: mkdir <directory>");

    pages.insert("touch", "touch: Create empty file or update timestamp.\n\nCreates new file node in VFS.\nUsage: touch <file>");

    pages.insert("rm", "rm: Remove file or directory.\n\nDeletes node from VFS.\nUsage: rm <path>");

    pages.insert("cp", "cp: Copy files.\n\nDuplicates file content within VFS.\nUsage: cp <source> <destination>");

    pages.insert("mv", "mv: Move or rename files/directories.\n\nRelocates node in VFS tree.\nUsage: mv <source> <destination>");

    pages.insert("echo", "echo: Display line of text.\n\nPrints arguments or redirects to file using > or >>.\nSupports shell redirection for file output.\nUsage: echo [text] [> or >> file]");

    pages.insert("clear", "clear: Clear terminal screen.\n\nResets display using ANSI escape sequences.\nUsage: clear");

    pages.insert("env", "env: Display environment variables.\n\nLists all current shell environment key=value pairs.\nConcept: Process environment inheritance.\nUsage: env");

    pages.insert("export", "export: Set environment variable.\n\nAdds or updates variable in shell context.\nUsage: export VAR=value");

    pages.insert("unset", "unset: Remove environment variable.\n\nDeletes variable from shell environment.\nUsage: unset VAR");

    pages.insert("python", "python: Execute Python script.\n\nRuns external Python code (host python3).\nUsage: python <script.py>");

    pages.insert("plugin", "plugin: List and execute Python plugins.\n\nPlugins receive context via stdin (COMMAND, USER, CWD etc.).\nConcept: Extensible userspace plugin system.\nUsage: plugin or plugins");

    pages.insert("jobs", "jobs: List background jobs.\n\nShows scheduler-managed background processes.\nConcept: Job control in shell.\nUsage: jobs");

    pages.insert("fg", "fg: Bring job to foreground.\n\nSimulated job control interface.\nUsage: fg");

    pages.insert("bg", "bg: Send job to background.\n\nSimulated job control.\nUsage: bg");

    pages.insert("su", "su: Switch user.\n\nChanges current user context in scheduler.\nUsage: su <username>");

    pages.insert("dmesg", "dmesg: Display kernel ring buffer messages.\n\nShows boot and subsystem initialization logs.\nConcept: Kernel logging.\nUsage: dmesg");

    pages.insert("vmstat", "vmstat: Report virtual memory statistics.\n\nShows process, memory, swap, I/O and CPU summary.\nConcept: System performance monitoring.\nUsage: vmstat");

    pages.insert("history", "history: Display command history.\n\nLists previously entered shell commands.\nUsage: history");

    pages.insert("uname", "uname: Print system information.\n\nReports kernel name, version and architecture.\nConcept: System identification.\nUsage: uname");

    pages.insert("date", "date: Print current date and time.\n\nUses host system time via chrono.\nUsage: date");

    pages.insert("uptime", "uptime: Show system uptime.\n\nDisplays time since simulator start and load average.\nConcept: Kernel timekeeping.\nUsage: uptime");

    pages.insert("hostname", "hostname: Show system hostname.\n\nFixed virtual hostname.\nUsage: hostname");

    pages.insert("ifconfig", "ifconfig: Configure and display network interfaces.\n\nShows real host interfaces, IP, MAC on Linux/macOS.\nConcept: Network stack abstraction.\nUsage: ifconfig");

    pages.insert("iwconfig", "iwconfig: Display wireless interface configuration.\n\nShows WiFi status and signal.\nUsage: iwconfig");

    pages.insert("wifi-scan", "wifi-scan: Scan for wireless networks.\n\nExecutes host WiFi scan commands (airport/nmcli).\nConcept: Hardware abstraction for networking.\nUsage: wifi-scan");

    pages.insert("wifi-connect", "wifi-connect: Connect to WiFi network.\n\nAttempts connection using host tools.\nUsage: wifi-connect <SSID> [password]");

    pages.insert("ping", "ping: Send ICMP echo requests.\n\nTests network reachability using host ping.\nUsage: ping <host>");

    pages.insert("nmcli", "nmcli: NetworkManager command line.\n\nShows active connections and status.\nUsage: nmcli");

    pages.insert("sandbox", "sandbox: Show sandbox information.\n\nConfirms operation inside Rust userspace with Linux namespaces where available.\nConcept: Security isolation.\nUsage: sandbox or nsinfo");

    pages.insert("version", "version: Show phase1 version.\n\nReports simulator version and build date.\nUsage: version");

    pages.insert("tree", "tree: Display directory tree.\n\nShows static VFS hierarchy.\nUsage: tree");

    pages.insert("exit", "exit: Exit the simulator.\n\nAlso accepts quit, shutdown, poweroff.\nTerminates shell cleanly.\nUsage: exit");

    // Aliases for convenience
    pages.insert("mem", pages.get("free").unwrap_or(&""));
    pages.insert("py", pages.get("python").unwrap_or(&""));
    pages.insert("plugins", pages.get("plugin").unwrap_or(&""));
    pages.insert("nsinfo", pages.get("sandbox").unwrap_or(&""));
    pages.insert("quit", pages.get("exit").unwrap_or(&""));
    pages.insert("shutdown", pages.get("exit").unwrap_or(&""));
    pages.insert("poweroff", pages.get("exit").unwrap_or(&""));

    pages.get(cmd).copied().map(|s| s.to_string())
}
