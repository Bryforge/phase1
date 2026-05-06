use crate::commands::Phase1Shell;
use crate::ui::BootConfig;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const GRAY: &str = "\x1b[90m";

const LOGO: &[&str] = &[
    "      ██████╗  ██╗ ",
    "      ██╔══██╗███║ ",
    "      ██████╔╝╚██║ ",
    "      ██╔═══╝  ██║ ",
    "      ██║      ██║ ",
    "      ╚═╝      ╚═╝ ",
    "  ╭────────────────╮",
    "  │  PHASE1  ░▒▓█  │",
    "  │  DEVKIT  █▓▒░  │",
    "  ╰────────────────╯",
    "     rainbow shell   ",
    "   fastfetch profile ",
];

const ASCII_LOGO: &[&str] = &[
    "      ____  _       ",
    "     |  _ \\| |      ",
    "     | |_) | |      ",
    "     |  __/| |___   ",
    "     |_|   |_____|  ",
    "  +----------------+",
    "  | PHASE1 DEVKIT  |",
    "  | RAINBOW SHELL  |",
    "  +----------------+",
    "   fastfetch style ",
];

pub fn run(shell: &mut Phase1Shell, config: BootConfig) -> String {
    shell.kernel.tick();
    shell.network.refresh();

    let display_version = crate::ui::display_version(crate::kernel::VERSION, config);
    let channel = if config.bleeding_edge {
        "bleeding-edge"
    } else {
        "release"
    };
    let security = if config.safe_mode {
        "safe-mode"
    } else {
        "host-capable"
    };
    let state = if config.persistent_state {
        "persistent"
    } else {
        "volatile"
    };
    let host_tools = if crate::policy::host_tools_allowed() {
        "enabled"
    } else {
        "disabled"
    };
    let jobs = shell.kernel.scheduler.jobs();
    let job_count = if jobs.trim() == "no background jobs" {
        0
    } else {
        jobs.lines().count()
    };
    let processes = shell.kernel.scheduler.ps().lines().skip(1).count();
    let audit_count = shell.kernel.audit.dump().lines().count();
    let pcie_count = shell.kernel.pcie.lspci().lines().count();
    let iface_count = shell
        .network
        .ifconfig()
        .lines()
        .filter(|line| line.contains(": flags=<"))
        .count();
    let uptime = shell.kernel.uptime().as_secs();
    let cwd = shell.kernel.vfs.cwd.display().to_string();

    let title = format!("{}@phase1", shell.user());
    let facts = vec![
        ("OS", "Phase1 Terminal OS Simulator".to_string()),
        ("Host", "Bryforge developer cyberdeck".to_string()),
        ("Kernel", format!("phase1 {display_version}")),
        ("Uptime", format!("{uptime}s")),
        (
            "Packages",
            format!("{} built-ins, Python/WASI plugin slots", crate::registry::COMMANDS.len()),
        ),
        ("Shell", "phase1 interactive shell".to_string()),
        ("Terminal", "ANSI operator console".to_string()),
        ("Theme", "rainbow".to_string()),
        ("Channel", channel.to_string()),
        ("Profile", config.profile_name()),
        ("Security", security.to_string()),
        ("Host Tools", host_tools.to_string()),
        ("Processes", format!("{processes} tasks, {job_count} background")),
        ("Hardware", format!("{pcie_count} PCIe devices, CR3 0x{:x}", shell.kernel.scheduler.get_cr3())),
        ("Network", format!("{iface_count} interfaces, privacy-safe summary")),
        ("Memory", "2.0 GiB / 4.0 GiB simulated".to_string()),
        ("Disk", "4 KiB / 1.0 GiB phase1fs".to_string()),
        ("Location", cwd),
        ("State", state.to_string()),
        ("Audit", format!("{audit_count} in-memory events")),
        ("Locale", "en_US.UTF-8".to_string()),
    ];

    let logo = if ascii_mode() { ASCII_LOGO } else { LOGO };
    let mut right = Vec::new();
    right.push(color_title(&title));
    right.push(color_rule(&"─".repeat(title.chars().count().max(16))));
    for (idx, (label, value)) in facts.iter().enumerate() {
        right.push(format!("{}{}{}: {}", rainbow_label(label, idx), if color_enabled() { RESET } else { "" }, value));
    }
    right.push(String::new());
    right.push(color_bars());

    let width = logo.iter().map(|line| line.chars().count()).max().unwrap_or(0);
    let rows = logo.len().max(right.len());
    let mut out = String::from("phase1 fastfetch\n");
    for idx in 0..rows {
        let left = logo.get(idx).copied().unwrap_or("");
        let left = rainbow_logo(left, idx);
        let plain_left = logo.get(idx).copied().unwrap_or("");
        let pad = width.saturating_sub(plain_left.chars().count());
        let right = right.get(idx).map(String::as_str).unwrap_or("");
        out.push_str(&left);
        out.push_str(&" ".repeat(pad + 3));
        out.push_str(right);
        out.push('\n');
    }
    out.push_str("privacy: simulated system facts only; no emails, passwords, tokens, or host secrets shown\n");
    out
}

