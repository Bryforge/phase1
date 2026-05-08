use crate::commands::Phase1Shell;
use crate::ui::{BootConfig, ThemePalette};

const TIPS: &[&str] = &[
    "Run dash for the edge operator deck, then dash --compact for a fast status strip.",
    "Use theme bleeding-edge after booting with e=edge for the edge-only palette.",
    "Run capabilities to see which developer commands are guarded, audited, or sandboxed.",
    "Use lang support and lang security before trusting host-backed language runtimes.",
    "Use avim for VFS-only editing when you want a no-shell-escape dev surface.",
    "Use wasm list to inspect sandboxed WASI-lite plugins without opening host shell access.",
    "Run update test quick for a safe validation plan before touching host git state.",
    "Safe mode blocks host-backed execution; TRUST HOST only arms the gate after SHIELD is off.",
    "Persistent state restores only phase1-managed /home files; keep real secrets outside Phase1.",
];

pub fn theme(shell: &mut Phase1Shell, args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("show") => theme_status(shell),
        Some("list") | Some("ls") => theme_list(),
        Some("mono") | Some("plain") => {
            std::env::set_var("PHASE1_NO_COLOR", "1");
            std::env::remove_var("PHASE1_ASCII");
            shell
                .env
                .insert("PHASE1_THEME".to_string(), "mono".to_string());
            shell
                .env
                .insert("PHASE1_NO_COLOR".to_string(), "1".to_string());
            shell
                .env
                .insert("PHASE1_ASCII".to_string(), "0".to_string());
            "theme: mono terminal enabled\n".to_string()
        }
        Some("ascii") => {
            std::env::set_var("PHASE1_ASCII", "1");
            std::env::set_var("PHASE1_NO_COLOR", "1");
            shell
                .env
                .insert("PHASE1_THEME".to_string(), "ascii".to_string());
            shell
                .env
                .insert("PHASE1_NO_COLOR".to_string(), "1".to_string());
            shell
                .env
                .insert("PHASE1_ASCII".to_string(), "1".to_string());
            "theme: ascii compatibility enabled\n".to_string()
        }
        Some("reset") => {
            if bleeding_edge_active() {
                set_palette(shell, ThemePalette::BleedingEdge);
                "theme: reset to edge operator deck default\n".to_string()
            } else {
                set_palette(shell, ThemePalette::Rainbow);
                "theme: reset to rainbow default\n".to_string()
            }
        }
        Some("default") | Some("deck") | Some("ops") | Some("operator") => {
            if bleeding_edge_active() {
                set_palette(shell, ThemePalette::BleedingEdge);
                "theme: edge operator deck default enabled\n".to_string()
            } else {
                set_palette(shell, ThemePalette::NeoTokyo);
                "theme: neo-tokyo operator default enabled\n".to_string()
            }
        }
        Some(raw) => match ThemePalette::parse(raw) {
            Some(ThemePalette::BleedingEdge) if !bleeding_edge_active() => {
                "theme: bleeding-edge palette requires booting with e=edge first\n".to_string()
            }
            Some(palette) => {
                set_palette(shell, palette);
                format!("theme: {} enabled\n", palette.name())
            }
            None => format!("theme: unknown theme '{raw}'\n{}", theme_list()),
        },
    }
}

