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

pub const CATEGORIES: &[&str] = &[
    "fs", "text", "proc", "net", "host", "dev", "editor", "arch", "sys", "user", "misc",
];

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
    cmd!("grep", &[], "text", "grep [-i] [-n] [-c] <pattern> <file>...", "Search VFS text files for a pattern.", "fs.read"),
    cmd!("wc", &[], "text", "wc [-l] [-w] [-c] <file>...", "Count VFS file lines, words, and bytes.", "fs.read"),
    cmd!("head", &[], "text", "head [-n count|-count] <file>...", "Show the first lines of VFS text files.", "fs.read"),
    cmd!("tail", &[], "text", "tail [-n count|-count] <file>...", "Show the last lines of VFS text files.", "fs.read"),
    cmd!("find", &[], "text", "find [path] [-name pattern] [-type f|d] [-maxdepth n]", "Search the VFS tree by name, type, and depth.", "fs.read"),
    cmd!("pipeline", &["pipes"], "text", "pipeline", "Show structured text pipeline help and supported filters.", "none"),
    cmd!("ps", &[], "proc", "ps", "Show simulated process table.", "proc.read"),
    cmd!("top", &[], "proc", "top", "Show scheduler state.", "proc.read"),
    cmd!("spawn", &[], "proc", "spawn <name> [args...] [--background]", "Create a simulated process through sys_spawn.", "proc.spawn"),
    cmd!("jobs", &[], "proc", "jobs", "List simulated background jobs.", "proc.read"),
    cmd!("fg", &[], "proc", "fg <pid>", "Move a simulated process to foreground.", "proc.manage"),
    cmd!("bg", &[], "proc", "bg <pid>", "Move a simulated process to background.", "proc.manage"),
    cmd!("kill", &[], "proc", "kill <pid>", "Terminate a simulated process through sys_kill.", "proc.kill"),
    cmd!("nice", &[], "proc", "nice <pid> <priority>", "Set simulated process priority.", "proc.manage"),
    cmd!("ifconfig", &[], "net", "ifconfig", "Show network interfaces. Safe mode uses simulated loopback only.", "net.read"),
    cmd!("iwconfig", &[], "net", "iwconfig", "Show WiFi information where available.", "net.read"),
    cmd!("wifi-scan", &[], "net", "wifi-scan", "List nearby WiFi networks only when trusted host tools are enabled.", "net.read"),
    cmd!("wifi-connect", &[], "net", "wifi-connect <ssid> [password]", "Dry-run WiFi connection unless host mutation is enabled.", "net.admin"),
    cmd!("ping", &[], "net", "ping <host>", "Run bounded host ping only when trusted host tools are enabled.", "net.read"),
    cmd!("nmcli", &[], "net", "nmcli", "Show NetworkManager state on Linux only when trusted host tools are enabled.", "net.read"),
    cmd!("browser", &[], "host", "browser <url|phase1|about>", "Fetch and render HTTP/HTTPS text using guarded curl.", "host.net"),
    cmd!("git", &[], "host", "git <args...>", "Run host Git through guarded operator passthrough at any Phase1 nest level.", "host.exec"),
    cmd!("gh", &["github"], "host", "gh <args...>", "Run GitHub CLI through guarded operator passthrough at any Phase1 nest level.", "host.exec"),
    cmd!("cargo", &[], "host", "cargo <args...>", "Run host Cargo through guarded operator passthrough at any Phase1 nest level.", "host.exec"),
    cmd!("rustc", &[], "host", "rustc <args...>", "Run host rustc through guarded operator passthrough at any Phase1 nest level.", "host.exec"),
    cmd!("python3", &[], "host", "python3 <args...>", "Run host Python 3 through guarded operator passthrough at any Phase1 nest level.", "host.exec"),
    cmd!("go", &["golang"], "host", "go <args...>", "Run host Go tooling through guarded operator passthrough at any Phase1 nest level.", "host.exec"),
    cmd!("python", &["py"], "host", "python <file.py> | python -c <code>", "Run Python with a timeout.", "host.exec"),
    cmd!("gcc", &["cc"], "host", "gcc <file.c> | gcc <code>", "Compile and run C with host compiler timeout guards.", "host.exec"),
    cmd!("plugins", &["plugin"], "host", "plugins", "List Python and WASI-lite plugins in ./plugins.", "host.exec"),
    cmd!("wasm", &["wasi"], "host", "wasm [list|inspect|run|validate] [plugin]", "Run or inspect sandboxed WASI-lite plugins without host shell access.", "wasm.exec"),
    cmd!("update", &["upgrade"], "host", "update [plan|check|--execute|protocol|test] [latest|stable] [--build]", "Safely plan updates or run developer validation suites; protocol prints patch-versioning rules.", "host.exec"),
    cmd!("ned", &["nano", "vi"], "host", "ned <file>", "Edit a VFS file with a small line editor.", "fs.write"),
    cmd!("avim", &["vim", "edit"], "dev", "avim <file>", "Advanced VFS-only modal editor with search, undo, yank/paste, substitute, and a security-focused no-shell-escape design.", "fs.write"),
    cmd!("emacs", &["emac", "phase1-emacs", "phase1emacs", "pemacs"], "editor", "emacs <file>", "Phase1 eMacs editor: VFS-native advanced editor powered by AVIM Pro.", "fs.write"),
    cmd!("dev", &["dock", "selfdev"], "dev", "dev [status|sync|branch|quick|test|commit|push|pr|merge|close|doctor]", "Phase1 self-development dock for working on Phase1 from inside Phase1.", "host.exec"),
    cmd!("repo", &["channels", "branches", "doctrine"], "dev", "repo [status|base|edge|checkpoint]", "Show the Phase1 repository channel doctrine: frozen base, active edge/stable path, checkpoints, and feature branch targets.", "none"),
    cmd!("fyr", &["phase1lang", "forge"], "dev", "fyr [status|spec|new|init|cat|color|check|build|test|self|run <file.fyr>]", "Phase1-native language target for self-construction and VFS automation.", "none"),
    cmd!("lang", &["language", "runlang"], "dev", "lang [list|support|status|doctor|detect|run|security]", "Native guarded multi-language runtime manager for major open-source programming languages.", "host.exec"),
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
    cmd!("sysinfo", &["fetch", "neofetch"], "sys", "sysinfo", "Show a privacy-safe one-screen phase1 system summary.", "sys.read"),
    cmd!("env", &[], "user", "env", "Print shell environment.", "user.read"),
    cmd!("export", &[], "user", "export VAR=value", "Set an environment variable.", "user.env"),
    cmd!("unset", &[], "user", "unset VAR", "Remove an environment variable.", "user.env"),
    cmd!("whoami", &[], "user", "whoami", "Print current simulated user.", "user.read"),
    cmd!("id", &[], "user", "id", "Print simulated user id.", "user.read"),
    cmd!("su", &[], "user", "su <user>", "Switch simulated user.", "user.switch"),
    cmd!("accounts", &["users"], "user", "accounts", "Explain and list simulated Unix accounts without real emails or credentials.", "user.read"),
    cmd!("history", &[], "user", "history [list|status|path|save|clear]", "Show shell command history and persistent-history status.", "user.read"),
    cmd!("learn", &["memory"], "user", "learn [status|doctor|import-history|note|teach|ask|explain|suggest|profile|forget|export]", "Use the local learning memory from inside Phase1 with sanitized notes, rules, command observations, and suggestions.", "learn.write"),
    cmd!("security", &["sec", "policy"], "user", "security", "Show safe mode, host tool gates, persistence, and privacy status.", "user.read"),
    cmd!("theme", &["style"], "user", "theme [show|list|rainbow|matrix|cyber|amber|ice|synthwave|crimson|mono|ascii|reset]", "Switch or inspect selectable terminal palettes. Rainbow remains the default.", "user.env"),
    cmd!("banner", &["splash"], "user", "banner [mobile|desktop|mono|rainbow|matrix|cyber|amber|ice|synthwave|crimson|ascii|safe|host|persist]", "Preview boot splash profile and color palette choices without changing saved config.", "user.read"),
    cmd!("tips", &["hint", "hints"], "user", "tips", "Show rotating operator tips for useful phase1 commands.", "user.read"),
    cmd!("help", &["commands"], "misc", "help", "Show grouped command map.", "none"),
    cmd!("man", &[], "misc", "man <command>", "Show generated command manual page.", "none"),
    cmd!("complete", &[], "misc", "complete [prefix]", "Show registry-backed command completions.", "none"),
    cmd!("capabilities", &["caps", "status"], "misc", "capabilities [features|--status]", "Show implementation status, command capability metadata, and guard status.", "none"),
    cmd!("dash", &["dashboard"], "misc", "dash [--compact]", "Show a compact operator dashboard snapshot.", "sys.read"),
    cmd!("matrix", &["rain"], "misc", "matrix [seconds|0|forever] [--speed ms] [--density n] [--chars set]", "Run enhanced Matrix rain. Press q to exit interactively; 0/forever runs until quit.", "none"),
    cmd!("bootcfg", &["bootconfig"], "misc", "bootcfg [show|save|reset|path]", "Show, save, reset, or locate the persisted boot profile in phase1.conf.", "none"),
    cmd!("clear", &[], "misc", "clear", "Clear terminal using an ANSI screen clear sequence.", "none"),
    cmd!("version", &["ver"], "misc", "version [--compare]", "Show release version or compare release with bleeding edge.", "none"),
    cmd!("roadmap", &["map"], "misc", "roadmap", "Show roadmap completion status for release versus bleeding edge.", "none"),
    cmd!("sandbox", &["nsinfo"], "misc", "sandbox", "Show safety model.", "none"),
    cmd!("nest", &["nests"], "misc", "nest [status|target|exit]", "Inspect and control nested Phase1 sessions, including exit-all unwinding.", "none"),
    cmd!("exit", &["quit", "shutdown", "poweroff"], "misc", "exit [all]", "Terminate simulator or unwind all nested Phase1 sessions.", "none"),
];

