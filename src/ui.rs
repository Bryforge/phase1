use crate::registry;
use std::fs;
use std::io::{self, Write};

const PANEL_WIDTH: usize = 62;
const MOBILE_WIDTH: usize = 44;
const BOOT_CONFIG_PATH: &str = "phase1.conf";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
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
            "rainbow" | "neon" | "default" => Some(Self::Rainbow),
            "matrix" | "green" => Some(Self::Matrix),
            "cyber" | "cyan" | "cyan-magenta" => Some(Self::Cyber),
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
            Self::Rainbow => "rainbow ANSI gradient, the default phase1 look",
            Self::Matrix => "green-on-black digital rain console",
            Self::Cyber => "cyan/magenta high-contrast cyberdeck",
            Self::Amber => "warm amber retro terminal",
            Self::Ice => "cool blue/cyan frost terminal",
            Self::Synth => "purple synthwave operator glow",
            Self::Crimson => "red alert / intrusion-response console",
            Self::BleedingEdge => "edge-only blue/magenta update channel console",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
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
        print!("boot> ");
        let _ = io::stdout().flush();

        input.clear();
        match stdin.read_line(&mut input) {
            Ok(0) | Err(_) => return BootSelection::Quit,
            Ok(_) => {}
        }

        match input.trim().to_ascii_lowercase().as_str() {
            "" | "1" | "b" | "boot" | "start" => {
                if let Err(err) = config.save() {
                    eprintln!("boot config save warning: {err}");
                }
                return BootSelection::Boot(config);
            }
            "2" | "c" | "color" | "colour" => config.color = !config.color,
            "3" | "a" | "ascii" => config.ascii_mode = !config.ascii_mode,
            "4" | "s" | "safe" | "safe-mode" => config.safe_mode = !config.safe_mode,
            "5" | "q" | "quick" | "quick-boot" => config.quick_boot = !config.quick_boot,
            "6" | "m" | "mobile" | "mobile-mode" => config.mobile_mode = !config.mobile_mode,
            "p" | "persist" | "persistent" | "persistent-state" => {
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
            "h" | "help" | "?" => pause("Secure default: safe mode is on. Toggle options, d opens storage helper status, e toggles bleeding edge UI, p toggles persistent state, 9 saves, 0 resets saved config, 1 boots, 7 reboots, 8 quits."),
            _ => pause("Unknown boot option. Press Enter to continue."),
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
    println!("[quick] profile={} :: shell armed", config.profile_name());
    println!();
}

pub fn print_help() {
    print!("{}", registry::command_map());
}

pub fn print_prompt(user: &str, path: &str) {
    print!("{}", prompt_text(user, path));
}

fn print_preboot(version: &str, config: BootConfig) {
    if config.ascii_mode || !config.color {
        print!("\x1b[2J\x1b[H");
    } else {
        print!("\x1b[2J\x1b[H{BOLD}");
    }
    print_boot_card(version, config, true);
    println!(
        "{}",
        value(
            config,
            "Secure default: safe=on  Enter=boot  e=edge  p=persist  h=help"
        )
    );
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
    println!("{}", card_rule(config, width));
    for row in splash_info(version, config) {
        println!("{}", card_line(config, width, &row));
    }
    println!("{}", card_line(config, width, "cyberdeck ready"));

    if selector {
        println!("{}", card_section(config, width, "BOOT"));
        for row in boot_rows(config) {
            println!("{}", card_line(config, width, &row));
        }
    } else {
        println!("{}", card_section(config, width, "QUICK"));
        println!("{}", card_line(config, width, "help  audit  ps  ls /"));
        println!(
            "{}",
            card_line(config, width, "matrix  sysinfo  security  theme")
        );
    }
    println!("{}", card_bottom(config, width));
    println!();
}

fn splash_info(version: &str, config: BootConfig) -> Vec<String> {
    let state_mode = if config.persistent_state {
        "persistent"
    } else {
        "volatile"
    };
    let security_mode = if config.safe_mode {
        "safe"
    } else {
        "host-enabled"
    };
    let channel = if config.bleeding_edge {
        "bleeding-edge"
    } else {
        "release"
    };
    vec![
        format!("version v{}", display_version(version, config)),
        format!("channel {channel}"),
        format!("profile {}", config.profile_name()),
        format!("security  {security_mode}"),
        format!(
            "device  {}",
            if config.mobile_mode {
                "mobile"
            } else {
                "desktop"
            }
        ),
        format!("display {}", display_mode(config)),
        format!("state   {state_mode}"),
        format!("config  {}", config_path()),
    ]
}

fn boot_rows(config: BootConfig) -> Vec<String> {
    vec![
        "1 boot system       save+start".to_string(),
        format!(
            "2 color output      {}",
            if config.color { "on" } else { "off" }
        ),
        format!(
            "3 ascii compatible  {}",
            if config.ascii_mode { "on" } else { "off" }
        ),
        format!(
            "4 safe mode         {}",
            if config.safe_mode { "on" } else { "off" }
        ),
        format!(
            "5 quick boot        {}",
            if config.quick_boot { "on" } else { "off" }
        ),
        format!(
            "6 mobile mode       {}",
            if config.mobile_mode { "on" } else { "off" }
        ),
        format!(
            "e bleeding edge     {}",
            if config.bleeding_edge { "on" } else { "off" }
        ),
        format!(
            "p persistent state  {}",
            if config.persistent_state { "on" } else { "off" }
        ),
        "d storage helper    status".to_string(),
        "7 reboot selector".to_string(),
        "8 quit boot".to_string(),
        "9 save config".to_string(),
        "0 reset saved config".to_string(),
    ]
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
    } else if active_theme_for_config(config) == ThemePalette::Rainbow {
        let colors = [RED, YELLOW, GREEN, CYAN, BLUE, MAGENTA];
        "Phase1"
            .chars()
            .enumerate()
            .map(|(idx, ch)| format!("{}{}{}", colors[idx % colors.len()], ch, RESET))
            .collect::<Vec<_>>()
            .join("")
    } else {
        let colors = palette(active_theme_for_config(config));
        format!("{}Phase1{}", colors.title, RESET)
    }
}

