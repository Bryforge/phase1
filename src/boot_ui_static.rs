use crate::registry;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const BOOT_CONFIG_PATH: &str = "phase1.conf";
const MOBILE_WIDTH: usize = 40;
const LAPTOP_WIDTH: usize = 56;
const DESKTOP_WIDTH: usize = 72;
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

struct Palette {
    border: &'static str,
    title: &'static str,
    accent: &'static str,
    muted: &'static str,
    prompt_user: &'static str,
    prompt_path: &'static str,
    ready: &'static str,
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
            "laptop" | "notebook" | "thinkpad" | "x200" | "m1" | "m2" | "m3" | "air" => {
                Some(Self::Laptop)
            }
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
        let requested_device = std::env::var("PHASE1_DEVICE_MODE")
            .ok()
            .and_then(|raw| DeviceMode::parse(&raw));
        let mobile = env_flag("PHASE1_MOBILE_MODE").unwrap_or(false)
            || requested_device == Some(DeviceMode::Mobile)
            || detect_mobile_terminal();
        let device = if mobile {
            DeviceMode::Mobile
        } else if let Some(device) = requested_device {
            device
        } else {
            default_large_screen_device()
        };
        set_device_mode(device);

        let mut config = Self {
            color: default_color_enabled(),
            ascii_mode: default_ascii_mode(device),
            safe_mode: env_flag("PHASE1_SAFE_MODE").unwrap_or(true),
            quick_boot: env_flag("PHASE1_QUICK_BOOT").unwrap_or(false),
            mobile_mode: device == DeviceMode::Mobile,
            persistent_state: env_flag("PHASE1_PERSISTENT_STATE").unwrap_or(false),
            bleeding_edge: env_flag("PHASE1_BLEEDING_EDGE").unwrap_or(false),
            host_tools: env_flag("PHASE1_ALLOW_HOST_TOOLS").unwrap_or(false),
        };
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
                "host_tools" | "allow_host_tools" | "trusted_host_tools" => {
                    config.host_tools = value
                }
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
        if let Some(value) = env_flag("PHASE1_ALLOW_HOST_TOOLS") {
            self.host_tools = value;
        }
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
            if std::env::var_os("PHASE1_FORCE_ASCII").is_none() {
                self.ascii_mode = false;
            }
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
    let boot_stamp = static_boot_stamp();
    std::env::set_var("PHASE1_BOOT_STAMP", &boot_stamp);
    crate::ops_log::log_event(
        "boot.selector",
        &format!("opened profile={}", config.profile_name()),
    );
    configure_boot_cooked(version, &mut config, &boot_stamp)
}

pub fn print_boot(version: &str) {
    let config = BootConfig::default();
    let boot_stamp = std::env::var("PHASE1_BOOT_STAMP").unwrap_or_else(|_| static_boot_stamp());
    if config.ascii_mode || !config.color {
        print_ascii_boot(version, config, &boot_stamp);
    } else {
        print_modern_boot(version, config, &boot_stamp);
    }
}

pub fn print_quick_boot(version: &str, config: BootConfig) {
    let boot_stamp = std::env::var("PHASE1_BOOT_STAMP").unwrap_or_else(|_| static_boot_stamp());
    print_boot_card(version, config, false, None, &boot_stamp);
    outln(&value(
        config,
        &format!(
            "[quick] profile={} :: shell armed :: boot {boot_stamp}",
            config.profile_name()
        ),
    ));
    outln("");
}

pub fn print_help() {
    print!("{}", registry::command_map());
}

pub fn print_prompt(user: &str, path: &str) {
    if compact_prompt_enabled() {
        print!("{}", prompt_text(user, path));
    } else {
        print!("{}", prompt_status_bar());
        print!("{}", prompt_text(user, path));
    }
}

fn configure_boot_cooked(
    version: &str,
    config: &mut BootConfig,
    boot_stamp: &str,
) -> BootSelection {
    let stdin = io::stdin();
    let mut input = String::new();
    let mut notice = Some("static boot clock".to_string());
    loop {
        print_preboot(version, *config, notice.as_deref(), boot_stamp);
        print!("{}", command_prompt(*config, "boot"));
        let _ = io::stdout().flush();
        input.clear();
        match stdin.read_line(&mut input) {
            Ok(0) | Err(_) => return BootSelection::Quit,
            Ok(_) => {}
        }
        let trimmed = input.trim();
        crate::ops_log::log_event("boot.input", trimmed);
        if let Some(selection) = handle_boot_input(config, trimmed, &mut notice) {
            crate::ops_log::log_event("boot.selection", selection_label(selection));
            return selection;
        }
    }
}