pub fn lookup(name: &str) -> Option<&'static CommandSpec> {
    COMMANDS
        .iter()
        .find(|cmd| cmd.name == name || cmd.aliases.contains(&name))
}

pub fn canonical_name(name: &str) -> Option<&'static str> {
    lookup(name).map(|cmd| cmd.name)
}

pub fn help(args: &[String]) -> String {
    let topic = args.first().map(String::as_str);

    match topic {
        None => command_map(),
        Some("--compact" | "-c" | "compact") => compact_command_map(),
        Some("ui" | "hud" | "palette" | "deck") => command_palette(),
        Some("flows" | "flow" | "workflow" | "workflows") => operator_flows(),
        Some(category) if CATEGORIES.contains(&category) => category_help(category),
        Some(command) => {
            if let Some(page) = man_page(command) {
                format!(
                    "phase1 help // {command}\n\n{page}\n\nroutes\n  man {command:<12} full manual\n  complete {command:<8} registry completions\n  help ui         command palette\n  help --compact  fast command map\n"
                )
            } else {
                format!(
                    "phase1 help // no match\n\nunknown topic : {command}\ntry           : help ui | help flows | help --compact | help fs | help host | help update | complete {command}\n"
                )
            }
        }
    }
}

pub fn command_map() -> String {
    let mut out = String::from("phase1 help // operator HUD\n");
    out.push_str("version       : v6 help surface\n");
    out.push_str("layout        : topic-aware command deck\n");
    out.push_str("guardrails    : safe-mode, host trust gate, audited writes\n\n");

    out.push_str("high signal\n");
    out.push_str("  dash             operator dashboard\n");
    out.push_str("  sysinfo          one-screen system summary\n");
    out.push_str("  security         trust, shield, persistence, privacy\n");
    out.push_str("  opslog           sanitized operator log controls\n");
    out.push_str("  update protocol  release and update rules\n\n");

    out.push_str("help routes\n");
    out.push_str("  help             full operator HUD\n");
    out.push_str("  help ui          visual command palette\n");
    out.push_str("  help flows       workflow launch paths\n");
    out.push_str("  help --compact   compact command map\n");
    out.push_str(
        "  help <category>  category deck: fs text proc net host dev editor arch sys user misc\n",
    );
    out.push_str("  help <command>   command manual card\n");
    out.push_str("  complete <text>  registry completions\n\n");

    out.push_str("command decks\n");
    for category in CATEGORIES {
        out.push_str(&format!("  {:<7} {}\n", category, command_names(category)));
    }

    out.push_str("\nquick routes\n");
    out.push_str("  status features | capabilities | version --compare | roadmap\n");
    out.push_str("  lang support | lang security | avim notes.rs | update test quick\n");
    out.push_str("  learn status | learn import-history | learn suggest | theme list | tips\n");
    out.push_str("  pipeline | wasm list | update protocol | sysinfo | security | opslog\n");
    out.push_str("\nvisual\n");
    out.push_str("  help ui          launch the command palette\n");
    out.push_str("  help flows       show operator workflows\n");
    out
}