fn color_title(text: &str) -> String {
    if !color_enabled() {
        return text.to_string();
    }
    format!("{BOLD}{CYAN}{text}{RESET}")
}

fn color_rule(text: &str) -> String {
    if !color_enabled() {
        return text.to_string();
    }
    format!("{GRAY}{text}{RESET}")
}

fn rainbow_label(label: &str, idx: usize) -> String {
    if !color_enabled() {
        return label.to_string();
    }
    let color = rainbow_color(idx);
    format!("{BOLD}{color}{label}")
}

fn rainbow_logo(line: &str, idx: usize) -> String {
    if !color_enabled() {
        return line.to_string();
    }
    format!("{}{line}{RESET}", rainbow_color(idx))
}

fn color_bars() -> String {
    if !color_enabled() {
        return "Colors: red yellow green cyan blue magenta".to_string();
    }
    format!(
        "{RED}████{YELLOW}████{GREEN}████{CYAN}████{BLUE}████{MAGENTA}████{RESET}"
    )
}

fn rainbow_color(idx: usize) -> &'static str {
    [RED, YELLOW, GREEN, CYAN, BLUE, MAGENTA][idx % 6]
}

fn color_enabled() -> bool {
    std::env::var_os("NO_COLOR").is_none()
        && std::env::var("PHASE1_NO_COLOR").ok().as_deref() != Some("1")
        && !ascii_mode()
}

fn ascii_mode() -> bool {
    std::env::var("PHASE1_ASCII").ok().as_deref() == Some("1")
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::commands::Phase1Shell;
    use crate::ui::BootConfig;

    fn config() -> BootConfig {
        BootConfig {
            color: true,
            ascii_mode: false,
            safe_mode: true,
            quick_boot: false,
            mobile_mode: false,
            persistent_state: false,
            bleeding_edge: true,
        }
    }

    #[test]
    fn fastfetch_reports_expected_fields() {
        std::env::set_var("PHASE1_ASCII", "1");
        std::env::set_var("PHASE1_NO_COLOR", "1");
        let mut shell = Phase1Shell::new();
        let out = run(&mut shell, config());
        assert!(out.contains("phase1 fastfetch"));
        assert!(out.contains("PHASE1 DEVKIT"));
        assert!(out.contains("OS:"));
        assert!(out.contains("Kernel:"));
        assert!(out.contains("Theme: rainbow"));
        assert!(out.contains("Packages:"));
        assert!(out.contains("Colors: red yellow green cyan blue magenta"));
        std::env::remove_var("PHASE1_ASCII");
        std::env::remove_var("PHASE1_NO_COLOR");
    }
}
