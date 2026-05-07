use crate::commands::Phase1Shell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LinuxColorDepth {
    TrueColor,
    Color256,
    Ansi16,
    Mono,
}

impl LinuxColorDepth {
    fn name(self) -> &'static str {
        match self {
            Self::TrueColor => "truecolor",
            Self::Color256 => "256color",
            Self::Ansi16 => "ansi16",
            Self::Mono => "mono",
        }
    }

    fn palette_name(self) -> &'static str {
        match self {
            Self::TrueColor => "cyber",
            Self::Color256 => "synthwave",
            Self::Ansi16 => "matrix",
            Self::Mono => "mono",
        }
    }

    fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "truecolor" | "24bit" | "rgb" | "full" | "max" => Some(Self::TrueColor),
            "256" | "256color" | "xterm-256" => Some(Self::Color256),
            "16" | "ansi" | "ansi16" | "tty" | "console" | "linux" | "x200" | "trisquel" => {
                Some(Self::Ansi16)
            }
            "mono" | "off" | "none" | "dumb" | "serial" | "vt100" | "rpi-safe" => {
                Some(Self::Mono)
            }
            _ => None,
        }
    }
}

pub fn is_linux_alias(raw: &str) -> bool {
    matches!(
        raw,
        "linux" | "linux-pack" | "linux-color" | "colors" | "colorpack"
    )
}

pub fn theme(shell: &mut Phase1Shell, args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status") | Some("show") => status(shell),
        Some("preview") | Some("swatch") | Some("swatches") => preview(detect()),
        Some("apply") | Some("auto") | Some("on") => apply(shell, detect()),
        Some("x200") | Some("trisquel") => apply(shell, LinuxColorDepth::Ansi16),
        Some("pi") | Some("raspi") | Some("raspberry-pi") | Some("raspberrypi") => {
            apply(shell, raspberry_pi_default())
        }
        Some("rpi5") | Some("pi5") | Some("raspberry-pi-5") | Some("safe-pi") => {
            apply(shell, LinuxColorDepth::Mono)
        }
        Some("off") | Some("reset") => {
            clear(shell);
            "theme linux: color pack disabled; existing palette remains active\n".to_string()
        }
        Some(raw) => match LinuxColorDepth::parse(raw) {
            Some(depth) => apply(shell, depth),
            None => format!("theme linux: unknown option '{raw}'\nusage: theme linux [status|preview|apply|truecolor|256|ansi|mono|x200|raspberry-pi|rpi5|off]\n"),
        },
    }
}

pub fn summary(shell: &Phase1Shell) -> String {
    match (
        value(shell, "PHASE1_COLOR_PACK"),
        value(shell, "PHASE1_COLOR_DEPTH"),
    ) {
        (Some(pack), Some(depth)) => format!("{pack}/{depth}"),
        (Some(pack), None) => pack,
        _ => "default".to_string(),
    }
}

pub fn status(shell: &Phase1Shell) -> String {
    let detected = detect();
    let configured_pack = value(shell, "PHASE1_COLOR_PACK").unwrap_or_else(|| "none".to_string());
    let configured_depth =
        value(shell, "PHASE1_COLOR_DEPTH").unwrap_or_else(|| detected.name().to_string());
    let term = std::env::var("TERM").unwrap_or_else(|_| "unknown".to_string());
    let colorterm = std::env::var("COLORTERM").unwrap_or_else(|_| "unset".to_string());
    format!(
        "theme linux color pack\nhost target      : {}\nplatform hint    : {}\nTERM             : {}\nCOLORTERM        : {}\ndetected depth   : {}\nconfigured pack  : {}\nconfigured depth : {}\nrpi5 compat      : {}\ncompat targets   : ThinkPad X200/Trisquel, Raspberry Pi 5 Pi OS, ANSI consoles\nsafe behavior    : env detection only; no host commands are executed\napply            : theme linux apply | theme linux x200 | theme linux raspberry-pi | theme linux rpi5 | theme linux truecolor | theme linux 256 | theme linux ansi\n",
        if cfg!(target_os = "linux") {
            "linux"
        } else {
            "non-linux compatible"
        },
        detected_platform_label(),
        term,
        colorterm,
        detected.name(),
        configured_pack,
        configured_depth,
        if rpi_compat_enabled() { "on" } else { "off" },
    )
}

