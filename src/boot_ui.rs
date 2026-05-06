use crate::registry;
use std::fs;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const DESKTOP_WIDTH: usize = 72;
const LAPTOP_WIDTH: usize = 56;
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
    pub host_tools: bool,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeviceMode {
    Mobile,
    Laptop,
    Desktop,
}

impl DeviceMode {
    fn name(self) -> &'static str {
        match self {
            Self::Mobile => "mobile",
            Self::Laptop => "laptop",
            Self::Desktop => "desktop",
        }
    }

    fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "mobile" | "phone" | "deck" => Some(Self::Mobile),
            "laptop" | "notebook" | "m1" | "m2" | "m3" | "air" => Some(Self::Laptop),
            "desktop" | "workstation" | "tower" | "rig" => Some(Self::Desktop),
            _ => None,
        }
    }

    fn cycle(self) -> Self {
        match self {
            Self::Mobile => Self::Laptop,
            Self::Laptop => Self::Desktop,
            Self::Desktop => Self::Mobile,
        }
    }
}

impl ThemePalette {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "neo-tokyo" | "neotokyo" | "tokyo" | "neo" | "city" | "hacker" => Some(Self::NeoTokyo),
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
            host_tools: env_flag("PHASE1_ALLOW_HOST_TOOLS").unwrap_or(false),
        };
        if config.mobile_mode {
            set_device_mode(DeviceMode::Mobile);
        } else if std::env::var("PHASE1_DEVICE_MODE").is_err() {
            set_device_mode(if terminal_width() < 100 { DeviceMode::Laptop } else { DeviceMode::Desktop });
        }
        config.normalize_channel();
        config
    }

    pub fn apply(self) {
        let device = device_mode(self);
        set_bool_env("PHASE1_ASCII", self.ascii_mode);
        set_bool_env("PHASE1_QUICK_BOOT", self.quick_boot);
        set_bool_env("PHASE1_MOBILE_MODE", device == DeviceMode::Mobile);
        set_bool_env("PHASE1_PERSISTENT_STATE", self.persistent_state);
        set_bool_env("PHASE1_BLEEDING_EDGE", self.bleeding_edge);
        set_bool_env("PHASE1_ALLOW_HOST_TOOLS", self.host_tools);
        std::env::set_var("PHASE1_DEVICE_MODE", device.name());
        std::env::set_var("PHASE1_DISPLAY_VERSION", display_version(crate::kernel::VERSION, self));
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
        if self.bleeding_edge { format!("{base}+edge") } else { base.to_string() }
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
            let key = key.trim();
            let value = value.trim();
            if matches!(key, "device" | "device_mode" | "ui_mode") {
                if let Some(device) = DeviceMode::parse(value) {
                    set_device_mode(device);
                    config.mobile_mode = device == DeviceMode::Mobile;
                }
                continue;
            }
            let Some(value) = parse_bool(value) else {
                continue;
            };
            match key {
                "color" => config.color = value,
                "ascii" | "ascii_mode" => config.ascii_mode = value,
                "safe" | "safe_mode" => config.safe_mode = value,
                "quick" | "quick_boot" => config.quick_boot = value,
                "mobile" | "mobile_mode" => config.mobile_mode = value,
                "persistent" | "persist" | "persistent_state" => config.persistent_state = value,
                "bleeding" | "bleeding_edge" | "edge" => config.bleeding_edge = value,
                "host_tools" | "allow_host_tools" | "trusted_host_tools" => config.host_tools = value,
                _ => {}
            }
        }
        if config.mobile_mode {
            set_device_mode(DeviceMode::Mobile);
        }
        config.normalize_channel();
        Some(config)
    }

    fn apply_env_overrides(&mut self) {
        if let Some(value) = env_flag("PHASE1_ASCII") { self.ascii_mode = value; }
        if let Some(value) = env_flag("PHASE1_NO_COLOR") { self.color = !value; }
        if let Some(value) = env_flag("NO_COLOR") { self.color = !value; }
        if let Some(value) = env_flag("PHASE1_SAFE_MODE") { self.safe_mode = value; }
        if let Some(value) = env_flag("PHASE1_QUICK_BOOT") { self.quick_boot = value; }
        if let Some(value) = env_flag("PHASE1_MOBILE_MODE") { self.mobile_mode = value; }
        if let Some(value) = env_flag("PHASE1_PERSISTENT_STATE") { self.persistent_state = value; }
        if let Some(value) = env_flag("PHASE1_BLEEDING_EDGE") { self.bleeding_edge = value; }
        if let Some(value) = env_flag("PHASE1_ALLOW_HOST_TOOLS") { self.host_tools = value; }
        if let Ok(raw) = std::env::var("PHASE1_DEVICE_MODE") {
            if let Some(device) = DeviceMode::parse(&raw) {
                self.mobile_mode = device == DeviceMode::Mobile;
            }
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
            "# phase1 boot configuration\n# safe=true is the secure default. Host tools also require safe=false before they can run.\ncolor={}\nascii={}\nsafe={}\nquick={}\nmobile={}\ndevice_mode={}\npersistent={}\nbleeding_edge={}\nhost_tools={}\n",
            self.color,
            self.ascii_mode,
            self.safe_mode,
            self.quick_boot,
            device_mode(self) == DeviceMode::Mobile,
            device_mode(self).name(),
            self.persistent_state,
            self.bleeding_edge,
            self.host_tools
        )
    }
}

