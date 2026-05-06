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

const MOBILE_CARD: &[&str] = &[
    "╭─ phase1 devkit ─────────╮",
    "│    ◢◤ PHASE1 CORE       │",
    "│ update everything        │",
    "│ code · test · ship       │",
    "╰── mobile fastfetch ─────╯",
];

const MOBILE_CARD_ASCII: &[&str] = &[
    ".-== phase1 devkit ==-.",
    "|   P1 CORE          |",
    "| update everything  |",
    "| code test ship     |",
    "'-- mobile fastfetch-'",
];

pub fn run(shell: &mut Phase1Shell, config: BootConfig) -> String {
    shell.kernel.tick();
    shell.network.refresh();

    let facts = collect_facts(shell, config);
    let color = config.color && !config.ascii_mode;
    if config.mobile_mode {
        render_mobile(&facts, color)
    } else {
        render_desktop(&facts, color)
    }
}

fn collect_facts(shell: &Phase1Shell, config: BootConfig) -> Vec<(&'static str, String)> {
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

    vec![
        ("OS", "Phase1".to_string()),
        ("Host", "Bryforge cyberdeck".to_string()),
        ("Kernel", format!("phase1 {display_version}")),
        ("Uptime", format!("{uptime}s")),
        (
            "Pkgs",
            format!("{} built-ins", crate::registry::COMMANDS.len()),
        ),
        ("Shell", "phase1".to_string()),
        ("Terminal", "ANSI console".to_string()),
        ("Theme", "rainbow".to_string()),
        ("Branch", channel.to_string()),
        ("State", state.to_string()),
        ("Security", security.to_string()),
        ("HostTools", host_tools.to_string()),
        ("Proc", format!("{processes} tasks, {job_count} bg")),
        (
            "HW",
            format!(
                "{pcie_count} PCIe, CR3 0x{:x}",
                shell.kernel.scheduler.get_cr3()
            ),
        ),
        ("Net", format!("{iface_count} iface")),
        ("Memory", "2.0/4.0 GiB sim".to_string()),
        ("Disk", "4 KiB/1 GiB phase1fs".to_string()),
        ("Path", cwd),
        ("Audit", format!("{audit_count} events")),
        ("DevKit", "ready to test code".to_string()),
        ("Tests", "update test quick".to_string()),
    ]
}

fn render_mobile(facts: &[(&str, String)], color: bool) -> String {
    let mut out = String::new();
    out.push_str(&prompt_line(color));
    out.push('\n');

    let card = if color {
        MOBILE_CARD
    } else {
        MOBILE_CARD_ASCII
    };
    for (idx, line) in card.iter().enumerate() {
        out.push_str(&rainbow_logo(line, idx, color));
        out.push('\n');
    }

    out.push_str(&color_title(color));
    out.push('\n');
    out.push_str(&mobile_rule(color));
    out.push('\n');

    for (idx, label) in [
        "OS", "Kernel", "Branch", "Security", "State", "Path", "DevKit", "Tests",
    ]
    .iter()
    .enumerate()
    {
        out.push_str(&compact_fact_line(
            label,
            fact_value(facts, label),
            idx,
            color,
        ));
        out.push('\n');
    }

    out.push_str(&mobile_section("system meters", 0, color));
    out.push('\n');
    out.push_str(&meter_line("CPU", 6, "Rust scheduler", 1, color));
    out.push('\n');
    out.push_str(&meter_line("MEM", 4, fact_value(facts, "Memory"), 2, color));
    out.push('\n');
    out.push_str(&meter_line("DSK", 2, fact_value(facts, "Disk"), 3, color));
    out.push('\n');
    out.push_str(&meter_line("NET", 1, fact_value(facts, "Net"), 4, color));
    out.push('\n');
    out.push_str(&meter_line(
        "SEC",
        8,
        fact_value(facts, "Security"),
        5,
        color,
    ));
    out.push('\n');

    out.push_str(&color_bars_compact(color));
    out.push('\n');
    out.push_str("privacy: simulated facts only\n");
    out
}