fn command_palette() -> String {
    let rows = [
        ("search", "complete <prefix>"),
        ("inspect", "help <cmd> | man <cmd>"),
        ("operate", "dash | sysinfo | security"),
        ("logs", "opslog | audit"),
        ("build", "dev | repo | fyr | lang"),
        ("host", "cargo | rustc | git | gh"),
        ("recover", "help host | update protocol"),
        ("map", "roadmap | capabilities"),
    ];

    let label_width = rows
        .iter()
        .map(|(label, _)| visible_cell_width(label))
        .max()
        .unwrap_or(0);

    let width = 38;

    // Start on a fresh line so the prompt does not consume card width.
    let mut out = String::from(
        "
",
    );
    out.push_str(&palette_top(width));
    out.push_str(&palette_row("phase1 command palette", width));
    out.push_str(&palette_rule(width));

    for (label, value) in rows {
        let row = format!("{label:<label_width$}  {value}");
        out.push_str(&palette_row(&row, width));
    }

    out.push_str(&palette_bottom(width));
    out.push_str(
        "
",
    );

    out.push_str(
        "hot zones
",
    );
    out.push_str(
        "  CORE   dash sysinfo security opslog
",
    );
    out.push_str(
        "  BUILD  dev repo fyr lang cargo rustc
",
    );
    out.push_str(
        "  TEXT   grep find head tail wc pipeline
",
    );
    out.push_str(
        "  VFS    ls tree cat touch mkdir cp mv rm
",
    );
    out.push_str(
        "  STYLE  theme banner matrix tips clear
",
    );
    out.push_str(
        "  SAFE   capabilities sandbox audit update

",
    );

    out.push_str(
        "launch examples
",
    );
    out.push_str(
        "  help host     guarded host-tool deck
",
    );
    out.push_str(
        "  help dev      development command deck
",
    );
    out.push_str(
        "  help update   update manual card
",
    );
    out.push_str(
        "  complete th   discover theme aliases
",
    );
    out.push_str(
        "  man security  full manual page
",
    );
    out
}