pub fn config_path() -> &'static str { BOOT_CONFIG_PATH }

pub fn display_version(release_version: &str, config: BootConfig) -> String {
    if config.bleeding_edge { crate::updater::CURRENT_EDGE_VERSION.to_string() } else { release_version.to_string() }
}

pub fn configure_boot(version: &str) -> BootSelection {
    let mut config = BootConfig::default();
    if let Some(_guard) = RawModeGuard::enter() {
        configure_boot_raw(version, &mut config)
    } else {
        configure_boot_cooked(version, &mut config)
    }
}

pub fn print_boot(version: &str) {
    let config = BootConfig::default();
    if device_mode(config) == DeviceMode::Mobile || terminal_width() < 72 {
        print_mobile_boot(version, config);
    } else if ascii_mode() {
        print_ascii_boot(version, config);
    } else {
        print_modern_boot(version, config);
    }
}

pub fn print_quick_boot(version: &str, config: BootConfig) {
    print_boot_card(version, config, false, None);
    println!("{}", value(config, &format!("[quick] profile={} :: shell armed :: clock {}", config.profile_name(), short_clock_utc())));
    println!();
}

pub fn print_help() { print!("{}", registry::command_map()); }

pub fn print_prompt(user: &str, path: &str) {
    print!("{}", prompt_status_bar());
    print!("{}", prompt_text(user, path));
}

fn configure_boot_raw(version: &str, config: &mut BootConfig) -> BootSelection {
    let mut stdin = io::stdin().lock();
    let mut out = io::stdout();
    let mut buf = [0_u8; 1];
    let mut notice = Some("live clock active :: press h for help".to_string());
    loop {
        print_preboot(version, *config, notice.as_deref());
        print!("{}", command_prompt(*config, "boot"));
        let _ = out.flush();
        notice = None;
        match stdin.read(&mut buf) {
            Ok(0) => continue,
            Ok(_) => {
                let input = match buf[0] {
                    b'\r' | b'\n' => "".to_string(),
                    byte => (byte as char).to_string(),
                };
                if let Some(selection) = handle_boot_input(config, &input, &mut notice) {
                    return selection;
                }
            }
            Err(_) => return BootSelection::Quit,
        }
    }
}

fn configure_boot_cooked(version: &str, config: &mut BootConfig) -> BootSelection {
    let stdin = io::stdin();
    let mut input = String::new();
    let mut notice = None;
    loop {
        print_preboot(version, *config, notice.as_deref());
        print!("{}", command_prompt(*config, "boot"));
        let _ = io::stdout().flush();
        input.clear();
        match stdin.read_line(&mut input) {
            Ok(0) | Err(_) => return BootSelection::Quit,
            Ok(_) => {}
        }
        if let Some(selection) = handle_boot_input(config, input.trim(), &mut notice) {
            return selection;
        }
    }
}