fn handle_boot_input(
    config: &mut BootConfig,
    input: &str,
    notice: &mut Option<String>,
) -> Option<BootSelection> {
    match input.trim().to_ascii_lowercase().as_str() {
        "" | "1" | "b" | "boot" | "start" | "jack-in" => {
            if let Err(err) = config.save() {
                eprintln!("boot config save warning: {err}");
            }
            Some(BootSelection::Boot(*config))
        }
        "2" | "c" | "color" | "colour" | "neon" => {
            config.color = !config.color;
            *notice = Some("color output toggled".to_string());
            None
        }
        "3" | "a" | "ascii" => {
            config.ascii_mode = !config.ascii_mode;
            *notice = Some("ASCII compatibility toggled".to_string());
            None
        }
        "4" | "s" | "safe" | "safe-mode" | "shield" => {
            config.safe_mode = !config.safe_mode;
            *notice = Some("safe shield toggled".to_string());
            None
        }
        "5" | "q" | "quick" | "quick-boot" => {
            config.quick_boot = !config.quick_boot;
            *notice = Some("quick boot toggled".to_string());
            None
        }
        "6" | "m" | "device" | "mode" => {
            set_device(config, device_mode(*config).cycle());
            *notice = Some(format!("device UI set to {}", device_mode(*config).name()));
            None
        }
        "l" | "laptop" => {
            set_device(config, DeviceMode::Laptop);
            *notice = Some("laptop UI selected".to_string());
            None
        }
        "w" | "workstation" | "desktop" => {
            set_device(config, DeviceMode::Desktop);
            *notice = Some("desktop UI selected".to_string());
            None
        }
        "phone" | "mobile" | "mobile-mode" => {
            set_device(config, DeviceMode::Mobile);
            *notice = Some("mobile UI selected".to_string());
            None
        }
        "t" | "trust" | "trusted" | "host-tools" | "hosttools" => {
            config.host_tools = !config.host_tools;
            *notice = Some("trust gate toggled".to_string());
            None
        }
        "p" | "persist" | "persistent" | "persistent-state" | "vault" => {
            config.persistent_state = !config.persistent_state;
            *notice = Some("persistent state toggled".to_string());
            None
        }
        "e" | "edge" | "bleeding" | "bleeding-edge" => {
            config.bleeding_edge = !config.bleeding_edge;
            config.normalize_channel();
            *notice = Some("release channel toggled".to_string());
            None
        }
        "d" | "dev" | "storage" | "storage-tools" | "workspace" => {
            if let Err(err) = config.save() {
                eprintln!("boot config save warning: {err}");
            }
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
        _ => {
            *notice = Some("unknown boot option; press h for help".to_string());
            None
        }
    }
}

fn selection_label(selection: BootSelection) -> &'static str {
    match selection {
        BootSelection::Boot(_) => "boot",
        BootSelection::StorageTools(_) => "storage-tools",
        BootSelection::Quit => "quit",
        BootSelection::Reboot => "reboot",
    }
}

fn set_device(config: &mut BootConfig, mode: DeviceMode) {
    set_device_mode(mode);
    config.mobile_mode = mode == DeviceMode::Mobile;
}

fn print_preboot(version: &str, config: BootConfig, notice: Option<&str>, boot_stamp: &str) {
    print!("\x1b[2J\x1b[H");
    if config.color && !config.ascii_mode {
        print!("{BOLD}");
    }
    print_boot_card(version, config, true, notice, boot_stamp);
    outln(&value(
        config,
        "Enter=boot  6=device  l=laptop  w=desktop  t=trust  h=help",
    ));
}

fn print_modern_boot(version: &str, config: BootConfig, boot_stamp: &str) {
    print!("\x1b[2J\x1b[H");
    print_boot_card(version, config, false, None, boot_stamp);
    ready_line(device_mode(config) != DeviceMode::Mobile);
}