fn palette_top(width: usize) -> String {
    format!(
        "╭{}╮
",
        "─".repeat(width)
    )
}

fn palette_rule(width: usize) -> String {
    format!(
        "├{}┤
",
        "─".repeat(width)
    )
}

fn palette_row(text: &str, width: usize) -> String {
    let inner = width.saturating_sub(2);
    let fitted = fit_visible(text, inner);
    let padding = " ".repeat(inner.saturating_sub(visible_cell_width(&fitted)));
    format!(
        "│ {fitted}{padding} │
"
    )
}

fn palette_bottom(width: usize) -> String {
    format!(
        "╰{}╯
",
        "─".repeat(width)
    )
}

fn fit_visible(text: &str, width: usize) -> String {
    if visible_cell_width(text) <= width {
        return text.to_string();
    }

    let ellipsis = "…";
    let limit = width.saturating_sub(visible_cell_width(ellipsis));
    let mut out = String::new();
    let mut cells = 0;

    for ch in text.chars() {
        let ch_width = char_cell_width(ch);
        if cells + ch_width > limit {
            break;
        }
        out.push(ch);
        cells += ch_width;
    }

    out.push_str(ellipsis);
    out
}

fn visible_cell_width(text: &str) -> usize {
    text.chars().map(char_cell_width).sum()
}