pub fn banner(config: BootConfig, args: &[String]) -> String {
    let mut preview = config;
    let mut display = None;
    for arg in args {
        match arg.as_str() {
            "mobile" | "--mobile" => preview.mobile_mode = true,
            "desktop" | "--desktop" | "workstation" | "--workstation" => {
                preview.mobile_mode = false;
            }
            "mono" | "--mono" => {
                preview.color = false;
                preview.ascii_mode = false;
                display = Some("mono".to_string());
            }
            "neon" | "rainbow" | "--color" => {
                preview.color = true;
                preview.ascii_mode = false;
                display = Some("rainbow".to_string());
            }
            "ascii" | "--ascii" => {
                preview.ascii_mode = true;
                preview.color = false;
                display = Some("ascii".to_string());
            }
            "edge" | "bleeding" | "bleeding-edge" | "--edge" | "deck" | "opsdeck" => {
                preview.bleeding_edge = true;
                preview.color = true;
                preview.ascii_mode = false;
                display = Some("edge-operator-deck".to_string());
            }
            "safe" | "--safe" => preview.safe_mode = true,
            "host" | "--host" => preview.safe_mode = false,
            "trust" | "host-tools" | "--trust-host" => preview.host_tools = true,
            "persist" | "persistent" | "--persistent" | "vault" => {
                preview.persistent_state = true;
            }
            "volatile" | "--volatile" => preview.persistent_state = false,
            other => {
                if let Some(palette) = ThemePalette::parse(other) {
                    if palette == ThemePalette::BleedingEdge && !preview.bleeding_edge {
                        continue;
                    }
                    preview.color = true;
                    preview.ascii_mode = false;
                    display = Some(palette.name().to_string());
                }
            }
        }
    }

    let display = display.unwrap_or_else(|| display_label(preview));
    format!(
        "banner preview // operator deck\nprofile : {}\nchannel : {}\nversion : {}\nsecurity: {}\ntrust   : {}\nstate   : {}\ndisplay : {}\nlayout  : signal-first developer cockpit\ntry     : cargo run, boot with e=edge, then run dash\n",
        preview.profile_name(),
        channel_label(preview),
        crate::ui::display_version(crate::kernel::VERSION, preview),
        if preview.safe_mode { "safe" } else { "host-capable" },
        trust_status(preview),
        if preview.persistent_state { "persistent" } else { "volatile" },
        display,
    )
}

pub fn sysinfo(shell: &mut Phase1Shell, config: BootConfig) -> String {
    shell.kernel.tick();
    let processes = shell.kernel.scheduler.ps().lines().skip(1).count();
    let jobs_output = shell.kernel.scheduler.jobs();
    let job_count = job_count(&jobs_output);
    let audit_count = shell.kernel.audit.dump().lines().count();
    let pcie_count = shell.kernel.pcie.lspci().lines().count();

    format!(
        "phase1 sysinfo\nversion     : {}\nchannel     : {}\nprofile     : {}\nuser        : {}\nuid         : {}\ncwd         : {}\nuptime      : {}s\nsecurity    : {}\ntrust       : {}\nstate       : {}\nprocesses   : {}\nbackground  : {}\npcie devices: {}\naudit events: {}\nhost tools  : {}\nui          : edge operator deck capable\nprivacy     : no emails, passwords, tokens, or host account secrets are shown\n",
        crate::ui::display_version(crate::kernel::VERSION, config),
        channel_label(config),
        config.profile_name(),
        shell.user(),
        shell.kernel.scheduler.current_uid,
        shell.kernel.vfs.cwd.display(),
        shell.kernel.uptime().as_secs(),
        if config.safe_mode { "safe" } else { "host-capable" },
        trust_status(config),
        if config.persistent_state { "persistent" } else { "volatile" },
        processes,
        job_count,
        pcie_count,
        audit_count,
        if crate::policy::host_tools_allowed() {
            "enabled"
        } else {
            "disabled"
        }
    )
}

