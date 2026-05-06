use crate::registry;
use std::fs;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const PANEL_WIDTH: usize = 64;
const MOBILE_WIDTH: usize = 40;
const BOOT_CONFIG_PATH: &str = "phase1.conf";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const GRAY: &str = "\x1b[90m";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootSelection {
    Boot(BootConfig),
    StorageTools(BootConfig),
    Quit,
    Reboot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootConfig {
    pub color: bool,
    pub ascii_mode: bool,
    pub safe_mode: bool,
    pub quick_boot: bool,
    pub mobile_mode: bool,
    pub persistent_state: bool,
    pub bleeding_edge: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThemePalette {
    NeoTokyo,
    Rainbow,
    Matrix,
    Cyber,
    Amber,
    Ice,
    Synth,
    Crimson,
    BleedingEdge,
}

impl ThemePalette {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "neo-tokyo" | "neotokyo" | "tokyo" | "neo" | "city" | "hacker" => {
                Some(Self::NeoTokyo)
            }
            "rainbow" | "default" | "classic" => Some(Self::Rainbow),
            "matrix" | "green" => Some(Self::Matrix),
            "cyber" | "cyan" | "cyan-magenta" | "neon" => Some(Self::Cyber),
            "amber" | "gold" | "terminal" => Some(Self::Amber),
            "ice" | "blue" | "frost" => Some(Self::Ice),
            "synth" | "synthwave" | "purple" => Some(Self::Synth),
            "crimson" | "red" | "alert" => Some(Self::Crimson),
            "bleeding" | "bleeding-edge" | "edge" => Some(Self::BleedingEdge),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::NeoTokyo => "neo-tokyo",
            Self::Rainbow => "rainbow",
            Self::Matrix => "matrix",
            Self::Cyber => "cyber",
            Self::Amber => "amber",
            Self::Ice => "ice",
            Self::Synth => "synthwave",
            Self::Crimson => "crimson",
            Self::BleedingEdge => "bleeding-edge",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::NeoTokyo => "cyan/magenta Neo Tokyo operator HUD, the default phase1 look",
            Self::Rainbow => "classic rainbow ANSI gradient",
            Self::Matrix => "green-on-black digital rain console",
            Self::Cyber => "cyan/magenta high-contrast operator console",
            Self::Amber => "warm amber retro terminal",
            Self::Ice => "cool blue/cyan frost terminal",
            Self::Synth => "purple synthwave operator glow",
            Self::Crimson => "red alert / intrusion-response console",
            Self::BleedingEdge => "edge-only blue/magenta update channel console",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::NeoTokyo,
            Self::Rainbow,
            Self::Matrix,
            Self::Cyber,
            Self::Amber,
            Self::Ice,
            Self::Synth,
            Self::Crimson,
            Self::BleedingEdge,
        ]
    }
}

struct Palette {
    border: &'static str,
    title: &'static str,
    accent: &'static str,
    muted: &'static str,
    prompt_user: &'static str,
    prompt_path: &'static str,
    ready: &'static str,
}

impl Default for BootConfig {
    fn default() -> Self {
        let mut config = Self::detected_defaults();
        if let Some(saved) = Self::load_saved() {
            config = saved;
        }
        config.apply_env_overrides();
        config.normalize_channel();
        config
    }
}

impl BootConfig {
    pub fn detected_defaults() -> Self {
        let mut config = Self {
            color: color_enabled(),
            ascii_mode: ascii_mode(),
            safe_mode: env_flag("PHASE1_SAFE_MODE").unwrap_or(true),
            quick_boot: env_flag("PHASE1_QUICK_BOOT").unwrap_or(false),
            mobile_mode: env_flag("PHASE1_MOBILE_MODE").unwrap_or(false) || detect_mobile(),
            persistent_state: env_flag("PHASE1_PERSISTENT_STATE").unwrap_or(false),
            bleeding_edge: env_flag("PHASE1_BLEEDING_EDGE").unwrap_or(false),
        };
        config.normalize_channel();
        config
    }