fn char_cell_width(ch: char) -> usize {
    if ch.is_control() {
        0
    } else if is_wide_cell(ch as u32) {
        2
    } else {
        1
    }
}

fn is_wide_cell(code: u32) -> bool {
    matches!(
        code,
        0x1100..=0x115F
            | 0x2329..=0x232A
            | 0x2E80..=0xA4CF
            | 0xAC00..=0xD7A3
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE19
            | 0xFE30..=0xFE6F
            | 0xFF00..=0xFF60
            | 0xFFE0..=0xFFE6
    )
}
fn operator_flows() -> String {
    let mut out = String::from("phase1 help // workflows\n\n");
    out.push_str("daily check\n");
    out.push_str("  sysinfo -> security -> opslog -> dash\n\n");
    out.push_str("safe update\n");
    out.push_str("  security -> update protocol -> update latest --trust-host --check\n\n");
    out.push_str("development\n");
    out.push_str("  repo status -> dev status -> cargo test -> gh pr create\n\n");
    out.push_str("recovery planning\n");
    out.push_str("  roadmap -> help host -> help update -> base1 docs/scripts\n\n");
    out.push_str("discoverability\n");
    out.push_str("  help ui -> help <category> -> help <command> -> complete <prefix>\n");
    out
}

fn compact_command_map() -> String {
    let mut out = String::from("phase1 help // compact\n\n");
    for category in CATEGORIES {
        out.push_str(&format!("{:<7}: {}\n", category, command_names(category)));
    }
    out.push_str("\ntry: help ui | help flows | help host | help update | help security | help --compact | complete th\n");
    out
}

fn category_help(category: &str) -> String {
    let mut out = format!("phase1 help // {category}\n\n");
    out.push_str("command        usage                              summary\n");
    for cmd in COMMANDS.iter().filter(|cmd| cmd.category == category) {
        out.push_str(&format!(
            "{:<14} {:<34} {}\n",
            cmd.name, cmd.usage, cmd.description
        ));
    }
    out.push_str("\nroutes\n");
    out.push_str(&format!(
        "  help ui         command palette\n  help --compact  compact command map\n  complete {category}     completions\n"
    ));
    out
}