fn print_ascii_boot(version: &str, mut config: BootConfig, boot_stamp: &str) {
    config.color = false;
    config.ascii_mode = true;
    print_boot_card(version, config, false, None, boot_stamp);
    ready_line(device_mode(config) != DeviceMode::Mobile);
}

fn print_boot_card(
    version: &str,
    config: BootConfig,
    selector: bool,
    notice: Option<&str>,
    boot_stamp: &str,
) {
    let width = card_width(config);
    outln("");
    outln(&card_top(config, width));
    outln(&card_line(config, width, &console_title(config)));
    outln(&card_line(
        config,
        width,
        &format!("node TOKYO-01 | boot {boot_stamp}"),
    ));
    outln(&card_rule(config, width));
    for row in skyline_rows(config) {
        outln(&card_line(config, width, &row));
    }
    outln(&card_section(config, width, "STATUS"));
    for row in splash_info(version, config, boot_stamp) {
        outln(&card_line(config, width, &row));
    }
    if selector {
        outln(&card_section(config, width, "BOOT CONFIG"));
        for row in boot_rows(config) {
            outln(&card_line(config, width, &row));
        }
        if let Some(notice) = notice {
            outln(&card_line(config, width, &format!("notice    {notice}")));
        }
    } else {
        outln(&card_section(config, width, "LIVE OPS"));
        outln(&card_line(
            config,
            width,
            "help dash sysinfo security opslog",
        ));
        outln(&card_line(
            config,
            width,
            "theme linux preview matrix audit ps",
        ));
        outln(&card_line(config, width, "ls /  storage via d"));
    }
    outln(&card_bottom(config, width));
    outln("");
}

fn skyline_rows(config: BootConfig) -> Vec<String> {
    let mode = if config.safe_mode {
        "shielded"
    } else {
        "host-capable"
    };
    vec![
        tint(config, "neural sync :: kernel PHASE1"),
        format!("mesh encrypted :: {mode}"),
        format!(
            "ui device {} :: kern/vfs/proc/net/audit/lang",
            device_mode(config).name()
        ),
    ]
}

fn splash_info(version: &str, config: BootConfig, boot_stamp: &str) -> Vec<String> {
    let state_mode = if config.persistent_state {
        "vault/persistent"
    } else {
        "ram/volatile"
    };
    let security_mode = if config.safe_mode {
        "safe shield"
    } else {
        "host bridge"
    };
    let channel = if config.bleeding_edge {
        "bleeding-edge"
    } else {
        "release"
    };
    let workspace =
        std::env::var("PHASE1_STORAGE_ROOT").unwrap_or_else(|_| "phase1.workspace".to_string());
    vec![
        status_row(
            config,
            "version",
            &format!("v{}", display_version(version, config)),
            true,
        ),
        status_row(config, "boot", boot_stamp, true),
        status_row(config, "channel", channel, config.bleeding_edge),
        status_row(config, "profile", &config.profile_name(), true),
        status_row(config, "device", device_mode(config).name(), true),
        status_row(config, "security", security_mode, config.safe_mode),
        status_row(config, "trust", trust_label(config), config.host_tools),
        status_row(config, "display", display_mode(config), true),
        status_row(config, "state", state_mode, config.persistent_state),
        status_row(config, "log", crate::ops_log::LOG_PATH, true),
        status_row(config, "workspace", &workspace, true),
        status_row(config, "nest", &nested_label(), nested_level() > 0),
    ]
}