    pub fn apply(self) {
        set_bool_env("PHASE1_ASCII", self.ascii_mode);
        set_bool_env("PHASE1_QUICK_BOOT", self.quick_boot);
        set_bool_env("PHASE1_MOBILE_MODE", self.mobile_mode);
        set_bool_env("PHASE1_PERSISTENT_STATE", self.persistent_state);
        set_bool_env("PHASE1_BLEEDING_EDGE", self.bleeding_edge);
        std::env::set_var(
            "PHASE1_DISPLAY_VERSION",
            display_version(crate::kernel::VERSION, self),
        );
        std::env::set_var("PHASE1_SAFE_MODE", if self.safe_mode { "1" } else { "0" });
        if self.color {
            std::env::remove_var("PHASE1_NO_COLOR");
        } else {
            std::env::set_var("PHASE1_NO_COLOR", "1");
        }
        if self.bleeding_edge && std::env::var("PHASE1_THEME").is_err() {
            std::env::set_var("PHASE1_THEME", ThemePalette::BleedingEdge.name());
        }
    }

    pub fn save(self) -> io::Result<()> {
        fs::write(config_path(), self.to_config_string())
    }

    pub fn remove_saved() -> io::Result<()> {
        match fs::remove_file(config_path()) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn profile_name(self) -> String {
        let base = match (self.mobile_mode, self.safe_mode, self.quick_boot) {
            (true, true, true) => "mobile-safe+quick",
            (true, true, false) => "mobile-safe",
            (true, false, true) => "mobile-quick",
            (true, false, false) => "mobile",
            (false, true, true) => "safe+quick",
            (false, true, false) => "safe",
            (false, false, true) => "quick",
            (false, false, false) => "operator",
        };
        if self.bleeding_edge {
            format!("{base}+edge")
        } else {
            base.to_string()
        }
    }

    fn load_saved() -> Option<Self> {
        let raw = fs::read_to_string(config_path()).ok()?;
        let mut config = Self::detected_defaults();
        for line in raw.lines().map(str::trim) {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let Some(value) = parse_bool(value.trim()) else {
                continue;
            };
            match key.trim() {
                "color" => config.color = value,
                "ascii" | "ascii_mode" => config.ascii_mode = value,
                "safe" | "safe_mode" => config.safe_mode = value,
                "quick" | "quick_boot" => config.quick_boot = value,
                "mobile" | "mobile_mode" => config.mobile_mode = value,
                "persistent" | "persist" | "persistent_state" => config.persistent_state = value,
                "bleeding" | "bleeding_edge" | "edge" => config.bleeding_edge = value,
                _ => {}
            }
        }
        config.normalize_channel();
        Some(config)
    }

    fn apply_env_overrides(&mut self) {
        if let Some(value) = env_flag("PHASE1_ASCII") {
            self.ascii_mode = value;
        }
        if let Some(value) = env_flag("PHASE1_NO_COLOR") {
            self.color = !value;
        }
        if let Some(value) = env_flag("NO_COLOR") {
            self.color = !value;
        }
        if let Some(value) = env_flag("PHASE1_SAFE_MODE") {
            self.safe_mode = value;
        }
        if let Some(value) = env_flag("PHASE1_QUICK_BOOT") {
            self.quick_boot = value;
        }
        if let Some(value) = env_flag("PHASE1_MOBILE_MODE") {
            self.mobile_mode = value;
        }
        if let Some(value) = env_flag("PHASE1_PERSISTENT_STATE") {
            self.persistent_state = value;
        }
        if let Some(value) = env_flag("PHASE1_BLEEDING_EDGE") {
            self.bleeding_edge = value;
        }
        self.normalize_channel();
    }

    fn normalize_channel(&mut self) {
        if self.bleeding_edge {
            self.color = true;
            self.ascii_mode = false;
        }
    }

    fn to_config_string(self) -> String {
        format!(
            "# phase1 boot configuration\n# safe=true is the secure default; set safe=false only when intentionally testing host-backed tools.\ncolor={}\nascii={}\nsafe={}\nquick={}\nmobile={}\npersistent={}\nbleeding_edge={}\n",
            self.color,
            self.ascii_mode,
            self.safe_mode,
            self.quick_boot,
            self.mobile_mode,
            self.persistent_state,
            self.bleeding_edge
        )
    }
}

pub fn config_path() -> &'static str {
    BOOT_CONFIG_PATH
}