fn handle_boot_input(config: &mut BootConfig, input: &str, notice: &mut Option<String>) -> Option<BootSelection> {
    match input.trim().to_ascii_lowercase().as_str() {
        "" | "1" | "b" | "boot" | "start" | "jack-in" => {
            if let Err(err) = config.save() { eprintln!("boot config save warning: {err}"); }
            Some(BootSelection::Boot(*config))
        }
        "2" | "c" | "color" | "colour" | "neon" => { config.color = !config.color; None }
        "3" | "a" | "ascii" => { config.ascii_mode = !config.ascii_mode; None }
        "4" | "s" | "safe" | "safe-mode" | "shield" => { config.safe_mode = !config.safe_mode; None }
        "5" | "q" | "quick" | "quick-boot" => { config.quick_boot = !config.quick_boot; None }
        "6" | "m" | "device" | "mode" => { set_device(config, device_mode(*config).cycle()); None }
        "l" | "laptop" => { set_device(config, DeviceMode::Laptop); None }
        "w" | "workstation" | "desktop" => { set_device(config, DeviceMode::Desktop); None }
        "phone" | "mobile" | "mobile-mode" => { set_device(config, DeviceMode::Mobile); None }
        "t" | "trust" | "trusted" | "host-tools" | "hosttools" => { config.host_tools = !config.host_tools; None }
        "p" | "persist" | "persistent" | "persistent-state" | "vault" => { config.persistent_state = !config.persistent_state; None }
        "e" | "edge" | "bleeding" | "bleeding-edge" => { config.bleeding_edge = !config.bleeding_edge; config.normalize_channel(); None }
        "d" | "dev" | "storage" | "storage-tools" | "workspace" => {
            if let Err(err) = config.save() { eprintln!("boot config save warning: {err}"); }
            Some(BootSelection::StorageTools(*config))
        }
        "7" | "reboot" | "restart" => Some(BootSelection::Reboot),
        "8" | "x" | "quit" | "exit" | "shutdown" => Some(BootSelection::Quit),
        "9" | "save" | "write" => {
            *notice = Some(match config.save() {
                Ok(()) => "saved phase1.conf".to_string(),
                Err(err) => format!("save failed: {err}"),
            });
            None
        }
        "0" | "r" | "reset" => {
            *config = BootConfig::detected_defaults();
            *notice = Some(match BootConfig::remove_saved() {
                Ok(()) => "reset to secure detected defaults".to_string(),
                Err(err) => format!("reset defaults; remove warning: {err}"),
            });
            None
        }
        "h" | "help" | "?" => {
            *notice = Some("Enter boots | 6 cycles mobile/laptop/desktop | l=laptop | w=desktop | t=trust | 4=shield | e=edge | p=vault | 9=save | 0=reset".to_string());
            None
        }
        _ => { *notice = Some("unknown boot option; press h for help".to_string()); None }
    }
}

fn set_device(config: &mut BootConfig, mode: DeviceMode) {
    set_device_mode(mode);
    config.mobile_mode = mode == DeviceMode::Mobile;
}

fn print_preboot(version: &str, config: BootConfig, notice: Option<&str>) {
    if config.ascii_mode || !config.color { print!("\x1b[2J\x1b[H"); } else { print!("\x1b[2J\x1b[H{BOLD}"); }
    print_boot_card(version, config, true, notice);
    println!("{}", value(config, "Enter=boot  6=device  l=laptop  w=desktop  t=trust  h=help"));
}

fn print_mobile_boot(version: &str, config: BootConfig) {
    println!("\x1b[2J\x1b[H");
    print_boot_card(version, config, false, None);
    ready_line(false);
}

fn print_modern_boot(version: &str, config: BootConfig) {
    println!("\x1b[2J\x1b[H");
    print_boot_card(version, config, false, None);
    ready_line(true);
}

fn print_ascii_boot(version: &str, mut config: BootConfig) {
    config.color = false;
    config.ascii_mode = true;
    print_boot_card(version, config, false, None);
    ready_line(true);
}