fn boot_rows(config: BootConfig) -> Vec<String> {
    vec![
        option_row(config, "[1] BOOT", "start shell", true),
        option_row(config, "[2] NEON", on_off(config.color), config.color),
        option_row(
            config,
            "[3] ASCII",
            on_off(config.ascii_mode),
            config.ascii_mode,
        ),
        option_row(
            config,
            "[4] SHIELD",
            on_off(config.safe_mode),
            config.safe_mode,
        ),
        option_row(
            config,
            "[5] QUICK",
            on_off(config.quick_boot),
            config.quick_boot,
        ),
        option_row(config, "[6] DEVICE", device_mode(config).name(), true),
        option_row(
            config,
            "[l] LAPTOP UI",
            if device_mode(config) == DeviceMode::Laptop {
                "ON"
            } else {
                "set"
            },
            device_mode(config) == DeviceMode::Laptop,
        ),
        option_row(
            config,
            "[w] DESKTOP UI",
            if device_mode(config) == DeviceMode::Desktop {
                "ON"
            } else {
                "set"
            },
            device_mode(config) == DeviceMode::Desktop,
        ),
        option_row(
            config,
            "[t] TRUST HOST",
            trust_label(config),
            config.host_tools && !config.safe_mode,
        ),
        option_row(
            config,
            "[e] EDGE",
            on_off(config.bleeding_edge),
            config.bleeding_edge,
        ),
        option_row(
            config,
            "[p] VAULT",
            on_off(config.persistent_state),
            config.persistent_state,
        ),
        option_row(config, "[d] STORAGE/GIT/RUST", "open dock", true),
        option_row(config, "[7] REBOOT", "selector", true),
        option_row(config, "[8] SHUTDOWN", "abort", true),
        option_row(config, "[9] SAVE", "phase1.conf", true),
        option_row(config, "[0] RESET", "defaults", true),
    ]
}


fn nested_level() -> u32 {
    std::env::var("PHASE1_NESTED_LEVEL")
        .ok()
        .and_then(|raw| raw.trim().parse::<u32>().ok())
        .unwrap_or(0)
}

fn nested_max() -> u32 {
    std::env::var("PHASE1_NESTED_MAX")
        .ok()
        .and_then(|raw| raw.trim().parse::<u32>().ok())
        .unwrap_or(1)
}

fn nested_label() -> String {
    format!("level {}/{}", nested_level(), nested_max())
}

fn trust_label(config: BootConfig) -> &'static str {
    match (config.host_tools, config.safe_mode) {
        (true, true) => "armed/safe",
        (true, false) => "enabled",
        (false, _) => "off",
    }
}

fn option_row(config: BootConfig, label: &str, value_text: &str, bright: bool) -> String {
    let value_text = if bright {
        tint(config, value_text)
    } else {
        dim(config, value_text)
    };
    format!("{label:<20} {value_text}")
}

fn status_row(config: BootConfig, label: &str, value_text: &str, bright: bool) -> String {
    let value_text = if bright {
        tint(config, value_text)
    } else {
        dim(config, value_text)
    };
    format!("{label:<10} {value_text}")
}

fn on_off(enabled: bool) -> &'static str {
    if enabled {
        "ON"
    } else {
        "off"
    }
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
        if matches!(theme, ThemePalette::Rainbow | ThemePalette::NeoTokyo) {
            let colors = [CYAN, MAGENTA, BLUE, GREEN, CYAN, MAGENTA];
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
    if compact_prompt_enabled() {
        return compact_prompt_text(user, path);
    }

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

fn compact_prompt_text(user: &str, path: &str) -> String {
    if !color_enabled() {
        return format!(
            "phase1://{} {} {} ⇢ ",
            user,
            path,
            compact_prompt_chips_plain()
        );
    }

    let colors = palette(active_theme());
    format!(
        "{}{}phase1{}{}://{}{}{}{} {}{}{} {} ⇢ ",
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
        RESET,
        compact_prompt_chips_colored()
    )
}

fn compact_prompt_enabled() -> bool {
    env_flag("PHASE1_COMPACT_PROMPT").unwrap_or(true)
}

fn compact_prompt_chips_plain() -> String {
    format!(
        "[{} {} {}]",
        prompt_channel_chip(),
        prompt_security_chip(),
        prompt_trust_chip()
    )
}

fn compact_prompt_chips_colored() -> String {
    format!(
        "{}[{}{}{} {}{}{} {}{}{}{}]",
        GRAY,
        prompt_channel_color(),
        prompt_channel_chip(),
        GRAY,
        prompt_security_color(),
        prompt_security_chip(),
        GRAY,
        prompt_trust_color(),
        prompt_trust_chip(),
        GRAY,
        RESET
    )
}

fn prompt_channel_chip() -> &'static str {
    if bleeding_edge_env_enabled() {
        "edge"
    } else {
        "release"
    }
}