pub fn display_version(release_version: &str, config: BootConfig) -> String {
    if config.bleeding_edge {
        crate::updater::CURRENT_EDGE_VERSION.to_string()
    } else {
        release_version.to_string()
    }
}

pub fn configure_boot(version: &str) -> BootSelection {
    let mut config = BootConfig::default();
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print_preboot(version, config);
        print!("{}", command_prompt(config, "boot"));
        let _ = io::stdout().flush();

        input.clear();
        match stdin.read_line(&mut input) {
            Ok(0) | Err(_) => return BootSelection::Quit,
            Ok(_) => {}
        }

        match input.trim().to_ascii_lowercase().as_str() {
            "" | "1" | "b" | "boot" | "start" | "jack-in" => {
                if let Err(err) = config.save() {
                    eprintln!("boot config save warning: {err}");
                }
                return BootSelection::Boot(config);
            }
            "2" | "c" | "color" | "colour" | "neon" => config.color = !config.color,
            "3" | "a" | "ascii" => config.ascii_mode = !config.ascii_mode,
            "4" | "s" | "safe" | "safe-mode" | "shield" => config.safe_mode = !config.safe_mode,
            "5" | "q" | "quick" | "quick-boot" => config.quick_boot = !config.quick_boot,
            "6" | "m" | "mobile" | "mobile-mode" => config.mobile_mode = !config.mobile_mode,
            "p" | "persist" | "persistent" | "persistent-state" | "vault" => {
                config.persistent_state = !config.persistent_state;
            }
            "e" | "edge" | "bleeding" | "bleeding-edge" => {
                config.bleeding_edge = !config.bleeding_edge;
                config.normalize_channel();
            }
            "d" | "dev" | "storage" | "storage-tools" | "workspace" => {
                if let Err(err) = config.save() {
                    eprintln!("boot config save warning: {err}");
                }
                return BootSelection::StorageTools(config);
            }
            "7" | "reboot" | "restart" => return BootSelection::Reboot,
            "8" | "x" | "quit" | "exit" | "shutdown" => return BootSelection::Quit,
            "9" | "save" | "write" => match config.save() {
                Ok(()) => pause("Saved boot configuration to phase1.conf."),
                Err(err) => pause(&format!("Could not save phase1.conf: {err}")),
            },
            "0" | "r" | "reset" => {
                config = BootConfig::detected_defaults();
                match BootConfig::remove_saved() {
                    Ok(()) => pause("Reset to secure detected defaults and removed phase1.conf."),
                    Err(err) => pause(&format!("Reset defaults, but could not remove phase1.conf: {err}")),
                }
            }
            "h" | "help" | "?" => pause("HUD dock: Enter boots, e=edge channel, p=vault persistence, d=storage/Rust/Git helper, 9=saves, 0=resets, 8=quits. Safe mode stays on unless you intentionally drop the shield."),
            _ => pause("Unknown boot option. Press Enter to re-open the Phase1 boot dock."),
        }
    }
}

pub fn print_boot(version: &str) {
    let config = BootConfig::default();
    if mobile_mode_enabled() || terminal_width() < 72 {
        print_mobile_boot(version, config);
    } else if ascii_mode() {
        print_ascii_boot(version, config);
    } else {
        print_modern_boot(version, config);
    }
}

pub fn print_quick_boot(version: &str, config: BootConfig) {
    print_boot_card(version, config, false);
    println!(
        "{}",
        value(
            config,
            &format!(
                "[quick] profile={} :: shell armed :: clock {}",
                config.profile_name(),
                clock_utc()
            )
        )
    );
    println!();
}

pub fn print_help() {
    print!("{}", registry::command_map());
}

pub fn print_prompt(user: &str, path: &str) {
    print!("{}", prompt_status_bar(user, path));
    print!("{}", prompt_text(user, path));
}