fn prompt_text(user: &str, path: &str) -> String {
    if ascii_mode() || !color_enabled() {
        format!("phase1://{} {} > ", user, path)
    } else {
        let colors = palette(active_theme());
        format!(
            "{}{}phase1{}{}://{}{}{}{} {}{}{} ❯ ",
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

fn ready_line(desktop: bool) {
    if color_enabled() {
        let colors = palette(active_theme());
        if desktop {
            println!(
                "{}[ready]{} all subsystems nominal {GRAY}:: operator shell armed{RESET}",
                colors.ready, RESET
            );
        } else {
            println!("{}[ready]{} all subsystems nominal", colors.ready, RESET);
        }
    } else if desktop {
        println!("[ready] all subsystems nominal :: operator shell armed");
    } else {
        println!("[ready] all subsystems nominal");
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
                ThemePalette::Rainbow
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
                ThemePalette::Rainbow
            }
        })
}

fn palette(theme: ThemePalette) -> Palette {
    match theme {
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
            accent: CYAN,
            muted: GRAY,
            prompt_user: MAGENTA,
            prompt_path: CYAN,
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
fn clip(text: &str, width: usize) -> String {
    text.chars().take(width).collect()
}

#[cfg(test)]
fn phase1_art() -> [&'static str; 5] {
    [
        " ____  _                     __ ",
        "|  _ \\| |__   __ _ ___  ___ /_ |",
        "| |_) | '_ \\ / _` / __|/ _ \\ | |",
        "|  __/| | | | (_| \\__ \\  __/ | |",
        "|_|   |_| |_|\\__,_|___/\\___| |_|",
    ]
}

#[cfg(test)]
mod tests {
    use super::{
        clip, console_title, display_mode, display_version, parse_bool, phase1_art, prompt_text,
        splash_info, strip_ansi, visible_len, BootConfig, ThemePalette, PANEL_WIDTH,
    };

    fn test_config() -> BootConfig {
        BootConfig {
            color: true,
            ascii_mode: false,
            safe_mode: true,
            quick_boot: false,
            mobile_mode: true,
            persistent_state: false,
            bleeding_edge: false,
        }
    }

    #[test]
    fn panel_width_stays_terminal_friendly() {
        assert!(PANEL_WIDTH <= 72);
    }

    #[test]
    fn clip_respects_character_count() {
        assert_eq!(clip("abcdef", 3), "abc");
    }

    #[test]
    fn prompt_text_includes_user_and_path() {
        let prompt = prompt_text("root", "~/work");
        assert!(prompt.contains("root"));
        assert!(prompt.contains("~/work"));
    }

    #[test]
    fn secure_boot_defaults_to_safe_mode() {
        assert!(BootConfig::detected_defaults().safe_mode);
    }

    #[test]
    fn boot_profile_names_cover_modes() {
        assert_eq!(
            BootConfig {
                color: true,
                ascii_mode: false,
                safe_mode: false,
                quick_boot: false,
                mobile_mode: false,
                persistent_state: false,
                bleeding_edge: false,
            }
            .profile_name(),
            "operator"
        );
        assert_eq!(
            BootConfig {
                color: true,
                ascii_mode: false,
                safe_mode: true,
                quick_boot: true,
                mobile_mode: true,
                persistent_state: true,
                bleeding_edge: true,
            }
            .profile_name(),
            "mobile-safe+quick+edge"
        );
    }

    #[test]
    fn phase1_art_is_mobile_width() {
        assert!(phase1_art().iter().all(|line| line.chars().count() <= 40));
    }

    #[test]
    fn boot_branding_uses_one_console_title() {
        let config = BootConfig {
            color: false,
            ascii_mode: true,
            safe_mode: true,
            quick_boot: false,
            mobile_mode: true,
            persistent_state: false,
            bleeding_edge: false,
        };
        assert_eq!(console_title(config), "Phase1 // Advanced Operator Console");
        let rows = splash_info("3.8.1", config);
        assert!(rows.contains(&"version v3.8.1".to_string()));
        assert!(!rows.iter().any(|row| row.contains("os      phase1")));
    }

    #[test]
    fn theme_palette_names_are_available() {
        assert_eq!(ThemePalette::parse("rainbow"), Some(ThemePalette::Rainbow));
        assert_eq!(ThemePalette::parse("synthwave"), Some(ThemePalette::Synth));
        assert_eq!(
            ThemePalette::parse("bleeding-edge"),
            Some(ThemePalette::BleedingEdge)
        );
        assert!(ThemePalette::all().len() >= 7);

        let config = test_config();
        std::env::set_var("PHASE1_THEME", "matrix");
        assert_eq!(display_mode(config), "matrix");
        std::env::remove_var("PHASE1_THEME");
    }

    #[test]
    fn bleeding_edge_updates_display_version_and_palette() {
        let mut config = test_config();
        config.bleeding_edge = true;
        config.color = true;
        config.ascii_mode = false;
        assert_eq!(
            display_version("3.6.0", config),
            crate::updater::CURRENT_EDGE_VERSION
        );
        assert_eq!(display_mode(config), "bleeding-edge");
        assert!(splash_info("3.6.0", config).contains(&"channel bleeding-edge".to_string()));
    }

    #[test]
    fn parse_bool_accepts_config_values() {
        assert_eq!(parse_bool("true"), Some(true));
        assert_eq!(parse_bool("off"), Some(false));
        assert_eq!(parse_bool("maybe"), None);
    }

    #[test]
    fn visible_len_ignores_ansi_sequences() {
        assert_eq!(visible_len("\x1b[31mPhase1\x1b[0m"), 6);
        assert_eq!(strip_ansi("\x1b[32mon\x1b[0m"), "on");
    }
}