fn prompt_security_chip() -> &'static str {
    if std::env::var("PHASE1_SAFE_MODE").ok().as_deref() == Some("0") {
        "host"
    } else {
        "safe"
    }
}

fn prompt_trust_chip() -> &'static str {
    if env_flag("PHASE1_ALLOW_HOST_TOOLS").unwrap_or(false) {
        "trust"
    } else {
        "no-trust"
    }
}

fn prompt_channel_color() -> &'static str {
    if bleeding_edge_env_enabled() {
        MAGENTA
    } else {
        CYAN
    }
}

fn prompt_security_color() -> &'static str {
    if prompt_security_chip() == "safe" {
        GREEN
    } else {
        YELLOW
    }
}

fn prompt_trust_color() -> &'static str {
    if prompt_trust_chip() == "trust" {
        CYAN
    } else {
        RED
    }
}

fn prompt_status_bar() -> String {
    let config = BootConfig::default();
    let width = card_width(config);
    let channel = if bleeding_edge_env_enabled() {
        "edge"
    } else {
        "release"
    };
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
    let trust = if std::env::var("PHASE1_ALLOW_HOST_TOOLS").ok().as_deref() == Some("1") {
        "trust"
    } else {
        "no-trust"
    };
    let device = std::env::var("PHASE1_DEVICE_MODE")
        .unwrap_or_else(|_| device_mode(config).name().to_string());
    let raw = format!(
        "HUD {} | {channel}/{safe}/{state}/{trust}/{device}",
        short_clock_utc()
    );
    let minimum_clock_width = 28;
    let render_width = width.max(minimum_clock_width);
    let padded = pad_visible(&clip_visible(&raw, render_width), render_width);
    if color_enabled() && !ascii_mode() {
        format!("{}{}{}\n", palette(active_theme()).accent, padded, RESET)
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
            outln(&format!(
                "{}[ready]{} systems synced {GRAY}:: operator shell armed :: ops log active{RESET}",
                colors.ready, RESET
            ));
        } else {
            outln(&format!("{}[ready]{} systems synced", colors.ready, RESET));
        }
    } else if desktop {
        outln("[ready] systems synced :: operator shell armed :: ops log active");
    } else {
        outln("[ready] systems synced");
    }
    outln("");
}

fn card_width(config: BootConfig) -> usize {
    let max = match device_mode(config) {
        DeviceMode::Mobile => MOBILE_WIDTH,
        DeviceMode::Laptop => LAPTOP_WIDTH,
        DeviceMode::Desktop => DESKTOP_WIDTH,
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
            "{}├{}{}{}{}┤{RESET}",
            colors.border,
            colors.accent,
            marker,
            colors.border,
            "─".repeat(fill)
        )
    } else {
        format!("+{marker}{}+", "-".repeat(fill))
    }
}