pub fn dashboard(shell: &mut Phase1Shell, config: BootConfig, args: &[String]) -> String {
    shell.kernel.tick();
    shell.network.refresh();

    let compact = args
        .iter()
        .any(|arg| matches!(arg.as_str(), "--compact" | "compact" | "-c"));
    let focus = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--focus=").map(str::to_string))
        .unwrap_or_else(|| "dev".to_string());
    let display_version = crate::ui::display_version(crate::kernel::VERSION, config);
    let uptime = shell.kernel.uptime().as_secs();
    let cwd = shell.kernel.vfs.cwd.display().to_string();
    let process_count = shell.kernel.scheduler.ps().lines().skip(1).count();
    let jobs_output = shell.kernel.scheduler.jobs();
    let job_count = job_count(&jobs_output);
    let ifconfig = shell.network.ifconfig();
    let iface_count = ifconfig
        .lines()
        .filter(|line| line.contains(": flags=<"))
        .count();
    let audit_dump = shell.kernel.audit.dump();
    let audit_count = audit_dump.lines().count();
    let audit_signal = audit_signal(&audit_dump);
    let pcie_count = shell.kernel.pcie.lspci().lines().count();
    let cr3 = shell.kernel.scheduler.get_cr3();
    let cr4 = shell.kernel.scheduler.cr4();
    let safety = if crate::policy::host_tools_allowed() {
        "host-enabled"
    } else {
        "safe-mode"
    };
    let dev_activity = developer_activity(shell);
    let command_heat = command_heat(shell);
    let trust_meter = trust_meter(config);
    let runtime_meter = runtime_meter(process_count, job_count, iface_count, audit_count);

    if compact {
        return format!(
            "PHASE1 EDGE OPERATOR DECK v{}\nCORE  user={} uptime={}s channel={} profile={}\nTRUST {} {} safety={} state={}\nDEV   focus={} activity={} cwd={}\nRUNTIME proc={} bg={} net={} pcie={} audit={}\nHW    cr3=0x{:x} {}\nNEXT  {}\n",
            display_version,
            shell.user(),
            uptime,
            channel_label(config),
            config.profile_name(),
            progress_bar(trust_meter, 12),
            trust_status(config),
            safety,
            if config.persistent_state { "persistent" } else { "volatile" },
            focus,
            dev_activity,
            trim_middle(&cwd, 32),
            process_count,
            job_count,
            iface_count,
            pcie_count,
            audit_count,
            cr3,
            cr4,
            next_action(config, &dev_activity),
        );
    }

    format!(
        "PHASE1 FULL-SCREEN TUI DASHBOARD v{}\nEDGE OPERATOR DECK // signal-first developer cockpit\nchannel : {}\nprofile : {}\nfocus   : {}\nprivacy : raw command/audit payloads are not displayed\n\n┌─ SIGNAL BUS ───────────────────────────────────────────────┐\n│ user {:<10} uid {:<5} uptime {:>6}s command-heat {:<8} │\n│ cwd  {:<52} │\n│ boot {:<15} state {:<12} theme {:<15} │\n└────────────────────────────────────────────────────────────┘\n┌─ TRUST BOUNDARY ───────────────────────────────────────────┐\n│ shield {:<11} host {:<13} meter {:<18} │\n│ policy {:<18} persistence {:<12} audit-class {:<12} │\n└────────────────────────────────────────────────────────────┘\n┌─ DEVELOPER COCKPIT ────────────────────────────────────────┐\n│ activity {:<14} next {:<34} │\n│ surfaces avim lang wasm update pipeline grep find theme     │\n│ quick    dash --compact | capabilities | lang security      │\n└────────────────────────────────────────────────────────────┘\n┌─ RUNTIME GRAPH ────────────────────────────────────────────┐\n│ proc {:<4} bg {:<4} net-if {:<4} pcie {:<4} audit {:<5} {:<12} │\n│ cr3  0x{:<12x} cr4 {:<20} │\n└────────────────────────────────────────────────────────────┘\n┌─ COMMAND RADAR ────────────────────────────────────────────┐\n│ {} │\n│ {} │\n│ {} │\n└────────────────────────────────────────────────────────────┘\ncontrols: dash --compact | dash --focus=net | theme deck | banner edge | reboot\nstatus  : edge-only operator UI, safe-by-default, developer-focused\n",
        display_version,
        channel_label(config),
        config.profile_name(),
        focus,
        shell.user(),
        shell.kernel.scheduler.current_uid,
        uptime,
        command_heat,
        trim_middle(&cwd, 52),
        if config.bleeding_edge { "edge" } else { "release" },
        if config.persistent_state { "persistent" } else { "volatile" },
        active_theme_label(),
        if config.safe_mode { "on" } else { "off" },
        trust_status(config),
        progress_bar(trust_meter, 18),
        safety,
        if config.persistent_state { "vault" } else { "ram" },
        audit_signal,
        dev_activity,
        next_action(config, &dev_activity),
        process_count,
        job_count,
        iface_count,
        pcie_count,
        audit_count,
        progress_bar(runtime_meter, 12),
        cr3,
        cr4,
        radar_line(0),
        radar_line(1),
        radar_line(2),
    )
}

