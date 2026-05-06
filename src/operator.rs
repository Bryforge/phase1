use crate::commands::Phase1Shell;
use crate::ui::{BootConfig, ThemePalette};

const TIPS: &[&str] = &[
    "Run security to verify safe mode, host tools, persistence, and privacy status.",
    "Run matrix 0 for digital rain, then press q to return cleanly.",
    "Use dash for the full-screen operator TUI or dash --compact for a fast snapshot.",
    "Use bootcfg show to inspect the saved preboot profile.",
    "Safe mode blocks host-backed execution by default.",
    "Persistent state only restores phase1-managed /home files.",
    "Try theme matrix, theme cyber, theme amber, theme ice, theme synthwave, or theme crimson.",
    "Boot with e=edge to enable the bleeding-edge version display and edge-only palette.",
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
                "theme: reset to bleeding-edge default\n".to_string()
            } else {
                set_palette(shell, ThemePalette::Rainbow);
                "theme: reset to rainbow default\n".to_string()
            }
        }
        Some("default") => {
            if bleeding_edge_active() {
                set_palette(shell, ThemePalette::BleedingEdge);
                "theme: bleeding-edge default enabled\n".to_string()
            } else {
                set_palette(shell, ThemePalette::Rainbow);
                "theme: rainbow default enabled\n".to_string()
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
            "desktop" | "--desktop" => preview.mobile_mode = false,
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
            "edge" | "bleeding" | "bleeding-edge" | "--edge" => {
                preview.bleeding_edge = true;
                preview.color = true;
                preview.ascii_mode = false;
                display = Some("bleeding-edge".to_string());
            }
            "safe" | "--safe" => preview.safe_mode = true,
            "host" | "--host" => preview.safe_mode = false,
            "trust" | "host-tools" | "--trust-host" => preview.host_tools = true,
            "persist" | "persistent" | "--persistent" => preview.persistent_state = true,
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

    let display = display.unwrap_or_else(|| {
        if preview.ascii_mode {
            "ascii".to_string()
        } else if preview.color {
            if preview.bleeding_edge {
                "bleeding-edge".to_string()
            } else {
                std::env::var("PHASE1_THEME")
                    .ok()
                    .and_then(|raw| {
                        ThemePalette::parse(&raw).map(|palette| palette.name().to_string())
                    })
                    .unwrap_or_else(|| "rainbow".to_string())
            }
        } else {
            "mono".to_string()
        }
    });

    format!(
        "banner preview\nprofile : {}\nchannel : {}\nversion : {}\nsecurity: {}\ntrust   : {}\nstate   : {}\ndisplay : {}\ntry     : cargo run, then use the preboot selector\n",
        preview.profile_name(),
        if preview.bleeding_edge { "bleeding-edge" } else { "release" },
        crate::ui::display_version(crate::kernel::VERSION, preview),
        if preview.safe_mode { "safe" } else { "host-capable" },
        if preview.host_tools && !preview.safe_mode { "enabled" } else if preview.host_tools { "armed/safe" } else { "off" },
        if preview.persistent_state { "persistent" } else { "volatile" },
        display,
    )
}

pub fn sysinfo(shell: &mut Phase1Shell, config: BootConfig) -> String {
    shell.kernel.tick();
    let processes = shell.kernel.scheduler.ps().lines().skip(1).count();
    let jobs = shell.kernel.scheduler.jobs();
    let job_count = if jobs.trim() == "no background jobs" {
        0
    } else {
        jobs.lines().count()
    };
    let audit_count = shell.kernel.audit.dump().lines().count();
    let pcie_count = shell.kernel.pcie.lspci().lines().count();

    format!(
        "phase1 sysinfo\nversion     : {}\nchannel     : {}\nprofile     : {}\nuser        : {}\nuid         : {}\ncwd         : {}\nuptime      : {}s\nsecurity    : {}\nstate       : {}\nprocesses   : {}\nbackground  : {}\npcie devices: {}\naudit events: {}\nhost tools  : {}\nprivacy     : no emails, passwords, tokens, or host account secrets are shown\n",
        crate::ui::display_version(crate::kernel::VERSION, config),
        if config.bleeding_edge { "bleeding-edge" } else { "release" },
        config.profile_name(),
        shell.user(),
        shell.kernel.scheduler.current_uid,
        shell.kernel.vfs.cwd.display(),
        shell.kernel.uptime().as_secs(),
        if config.safe_mode { "safe" } else { "host-capable" },
        if config.persistent_state { "persistent" } else { "volatile" },
        processes,
        job_count,
        pcie_count,
        audit_count,
        if crate::policy::host_tools_allowed() { "enabled" } else { "disabled" }
    )
}

pub fn dashboard(shell: &mut Phase1Shell, config: BootConfig, args: &[String]) -> String {
    shell.kernel.tick();
    shell.network.refresh();

    let compact = args
        .iter()
        .any(|arg| matches!(arg.as_str(), "--compact" | "compact" | "-c"));
    let display_version = crate::ui::display_version(crate::kernel::VERSION, config);
    let channel = if config.bleeding_edge {
        "bleeding-edge"
    } else {
        "release"
    };
    let uptime = shell.kernel.uptime().as_secs();
    let cwd = shell.kernel.vfs.cwd.display();
    let ps_output = shell.kernel.scheduler.ps();
    let process_count = ps_output.lines().skip(1).count();
    let jobs_output = shell.kernel.scheduler.jobs();
    let job_count = if jobs_output.trim() == "no background jobs" {
        0
    } else {
        jobs_output.lines().count()
    };
    let iface_count = shell
        .network
        .ifconfig()
        .lines()
        .filter(|line| line.contains(": flags=<"))
        .count();
    let audit_tail = shell
        .kernel
        .audit
        .dump()
        .lines()
        .last()
        .unwrap_or("audit log empty")
        .to_string();
    let audit_count = shell.kernel.audit.dump().lines().count();
    let pcie_count = shell.kernel.pcie.lspci().lines().count();
    let cr4 = shell.kernel.scheduler.cr4();
    let safety = if crate::policy::host_tools_allowed() {
        "host-enabled"
    } else {
        "safe-mode"
    };

    if compact {
        return format!(
            "PHASE1 DASHBOARD v{}\nCORE  user={} uptime={}s mode=operator channel={}\nPROC  tasks={} bg={}\nVFS   cwd={} mounts=/,/proc,/dev,/tmp,/var/log\nNET   interfaces={} safety={}\nHW    cr3=0x{:x} {} pcie={}\nAUDIT latest={}\n",
            display_version,
            shell.user(),
            uptime,
            channel,
            process_count,
            job_count,
            cwd,
            iface_count,
            safety,
            shell.kernel.scheduler.get_cr3(),
            cr4,
            pcie_count,
            audit_tail
        );
    }

    format!(
        "PHASE1 FULL-SCREEN TUI DASHBOARD v{}\nchannel : {}\nprofile : {}\nmode    : {}\npanels  : core proc vfs net hw audit\n\n┌ CORE ─────────────────────────────────────┐\n│ user {:<10} uid {:<5} uptime {:>6}s │\n│ state {:<10} cwd {:<20} │\n└───────────────────────────────────────────┘\n┌ PROC ─────────────────────────────────────┐\n│ tasks {:<5} background {:<5} scheduler live │\n└───────────────────────────────────────────┘\n┌ VFS / NET ────────────────────────────────┐\n│ mounts /,/proc,/dev,/tmp,/var/log         │\n│ interfaces {:<5} safety {:<15} │\n└───────────────────────────────────────────┘\n┌ HW / AUDIT ───────────────────────────────┐\n│ cr3 0x{:<12x} cr4 {:<14} │\n│ pcie {:<5} audit events {:<6}             │\n│ latest {:<35} │\n└───────────────────────────────────────────┘\ncontrols: dash --compact | sysinfo | security | reboot\nstatus  : live snapshot, privacy-safe, no host secrets shown\n",
        display_version,
        channel,
        config.profile_name(),
        if config.safe_mode { "safe" } else { "host-capable" },
        shell.user(),
        shell.kernel.scheduler.current_uid,
        uptime,
        if config.persistent_state { "persistent" } else { "volatile" },
        cwd,
        process_count,
        job_count,
        iface_count,
        safety,
        shell.kernel.scheduler.get_cr3(),
        cr4,
        pcie_count,
        audit_count,
        audit_tail.chars().take(35).collect::<String>(),
    )
}

pub fn tips(shell: &Phase1Shell) -> String {
    let seed = shell.history.len() + shell.kernel.uptime().as_secs() as usize;
    let mut out = String::from("phase1 tips\n");
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
            .unwrap_or_else(|| {
                if bleeding_edge_active() {
                    ThemePalette::BleedingEdge.name().to_string()
                } else {
                    ThemePalette::Rainbow.name().to_string()
                }
            })
    };

    format!(
        "theme status\nactive : {active}\nchannel: {}\ncolor  : {}\nascii  : {}\n",
        if bleeding_edge_active() {
            "bleeding-edge"
        } else {
            "release"
        },
        if color { "on" } else { "off" },
        if ascii { "on" } else { "off" }
    )
}