fn print_preboot(version: &str, config: BootConfig) {
    if config.ascii_mode || !config.color {
        print!("\x1b[2J\x1b[H");
    } else {
        print!("\x1b[2J\x1b[H{BOLD}");
    }
    print_boot_card(version, config, true);
    println!("{}", value(config, "Enter=boot  e=edge  p=vault  d=storage/Git/Rust  h=help"));
}

fn print_mobile_boot(version: &str, config: BootConfig) {
    println!("\x1b[2J\x1b[H");
    print_boot_card(version, config, false);
    ready_line(false);
}

fn print_modern_boot(version: &str, config: BootConfig) {
    println!("\x1b[2J\x1b[H");
    print_boot_card(version, config, false);
    ready_line(true);
}

fn print_ascii_boot(version: &str, mut config: BootConfig) {
    config.color = false;
    config.ascii_mode = true;
    print_boot_card(version, config, false);
    ready_line(true);
}

fn print_boot_card(version: &str, config: BootConfig, selector: bool) {
    let width = card_width(config);
    println!();
    println!("{}", card_top(config, width));
    println!("{}", card_line(config, width, &console_title(config)));
    println!("{}", card_line(config, width, &format!("node TOKYO-01 // vOS link // clock {}", clock_utc())));
    println!("{}", card_rule(config, width));

    for row in skyline_rows(config) {
        println!("{}", card_line(config, width, &row));
    }

    println!("{}", card_section(config, width, "STATUS"));
    for row in splash_info(version, config) {
        println!("{}", card_line(config, width, &row));
    }

    if selector {
        println!("{}", card_section(config, width, "BOOT DOCK"));
        for row in boot_rows(config) {
            println!("{}", card_line(config, width, &row));
        }
    } else {
        println!("{}", card_section(config, width, "LIVE OPS"));
        println!("{}", card_line(config, width, "help  dash  sysinfo  security  theme list"));
        println!("{}", card_line(config, width, "matrix  audit  ps  ls /  storage via d"));
    }

    println!("{}", card_bottom(config, width));
    println!();
}

fn skyline_rows(config: BootConfig) -> Vec<String> {
    let mode = if config.safe_mode { "shielded" } else { "host-capable" };
    vec![
        tint(config, "neural link locked :: kernel PHASE1"),
        format!("mesh encrypted :: mode {mode}"),
        "bus kernel vfs proc net audit lang git rust".to_string(),
    ]
}

fn splash_info(version: &str, config: BootConfig) -> Vec<String> {
    let state_mode = if config.persistent_state { "vault/persistent" } else { "ram/volatile" };
    let security_mode = if config.safe_mode { "safe shield" } else { "host bridge" };
    let channel = if config.bleeding_edge { "bleeding-edge" } else { "release" };
    let workspace = std::env::var("PHASE1_STORAGE_ROOT").unwrap_or_else(|_| "phase1.workspace".to_string());
    vec![
        status_row(config, "version", &format!("v{}", display_version(version, config)), true),
        status_row(config, "clock", &clock_utc(), true),
        status_row(config, "channel", channel, config.bleeding_edge),
        status_row(config, "profile", &config.profile_name(), true),
        status_row(config, "security", security_mode, config.safe_mode),
        status_row(config, "display", display_mode(config), true),
        status_row(config, "state", state_mode, config.persistent_state),
        status_row(config, "workspace", &workspace, true),
    ]
}

fn boot_rows(config: BootConfig) -> Vec<String> {
    vec![
        option_row(config, "[1] BOOT", "start shell", true),
        option_row(config, "[2] NEON", on_off(config.color), config.color),
        option_row(config, "[3] ASCII", on_off(config.ascii_mode), config.ascii_mode),
        option_row(config, "[4] SHIELD", on_off(config.safe_mode), config.safe_mode),
        option_row(config, "[5] QUICK", on_off(config.quick_boot), config.quick_boot),
        option_row(config, "[6] MOBILE", on_off(config.mobile_mode), config.mobile_mode),
        option_row(config, "[e] EDGE", on_off(config.bleeding_edge), config.bleeding_edge),
        option_row(config, "[p] VAULT", on_off(config.persistent_state), config.persistent_state),
        option_row(config, "[d] STORAGE+GIT+RUST", "open dock", true),
        option_row(config, "[7] REBOOT", "selector", true),
        option_row(config, "[8] SHUTDOWN", "abort", true),
        option_row(config, "[9] SAVE", "phase1.conf", true),
        option_row(config, "[0] RESET", "defaults", true),
    ]
}

