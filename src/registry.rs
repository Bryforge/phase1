#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommandSpec {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub category: &'static str,
    pub usage: &'static str,
    pub description: &'static str,
    pub capability: &'static str,
}

macro_rules! cmd {
    ($name:expr, $aliases:expr, $category:expr, $usage:expr, $description:expr, $capability:expr) => {
        CommandSpec {
            name: $name,
            aliases: $aliases,
            category: $category,
            usage: $usage,
            description: $description,
            capability: $capability,
        }
    };
}

pub const CATEGORIES: &[&str] = &["fs", "proc", "net", "host", "arch", "sys", "user", "misc"];

pub const COMMANDS: &[CommandSpec] = &[
    cmd!("ls", &[], "fs", "ls [-l] [path]", "List VFS directory contents.", "fs.read"),
    cmd!("cd", &[], "fs", "cd [dir]", "Change VFS working directory.", "fs.read"),
    cmd!("pwd", &[], "fs", "pwd", "Print the current VFS path.", "fs.read"),
    cmd!("cat", &[], "fs", "cat <file>", "Read a VFS file through sys_read.", "fs.read"),
    cmd!("mkdir", &[], "fs", "mkdir <dir>", "Create a VFS directory.", "fs.write"),
    cmd!("touch", &[], "fs", "touch <file>", "Create a VFS file.", "fs.write"),
    cmd!("rm", &[], "fs", "rm <path>", "Remove a VFS node.", "fs.write"),
    cmd!("cp", &[], "fs", "cp <src> <dst>", "Copy a VFS file.", "fs.write"),
    cmd!("mv", &[], "fs", "mv <src> <dst>", "Move or rename a VFS node.", "fs.write"),
    cmd!("tree", &[], "fs", "tree", "Display the VFS tree.", "fs.read"),
    cmd!("echo", &[], "fs", "echo <text> [> file | >> file]", "Print text or redirect into the VFS.", "fs.write"),

    cmd!("ps", &[], "proc", "ps", "Show simulated process table.", "proc.read"),
    cmd!("top", &[], "proc", "top", "Show scheduler state.", "proc.read"),
    cmd!("spawn", &[], "proc", "spawn <name> [args...] [--background]", "Create a simulated process through sys_spawn.", "proc.spawn"),
    cmd!("jobs", &[], "proc", "jobs", "List simulated background jobs.", "proc.read"),
    cmd!("fg", &[], "proc", "fg <pid>", "Move a simulated process to foreground.", "proc.manage"),
    cmd!("bg", &[], "proc", "bg <pid>", "Move a simulated process to background.", "proc.manage"),
    cmd!("kill", &[], "proc", "kill <pid>", "Terminate a simulated process through sys_kill.", "proc.kill"),
    cmd!("nice", &[], "proc", "nice <pid> <priority>", "Set simulated process priority.", "proc.manage"),

    cmd!("ifconfig", &[], "net", "ifconfig", "Show discovered host network interfaces.", "net.read"),
    cmd!("iwconfig", &[], "net", "iwconfig", "Show WiFi information where available.", "net.read"),
    cmd!("wifi-scan", &[], "net", "wifi-scan", "List nearby WiFi networks with host tools.", "net.read"),
    cmd!("wifi-connect", &[], "net", "wifi-connect <ssid> [password]", "Dry-run WiFi connection unless host mutation is enabled.", "net.admin"),
    cmd!("ping", &[], "net", "ping <host>", "Run bounded host ping.", "net.read"),
    cmd!("nmcli", &[], "net", "nmcli", "Show NetworkManager state on Linux.", "net.read"),

    cmd!("browser", &[], "host", "browser <url|phase1|about>", "Fetch and render HTTP/HTTPS text using guarded curl.", "host.net"),
    cmd!("python", &["py"], "host", "python <file.py> | python -c <code>", "Run Python with a timeout.", "host.exec"),
    cmd!("gcc", &["cc"], "host", "gcc <file.c> | gcc <code>", "Compile and run C with host compiler timeout guards.", "host.exec"),
    cmd!("plugins", &["plugin"], "host", "plugins", "List Python plugins in ./plugins.", "host.exec"),
    cmd!("ned", &["nano", "vi"], "host", "ned <file>", "Edit a VFS file with a small line editor.", "fs.write"),

    cmd!("lspci", &[], "arch", "lspci", "List simulated PCIe devices.", "hw.read"),
    cmd!("pcie", &[], "arch", "pcie", "Show PCIe subsystem summary.", "hw.read"),
    cmd!("cr3", &[], "arch", "cr3", "Show simulated CR3 value.", "hw.read"),
    cmd!("loadcr3", &[], "arch", "loadcr3 <hex|decimal>", "Load simulated CR3 with alignment validation.", "hw.write"),
    cmd!("cr4", &[], "arch", "cr4", "Show simulated CR4 flags.", "hw.read"),
    cmd!("pcide", &[], "arch", "pcide on|off", "Toggle simulated CR4.PCIDE.", "hw.write"),

    cmd!("free", &["mem"], "sys", "free", "Show simulated memory information.", "sys.read"),
    cmd!("df", &[], "sys", "df", "Show simulated filesystem capacity.", "sys.read"),
    cmd!("dmesg", &[], "sys", "dmesg", "Show simulated boot messages.", "sys.log"),
    cmd!("vmstat", &[], "sys", "vmstat", "Show compact virtual system stats.", "sys.read"),
    cmd!("uname", &[], "sys", "uname", "Show simulator kernel identity.", "sys.read"),
    cmd!("date", &[], "sys", "date", "Show host UNIX timestamp.", "sys.read"),
    cmd!("uptime", &[], "sys", "uptime", "Show simulator uptime.", "sys.read"),
    cmd!("hostname", &[], "sys", "hostname", "Show virtual hostname.", "sys.read"),
    cmd!("audit", &[], "sys", "audit", "Show in-memory kernel audit events.", "sys.audit"),

    cmd!("env", &[], "user", "env", "Print shell environment.", "user.read"),
    cmd!("export", &[], "user", "export VAR=value", "Set an environment variable.", "user.env"),
    cmd!("unset", &[], "user", "unset VAR", "Remove an environment variable.", "user.env"),
    cmd!("whoami", &[], "user", "whoami", "Print current simulated user.", "user.read"),
    cmd!("id", &[], "user", "id", "Print simulated user id.", "user.read"),
    cmd!("su", &[], "user", "su <user>", "Switch simulated user.", "user.switch"),
    cmd!("history", &[], "user", "history", "Show shell command history.", "user.read"),

    cmd!("help", &["commands"], "misc", "help", "Show grouped command map.", "none"),
    cmd!("man", &[], "misc", "man <command>", "Show generated command manual page.", "none"),
    cmd!("complete", &[], "misc", "complete [prefix]", "Show registry-backed command completions.", "none"),
    cmd!("clear", &[], "misc", "clear", "Clear terminal using newlines.", "none"),
    cmd!("version", &[], "misc", "version", "Show phase1 version.", "none"),
    cmd!("sandbox", &["nsinfo"], "misc", "sandbox", "Show safety model.", "none"),
    cmd!("exit", &["quit", "shutdown", "poweroff"], "misc", "exit", "Terminate simulator.", "none"),
];

