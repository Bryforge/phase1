#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommandSpec {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub category: &'static str,
    pub usage: &'static str,
    pub description: &'static str,
    pub capability: &'static str,
}

pub const COMMANDS: &[CommandSpec] = &[
    CommandSpec { name: "ls", aliases: &[], category: "fs", usage: "ls [-l] [path]", description: "List VFS directory contents.", capability: "fs.read" },
    CommandSpec { name: "cd", aliases: &[], category: "fs", usage: "cd [dir]", description: "Change VFS working directory.", capability: "fs.read" },
    CommandSpec { name: "pwd", aliases: &[], category: "fs", usage: "pwd", description: "Print the current VFS path.", capability: "fs.read" },
    CommandSpec { name: "cat", aliases: &[], category: "fs", usage: "cat <file>", description: "Read a VFS file.", capability: "fs.read" },
    CommandSpec { name: "mkdir", aliases: &[], category: "fs", usage: "mkdir <dir>", description: "Create a VFS directory.", capability: "fs.write" },
    CommandSpec { name: "touch", aliases: &[], category: "fs", usage: "touch <file>", description: "Create or update a VFS file.", capability: "fs.write" },
    CommandSpec { name: "rm", aliases: &[], category: "fs", usage: "rm <path>", description: "Remove a VFS node.", capability: "fs.write" },
    CommandSpec { name: "cp", aliases: &[], category: "fs", usage: "cp <src> <dst>", description: "Copy a VFS file.", capability: "fs.write" },
    CommandSpec { name: "mv", aliases: &[], category: "fs", usage: "mv <src> <dst>", description: "Move or rename a VFS node.", capability: "fs.write" },
    CommandSpec { name: "tree", aliases: &[], category: "fs", usage: "tree", description: "Show VFS tree.", capability: "fs.read" },
    CommandSpec { name: "echo", aliases: &[], category: "fs", usage: "echo <text> [> file | >> file]", description: "Print text or redirect it into the VFS.", capability: "fs.write" },

    CommandSpec { name: "ps", aliases: &[], category: "proc", usage: "ps", description: "Show simulated process table.", capability: "proc.read" },
    CommandSpec { name: "top", aliases: &[], category: "proc", usage: "top", description: "Show simulated scheduler state.", capability: "proc.read" },
    CommandSpec { name: "spawn", aliases: &[], category: "proc", usage: "spawn <name> [args...] [--background]", description: "Create a simulated process.", capability: "proc.spawn" },
    CommandSpec { name: "jobs", aliases: &[], category: "proc", usage: "jobs", description: "List simulated background jobs.", capability: "proc.read" },
    CommandSpec { name: "fg", aliases: &[], category: "proc", usage: "fg <pid>", description: "Move a simulated job to foreground.", capability: "proc.manage" },
    CommandSpec { name: "bg", aliases: &[], category: "proc", usage: "bg <pid>", description: "Move a simulated job to background.", capability: "proc.manage" },
    CommandSpec { name: "kill", aliases: &[], category: "proc", usage: "kill <pid>", description: "Terminate a simulated process.", capability: "proc.kill" },
    CommandSpec { name: "nice", aliases: &[], category: "proc", usage: "nice <pid> <priority>", description: "Set simulated process priority.", capability: "proc.manage" },

    CommandSpec { name: "ifconfig", aliases: &[], category: "net", usage: "ifconfig", description: "Show discovered host network interfaces.", capability: "net.read" },
    CommandSpec { name: "iwconfig", aliases: &[], category: "net", usage: "iwconfig", description: "Show WiFi information where available.", capability: "net.read" },
    CommandSpec { name: "wifi-scan", aliases: &[], category: "net", usage: "wifi-scan", description: "List nearby WiFi networks with host tools.", capability: "net.read" },
    CommandSpec { name: "wifi-connect", aliases: &[], category: "net", usage: "wifi-connect <ssid> [password]", description: "Dry-run WiFi connection unless host network changes are enabled.", capability: "net.admin" },
    CommandSpec { name: "ping", aliases: &[], category: "net", usage: "ping <host>", description: "Run bounded host ping.", capability: "net.read" },
    CommandSpec { name: "nmcli", aliases: &[], category: "net", usage: "nmcli", description: "Show NetworkManager state on Linux.", capability: "net.read" },

    CommandSpec { name: "browser", aliases: &[], category: "host", usage: "browser <url|phase1|about>", description: "Fetch and render HTTP/HTTPS page text using guarded curl.", capability: "host.net" },
    CommandSpec { name: "python", aliases: &["py"], category: "host", usage: "python <file.py> | python -c <code>", description: "Run Python from VFS or inline code with a timeout.", capability: "host.exec" },
    CommandSpec { name: "gcc", aliases: &["cc"], category: "host", usage: "gcc <file.c> | gcc <code>", description: "Compile and run C with host compiler using timeouts.", capability: "host.exec" },
    CommandSpec { name: "plugins", aliases: &["plugin"], category: "host", usage: "plugins", description: "List Python plugins in ./plugins.", capability: "host.exec" },
    CommandSpec { name: "ned", aliases: &["nano", "vi"], category: "host", usage: "ned <file>", description: "Small VFS-backed line editor.", capability: "fs.write" },

    CommandSpec { name: "lspci", aliases: &[], category: "arch", usage: "lspci", description: "List simulated PCIe devices.", capability: "hw.read" },
    CommandSpec { name: "pcie", aliases: &[], category: "arch", usage: "pcie", description: "Show PCIe subsystem summary.", capability: "hw.read" },
    CommandSpec { name: "cr3", aliases: &[], category: "arch", usage: "cr3", description: "Show simulated CR3 value.", capability: "hw.read" },
    CommandSpec { name: "loadcr3", aliases: &[], category: "arch", usage: "loadcr3 <hex|decimal>", description: "Load simulated CR3 value with validation.", capability: "hw.write" },
    CommandSpec { name: "cr4", aliases: &[], category: "arch", usage: "cr4", description: "Show simulated CR4 flags.", capability: "hw.read" },
    CommandSpec { name: "pcide", aliases: &[], category: "arch", usage: "pcide on|off", description: "Toggle simulated CR4.PCIDE.", capability: "hw.write" },

    CommandSpec { name: "free", aliases: &["mem"], category: "sys", usage: "free", description: "Show simulated memory information.", capability: "sys.read" },
    CommandSpec { name: "df", aliases: &[], category: "sys", usage: "df", description: "Show simulated VFS capacity.", capability: "sys.read" },
    CommandSpec { name: "dmesg", aliases: &[], category: "sys", usage: "dmesg", description: "Show simulated boot messages.", capability: "sys.log" },
    CommandSpec { name: "vmstat", aliases: &[], category: "sys", usage: "vmstat", description: "Show simulated process and memory stats.", capability: "sys.read" },
    CommandSpec { name: "uname", aliases: &[], category: "sys", usage: "uname", description: "Show simulator kernel identity.", capability: "sys.read" },
    CommandSpec { name: "date", aliases: &[], category: "sys", usage: "date", description: "Show host UNIX timestamp.", capability: "sys.read" },
    CommandSpec { name: "uptime", aliases: &[], category: "sys", usage: "uptime", description: "Show simulator uptime.", capability: "sys.read" },
    CommandSpec { name: "hostname", aliases: &[], category: "sys", usage: "hostname", description: "Show virtual hostname.", capability: "sys.read" },

    CommandSpec { name: "env", aliases: &[], category: "user", usage: "env", description: "Print shell environment.", capability: "user.read" },
    CommandSpec { name: "export", aliases: &[], category: "user", usage: "export VAR=value", description: "Set an environment variable.", capability: "user.env" },
    CommandSpec { name: "unset", aliases: &[], category: "user", usage: "unset VAR", description: "Remove an environment variable.", capability: "user.env" },
    CommandSpec { name: "whoami", aliases: &[], category: "user", usage: "whoami", description: "Print current simulated user.", capability: "user.read" },
    CommandSpec { name: "id", aliases: &[], category: "user", usage: "id", description: "Print simulated user id.", capability: "user.read" },
    CommandSpec { name: "su", aliases: &[], category: "user", usage: "su <user>", description: "Switch simulated user.", capability: "user.switch" },
    CommandSpec { name: "history", aliases: &[], category: "user", usage: "history", description: "Show shell command history.", capability: "user.read" },

    CommandSpec { name: "help", aliases: &["commands"], category: "misc", usage: "help", description: "Show grouped command map.", capability: "none" },
    CommandSpec { name: "man", aliases: &[], category: "misc", usage: "man <command>", description: "Show a manual page.", capability: "none" },
    CommandSpec { name: "clear", aliases: &[], category: "misc", usage: "clear", description: "Clear terminal.", capability: "none" },
    CommandSpec { name: "version", aliases: &[], category: "misc", usage: "version", description: "Show phase1 version.", capability: "none" },
    CommandSpec { name: "sandbox", aliases: &["nsinfo"], category: "misc", usage: "sandbox", description: "Show safety model.", capability: "none" },
    CommandSpec { name: "exit", aliases: &["quit", "shutdown", "poweroff"], category: "misc", usage: "exit", description: "Terminate simulator.", capability: "none" },
];

pub const CATEGORIES: &[&str] = &["fs", "proc", "net", "host", "arch", "sys", "user", "misc"];

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
    out.push_str("\nquick : man browser | browser phase1 | ps | ls /\n");
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
    fn map_contains_core_categories() {
        let map = command_map();
        assert!(map.contains("fs"));
        assert!(map.contains("proc"));
        assert!(map.contains("net"));
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
