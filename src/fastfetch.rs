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
const WHITE: &str = "\x1b[97m";

const DESKTOP_LOGO: &[&str] = &[
    "      .-==++#%@@%#++==-.      ",
    "   .+##*-............-*##+.   ",
    "  =##+:   .-======-.   :+##=  ",
    " +##:  .+####----####+.  :##+ ",
    " ##+  +####:  P1  :####+  +## ",
    " ##+  +####.PHASE1.####+  +## ",
    " +##:  .+####----####+.  :##+ ",
    "  =##+:   .-======-.   :+##=  ",
    "   .+##*-..update..-*##+.     ",
    "      '-==++#%@@%#++==-'      ",
];

const MOBILE_LOGO: &[&str] = &[
    "   .-== PHASE1 ==-.",
    "  /  P1 DEVKIT    \",
    " | update everything |",
    " | rainbow shell     |",
    "  \__ fastfetch ___/",
];

pub fn run(shell: &mut Phase1Shell, config: BootConfig) -> String {
    shell.kernel.tick();
    shell.network.refresh();

    let facts = collect_facts(shell, config);
    if config.mobile_mode {
        render_mobile(&facts)
    } else {
        render_desktop(&facts)
    }
}

fn collect_facts(shell: &Phase1Shell, config: BootConfig) -> Vec<(&'static str, String)> {
    let display_version = crate::ui::display_version(crate::kernel::VERSION, config);
    let channel = if config.bleeding_edge { "bleeding-edge" } else { "release" };
    let security = if config.safe_mode { "safe-mode" } else { "host-capable" };
    let state = if config.persistent_state { "persistent" } else { "volatile" };
    let host_tools = if crate::policy::host_tools_allowed() { "enabled" } else { "disabled" };
    let jobs = shell.kernel.scheduler.jobs();
    let job_count = if jobs.trim() == "no background jobs" { 0 } else { jobs.lines().count() };
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

    vec![
        ("OS", "Phase1".to_string()),
        ("Host", "Bryforge cyberdeck".to_string()),
        ("Kernel", format!("phase1 {display_version}")),
        ("Uptime", format!("{uptime}s")),
        ("Pkgs", format!("{} built-ins", crate::registry::COMMANDS.len())),
        ("Shell", "phase1".to_string()),
        ("Terminal", "ANSI console".to_string()),
        ("Theme", "rainbow".to_string()),
        ("Branch", channel.to_string()),
        ("State", state.to_string()),
        ("Security", security.to_string()),
        ("HostTools", host_tools.to_string()),
        ("Proc", format!("{processes} tasks, {job_count} bg")),
        ("HW", format!("{pcie_count} PCIe, CR3 0x{:x}", shell.kernel.scheduler.get_cr3())),
        ("Net", format!("{iface_count} iface")),
        ("Memory", "2.0/4.0 GiB sim".to_string()),
        ("Disk", "4 KiB/1 GiB phase1fs".to_string()),
        ("Path", cwd),
        ("Audit", format!("{audit_count} events")),
        ("DevKit", "ready to test code".to_string()),
        ("Tests", "update test quick".to_string()),
    ]
}

fn render_mobile(facts: &[(&str, String)]) -> String {
    let mut out = String::new();
    out.push_str(&prompt_line());
    out.push('\n');
    for (idx, line) in MOBILE_LOGO.iter().enumerate() {
        out.push_str(&rainbow_logo(line, idx));
        out.push('\n');
    }
    out.push_str(&color_title());
    out.push('\n');
    out.push_str(&color_rule("----------------------"));
    out.push('\n');
    for (idx, (label, value)) in facts.iter().enumerate() {
        out.push_str(&compact_fact_line(label, value, idx));
        out.push('\n');
    }
    out.push_str(&color_bars_compact());
    out.push('\n');
    out.push_str("privacy: simulated facts only\n");
    out
}

fn render_desktop(facts: &[(&str, String)]) -> String {
    let mut right = Vec::new();
    right.push(color_title());
    right.push(color_rule("------------------------------"));
    for (idx, (label, value)) in facts.iter().enumerate() {
        right.push(fact_line(label, value, idx));
    }
    right.push(String::new());
    right.push(color_bars());

    let width = DESKTOP_LOGO
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    let rows = DESKTOP_LOGO.len().max(right.len());
    let mut out = String::new();
    out.push_str(&prompt_line());
    out.push('\n');
    for idx in 0..rows {
        let plain_left = DESKTOP_LOGO.get(idx).copied().unwrap_or("");
        let left = rainbow_logo(plain_left, idx);
        let pad = width.saturating_sub(plain_left.chars().count());
        let right = right.get(idx).map(String::as_str).unwrap_or("");
        out.push_str(&left);
        out.push_str(&" ".repeat(pad + 3));
        out.push_str(right);
        out.push('\n');
    }
    out.push_str("privacy: simulated facts only\n");
    out
}