fn render_desktop(facts: &[(&str, String)], color: bool) -> String {
    let mut right = Vec::new();
    right.push(color_title(color));
    right.push(color_rule("------------------------------", color));
    for (idx, (label, value)) in facts.iter().enumerate() {
        right.push(fact_line(label, value, idx, color));
    }
    right.push(String::new());
    right.push(color_bars(color));

    let width = DESKTOP_LOGO
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    let rows = DESKTOP_LOGO.len().max(right.len());
    let mut out = String::new();
    out.push_str(&prompt_line(color));
    out.push('\n');
    for idx in 0..rows {
        let plain_left = DESKTOP_LOGO.get(idx).copied().unwrap_or("");
        let left = rainbow_logo(plain_left, idx, color);
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

fn prompt_line(color: bool) -> String {
    if !color {
        return "dev@localhost ~ $ fastfetch".to_string();
    }
    format!("{GREEN}dev{RESET}@{MAGENTA}localhost{RESET} {BLUE}~{RESET} $ fastfetch")
}

fn color_title(color: bool) -> String {
    if !color {
        return "dev@localhost".to_string();
    }
    format!("{BOLD}{RED}dev{RESET}@{BOLD}{YELLOW}local{GREEN}host{RESET}")
}

fn color_rule(text: &str, color: bool) -> String {
    if !color {
        return text.to_string();
    }
    format!("{GRAY}{text}{RESET}")
}

fn mobile_rule(color: bool) -> String {
    if !color {
        "------------------------".to_string()
    } else {
        color_rule("────────────────────────", color)
    }
}

fn mobile_section(title: &str, idx: usize, color: bool) -> String {
    if !color {
        return format!("-- {title} --");
    }
    let color_code = rainbow_color(idx);
    format!("{color_code}╭─ {title} ─────────╮{RESET}")
}

fn fact_line(label: &str, value: &str, idx: usize, color: bool) -> String {
    if !color {
        return format!("{label:<10}: {value}");
    }
    let color_code = rainbow_color(idx);
    format!("{color_code}{BOLD}{label:<10}{RESET} {value}")
}

fn compact_fact_line(label: &str, value: &str, idx: usize, color: bool) -> String {
    if !color {
        return format!("{label:<9}: {value}");
    }
    let color_code = rainbow_color(idx);
    format!("{color_code}{BOLD}{label:<9}{RESET} {value}")
}

fn meter_line(label: &str, filled: usize, value: &str, idx: usize, color: bool) -> String {
    let total = 8;
    let filled = filled.min(total);
    if !color {
        let bar = format!("{}{}", "#".repeat(filled), ".".repeat(total - filled));
        return format!("{label:<3} [{bar}] {value}");
    }
    let color_code = rainbow_color(idx);
    let bar = format!(
        "{color_code}{}{GRAY}{}{RESET}",
        "█".repeat(filled),
        "░".repeat(total - filled)
    );
    format!("{color_code}{BOLD}{label:<3}{RESET} [{bar}] {value}")
}

fn fact_value<'a>(facts: &'a [(&str, String)], label: &str) -> &'a str {
    facts
        .iter()
        .find(|(name, _)| *name == label)
        .map(|(_, value)| value.as_str())
        .unwrap_or("n/a")
}

fn rainbow_logo(line: &str, idx: usize, color: bool) -> String {
    if !color {
        return line.to_string();
    }
    format!("{}{line}{RESET}", rainbow_color(idx))
}

fn color_bars(color: bool) -> String {
    if !color {
        return "Colors: black red yellow green cyan blue magenta white".to_string();
    }
    format!("{GRAY}███{RESET} {RED}███{RESET} {YELLOW}███{RESET} {GREEN}███{RESET} {CYAN}███{RESET} {BLUE}███{RESET} {MAGENTA}███{RESET} {WHITE}███{RESET}")
}

fn color_bars_compact(color: bool) -> String {
    if !color {
        return "Colors: blk red yel grn cyn blu mag wht".to_string();
    }
    format!("{GRAY}██{RESET} {RED}██{RESET} {YELLOW}██{RESET} {GREEN}██{RESET} {CYAN}██{RESET} {BLUE}██{RESET} {MAGENTA}██{RESET} {WHITE}██{RESET}")
}

fn rainbow_color(idx: usize) -> &'static str {
    [RED, YELLOW, GREEN, CYAN, BLUE, MAGENTA][idx % 6]
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::commands::Phase1Shell;
    use crate::ui::BootConfig;

    fn config(mobile_mode: bool) -> BootConfig {
        BootConfig {
            color: false,
            ascii_mode: true,
            safe_mode: true,
            quick_boot: false,
            mobile_mode,
            persistent_state: false,
            bleeding_edge: true,
            host_tools: false,
        }
    }

    #[test]
    fn fastfetch_reports_expected_fields() {
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
    }

    #[test]
    fn mobile_fastfetch_stacks_instead_of_wrapping() {
        let mut shell = Phase1Shell::new();
        let out = run(&mut shell, config(true));
        assert!(out.contains("dev@localhost"));
        assert!(out.contains("update everything"));
        assert!(out.contains("system meters"));
        assert!(out.contains("code test ship"));
        assert!(out.contains("Colors: blk red yel grn cyn blu mag wht"));
        assert!(out.lines().all(|line| line.chars().count() <= 44));
    }
}