fn apply(shell: &mut Phase1Shell, depth: LinuxColorDepth) -> String {
    let pack_name = if rpi_compat_enabled() || platform_is_raspberry_pi() {
        "raspberry-pi"
    } else {
        "linux"
    };
    std::env::set_var("PHASE1_COLOR_PACK", pack_name);
    std::env::set_var("PHASE1_COLOR_DEPTH", depth.name());
    shell
        .env
        .insert("PHASE1_COLOR_PACK".to_string(), pack_name.to_string());
    shell
        .env
        .insert("PHASE1_COLOR_DEPTH".to_string(), depth.name().to_string());

    match depth {
        LinuxColorDepth::Mono => {
            std::env::set_var("PHASE1_NO_COLOR", "1");
            std::env::set_var("PHASE1_ASCII", "1");
            shell
                .env
                .insert("PHASE1_THEME".to_string(), "mono".to_string());
            shell
                .env
                .insert("PHASE1_NO_COLOR".to_string(), "1".to_string());
            shell
                .env
                .insert("PHASE1_ASCII".to_string(), "1".to_string());
        }
        _ => {
            std::env::remove_var("PHASE1_NO_COLOR");
            std::env::remove_var("PHASE1_ASCII");
            std::env::set_var("PHASE1_THEME", depth.palette_name());
            shell
                .env
                .insert("PHASE1_THEME".to_string(), depth.palette_name().to_string());
            shell
                .env
                .insert("PHASE1_NO_COLOR".to_string(), "0".to_string());
            shell
                .env
                .insert("PHASE1_ASCII".to_string(), "0".to_string());
        }
    }

    format!(
        "theme linux: {} color pack enabled using base palette {}\n{}",
        depth.name(),
        depth.palette_name(),
        preview(depth)
    )
}

fn clear(shell: &mut Phase1Shell) {
    std::env::remove_var("PHASE1_COLOR_PACK");
    std::env::remove_var("PHASE1_COLOR_DEPTH");
    shell.env.remove("PHASE1_COLOR_PACK");
    shell.env.remove("PHASE1_COLOR_DEPTH");
}

fn detect() -> LinuxColorDepth {
    if std::env::var_os("NO_COLOR").is_some()
        || std::env::var("PHASE1_NO_COLOR").ok().as_deref() == Some("1")
        || std::env::var("PHASE1_ASCII").ok().as_deref() == Some("1")
        || rpi_compat_enabled()
    {
        return LinuxColorDepth::Mono;
    }
    if let Ok(raw) = std::env::var("PHASE1_COLOR_DEPTH") {
        if let Some(depth) = LinuxColorDepth::parse(&raw) {
            return depth;
        }
    }
    if platform_is_raspberry_pi() {
        return raspberry_pi_default();
    }
    let colorterm = std::env::var("COLORTERM")
        .unwrap_or_default()
        .to_ascii_lowercase();
    if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return LinuxColorDepth::TrueColor;
    }
    let term = std::env::var("TERM")
        .unwrap_or_default()
        .to_ascii_lowercase();
    if term.contains("256color") {
        return LinuxColorDepth::Color256;
    }
    if term == "dumb" || term.is_empty() {
        LinuxColorDepth::Mono
    } else {
        LinuxColorDepth::Ansi16
    }
}

fn raspberry_pi_default() -> LinuxColorDepth {
    if rpi_compat_enabled() {
        return LinuxColorDepth::Mono;
    }
    let term = std::env::var("TERM")
        .unwrap_or_default()
        .to_ascii_lowercase();
    if term.contains("256color") {
        LinuxColorDepth::Color256
    } else if term == "dumb" || term.contains("vt100") || term.contains("linux") {
        LinuxColorDepth::Mono
    } else {
        LinuxColorDepth::Ansi16
    }
}

fn platform_hint() -> String {
    ["PHASE1_PLATFORM", "PHASE1_DEVICE_MODE", "HOSTNAME"]
        .iter()
        .filter_map(|key| std::env::var(key).ok())
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn platform_is_raspberry_pi() -> bool {
    let hint = platform_hint();
    hint.contains("raspberry") || hint.contains("raspi") || hint.contains("rpi")
}

fn rpi_compat_enabled() -> bool {
    matches!(
        std::env::var("PHASE1_RPI_COMPAT").ok().as_deref(),
        Some("1" | "true" | "yes" | "on")
    )
}

fn detected_platform_label() -> &'static str {
    let hint = platform_hint();
    if hint.contains("raspberry") || hint.contains("raspi") || hint.contains("rpi") {
        "raspberry-pi"
    } else if hint.contains("x200") || hint.contains("trisquel") {
        "x200/trisquel"
    } else {
        "generic"
    }
}

