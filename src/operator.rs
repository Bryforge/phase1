use crate::commands::Phase1Shell;
use crate::kernel::VERSION;
use crate::ui::{BootConfig, ThemePalette};

const TIPS: &[&str] = &[
    "Run security to verify safe mode, host tools, persistence, and privacy status.",
    "Run matrix 0 for digital rain, then press q to return cleanly.",
    "Use dash --compact for a fast operator snapshot.",
    "Use bootcfg show to inspect the saved preboot profile.",
    "Safe mode blocks host-backed execution by default.",
    "Persistent state only restores phase1-managed /home files.",
    "Try theme matrix, theme cyber, theme amber, theme ice, theme synthwave, or theme crimson.",
];

pub fn theme(shell: &mut Phase1Shell, args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("show") => theme_status(shell),
        Some("list") | Some("ls") => theme_list(),
        Some("mono") | Some("plain") => {
            std::env::set_var("PHASE1_NO_COLOR", "1");
            std::env::remove_var("PHASE1_ASCII");
            shell.env.insert("PHASE1_THEME".to_string(), "mono".to_string());
            shell.env.insert("PHASE1_NO_COLOR".to_string(), "1".to_string());
            shell.env.insert("PHASE1_ASCII".to_string(), "0".to_string());
            "theme: mono terminal enabled\n".to_string()
        }
        Some("ascii") => {
            std::env::set_var("PHASE1_ASCII", "1");
            std::env::set_var("PHASE1_NO_COLOR", "1");
            shell.env.insert("PHASE1_THEME".to_string(), "ascii".to_string());
            shell.env.insert("PHASE1_NO_COLOR".to_string(), "1".to_string());
            shell.env.insert("PHASE1_ASCII".to_string(), "1".to_string());
            "theme: ascii compatibility enabled\n".to_string()
        }
        Some("reset") => {
            set_palette(shell, ThemePalette::Rainbow);
            "theme: reset to rainbow default\n".to_string()
        }
        Some("default") => {
            set_palette(shell, ThemePalette::Rainbow);
            "theme: rainbow default enabled\n".to_string()
        }
        Some(raw) => match ThemePalette::parse(raw) {
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
            "safe" | "--safe" => preview.safe_mode = true,
            "host" | "--host" => preview.safe_mode = false,
            "persist" | "persistent" | "--persistent" => preview.persistent_state = true,
            "volatile" | "--volatile" => preview.persistent_state = false,
            other => {
                if let Some(palette) = ThemePalette::parse(other) {
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
            std::env::var("PHASE1_THEME")
                .ok()
                .and_then(|raw| ThemePalette::parse(&raw).map(|palette| palette.name().to_string()))
                .unwrap_or_else(|| "rainbow".to_string())
        } else {
            "mono".to_string()
        }
    });

    format!(
        "banner preview\nprofile : {}\nsecurity: {}\nstate   : {}\ndisplay : {}\ntry     : cargo run, then use the preboot selector\n",
        preview.profile_name(),
        if preview.safe_mode { "safe" } else { "host-capable" },
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
        "phase1 sysinfo\nversion     : {VERSION}\nprofile     : {}\nuser        : {}\nuid         : {}\ncwd         : {}\nuptime      : {}s\nsecurity    : {}\nstate       : {}\nprocesses   : {}\nbackground  : {}\npcie devices: {}\naudit events: {}\nhost tools  : {}\nprivacy     : no emails, passwords, tokens, or host account secrets are shown\n",
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
            .unwrap_or_else(|| ThemePalette::Rainbow.name().to_string())
    };

    format!(
        "theme status\nactive : {active}\ncolor  : {}\nascii  : {}\n",
        if color { "on" } else { "off" },
        if ascii { "on" } else { "off" }
    )
}

fn theme_list() -> String {
    let mut out = String::from("theme list\n");
    for palette in ThemePalette::all() {
        out.push_str(&format!("- {:<10} {}\n", palette.name(), palette.label()));
    }
    out.push_str("- mono       no color, normal unicode\n");
    out.push_str("- ascii      no color, ASCII-compatible prompt\n");
    out.push_str("- reset      rainbow default\n");
    out
}

#[cfg(test)]
mod tests {
    use super::{banner, theme};
    use crate::commands::Phase1Shell;
    use crate::ui::BootConfig;

    #[test]
    fn banner_preview_reports_modes() {
        let config = BootConfig {
            color: true,
            ascii_mode: false,
            safe_mode: true,
            quick_boot: false,
            mobile_mode: true,
            persistent_state: false,
        };
        let out = banner(config, &["persistent".to_string(), "mono".to_string()]);
        assert!(out.contains("profile : mobile-safe"));
        assert!(out.contains("state   : persistent"));
        assert!(out.contains("display : mono"));

        let cyber = banner(config, &["cyber".to_string()]);
        assert!(cyber.contains("display : cyber"));
    }

    #[test]
    fn theme_list_is_available() {
        let mut shell = Phase1Shell::new();
        let out = theme(&mut shell, &["list".to_string()]);
        assert!(out.contains("rainbow"));
        assert!(out.contains("matrix"));
        assert!(out.contains("cyber"));
        assert!(out.contains("synthwave"));
        assert!(out.contains("ascii"));
    }

    #[test]
    fn theme_sets_named_palette() {
        let mut shell = Phase1Shell::new();
        let out = theme(&mut shell, &["matrix".to_string()]);
        assert!(out.contains("matrix enabled"));
        assert_eq!(shell.env.get("PHASE1_THEME").map(String::as_str), Some("matrix"));
        let status = theme(&mut shell, &[]);
        assert!(status.contains("active : matrix"));
    }
}