pub fn tips(shell: &Phase1Shell) -> String {
    let seed = shell.history.len() + shell.kernel.uptime().as_secs() as usize;
    let mut out = String::from("phase1 tips // edge operator deck\n");
    for idx in 0..3 {
        let tip = TIPS[(seed + idx) % TIPS.len()];
        out.push_str(&format!("- {tip}\n"));
    }
    out
}

fn set_palette(shell: &mut Phase1Shell, palette: ThemePalette) {
    std::env::remove_var("PHASE1_NO_COLOR");
    std::env::remove_var("PHASE1_ASCII");
    std::env::set_var("PHASE1_THEME", palette.name());
    shell
        .env
        .insert("PHASE1_THEME".to_string(), palette.name().to_string());
    shell
        .env
        .insert("PHASE1_NO_COLOR".to_string(), "0".to_string());
    shell
        .env
        .insert("PHASE1_ASCII".to_string(), "0".to_string());
}

fn bleeding_edge_active() -> bool {
    std::env::var("PHASE1_BLEEDING_EDGE").ok().as_deref() == Some("1")
}

fn theme_status(shell: &Phase1Shell) -> String {
    let color = std::env::var("PHASE1_NO_COLOR").ok().as_deref() != Some("1");
    let ascii = std::env::var("PHASE1_ASCII").ok().as_deref() == Some("1");
    let active = if ascii {
        "ascii".to_string()
    } else if !color {
        "mono".to_string()
    } else {
        shell
            .env
            .get("PHASE1_THEME")
            .and_then(|raw| ThemePalette::parse(raw).map(|theme| theme.name().to_string()))
            .or_else(|| {
                std::env::var("PHASE1_THEME")
                    .ok()
                    .and_then(|raw| ThemePalette::parse(&raw).map(|theme| theme.name().to_string()))
            })
            .unwrap_or_else(active_theme_label)
    };

    format!(
        "theme status\nactive : {active}\nchannel: {}\ncolor  : {}\nascii  : {}\ndeck   : {}\n",
        if bleeding_edge_active() {
            "bleeding-edge"
        } else {
            "release"
        },
        if color { "on" } else { "off" },
        if ascii { "on" } else { "off" },
        if bleeding_edge_active() {
            "edge operator deck"
        } else {
            "standard operator console"
        }
    )
}

fn theme_list() -> String {
    let mut out = String::from("theme list\n");
    out.push_str("- deck          edge-aware operator deck default\n");
    out.push_str("- operator      alias for deck/default\n");
    for palette in ThemePalette::all() {
        out.push_str(&format!("- {:<13} {}\n", palette.name(), palette.label()));
    }
    out.push_str("- mono          no color, normal unicode\n");
    out.push_str("- ascii         no color, ASCII-compatible prompt\n");
    out.push_str("- reset         channel default\n");
    out
}

fn trust_status(config: BootConfig) -> &'static str {
    match (config.safe_mode, config.host_tools) {
        (true, true) => "armed/safe",
        (true, false) => "sealed",
        (false, true) => "host-enabled",
        (false, false) => "host-off",
    }
}

fn trust_meter(config: BootConfig) -> usize {
    match (config.safe_mode, config.host_tools, config.persistent_state) {
        (true, false, false) => 10,
        (true, false, true) => 9,
        (true, true, _) => 7,
        (false, false, _) => 5,
        (false, true, false) => 3,
        (false, true, true) => 2,
    }
}