fn on_off(enabled: bool) -> &'static str {
    if enabled { "ON" } else { "off" }
}

fn option_row(config: BootConfig, label: &str, value_text: &str, bright: bool) -> String {
    let value_text = if bright { tint(config, value_text) } else { dim(config, value_text) };
    format!("{label:<23} {value_text}")
}

fn status_row(config: BootConfig, label: &str, value_text: &str, bright: bool) -> String {
    let value_text = if bright { tint(config, value_text) } else { dim(config, value_text) };
    format!("{label:<10} {value_text}")
}

fn console_title(config: BootConfig) -> String {
    if !config.color || config.ascii_mode {
        format!("{} // Advanced Operator Console", phase1_wordmark(config))
    } else {
        let colors = palette(active_theme_for_config(config));
        format!(
            "{}{} // Advanced Operator Console{}",
            phase1_wordmark(config),
            colors.title,
            RESET
        )
    }
}

fn phase1_wordmark(config: BootConfig) -> String {
    if !config.color || config.ascii_mode {
        "Phase1".to_string()
    } else {
        let theme = active_theme_for_config(config);
        if theme == ThemePalette::Rainbow {
            let colors = [RED, YELLOW, GREEN, CYAN, BLUE, MAGENTA];
            "Phase1"
                .chars()
                .enumerate()
                .map(|(idx, ch)| format!("{}{}{}", colors[idx % colors.len()], ch, RESET))
                .collect::<Vec<_>>()
                .join("")
        } else if theme == ThemePalette::NeoTokyo {
            let colors = [CYAN, MAGENTA, BLUE, CYAN, MAGENTA, GREEN];
            "Phase1"
                .chars()
                .enumerate()
                .map(|(idx, ch)| format!("{}{}{}{}", BOLD, colors[idx % colors.len()], ch, RESET))
                .collect::<Vec<_>>()
                .join("")
        } else {
            let colors = palette(theme);
            format!("{}Phase1{}", colors.title, RESET)
        }
    }
}

fn prompt_text(user: &str, path: &str) -> String {
    if ascii_mode() || !color_enabled() {
        format!("phase1://{} {} > ", user, path)
    } else {
        let colors = palette(active_theme());
        format!(
            "{}{}phase1{}{}://{}{}{}{} {}{}{} ⇢ ",
            BOLD,
            colors.title,
            RESET,
            GRAY,
            RESET,
            colors.prompt_user,
            user,
            RESET,
            colors.prompt_path,
            path,
            RESET
        )
    }
}

fn prompt_status_bar(user: &str, path: &str) -> String {
    let width = card_width(BootConfig::default());
    let version = std::env::var("PHASE1_DISPLAY_VERSION").unwrap_or_else(|_| crate::kernel::VERSION.to_string());
    let channel = std::env::var("PHASE1_CHANNEL").unwrap_or_else(|_| {
        if bleeding_edge_env_enabled() {
            "edge".to_string()
        } else {
            "release".to_string()
        }
    });
    let safe = if std::env::var("PHASE1_SAFE_MODE").ok().as_deref() == Some("0") {
        "host"
    } else {
        "safe"
    };
    let state = if std::env::var("PHASE1_PERSISTENT_STATE").ok().as_deref() == Some("1") {
        "vault"
    } else {
        "ram"
    };
    let raw = format!("HUD {version} | {channel} | {safe} | {state} | {user}@{path} | {}", clock_utc());
    let clipped = clip_visible(&raw, width);
    let visible = visible_len(&clipped);
    let padded = format!("{clipped}{}", " ".repeat(width.saturating_sub(visible)));

    if color_enabled() && !ascii_mode() {
        let colors = palette(active_theme());
        format!("{}{}{}\n", colors.accent, padded, RESET)
    } else {
        format!("{padded}\n")
    }
}