fn theme_list() -> String {
    let mut out = String::from("theme list\n");
    for palette in ThemePalette::all() {
        out.push_str(&format!("- {:<13} {}\n", palette.name(), palette.label()));
    }
    out.push_str("- mono          no color, normal unicode\n");
    out.push_str("- ascii         no color, ASCII-compatible prompt\n");
    out.push_str("- reset         channel default\n");
    out
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
            bleeding_edge: false,
            host_tools: false,
        }
    }

    #[test]
    fn banner_preview_reports_modes() {
        let config = config();
        let out = banner(config, &["persistent".to_string(), "mono".to_string()]);
        assert!(out.contains("profile : mobile-safe"));
        assert!(out.contains("channel : release"));
        assert!(out.contains("state   : persistent"));
        assert!(out.contains("display : mono"));

        let cyber = banner(config, &["cyber".to_string()]);
        assert!(cyber.contains("display : cyber"));

        let edge = banner(config, &["edge".to_string()]);
        assert!(edge.contains("channel : bleeding-edge"));
        assert!(edge.contains("display : bleeding-edge"));

        let trusted = banner(config, &["host".to_string(), "trust".to_string()]);
        assert!(trusted.contains("trust   : enabled"));
    }

    #[test]
    fn theme_list_is_available() {
        let mut shell = Phase1Shell::new();
        let out = theme(&mut shell, &["list".to_string()]);
        assert!(out.contains("rainbow"));
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
    fn dashboard_reports_fullscreen_and_compact_modes() {
        let mut shell = Phase1Shell::new();
        let full = dashboard(&mut shell, config(), &[]);
        assert!(full.contains("PHASE1 FULL-SCREEN TUI DASHBOARD"));
        assert!(full.contains("panels  : core proc vfs net hw audit"));
        assert!(full.contains("controls: dash --compact"));

        let compact = dashboard(&mut shell, config(), &["--compact".to_string()]);
        assert!(compact.contains("PHASE1 DASHBOARD v3.6.0"));
        assert!(compact.contains("CORE  user=root"));
    }
}