fn runtime_meter(
    process_count: usize,
    job_count: usize,
    iface_count: usize,
    audit_count: usize,
) -> usize {
    let score = process_count + job_count * 2 + iface_count + audit_count.min(8);
    score.clamp(1, 10)
}

fn command_heat(shell: &Phase1Shell) -> &'static str {
    match shell.history.len() {
        0..=2 => "cold",
        3..=8 => "warm",
        9..=24 => "active",
        _ => "hot",
    }
}

fn developer_activity(shell: &Phase1Shell) -> String {
    let recent = shell
        .history
        .iter()
        .rev()
        .take(12)
        .map(|line| line.split_whitespace().next().unwrap_or(""))
        .collect::<Vec<_>>();
    for needle in [
        "lang", "avim", "wasm", "update", "grep", "find", "matrix", "theme",
    ] {
        if recent.iter().any(|cmd| *cmd == needle) {
            return needle.to_string();
        }
    }
    "standby".to_string()
}

fn next_action(config: BootConfig, activity: &str) -> &'static str {
    match activity {
        "standby" => "help or dash --compact",
        "lang" => "lang security",
        "avim" => "avim :w then grep",
        "wasm" => "wasm validate",
        "update" => "update test quick",
        "grep" | "find" => "pipeline",
        "matrix" => "theme deck",
        "theme" => "banner edge",
        _ if config.safe_mode => "security",
        _ => "capabilities",
    }
}

fn audit_signal(audit_dump: &str) -> String {
    let Some(line) = audit_dump.lines().last() else {
        return "empty".to_string();
    };
    line.split_whitespace()
        .next()
        .map(|token| token.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '.'))
        .filter(|token| !token.is_empty())
        .unwrap_or("event")
        .chars()
        .take(12)
        .collect()
}

fn active_theme_label() -> String {
    std::env::var("PHASE1_THEME")
        .ok()
        .and_then(|raw| ThemePalette::parse(&raw).map(|theme| theme.name().to_string()))
        .unwrap_or_else(|| {
            if bleeding_edge_active() {
                ThemePalette::BleedingEdge.name().to_string()
            } else {
                ThemePalette::NeoTokyo.name().to_string()
            }
        })
}

fn progress_bar(value: usize, width: usize) -> String {
    let filled = value.min(10) * width / 10;
    let mut out = String::with_capacity(width + 2);
    out.push('[');
    for idx in 0..width {
        out.push(if idx < filled { '█' } else { '·' });
    }
    out.push(']');
    out
}

fn trim_middle(raw: &str, width: usize) -> String {
    let len = raw.chars().count();
    if len <= width {
        return format!("{raw:<width$}");
    }
    if width <= 3 {
        return raw.chars().take(width).collect();
    }
    let left = (width - 1) / 2;
    let right = width - left - 1;
    let start = raw.chars().take(left).collect::<String>();
    let end = raw
        .chars()
        .rev()
        .take(right)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<String>();
    format!("{start}…{end}")
}

fn radar_line(idx: usize) -> String {
    let lines = [
        "dev  avim ━ lang ━ wasm ━ update ━ tests".to_string(),
        "sec  shield ━ policy ━ audit ━ capabilities".to_string(),
        "ops  dash ━ sysinfo ━ matrix ━ reboot".to_string(),
    ];
    format!("{:<58}", lines[idx])
}

fn channel_label(config: BootConfig) -> &'static str {
    if config.bleeding_edge {
        "bleeding-edge"
    } else {
        "release"
    }
}

fn display_label(config: BootConfig) -> String {
    if config.ascii_mode {
        "ascii".to_string()
    } else if !config.color {
        "mono".to_string()
    } else if config.bleeding_edge {
        "edge-operator-deck".to_string()
    } else {
        "neo-tokyo".to_string()
    }
}