fn preview(depth: LinuxColorDepth) -> String {
    let mut out = format!("linux color preview // {}\n", depth.name());
    out.push_str("core  : ");
    out.push_str(&swatch(depth, "phase1", 0xff2bd6, 201, "magenta"));
    out.push(' ');
    out.push_str(&swatch(depth, "kernel", 0x38ff9c, 48, "green"));
    out.push(' ');
    out.push_str(&swatch(depth, "vfs", 0x29b6ff, 39, "cyan"));
    out.push(' ');
    out.push_str(&swatch(depth, "audit", 0xffcc33, 220, "yellow"));
    out.push('\n');
    out.push_str(
        "notes : RGB for truecolor, xterm indexes for 256color, ANSI fallback for older consoles\n",
    );
    out
}

fn esc(code: &str) -> String {
    format!("{}[{code}m", char::from(27))
}
fn reset() -> String {
    esc("0")
}

fn swatch(depth: LinuxColorDepth, label: &str, rgb: u32, xterm: u8, ansi: &str) -> String {
    match depth {
        LinuxColorDepth::TrueColor => {
            let r = (rgb >> 16) & 0xff;
            let g = (rgb >> 8) & 0xff;
            let b = rgb & 0xff;
            format!("{}{label}{}", esc(&format!("38;2;{r};{g};{b}")), reset())
        }
        LinuxColorDepth::Color256 => format!("{}{label}{}", esc(&format!("38;5;{xterm}")), reset()),
        LinuxColorDepth::Ansi16 => format!("{}{label}{}", ansi_code(ansi), reset()),
        LinuxColorDepth::Mono => label.to_string(),
    }
}

fn ansi_code(name: &str) -> String {
    esc(match name {
        "magenta" => "95",
        "green" => "92",
        "cyan" => "96",
        "yellow" => "93",
        _ => "97",
    })
}

fn value(shell: &Phase1Shell, key: &str) -> Option<String> {
    shell
        .env
        .get(key)
        .cloned()
        .or_else(|| std::env::var(key).ok())
}

#[cfg(test)]
mod tests {
    use super::{summary, theme};
    use crate::commands::Phase1Shell;

    #[test]
    fn linux_pack_applies_truecolor() {
        let mut shell = Phase1Shell::new();
        let out = theme(&mut shell, &["truecolor".to_string()]);
        assert!(out.contains("truecolor color pack enabled"));
        assert!(out.contains("38;2;"));
        assert_eq!(
            shell.env.get("PHASE1_COLOR_PACK").map(String::as_str),
            Some("linux")
        );
        assert_eq!(summary(&shell), "linux/truecolor");
        std::env::remove_var("PHASE1_COLOR_PACK");
        std::env::remove_var("PHASE1_COLOR_DEPTH");
    }

    #[test]
    fn linux_pack_supports_x200_and_pi_presets() {
        let mut shell = Phase1Shell::new();
        let x200 = theme(&mut shell, &["x200".to_string()]);
        assert!(x200.contains("ansi16 color pack enabled"));
        let pi = theme(&mut shell, &["raspberry-pi".to_string()]);
        assert!(pi.contains("color pack enabled"));
        std::env::remove_var("PHASE1_COLOR_PACK");
        std::env::remove_var("PHASE1_COLOR_DEPTH");
    }

    #[test]
    fn rpi5_safe_color_mode_forces_mono() {
        std::env::set_var("PHASE1_RPI_COMPAT", "1");
        let mut shell = Phase1Shell::new();
        let pi = theme(&mut shell, &["rpi5".to_string()]);
        assert!(pi.contains("mono color pack enabled"));
        assert_eq!(summary(&shell), "raspberry-pi/mono");
        std::env::remove_var("PHASE1_RPI_COMPAT");
        std::env::remove_var("PHASE1_COLOR_PACK");
        std::env::remove_var("PHASE1_COLOR_DEPTH");
        std::env::remove_var("PHASE1_NO_COLOR");
        std::env::remove_var("PHASE1_ASCII");
    }
}