fn print_boot_card(version: &str, config: BootConfig, selector: bool, notice: Option<&str>) {
    let width = card_width(config);
    println!();
    println!("{}", card_top(config, width));
    println!("{}", card_line(config, width, &console_title(config)));
    println!("{}", card_line(config, width, &format!("node TOKYO-01 | clock {}", clock_utc())));
    println!("{}", card_rule(config, width));
    for row in skyline_rows(config) { println!("{}", card_line(config, width, &row)); }
    println!("{}", card_section(config, width, "STATUS"));
    for row in splash_info(version, config) { println!("{}", card_line(config, width, &row)); }
    if selector {
        println!("{}", card_section(config, width, "BOOT CONFIG"));
        for row in boot_rows(config) { println!("{}", card_line(config, width, &row)); }
        if let Some(notice) = notice { println!("{}", card_line(config, width, &format!("notice    {notice}"))); }
    } else {
        println!("{}", card_section(config, width, "LIVE OPS"));
        println!("{}", card_line(config, width, "help dash sysinfo security"));
        println!("{}", card_line(config, width, "theme linux preview matrix audit ps"));
        println!("{}", card_line(config, width, "ls /  storage via d"));
    }
    println!("{}", card_bottom(config, width));
    println!();
}

fn skyline_rows(config: BootConfig) -> Vec<String> {
    let mode = if config.safe_mode { "shielded" } else { "host-capable" };
    let device = device_mode(config).name();
    vec![
        tint(config, "neural sync :: kernel PHASE1"),
        format!("mesh encrypted :: {mode}"),
        format!("ui device {device} :: kern/vfs/proc/net/audit/lang"),
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
        status_row(config, "device", device_mode(config).name(), true),
        status_row(config, "security", security_mode, config.safe_mode),
        status_row(config, "trust", trust_label(config), config.host_tools),
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
        option_row(config, "[6] DEVICE", device_mode(config).name(), true),
        option_row(config, "[l] LAPTOP UI", if device_mode(config) == DeviceMode::Laptop { "ON" } else { "set" }, device_mode(config) == DeviceMode::Laptop),
        option_row(config, "[w] DESKTOP UI", if device_mode(config) == DeviceMode::Desktop { "ON" } else { "set" }, device_mode(config) == DeviceMode::Desktop),
        option_row(config, "[t] TRUST HOST", trust_label(config), config.host_tools && !config.safe_mode),
        option_row(config, "[e] EDGE", on_off(config.bleeding_edge), config.bleeding_edge),
        option_row(config, "[p] VAULT", on_off(config.persistent_state), config.persistent_state),
        option_row(config, "[d] STORAGE/GIT/RUST", "open dock", true),
        option_row(config, "[7] REBOOT", "selector", true),
        option_row(config, "[8] SHUTDOWN", "abort", true),
        option_row(config, "[9] SAVE", "phase1.conf", true),
        option_row(config, "[0] RESET", "defaults", true),
    ]
}

fn trust_label(config: BootConfig) -> &'static str {
    match (config.host_tools, config.safe_mode) {
        (true, true) => "armed/safe",
        (true, false) => "enabled",
        (false, _) => "off",
    }
}

fn on_off(enabled: bool) -> &'static str { if enabled { "ON" } else { "off" } }

fn option_row(config: BootConfig, label: &str, value_text: &str, bright: bool) -> String {
    let value_text = if bright { tint(config, value_text) } else { dim(config, value_text) };
    format!("{label:<20} {value_text}")
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
        format!("{}{} // Advanced Operator Console{}", phase1_wordmark(config), colors.title, RESET)
    }
}