fn card_line(config: BootConfig, width: usize, text: &str) -> String {
    let padded = pad_visible(&clip_visible(text, width), width);
    if config.color && !config.ascii_mode {
        let border = palette(active_theme_for_config(config)).border;
        format!("{border}│{RESET}{padded}{border}│{RESET}")
    } else {
        format!("|{padded}|")
    }
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

fn display_mode(config: BootConfig) -> &'static str {
    if config.ascii_mode {
        "ascii"
    } else if !config.color {
        "mono"
    } else {
        active_theme_for_config(config).name()
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

fn device_mode(config: BootConfig) -> DeviceMode {
    if config.mobile_mode {
        return DeviceMode::Mobile;
    }
    std::env::var("PHASE1_DEVICE_MODE")
        .ok()
        .and_then(|raw| DeviceMode::parse(&raw))
        .filter(|mode| *mode != DeviceMode::Mobile)
        .unwrap_or_else(default_large_screen_device)
}

fn default_large_screen_device() -> DeviceMode {
    if terminal_width() >= 110 {
        DeviceMode::Desktop
    } else {
        DeviceMode::Laptop
    }
}
fn set_device_mode(mode: DeviceMode) {
    std::env::set_var("PHASE1_DEVICE_MODE", mode.name());
}
fn set_bool_env(name: &str, enabled: bool) {
    if enabled {
        std::env::set_var(name, "1");
    } else {
        std::env::remove_var(name);
    }
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

fn default_color_enabled() -> bool {
    std::env::var_os("NO_COLOR").is_none()
        && std::env::var("PHASE1_NO_COLOR").ok().as_deref() != Some("1")
        && std::env::var("TERM").ok().as_deref() != Some("dumb")
}
fn color_enabled() -> bool {
    default_color_enabled()
}
fn ascii_mode() -> bool {
    std::env::var("PHASE1_ASCII").ok().as_deref() == Some("1")
}
fn bleeding_edge_env_enabled() -> bool {
    std::env::var("PHASE1_BLEEDING_EDGE").ok().as_deref() == Some("1")
}

fn default_ascii_mode(device: DeviceMode) -> bool {
    if ascii_mode() || std::env::var("TERM").ok().as_deref() == Some("dumb") {
        return true;
    }
    if std::env::var_os("PHASE1_UNICODE").is_some() {
        return false;
    }
    if device == DeviceMode::Mobile {
        return false;
    }
    !locale_is_utf8() || terminal_width() < 48
}

fn locale_is_utf8() -> bool {
    ["LC_ALL", "LC_CTYPE", "LANG"]
        .iter()
        .filter_map(|name| std::env::var(name).ok())
        .any(|value| {
            let upper = value.to_ascii_uppercase();
            upper.contains("UTF-8") || upper.contains("UTF8")
        })
}

fn terminal_width() -> usize {
    if let Ok(raw) = std::env::var("COLUMNS") {
        if let Ok(width) = raw.parse::<usize>() {
            return width.max(32);
        }
    }
    stty(&["size"])
        .ok()
        .and_then(|raw| {
            raw.split_whitespace()
                .nth(1)
                .and_then(|value| value.parse::<usize>().ok())
        })
        .unwrap_or(80)
        .max(32)
}

fn detect_mobile_terminal() -> bool {
    ["TERM_PROGRAM", "TERM", "SSH_CLIENT", "PHASE1_DEVICE"]
        .iter()
        .filter_map(|name| std::env::var(name).ok())
        .any(|value| {
            let upper = value.to_ascii_uppercase();
            ["IPHONE", "ANDROID", "BLINK", "ISH", "TERMUX", "MOBILE"]
                .iter()
                .any(|needle| upper.contains(needle))
        })
}

fn static_boot_stamp() -> String {
    clock_utc()
}
fn clock_utc() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        % 86_400;
    format!(
        "{:02}:{:02}:{:02} UTC",
        seconds / 3_600,
        (seconds % 3_600) / 60,
        seconds % 60
    )
}
fn short_clock_utc() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        % 86_400;
    format!("{:02}:{:02} UTC", seconds / 3_600, (seconds % 3_600) / 60)
}
fn outln(text: &str) {
    print!("{text}\r\n");
}

fn pad_visible(text: &str, width: usize) -> String {
    let visible = visible_len(text);
    format!("{text}{}", " ".repeat(width.saturating_sub(visible)))
}
fn clip_visible(text: &str, width: usize) -> String {
    let plain = strip_ansi(text);
    if plain.chars().count() <= width {
        text.to_string()
    } else {
        plain.chars().take(width).collect()
    }
}
fn visible_len(text: &str) -> usize {
    strip_ansi(text).chars().count()
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

fn stty(args: &[&str]) -> io::Result<String> {
    let output = Command::new("stty")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(io::Error::other("stty failed"))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, OnceLock};

    fn env_test_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|err| err.into_inner())
    }

    use super::{
        clock_utc, default_large_screen_device, display_version, parse_bool, prompt_status_bar,
        strip_ansi, BootConfig, DeviceMode, ThemePalette,
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
            host_tools: false,
        }
    }

    #[test]
    fn clock_uses_utc_suffix() {
        assert!(clock_utc().ends_with(" UTC"));
    }

    #[test]
    fn boot_hud_reports_nested_level() {
        let _env_lock = env_test_lock();
        std::env::set_var("PHASE1_NESTED_LEVEL", "1");
        std::env::set_var("PHASE1_NESTED_MAX", "2");
        let rows = super::splash_info("4.3.0-dev", config(), "00:00:00 UTC");
        assert!(rows.iter().any(|row| row.contains("nest")));
        assert!(rows.iter().any(|row| row.contains("level 1/2")));
        std::env::remove_var("PHASE1_NESTED_LEVEL");
        std::env::remove_var("PHASE1_NESTED_MAX");
    }

    #[test]
    fn compact_prompt_inlines_dynamic_status_chips_for_all_modes() {
        let _env_lock = env_test_lock();
        std::env::set_var("NO_COLOR", "1");
        std::env::remove_var("PHASE1_COMPACT_PROMPT");
        std::env::set_var("PHASE1_BLEEDING_EDGE", "1");
        std::env::set_var("PHASE1_SAFE_MODE", "1");
        std::env::set_var("PHASE1_ALLOW_HOST_TOOLS", "1");

        for mode in ["mobile", "laptop", "desktop"] {
            std::env::set_var("PHASE1_DEVICE_MODE", mode);
            let prompt = super::prompt_text("root", "~");
            assert!(super::compact_prompt_enabled());
            assert!(prompt.contains("phase1://root ~ [edge safe trust] ⇢ "));
            assert!(!prompt.contains("HUD"));
        }

        std::env::remove_var("NO_COLOR");
        std::env::remove_var("PHASE1_DEVICE_MODE");
        std::env::remove_var("PHASE1_BLEEDING_EDGE");
        std::env::remove_var("PHASE1_SAFE_MODE");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
    }

    #[test]
    fn compact_prompt_can_be_disabled_for_legacy_hud_prompt() {
        let _env_lock = env_test_lock();
        std::env::set_var("PHASE1_COMPACT_PROMPT", "0");
        assert!(!super::compact_prompt_enabled());
        std::env::remove_var("PHASE1_COMPACT_PROMPT");
    }

    #[test]
    fn compact_prompt_colorizes_dynamic_chips_when_color_is_enabled() {
        let _env_lock = env_test_lock();
        std::env::remove_var("NO_COLOR");
        std::env::remove_var("PHASE1_NO_COLOR");
        std::env::remove_var("PHASE1_ASCII");
        std::env::remove_var("PHASE1_COMPACT_PROMPT");
        std::env::set_var("PHASE1_DEVICE_MODE", "desktop");
        std::env::set_var("PHASE1_BLEEDING_EDGE", "1");
        std::env::set_var("PHASE1_SAFE_MODE", "1");
        std::env::set_var("PHASE1_ALLOW_HOST_TOOLS", "1");

        let prompt = super::prompt_text("root", "~");
        assert!(prompt.contains("\x1b["));
        assert!(prompt.contains("["));
        assert!(prompt.contains("edge"));
        assert!(prompt.contains("safe"));
        assert!(prompt.contains("trust"));
        assert!(prompt.contains("⇢"));

        std::env::remove_var("PHASE1_DEVICE_MODE");
        std::env::remove_var("PHASE1_BLEEDING_EDGE");
        std::env::remove_var("PHASE1_SAFE_MODE");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
    }

    #[test]
    fn prompt_status_bar_contains_clock_without_overflowing() {
        let _env_lock = env_test_lock();
        std::env::set_var("NO_COLOR", "1");
        std::env::set_var("COLUMNS", "32");
        let bar = prompt_status_bar();
        let plain = strip_ansi(&bar);
        assert!(plain.contains("HUD"));
        assert!(plain.contains("UTC"));
        assert!(plain.trim_end().chars().count() >= 28);
        std::env::remove_var("NO_COLOR");
        std::env::remove_var("COLUMNS");
    }

    #[test]
    fn theme_palette_names_are_available() {
        assert!(ThemePalette::all()
            .iter()
            .any(|theme| theme.name() == "neo-tokyo"));
        assert!(ThemePalette::all()
            .iter()
            .any(|theme| theme.name() == "bleeding-edge"));
    }

    #[test]
    fn display_version_uses_edge_channel_when_requested() {
        let _env_lock = env_test_lock();
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

    #[test]
    fn laptop_is_default_for_x200_width() {
        let _env_lock = env_test_lock();
        std::env::set_var("COLUMNS", "80");
        assert_eq!(default_large_screen_device(), DeviceMode::Laptop);
        std::env::remove_var("COLUMNS");
    }
}