pub fn lookup(name: &str) -> Option<&'static CommandSpec> {
    COMMANDS
        .iter()
        .find(|cmd| cmd.name == name || cmd.aliases.iter().any(|alias| *alias == name))
}

pub fn command_map() -> String {
    let mut out = String::from("phase1 // command map\n\n");
    for category in CATEGORIES {
        let names = COMMANDS
            .iter()
            .filter(|cmd| cmd.category == *category)
            .map(|cmd| cmd.name)
            .collect::<Vec<_>>()
            .join(" ");
        out.push_str(&format!("{:<5}: {}\n", category, names));
    }
    out.push_str("\nquick : man browser | complete p | audit | ps | ls /\n");
    out
}

pub fn man_page(name: &str) -> Option<String> {
    let cmd = lookup(name)?;
    let aliases = if cmd.aliases.is_empty() {
        "none".to_string()
    } else {
        cmd.aliases.join(", ")
    };
    Some(format!(
        "{}\n\nusage      : {}\ncategory   : {}\naliases    : {}\ncapability : {}\n\n{}",
        cmd.name, cmd.usage, cmd.category, aliases, cmd.capability, cmd.description
    ))
}

pub fn completions(prefix: &str) -> Vec<&'static str> {
    let mut out = Vec::new();
    for cmd in COMMANDS {
        if cmd.name.starts_with(prefix) {
            out.push(cmd.name);
        }
        for alias in cmd.aliases {
            if alias.starts_with(prefix) {
                out.push(alias);
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::{command_map, completions, lookup, man_page};

    #[test]
    fn lookup_supports_aliases() {
        assert_eq!(lookup("py").map(|cmd| cmd.name), Some("python"));
        assert_eq!(lookup("quit").map(|cmd| cmd.name), Some("exit"));
    }

    #[test]
    fn command_map_contains_audit_and_complete() {
        let map = command_map();
        assert!(map.contains("audit"));
        assert!(map.contains("complete"));
    }

    #[test]
    fn man_pages_are_generated() {
        let page = man_page("browser").expect("browser man page");
        assert!(page.contains("HTTP/HTTPS"));
        assert!(page.contains("capability"));
    }

    #[test]
    fn completions_include_aliases() {
        assert!(completions("p").contains(&"python"));
        assert!(completions("p").contains(&"py"));
    }
}