fn command_names(category: &str) -> String {
    COMMANDS
        .iter()
        .filter(|cmd| cmd.category == category)
        .map(|cmd| cmd.name)
        .collect::<Vec<_>>()
        .join(" ")
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

pub fn capabilities_report() -> String {
    let mut out = feature_status_summary();
    out.push_str("\ncommand        category capability  guard\n");
    for cmd in COMMANDS {
        out.push_str(&format!(
            "{:<14} {:<8} {:<11} {}\n",
            cmd.name,
            cmd.category,
            cmd.capability,
            guard_status(cmd.capability)
        ));
    }
    out
}

fn feature_status_summary() -> String {
    "Phase1 feature status\n\nImplemented:\n  shell, VFS, simulated process table, audit log, /proc, text tools, dashboards, local learning, WASI-lite plugins\n\nExperimental:\n  lang run, direct Python wrapper, direct C/GCC wrapper, Git/storage helper, Rust/Cargo host workflows, browser host fetch\n\nRestricted:\n  host network inspection, host network mutation, self-update execution\n\nPlanned:\n  unified legacy runtime wrappers, doctor mobile, named boot profiles, broader language support\n\nNot planned:\n  full OS replacement, hardened VM/chroot/container sandbox\n\nMore detail:\n  FEATURE_STATUS.md\n"
        .to_string()
}

fn guard_status(capability: &str) -> &'static str {
    match capability {
        "none" => "open",
        "host.exec" | "host.net" => "safe-mode + PHASE1_ALLOW_HOST_TOOLS",
        "wasm.exec" => "phase1-wasi sandbox",
        "net.admin" => "safe-mode + host-tools + network-change opt-in",
        "learn.write" => "local sanitized memory",
        "hw.write" => "validated",
        "fs.write" | "proc.kill" | "proc.spawn" | "proc.manage" | "user.switch" | "user.env" => {
            "audited"
        }
        _ => "read-only/audited",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        canonical_name, capabilities_report, command_map, completions, help, lookup, man_page,
    };

    #[test]
    fn lookup_supports_aliases() {
        assert_eq!(lookup("py").map(|cmd| cmd.name), Some("python"));
        assert_eq!(lookup("quit").map(|cmd| cmd.name), Some("exit"));
        assert_eq!(lookup("rain").map(|cmd| cmd.name), Some("matrix"));
        assert_eq!(lookup("bootconfig").map(|cmd| cmd.name), Some("bootcfg"));
        assert_eq!(lookup("users").map(|cmd| cmd.name), Some("accounts"));
        assert_eq!(lookup("sec").map(|cmd| cmd.name), Some("security"));
        assert_eq!(lookup("policy").map(|cmd| cmd.name), Some("security"));
        assert_eq!(lookup("fetch").map(|cmd| cmd.name), Some("sysinfo"));
        assert_eq!(lookup("style").map(|cmd| cmd.name), Some("theme"));
        assert_eq!(lookup("splash").map(|cmd| cmd.name), Some("banner"));
        assert_eq!(lookup("hint").map(|cmd| cmd.name), Some("tips"));
        assert_eq!(lookup("upgrade").map(|cmd| cmd.name), Some("update"));
        assert_eq!(lookup("pipes").map(|cmd| cmd.name), Some("pipeline"));
        assert_eq!(lookup("map").map(|cmd| cmd.name), Some("roadmap"));
        assert_eq!(lookup("wasi").map(|cmd| cmd.name), Some("wasm"));
        assert_eq!(lookup("vim").map(|cmd| cmd.name), Some("avim"));
        assert_eq!(lookup("language").map(|cmd| cmd.name), Some("lang"));
        assert_eq!(lookup("status").map(|cmd| cmd.name), Some("capabilities"));
        assert_eq!(lookup("nests").map(|cmd| cmd.name), Some("nest"));
        assert_eq!(lookup("dock").map(|cmd| cmd.name), Some("dev"));
        assert_eq!(lookup("selfdev").map(|cmd| cmd.name), Some("dev"));
        assert_eq!(lookup("emac").map(|cmd| cmd.name), Some("emacs"));
        assert_eq!(lookup("phase1-emacs").map(|cmd| cmd.name), Some("emacs"));
        assert_eq!(lookup("phase1emacs").map(|cmd| cmd.name), Some("emacs"));
        assert_eq!(lookup("pemacs").map(|cmd| cmd.name), Some("emacs"));
        assert_eq!(lookup("channels").map(|cmd| cmd.name), Some("repo"));
        assert_eq!(lookup("branches").map(|cmd| cmd.name), Some("repo"));
        assert_eq!(lookup("doctrine").map(|cmd| cmd.name), Some("repo"));
        assert_eq!(lookup("phase1lang").map(|cmd| cmd.name), Some("fyr"));
    }

    #[test]
    fn canonical_name_normalizes_aliases() {
        assert_eq!(canonical_name("py"), Some("python"));
        assert_eq!(canonical_name("commands"), Some("help"));
        assert_eq!(canonical_name("caps"), Some("capabilities"));
        assert_eq!(canonical_name("status"), Some("capabilities"));
        assert_eq!(canonical_name("dashboard"), Some("dash"));
        assert_eq!(canonical_name("rain"), Some("matrix"));
        assert_eq!(canonical_name("bootconfig"), Some("bootcfg"));
        assert_eq!(canonical_name("users"), Some("accounts"));
        assert_eq!(canonical_name("sec"), Some("security"));
        assert_eq!(canonical_name("policy"), Some("security"));
        assert_eq!(canonical_name("neofetch"), Some("sysinfo"));
        assert_eq!(canonical_name("style"), Some("theme"));
        assert_eq!(canonical_name("upgrade"), Some("update"));
        assert_eq!(canonical_name("pipes"), Some("pipeline"));
        assert_eq!(canonical_name("ver"), Some("version"));
        assert_eq!(canonical_name("map"), Some("roadmap"));
        assert_eq!(canonical_name("wasi"), Some("wasm"));
        assert_eq!(canonical_name("edit"), Some("avim"));
        assert_eq!(canonical_name("runlang"), Some("lang"));
        assert_eq!(canonical_name("channels"), Some("repo"));
        assert_eq!(canonical_name("forge"), Some("fyr"));
    }

    #[test]
    fn command_map_contains_audit_and_complete() {
        let map = command_map();
        assert!(map.contains("audit"));
        assert!(map.contains("complete"));
        assert!(map.contains("capabilities"));
        assert!(map.contains("status features"));
        assert!(map.contains("dash"));
        assert!(map.contains("matrix"));
        assert!(map.contains("bootcfg"));
        assert!(map.contains("nest"));
        assert!(map.contains("accounts"));
        assert!(map.contains("security"));
        assert!(map.contains("sysinfo"));
        assert!(map.contains("theme"));
        assert!(map.contains("theme list"));
        assert!(map.contains("banner"));
        assert!(map.contains("tips"));
        assert!(map.contains("grep"));
        assert!(map.contains("find"));
        assert!(map.contains("update"));
        assert!(map.contains("update protocol"));
        assert!(map.contains("update test quick"));
        assert!(map.contains("pipeline"));
        assert!(map.contains("roadmap"));
        assert!(map.contains("wasm"));
        assert!(map.contains("avim"));
        assert!(map.contains("lang"));
        assert!(map.contains("dev"));
        assert!(map.contains("repo"));
        assert!(map.contains("fyr"));
    }

    #[test]
    fn modern_help_supports_topics_categories_and_compact_mode() {
        let empty: Vec<String> = Vec::new();
        let map = help(&empty);

        assert!(map.contains("phase1 help // operator HUD"));
        assert!(map.contains("help --compact"));
        assert!(map.contains("help <category>"));
        assert!(map.contains("guardrails"));
        assert!(map.contains("quick routes"));
        assert!(map.contains("theme list"));
        assert!(map.contains("update protocol"));

        let compact = help(&[String::from("--compact")]);
        assert!(compact.contains("phase1 help // compact"));
        assert!(compact.contains("host"));
        assert!(compact.contains("dev"));

        let host = help(&[String::from("host")]);
        assert!(host.contains("phase1 help // host"));
        assert!(host.contains("git"));
        assert!(host.contains("cargo"));

        let update = help(&[String::from("update")]);
        assert!(update.contains("phase1 help // update"));
        assert!(update.contains("validation suites"));
        assert!(update.contains("host.exec"));

        let ui = help(&[String::from("ui")]);
        assert!(ui.contains("phase1 command palette"));
        assert!(ui.contains("hot zones"));
        assert!(ui.contains("launch examples"));
        assert!(ui.contains("CORE"));
        assert!(ui.contains("BUILD"));

        let flows = help(&[String::from("flows")]);
        assert!(flows.contains("phase1 help // workflows"));
        assert!(flows.contains("daily check"));
        assert!(flows.contains("safe update"));
        assert!(flows.contains("recovery planning"));
    }

    #[test]
    fn man_pages_are_generated() {
        let page = man_page("update").expect("update man page");
        assert!(page.contains("protocol"));
        assert!(page.contains("validation suites"));
        assert!(page.contains("host.exec"));
        let pipeline = man_page("pipeline").expect("pipeline man page");
        assert!(pipeline.contains("structured text pipeline"));
        let wasm = man_page("wasm").expect("wasm man page");
        assert!(wasm.contains("WASI-lite"));
        assert!(wasm.contains("wasm.exec"));
        let theme = man_page("theme").expect("theme man page");
        assert!(theme.contains("matrix"));
        assert!(theme.contains("Rainbow remains the default"));
        let avim = man_page("avim").expect("avim man page");
        assert!(avim.contains("modal editor"));
        let lang = man_page("lang").expect("lang man page");
        assert!(lang.contains("multi-language"));
        let status = man_page("status").expect("status man page");
        assert!(status.contains("capabilities [features|--status]"));
        assert!(status.contains("implementation status"));
    }

    #[test]
    fn completions_include_aliases() {
        assert!(completions("p").contains(&"python"));
        assert!(completions("p").contains(&"py"));
        assert!(completions("p").contains(&"pipeline"));
        assert!(completions("r").contains(&"rain"));
        assert!(completions("r").contains(&"roadmap"));
        assert!(completions("boot").contains(&"bootcfg"));
        assert!(completions("sec").contains(&"security"));
        assert!(completions("st").contains(&"status"));
        assert!(completions("the").contains(&"theme"));
        assert!(completions("f").contains(&"find"));
        assert!(completions("f").contains(&"fyr"));
        assert!(completions("fo").contains(&"forge"));
        assert!(completions("g").contains(&"grep"));
        assert!(completions("u").contains(&"update"));
        assert!(completions("u").contains(&"upgrade"));
        assert!(completions("wa").contains(&"wasm"));
        assert!(completions("wa").contains(&"wasi"));
        assert!(completions("a").contains(&"avim"));
        assert!(completions("em").contains(&"emacs"));
        assert!(completions("ph").contains(&"phase1-emacs"));
        assert!(completions("v").contains(&"vim"));
        assert!(completions("la").contains(&"lang"));
        assert!(completions("de").contains(&"dev"));
        assert!(completions("do").contains(&"dock"));
        assert!(completions("do").contains(&"doctrine"));
        assert!(completions("re").contains(&"repo"));
        assert!(completions("ne").contains(&"nest"));
        assert!(completions("ne").contains(&"nests"));
    }

    #[test]
    fn host_passthrough_commands_are_registered() {
        for name in ["git", "gh", "cargo", "rustc", "python3", "go"] {
            let cmd = lookup(name).expect("host passthrough command registered");
            assert_eq!(cmd.category, "host");
            assert_eq!(cmd.capability, "host.exec");
        }
        assert_eq!(canonical_name("github"), Some("gh"));
        assert_eq!(canonical_name("golang"), Some("go"));
    }

    #[test]
    fn capabilities_report_includes_guard_status() {
        let report = capabilities_report();
        assert!(report.contains("Phase1 feature status"));
        assert!(report.contains("Implemented:"));
        assert!(report.contains("Experimental:"));
        assert!(report.contains("Restricted:"));
        assert!(report.contains("Not planned:"));
        assert!(report.contains("full OS replacement"));
        assert!(report.contains("FEATURE_STATUS.md"));
        assert!(report.contains("wifi-connect"));
        assert!(report.contains("network-change opt-in"));
        assert!(report.contains("PHASE1_ALLOW_HOST_TOOLS"));
        assert!(report.contains("theme"));
        assert!(report.contains("grep"));
        assert!(report.contains("update"));
        assert!(report.contains("pipeline"));
        assert!(report.contains("wasm"));
        assert!(report.contains("phase1-wasi sandbox"));
        assert!(report.contains("avim"));
        assert!(report.contains("lang"));
    }
}