fn command_prompt(config: BootConfig, label: &str) -> String {
    if config.color && !config.ascii_mode {
        let colors = palette(active_theme_for_config(config));
        format!(
            "{}{}phase1{}{}://{}{} {}❯{} ",
            BOLD, colors.title, RESET, GRAY, RESET, label, colors.accent, RESET
        )
    } else {
        format!("{label}> ")
    }
}

fn ready_line(desktop: bool) {
    if color_enabled() {
        let colors = palette(active_theme());
        if desktop {
            println!(
                "{}[ready]{} systems synced {GRAY}:: operator shell armed :: HUD clock active{RESET}",
                colors.ready, RESET
            );
        } else {
            println!("{}[ready]{} systems synced", colors.ready, RESET);
        }
    } else if desktop {
        println!("[ready] systems synced :: operator shell armed :: HUD clock active");
    } else {
        println!("[ready] systems synced");
    }
    println!();
}

fn display_mode(config: BootConfig) -> &'static str {
    if config.ascii_mode {
        "ascii"
    } else if !config.color {
        "mono"
    } else {
        active_theme_for_config(config).name()
    }
}

fn active_theme() -> ThemePalette {
    std::env::var("PHASE1_THEME")
        .ok()
        .and_then(|raw| ThemePalette::parse(&raw))
        .filter(|theme| *theme != ThemePalette::BleedingEdge || bleeding_edge_env_enabled())
        .unwrap_or_else(|| {
            if bleeding_edge_env_enabled() {
                ThemePalette::BleedingEdge
            } else {
                ThemePalette::NeoTokyo
            }
        })
}

fn active_theme_for_config(config: BootConfig) -> ThemePalette {
    std::env::var("PHASE1_THEME")
        .ok()
        .and_then(|raw| ThemePalette::parse(&raw))
        .filter(|theme| *theme != ThemePalette::BleedingEdge || config.bleeding_edge)
        .unwrap_or_else(|| {
            if config.bleeding_edge {
                ThemePalette::BleedingEdge
            } else {
                ThemePalette::NeoTokyo
            }
        })
}

fn palette(theme: ThemePalette) -> Palette {
    match theme {
        ThemePalette::NeoTokyo => Palette {
            border: BLUE,
            title: MAGENTA,
            accent: GREEN,
            muted: GRAY,
            prompt_user: MAGENTA,
            prompt_path: GREEN,
            ready: GREEN,
        },
        ThemePalette::Rainbow => Palette {
            border: CYAN,
            title: GREEN,
            accent: CYAN,
            muted: GRAY,
            prompt_user: CYAN,
            prompt_path: BLUE,
            ready: GREEN,
        },
        ThemePalette::Matrix => Palette {
            border: GREEN,
            title: GREEN,
            accent: GREEN,
            muted: GRAY,
            prompt_user: GREEN,
            prompt_path: GREEN,
            ready: GREEN,
        },
        ThemePalette::Cyber => Palette {
            border: MAGENTA,
            title: CYAN,
            accent: MAGENTA,
            muted: GRAY,
            prompt_user: MAGENTA,
            prompt_path: CYAN,
            ready: CYAN,
        },
        ThemePalette::Amber => Palette {
            border: YELLOW,
            title: YELLOW,
            accent: YELLOW,
            muted: GRAY,
            prompt_user: YELLOW,
            prompt_path: YELLOW,
            ready: YELLOW,
        },
        ThemePalette::Ice => Palette {
            border: CYAN,
            title: BLUE,
            accent: CYAN,
            muted: GRAY,
            prompt_user: CYAN,
            prompt_path: BLUE,
            ready: CYAN,
        },
        ThemePalette::Synth => Palette {
            border: MAGENTA,
            title: MAGENTA,
            accent: CYAN,
            muted: GRAY,
            prompt_user: CYAN,
            prompt_path: MAGENTA,
            ready: MAGENTA,
        },
        ThemePalette::Crimson => Palette {
            border: RED,
            title: RED,
            accent: YELLOW,
            muted: GRAY,
            prompt_user: RED,
            prompt_path: YELLOW,
            ready: RED,
        },
        ThemePalette::BleedingEdge => Palette {
            border: BLUE,
            title: MAGENTA,
            accent: GREEN,
            muted: GRAY,
            prompt_user: MAGENTA,
            prompt_path: GREEN,
            ready: MAGENTA,
        },
    }
}