fn phase1_wordmark(config: BootConfig) -> String {
    if !config.color || config.ascii_mode {
        "Phase1".to_string()
    } else {
        let theme = active_theme_for_config(config);
        if theme == ThemePalette::Rainbow || theme == ThemePalette::NeoTokyo {
            let colors = [CYAN, MAGENTA, BLUE, GREEN, CYAN, MAGENTA];
            "Phase1".chars().enumerate().map(|(idx, ch)| format!("{}{}{}{}", BOLD, colors[idx % colors.len()], ch, RESET)).collect::<Vec<_>>().join("")
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
        format!("{}{}phase1{}{}://{}{}{}{} {}{}{} ⇢ ", BOLD, colors.title, RESET, GRAY, RESET, colors.prompt_user, user, RESET, colors.prompt_path, path, RESET)
    }
}

fn prompt_status_bar() -> String {
    let width = card_width(BootConfig::default());
    let channel = if bleeding_edge_env_enabled() { "edge" } else { "release" };
    let safe = if std::env::var("PHASE1_SAFE_MODE").ok().as_deref() == Some("0") { "host" } else { "safe" };
    let state = if std::env::var("PHASE1_PERSISTENT_STATE").ok().as_deref() == Some("1") { "vault" } else { "ram" };
    let trust = if std::env::var("PHASE1_ALLOW_HOST_TOOLS").ok().as_deref() == Some("1") { "trust" } else { "no-trust" };
    let device = std::env::var("PHASE1_DEVICE_MODE").unwrap_or_else(|_| "desktop".to_string());
    let raw = format!("HUD {channel}/{safe}/{state}/{trust}/{device} | {}", short_clock_utc());
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
        format!("{}{}phase1{}{}://{}{} {}❯{} ", BOLD, colors.title, RESET, GRAY, RESET, label, colors.accent, RESET)
    } else {
        format!("{label}> ")
    }
}