fn prompt_line() -> String {
    if !color_enabled() {
        return "dev@localhost ~ $ fastfetch".to_string();
    }
    format!("{GREEN}dev{RESET}@{MAGENTA}localhost{RESET} {BLUE}~{RESET} $ fastfetch")
}

fn color_title() -> String {
    if !color_enabled() {
        return "dev@localhost".to_string();
    }
    format!("{BOLD}{RED}dev{RESET}@{BOLD}{YELLOW}local{GREEN}host{RESET}")
}

fn color_rule(text: &str) -> String {
    if !color_enabled() {
        return text.to_string();
    }
    format!("{GRAY}{text}{RESET}")
}

fn fact_line(label: &str, value: &str, idx: usize) -> String {
    if !color_enabled() {
        return format!("{label:<10}: {value}");
    }
    let color = rainbow_color(idx);
    format!("{color}{BOLD}{label:<10}{RESET} {value}")
}

fn compact_fact_line(label: &str, value: &str, idx: usize) -> String {
    if !color_enabled() {
        return format!("{label:<9}: {value}");
    }
    let color = rainbow_color(idx);
    format!("{color}{BOLD}{label:<9}{RESET} {value}")
}

fn rainbow_logo(line: &str, idx: usize) -> String {
    if !color_enabled() {
        return line.to_string();
    }
    format!("{}{line}{RESET}", rainbow_color(idx))
}

fn color_bars() -> String {
    if !color_enabled() {
        return "Colors: black red yellow green cyan blue magenta white".to_string();
    }
    format!("{GRAY}███{RESET} {RED}███{RESET} {YELLOW}███{RESET} {GREEN}███{RESET} {CYAN}███{RESET} {BLUE}███{RESET} {MAGENTA}███{RESET} {WHITE}███{RESET}")
}

fn color_bars_compact() -> String {
    if !color_enabled() {
        return "Colors: blk red yel grn cyn blu mag wht".to_string();
    }
    format!("{GRAY}██{RESET} {RED}██{RESET} {YELLOW}██{RESET} {GREEN}██{RESET} {CYAN}██{RESET} {BLUE}██{RESET} {MAGENTA}██{RESET} {WHITE}██{RESET}")
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

    fn config(mobile_mode: bool) -> BootConfig {
        BootConfig {
            color: true,
            ascii_mode: false,
            safe_mode: true,
            quick_boot: false,
            mobile_mode,
            persistent_state: false,
            bleeding_edge: true,
        }
    }

    #[test]
    fn fastfetch_reports_expected_fields() {
        std::env::set_var("PHASE1_ASCII", "1");
        std::env::set_var("PHASE1_NO_COLOR", "1");
        let mut shell = Phase1Shell::new();
        let out = run(&mut shell, config(false));
        assert!(out.contains("dev@localhost ~ $ fastfetch"));
        assert!(out.contains("PHASE1"));
        assert!(out.contains("OS"));
        assert!(out.contains("Kernel"));
        assert!(out.contains("Theme"));
        assert!(out.contains("rainbow"));
        assert!(out.contains("Pkgs"));
        assert!(out.contains("DevKit"));
        assert!(out.contains("ready to test code"));
        assert!(out.contains("Colors: black red yellow green cyan blue magenta white"));
        std::env::remove_var("PHASE1_ASCII");
        std::env::remove_var("PHASE1_NO_COLOR");
    }

    #[test]
    fn mobile_fastfetch_stacks_instead_of_wrapping() {
        std::env::set_var("PHASE1_ASCII", "1");
        std::env::set_var("PHASE1_NO_COLOR", "1");
        let mut shell = Phase1Shell::new();
        let out = run(&mut shell, config(true));
        assert!(out.contains("dev@localhost"));
        assert!(out.contains("update everything"));
        assert!(out.contains("Colors: blk red yel grn cyn blu mag wht"));
        assert!(out.lines().all(|line| line.chars().count() <= 44));
        std::env::remove_var("PHASE1_ASCII");
        std::env::remove_var("PHASE1_NO_COLOR");
    }
}