fn card_width(config: BootConfig) -> usize {
    let max = if config.mobile_mode || detect_mobile() {
        MOBILE_WIDTH
    } else {
        PANEL_WIDTH
    };
    terminal_width().clamp(32, max)
}

fn card_top(config: BootConfig, width: usize) -> String {
    if config.color && !config.ascii_mode {
        format!(
            "{}╭{}╮{RESET}",
            palette(active_theme_for_config(config)).border,
            "─".repeat(width)
        )
    } else {
        format!("+{}+", "-".repeat(width))
    }
}

fn card_bottom(config: BootConfig, width: usize) -> String {
    if config.color && !config.ascii_mode {
        format!(
            "{}╰{}╯{RESET}",
            palette(active_theme_for_config(config)).border,
            "─".repeat(width)
        )
    } else {
        format!("+{}+", "-".repeat(width))
    }
}

fn card_rule(config: BootConfig, width: usize) -> String {
    if config.color && !config.ascii_mode {
        format!(
            "{}├{}┤{RESET}",
            palette(active_theme_for_config(config)).border,
            "─".repeat(width)
        )
    } else {
        format!("+{}+", "-".repeat(width))
    }
}

fn card_section(config: BootConfig, width: usize, label: &str) -> String {
    let marker = format!(" {label} ");
    let fill = width.saturating_sub(marker.chars().count());
    if config.color && !config.ascii_mode {
        let colors = palette(active_theme_for_config(config));
        format!(
            "{}├{}{}{}┤{RESET}",
            colors.border,
            colors.accent,
            marker,
            "─".repeat(fill)
        )
    } else {
        format!("+{marker}{}+", "-".repeat(fill))
    }
}

fn card_line(config: BootConfig, width: usize, text: &str) -> String {
    let clipped = clip_visible(text, width);
    let visible = visible_len(&clipped);
    let padded = format!("{clipped}{}", " ".repeat(width.saturating_sub(visible)));
    if config.color && !config.ascii_mode {
        let border = palette(active_theme_for_config(config)).border;
        format!("{border}│{RESET}{padded}{border}│{RESET}")
    } else {
        format!("|{padded}|")
    }
}

fn value(config: BootConfig, text: &str) -> String {
    if config.color && !config.ascii_mode {
        format!(
            "{}{text}{RESET}",
            palette(active_theme_for_config(config)).muted
        )
    } else {
        text.to_string()
    }
}

fn tint(config: BootConfig, text: &str) -> String {
    if config.color && !config.ascii_mode {
        format!(
            "{}{}{}",
            palette(active_theme_for_config(config)).accent,
            text,
            RESET
        )
    } else {
        text.to_string()
    }
}

fn dim(config: BootConfig, text: &str) -> String {
    if config.color && !config.ascii_mode {
        format!(
            "{}{DIM}{text}{RESET}",
            palette(active_theme_for_config(config)).muted
        )
    } else {
        text.to_string()
    }
}

fn pause(message: &str) {
    println!("{message}");
    let _ = io::stdout().flush();
    let mut ignored = String::new();
    let _ = io::stdin().read_line(&mut ignored);
}

fn set_bool_env(name: &str, enabled: bool) {
    if enabled {
        std::env::set_var(name, "1");
    } else {
        std::env::remove_var(name);
    }
}

fn terminal_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(MOBILE_WIDTH)
}

fn detect_mobile() -> bool {
    if terminal_width() < 72 {
        return true;
    }
    let indicators = ["IPHONE", "ANDROID", "BLINK", "ISH", "TERMUX", "MOBILE"];
    ["TERM_PROGRAM", "TERM", "SSH_CLIENT", "PHASE1_DEVICE"]
        .iter()
        .filter_map(|name| std::env::var(name).ok())
        .any(|value| {
            let upper = value.to_ascii_uppercase();
            indicators.iter().any(|needle| upper.contains(needle))
        })
}