fn ready_line(desktop: bool) {
    if color_enabled() {
        let colors = palette(active_theme());
        if desktop {
            println!("{}[ready]{} systems synced {GRAY}:: operator shell armed :: HUD clock active{RESET}", colors.ready, RESET);
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
    if config.ascii_mode { "ascii" } else if !config.color { "mono" } else { active_theme_for_config(config).name() }
}

fn active_theme() -> ThemePalette {
    std::env::var("PHASE1_THEME").ok().and_then(|raw| ThemePalette::parse(&raw)).filter(|theme| *theme != ThemePalette::BleedingEdge || bleeding_edge_env_enabled()).unwrap_or_else(|| if bleeding_edge_env_enabled() { ThemePalette::BleedingEdge } else { ThemePalette::NeoTokyo })
}

fn active_theme_for_config(config: BootConfig) -> ThemePalette {
    std::env::var("PHASE1_THEME").ok().and_then(|raw| ThemePalette::parse(&raw)).filter(|theme| *theme != ThemePalette::BleedingEdge || config.bleeding_edge).unwrap_or_else(|| if config.bleeding_edge { ThemePalette::BleedingEdge } else { ThemePalette::NeoTokyo })
}

fn palette(theme: ThemePalette) -> Palette {
    match theme {
        ThemePalette::NeoTokyo => Palette { border: BLUE, title: MAGENTA, accent: GREEN, muted: GRAY, prompt_user: MAGENTA, prompt_path: GREEN, ready: GREEN },
        ThemePalette::Rainbow => Palette { border: CYAN, title: GREEN, accent: CYAN, muted: GRAY, prompt_user: CYAN, prompt_path: BLUE, ready: GREEN },
        ThemePalette::Matrix => Palette { border: GREEN, title: GREEN, accent: GREEN, muted: GRAY, prompt_user: GREEN, prompt_path: GREEN, ready: GREEN },
        ThemePalette::Cyber => Palette { border: MAGENTA, title: CYAN, accent: MAGENTA, muted: GRAY, prompt_user: MAGENTA, prompt_path: CYAN, ready: CYAN },
        ThemePalette::Amber => Palette { border: YELLOW, title: YELLOW, accent: YELLOW, muted: GRAY, prompt_user: YELLOW, prompt_path: YELLOW, ready: YELLOW },
        ThemePalette::Ice => Palette { border: CYAN, title: BLUE, accent: CYAN, muted: GRAY, prompt_user: CYAN, prompt_path: BLUE, ready: CYAN },
        ThemePalette::Synth => Palette { border: MAGENTA, title: MAGENTA, accent: CYAN, muted: GRAY, prompt_user: CYAN, prompt_path: MAGENTA, ready: MAGENTA },
        ThemePalette::Crimson => Palette { border: RED, title: RED, accent: YELLOW, muted: GRAY, prompt_user: RED, prompt_path: YELLOW, ready: RED },
        ThemePalette::BleedingEdge => Palette { border: BLUE, title: MAGENTA, accent: GREEN, muted: GRAY, prompt_user: MAGENTA, prompt_path: GREEN, ready: MAGENTA },
    }
}

fn card_width(config: BootConfig) -> usize {
    let max = match device_mode(config) {
        DeviceMode::Mobile => MOBILE_WIDTH,
        DeviceMode::Laptop => LAPTOP_WIDTH,
        DeviceMode::Desktop => DESKTOP_WIDTH,
    };
    terminal_width().clamp(32, max)
}

fn card_top(config: BootConfig, width: usize) -> String { if config.color && !config.ascii_mode { format!("{}╭{}╮{RESET}", palette(active_theme_for_config(config)).border, "─".repeat(width)) } else { format!("+{}+", "-".repeat(width)) } }
fn card_bottom(config: BootConfig, width: usize) -> String { if config.color && !config.ascii_mode { format!("{}╰{}╯{RESET}", palette(active_theme_for_config(config)).border, "─".repeat(width)) } else { format!("+{}+", "-".repeat(width)) } }
fn card_rule(config: BootConfig, width: usize) -> String { if config.color && !config.ascii_mode { format!("{}├{}┤{RESET}", palette(active_theme_for_config(config)).border, "─".repeat(width)) } else { format!("+{}+", "-".repeat(width)) } }

fn card_section(config: BootConfig, width: usize, label: &str) -> String {
    let marker = format!(" {label} ");
    let fill = width.saturating_sub(marker.chars().count());
    if config.color && !config.ascii_mode {
        let colors = palette(active_theme_for_config(config));
        format!("{}├{}{}{}{}┤{RESET}", colors.border, colors.accent, marker, colors.border, "─".repeat(fill))
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

fn value(config: BootConfig, text: &str) -> String { if config.color && !config.ascii_mode { format!("{}{text}{RESET}", palette(active_theme_for_config(config)).muted) } else { text.to_string() } }
fn tint(config: BootConfig, text: &str) -> String { if config.color && !config.ascii_mode { format!("{}{}{}", palette(active_theme_for_config(config)).accent, text, RESET) } else { text.to_string() } }
fn dim(config: BootConfig, text: &str) -> String { if config.color && !config.ascii_mode { format!("{}{DIM}{text}{RESET}", palette(active_theme_for_config(config)).muted) } else { text.to_string() } }

fn set_bool_env(name: &str, enabled: bool) { if enabled { std::env::set_var(name, "1"); } else { std::env::remove_var(name); } }
fn terminal_width() -> usize { std::env::var("COLUMNS").ok().and_then(|raw| raw.parse().ok()).unwrap_or(MOBILE_WIDTH) }
fn detect_mobile() -> bool { terminal_width() < 72 || ["TERM_PROGRAM", "TERM", "SSH_CLIENT", "PHASE1_DEVICE"].iter().filter_map(|name| std::env::var(name).ok()).any(|value| { let upper = value.to_ascii_uppercase(); ["IPHONE", "ANDROID", "BLINK", "ISH", "TERMUX", "MOBILE"].iter().any(|needle| upper.contains(needle)) }) }
fn bleeding_edge_env_enabled() -> bool { std::env::var("PHASE1_BLEEDING_EDGE").ok().as_deref() == Some("1") }
fn env_flag(name: &str) -> Option<bool> { std::env::var(name).ok().and_then(|value| parse_bool(&value)) }
fn parse_bool(value: &str) -> Option<bool> { match value.trim().to_ascii_lowercase().as_str() { "1" | "true" | "yes" | "on" => Some(true), "0" | "false" | "no" | "off" => Some(false), _ => None } }

fn device_mode(config: BootConfig) -> DeviceMode {
    if config.mobile_mode { return DeviceMode::Mobile; }
    std::env::var("PHASE1_DEVICE_MODE").ok().and_then(|raw| DeviceMode::parse(&raw)).filter(|mode| *mode != DeviceMode::Mobile).unwrap_or(DeviceMode::Desktop)
}
fn set_device_mode(mode: DeviceMode) { std::env::set_var("PHASE1_DEVICE_MODE", mode.name()); }

fn clock_utc() -> String {
    let seconds = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() % 86_400;
    let hours = seconds / 3_600;
    let minutes = (seconds % 3_600) / 60;
    let seconds = seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02} UTC")
}
fn short_clock_utc() -> String {
    let seconds = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() % 86_400;
    let hours = seconds / 3_600;
    let minutes = (seconds % 3_600) / 60;
    format!("{hours:02}:{minutes:02} UTC")
}
fn strip_ansi(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            for code in chars.by_ref() { if code.is_ascii_alphabetic() { break; } }
        } else { out.push(ch); }
    }
    out
}
fn visible_len(text: &str) -> usize { strip_ansi(text).chars().count() }
fn clip_visible(text: &str, width: usize) -> String { let plain = strip_ansi(text); if plain.chars().count() <= width { text.to_string() } else { plain.chars().take(width).collect() } }
fn color_enabled() -> bool { std::env::var_os("NO_COLOR").is_none() && std::env::var("PHASE1_NO_COLOR").ok().as_deref() != Some("1") }
fn ascii_mode() -> bool { std::env::var("PHASE1_ASCII").ok().as_deref() == Some("1") }

struct RawModeGuard { original: Option<String> }
impl RawModeGuard {
    fn enter() -> Option<Self> {
        if std::env::var_os("PHASE1_COOKED_INPUT").is_some() { return None; }
        let original = stty(&["-g"]).ok().filter(|raw| !raw.trim().is_empty())?;
        if stty(&["raw", "-echo", "-icanon", "min", "0", "time", "10"]).is_err() { return None; }
        Some(Self { original: Some(original.trim().to_string()) })
    }
}
impl Drop for RawModeGuard { fn drop(&mut self) { if let Some(original) = self.original.take() { let _ = stty(&[&original]); } } }
fn stty(args: &[&str]) -> io::Result<String> {
    let output = Command::new("stty").args(args).stdin(Stdio::inherit()).stdout(Stdio::piped()).stderr(Stdio::null()).output()?;
    if output.status.success() { Ok(String::from_utf8_lossy(&output.stdout).to_string()) } else { Err(io::Error::other("stty failed")) }
}

#[cfg(test)]
mod tests {
    use super::{clock_utc, device_mode, display_version, parse_bool, prompt_status_bar, strip_ansi, BootConfig, DeviceMode, ThemePalette};

    fn config() -> BootConfig {
        BootConfig { color: true, ascii_mode: false, safe_mode: true, quick_boot: false, mobile_mode: false, persistent_state: false, bleeding_edge: false, host_tools: false }
    }

    #[test]
    fn clock_uses_utc_suffix() { assert!(clock_utc().ends_with(" UTC")); }

    #[test]
    fn device_mode_supports_laptop_and_desktop() {
        let mut cfg = config();
        std::env::set_var("PHASE1_DEVICE_MODE", "laptop");
        assert_eq!(device_mode(cfg), DeviceMode::Laptop);
        std::env::set_var("PHASE1_DEVICE_MODE", "desktop");
        assert_eq!(device_mode(cfg), DeviceMode::Desktop);
        cfg.mobile_mode = true;
        assert_eq!(device_mode(cfg), DeviceMode::Mobile);
        std::env::remove_var("PHASE1_DEVICE_MODE");
    }

    #[test]
    fn prompt_status_bar_contains_device_and_clock() {
        std::env::set_var("NO_COLOR", "1");
        std::env::set_var("PHASE1_DEVICE_MODE", "laptop");
        let bar = prompt_status_bar();
        assert!(bar.contains("HUD"));
        assert!(bar.contains("UTC"));
        assert!(bar.contains("laptop"));
        std::env::remove_var("NO_COLOR");
        std::env::remove_var("PHASE1_DEVICE_MODE");
    }

    #[test]
    fn theme_palette_names_are_available() {
        assert!(ThemePalette::all().iter().any(|theme| theme.name() == "neo-tokyo"));
        assert!(ThemePalette::all().iter().any(|theme| theme.name() == "bleeding-edge"));
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
    fn ansi_stripper_removes_escape_codes() { assert_eq!(strip_ansi("\x1b[36mPhase1\x1b[0m"), "Phase1"); }
}