fn job_count(jobs_output: &str) -> usize {
    if jobs_output.trim() == "no background jobs" {
        0
    } else {
        jobs_output.lines().count()
    }
}

#[cfg(test)]
mod tests {
    use super::{banner, dashboard, theme};
    use crate::commands::Phase1Shell;
    use crate::ui::BootConfig;

    fn config() -> BootConfig {
        BootConfig {
            color: true,
            ascii_mode: false,
            safe_mode: true,
            quick_boot: false,
            mobile_mode: true,
            persistent_state: false,
            bleeding_edge: true,
            host_tools: false,
        }
    }

    #[test]
    fn banner_preview_reports_modes() {
        let config = config();
        let out = banner(config, &["persistent".to_string(), "mono".to_string()]);
        assert!(out.contains("profile : mobile-safe+edge"));
        assert!(out.contains("channel : bleeding-edge"));
        assert!(out.contains("state   : persistent"));
        assert!(out.contains("display : mono"));
        assert!(out.contains("operator deck"));

        let cyber = banner(config, &["cyber".to_string()]);
        assert!(cyber.contains("display : cyber"));

        let edge = banner(config, &["edge".to_string()]);
        assert!(edge.contains("channel : bleeding-edge"));
        assert!(edge.contains("display : edge-operator-deck"));

        let trusted = banner(config, &["host".to_string(), "trust".to_string()]);
        assert!(trusted.contains("trust   : host-enabled"));
    }

    #[test]
    fn theme_list_is_available() {
        let mut shell = Phase1Shell::new();
        let out = theme(&mut shell, &["list".to_string()]);
        assert!(out.contains("deck"));
        assert!(out.contains("neo-tokyo"));
        assert!(out.contains("matrix"));
        assert!(out.contains("cyber"));
        assert!(out.contains("synthwave"));
        assert!(out.contains("bleeding-edge"));
        assert!(out.contains("ascii"));
    }

    #[test]
    fn theme_sets_named_palette() {
        let mut shell = Phase1Shell::new();
        let out = theme(&mut shell, &["matrix".to_string()]);
        assert!(out.contains("matrix enabled"));
        assert_eq!(
            shell.env.get("PHASE1_THEME").map(String::as_str),
            Some("matrix")
        );
        let status = theme(&mut shell, &[]);
        assert!(status.contains("active : matrix"));
    }

    #[test]
    fn bleeding_edge_theme_requires_edge_mode() {
        std::env::remove_var("PHASE1_BLEEDING_EDGE");
        let mut shell = Phase1Shell::new();
        let blocked = theme(&mut shell, &["bleeding-edge".to_string()]);
        assert!(blocked.contains("requires booting with e=edge"));

        std::env::set_var("PHASE1_BLEEDING_EDGE", "1");
        let enabled = theme(&mut shell, &["bleeding-edge".to_string()]);
        assert!(enabled.contains("bleeding-edge enabled"));
        std::env::remove_var("PHASE1_BLEEDING_EDGE");
    }

    #[test]
    fn dashboard_reports_edge_operator_deck_and_compact_modes() {
        let mut shell = Phase1Shell::new();
        let full = dashboard(&mut shell, config(), &[]);
        assert!(full.contains("PHASE1 FULL-SCREEN TUI DASHBOARD"));
        assert!(full.contains("EDGE OPERATOR DECK"));
        assert!(full.contains("SIGNAL BUS"));
        assert!(full.contains("TRUST BOUNDARY"));
        assert!(full.contains("DEVELOPER COCKPIT"));
        assert!(full.contains("privacy : raw command/audit payloads are not displayed"));
        assert!(full.contains("controls: dash --compact"));

        let compact = dashboard(&mut shell, config(), &["--compact".to_string()]);
        assert!(compact.contains(&format!(
            "PHASE1 EDGE OPERATOR DECK v{}",
            env!("CARGO_PKG_VERSION")
        )));
        assert!(compact.contains("CORE  user=root"));
        assert!(compact.contains("TRUST"));
    }
}