fn mobile_mode_enabled() -> bool {
    std::env::var("PHASE1_MOBILE_MODE").ok().as_deref() == Some("1") || detect_mobile()
}

fn bleeding_edge_env_enabled() -> bool {
    std::env::var("PHASE1_BLEEDING_EDGE").ok().as_deref() == Some("1")
}

fn env_flag(name: &str) -> Option<bool> {
    std::env::var(name)
        .ok()
        .and_then(|value| parse_bool(&value))
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn clock_utc() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        % 86_400;
    let hours = seconds / 3_600;
    let minutes = (seconds % 3_600) / 60;
    let seconds = seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02} UTC")
}

fn strip_ansi(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            for code in chars.by_ref() {
                if code.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn visible_len(text: &str) -> usize {
    strip_ansi(text).chars().count()
}

fn clip_visible(text: &str, width: usize) -> String {
    let plain = strip_ansi(text);
    if plain.chars().count() <= width {
        text.to_string()
    } else {
        plain.chars().take(width).collect()
    }
}

fn color_enabled() -> bool {
    std::env::var_os("NO_COLOR").is_none()
        && std::env::var("PHASE1_NO_COLOR").ok().as_deref() != Some("1")
}

fn ascii_mode() -> bool {
    std::env::var("PHASE1_ASCII").ok().as_deref() == Some("1")
}

#[cfg(test)]
mod tests {
    use super::{
        clock_utc, console_title, display_mode, display_version, parse_bool, prompt_status_bar,
        strip_ansi, visible_len, BootConfig, ThemePalette, PANEL_WIDTH,
    };

    fn config() -> BootConfig {
        BootConfig {
            color: true,
            ascii_mode: false,
            safe_mode: true,
            quick_boot: false,
            mobile_mode: false,
            persistent_state: false,
            bleeding_edge: false,
        }
    }

    #[test]
    fn neo_tokyo_is_default_theme() {
        std::env::remove_var("PHASE1_THEME");
        std::env::remove_var("PHASE1_BLEEDING_EDGE");
        assert_eq!(display_mode(config()), "neo-tokyo");
    }

    #[test]
    fn console_title_keeps_advanced_operator_branding() {
        std::env::remove_var("PHASE1_THEME");
        std::env::remove_var("PHASE1_BLEEDING_EDGE");
        assert_eq!(
            strip_ansi(&console_title(config())),
            "Phase1 // Advanced Operator Console"
        );
    }

    #[test]
    fn prompt_status_bar_contains_clock_without_overflowing() {
        std::env::remove_var("PHASE1_THEME");
        std::env::set_var("COLUMNS", "80");
        let bar = prompt_status_bar("root", "~");
        let plain = strip_ansi(&bar);
        assert!(plain.contains("HUD"));
        assert!(plain.contains("UTC"));
        assert!(plain.lines().all(|line| visible_len(line) <= PANEL_WIDTH));
        std::env::remove_var("COLUMNS");
    }

    #[test]
    fn clock_uses_utc_suffix() {
        assert!(clock_utc().ends_with(" UTC"));
    }

    #[test]
    fn theme_parser_accepts_neo_tokyo_aliases() {
        assert_eq!(ThemePalette::parse("neo-tokyo"), Some(ThemePalette::NeoTokyo));
        assert_eq!(ThemePalette::parse("tokyo"), Some(ThemePalette::NeoTokyo));
        assert_eq!(ThemePalette::parse("matrix"), Some(ThemePalette::Matrix));
    }

    #[test]
    fn display_version_uses_edge_channel_when_requested() {
        let mut edge = config();
        edge.bleeding_edge = true;
        assert_ne!(display_version("3.6.0", edge), "3.6.0");
    }

    #[test]
    fn bool_parser_handles_common_values() {
        assert_eq!(parse_bool("on"), Some(true));
        assert_eq!(parse_bool("0"), Some(false));
        assert_eq!(parse_bool("maybe"), None);
    }

    #[test]
    fn ansi_stripper_removes_escape_codes() {
        assert_eq!(strip_ansi("\x1b[36mPhase1\x1b[0m"), "Phase1");
    }
}
