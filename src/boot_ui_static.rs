use crate::registry;
use std::fs;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const BOOT_CONFIG_PATH: &str = "phase1.conf";
const MOBILE_WIDTH: usize = 46;
const LAPTOP_WIDTH: usize = 64;
const DESKTOP_WIDTH: usize = 78;
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
            "laptop" | "notebook" | "thinkpad" | "x200" | "m1" | "m2" | "m3" | "air" => Some(Self::Laptop),
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
            Self::NeoTokyo => "Tokyo operator HUD with cyan/magenta signal lines",
            Self::Rainbow => "classic rainbow ANSI gradient",
            Self::Matrix => "green-on-black developer console",
            Self::Cyber => "cyan/magenta high-contrast operator console",
            Self::Amber => "warm amber retro terminal",
            Self::Ice => "cool blue/cyan frost terminal",
            Self::Synth => "purple synthwave operator glow",
            Self::Crimson => "red alert / incident-response console",
            Self::BleedingEdge => "edge-only command deck palette",
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
        if let Some(saved) = Self::load_saved_from_disk() {
            config = saved;
        }
        config.apply_env_overrides();
        config.normalize_channel();
        config
    }
}

impl BootConfig {
    pub fn detected_defaults() -> Self {
        let requested_device = std::env::var("PHASE1_DEVICE_MODE").ok().and_then(|raw| DeviceMode::parse(&raw));
        let mobile = env_flag("PHASE1_MOBILE_MODE").unwrap_or(false) || requested_device == Some(DeviceMode::Mobile);
        let device = if mobile { DeviceMode::Mobile } else { requested_device.unwrap_or_else(default_large_screen_device) };
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
        std::env::set_var("PHASE1_DISPLAY_VERSION", display_version(crate::kernel::VERSION, self));
        std::env::set_var("PHASE1_SAFE_MODE", if self.safe_mode { "1" } else { "0" });
        if self.color {
            std::env::remove_var("PHASE1_NO_COLOR");
        } else {
            std::env::set_var("PHASE1_NO_COLOR", "1");
        }
        if self.bleeding_edge {
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
        let base = match (device_mode(self), self.safe_mode, self.quick_boot) {
            (DeviceMode::Mobile, true, true) => "mobile-safe+quick",
            (DeviceMode::Mobile, true, false) => "mobile-safe",
            (DeviceMode::Mobile, false, true) => "mobile-quick",
            (DeviceMode::Mobile, false, false) => "mobile",
            (_, true, true) => "safe+quick",
            (_, true, false) => "safe",
            (_, false, true) => "operator+quick",
            (_, false, false) => "operator",
        };
        if self.bleeding_edge { format!("{base}+edge") } else { base.to_string() }
    }

    fn load_saved_from_disk() -> Option<Self> {
        let raw = fs::read_to_string(config_path()).ok()?;
        let mut config = Self::detected_defaults();
        for line in raw.lines().map(str::trim) {
            if line.is_empty() || line.starts_with('#') { continue; }
            let Some((key, value)) = line.split_once('=') else { continue; };
            let key = key.trim();
            let value = value.trim();
            if matches!(key, "device" | "device_mode" | "ui_mode") {
                if let Some(device) = DeviceMode::parse(value) {
                    set_device_mode(device);
                    config.mobile_mode = device == DeviceMode::Mobile;
                }
                continue;
            }
            let Some(value) = parse_bool(value) else { continue; };
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
                set_device_mode(device);
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
            self.host_tools,
        )
    }
}

pub fn config_path() -> &'static str { BOOT_CONFIG_PATH }

pub fn display_version(release_version: &str, config: BootConfig) -> String {
    if config.bleeding_edge { crate::updater::CURRENT_EDGE_VERSION.to_string() } else { release_version.to_string() }
}

pub fn configure_boot(version: &str) -> BootSelection {
    let mut config = BootConfig::default();
    let boot_stamp = static_boot_stamp();
    std::env::set_var("PHASE1_BOOT_STAMP", &boot_stamp);
    crate::ops_log::log_event("boot.selector", &format!("opened profile={}", config.profile_name()));
    print_phase1_splash(config, version, "selector");
    configure_boot_cooked(version, &mut config, &boot_stamp)
}

pub fn print_boot(version: &str) {
    let config = BootConfig::default();
    let boot_stamp = std::env::var("PHASE1_BOOT_STAMP").unwrap_or_else(|_| static_boot_stamp());
    if !env_flag("PHASE1_SKIP_BOOT_SPLASH").unwrap_or(false) {
        print_phase1_splash(config, version, "boot");
    }
    print_boot_card(version, config, false, None, &boot_stamp, false);
    ready_line(device_mode(config) != DeviceMode::Mobile);
}

pub fn print_quick_boot(version: &str, config: BootConfig) {
    let boot_stamp = std::env::var("PHASE1_BOOT_STAMP").unwrap_or_else(|_| static_boot_stamp());
    print_boot_card(version, config, false, None, &boot_stamp, false);
    outln(&value(config, &format!("[quick] profile={} :: shell armed :: boot {boot_stamp}", config.profile_name())));
    outln("");
}

pub fn print_help() {
    print!("{}", registry::command_map());
}

pub fn print_prompt(user: &str, path: &str) {
    print!("{}", prompt_status_bar());
    print!("{}", prompt_text(user, path));
}

fn configure_boot_cooked(version: &str, config: &mut BootConfig, boot_stamp: &str) -> BootSelection {
    let stdin = io::stdin();
    let mut input = String::new();
    let mut notice = Some("Boot selector ready. Press h for the full command guide.".to_string());
    let mut show_help = false;
    loop {
        print_preboot(version, *config, notice.as_deref(), boot_stamp, show_help);
        print!("{}", command_prompt(*config, "boot"));
        let _ = io::stdout().flush();
        input.clear();
        match stdin.read_line(&mut input) {
            Ok(0) | Err(_) => return BootSelection::Quit,
            Ok(_) => {}
        }
        let trimmed = input.trim();
        crate::ops_log::log_event("boot.input", trimmed);
        if let Some(selection) = handle_boot_input(config, trimmed, &mut notice, &mut show_help) {
            crate::ops_log::log_event("boot.selection", selection_label(selection));
            return selection;
        }
    }
}

fn handle_boot_input(
    config: &mut BootConfig,
    input: &str,
    notice: &mut Option<String>,
    show_help: &mut bool,
) -> Option<BootSelection> {
    match input.trim().to_ascii_lowercase().as_str() {
        "" | "1" | "b" | "boot" | "start" | "jack-in" => { let _ = config.save(); Some(BootSelection::Boot(*config)) }
        "2" | "c" | "color" | "colour" | "neon" => { config.color = !config.color; *notice = Some("Neon output toggled.".to_string()); None }
        "3" | "a" | "ascii" => { config.ascii_mode = !config.ascii_mode; *notice = Some("ASCII compatibility toggled.".to_string()); None }
        "4" | "s" | "safe" | "safe-mode" | "shield" => { config.safe_mode = !config.safe_mode; *notice = Some("Safe shield toggled. Host tools still require TRUST HOST.".to_string()); None }
        "5" | "q" | "quick" | "quick-boot" => { config.quick_boot = !config.quick_boot; *notice = Some("Quick boot toggled.".to_string()); None }
        "6" | "m" | "device" | "mode" => { set_device(config, device_mode(*config).cycle()); *notice = Some(format!("Device UI set to {}.", device_mode(*config).name())); None }
        "l" | "laptop" => { set_device(config, DeviceMode::Laptop); *notice = Some("Laptop UI selected.".to_string()); None }
        "w" | "workstation" | "desktop" => { set_device(config, DeviceMode::Desktop); *notice = Some("Desktop UI selected.".to_string()); None }
        "phone" | "mobile" | "mobile-mode" => { set_device(config, DeviceMode::Mobile); *notice = Some("Mobile UI selected.".to_string()); None }
        "t" | "trust" | "trusted" | "host-tools" | "hosttools" => { config.host_tools = !config.host_tools; *notice = Some("TRUST HOST toggled. Turn SHIELD off before host-backed tools can run.".to_string()); None }
        "p" | "persist" | "persistent" | "persistent-state" | "vault" => { config.persistent_state = !config.persistent_state; *notice = Some("Vault persistence toggled.".to_string()); None }
        "e" | "edge" | "bleeding" | "bleeding-edge" => { config.bleeding_edge = !config.bleeding_edge; config.normalize_channel(); *notice = Some("Release channel toggled.".to_string()); None }
        "d" | "i" | "install" | "dev" | "storage" | "storage-tools" | "workspace" => { let _ = config.save(); Some(BootSelection::StorageTools(*config)) }
        "7" | "reboot" | "restart" => Some(BootSelection::Reboot),
        "8" | "x" | "quit" | "exit" | "shutdown" => Some(BootSelection::Quit),
        "9" | "save" | "write" => { *notice = Some(match config.save() { Ok(()) => "Saved phase1.conf.".to_string(), Err(err) => format!("Save failed: {err}.") }); None }
        "0" | "r" | "reset" => { *config = BootConfig::detected_defaults(); *notice = Some(match BootConfig::remove_saved() { Ok(()) => "Reset to secure detected defaults.".to_string(), Err(err) => format!("Reset defaults; remove warning: {err}.") }); None }
        "h" | "help" | "?" => { *show_help = !*show_help; *notice = Some(if *show_help { "Boot help expanded." } else { "Boot help hidden." }.to_string()); None }
        _ => { *notice = Some("Unknown boot option. Press h for boot help.".to_string()); None }
    }
}

fn selection_label(selection: BootSelection) -> &'static str {
    match selection {
        BootSelection::Boot(_) => "boot",
        BootSelection::StorageTools(_) => "install-storage-tools",
        BootSelection::Quit => "quit",
        BootSelection::Reboot => "reboot",
    }
}

fn set_device(config: &mut BootConfig, mode: DeviceMode) {
    set_device_mode(mode);
    config.mobile_mode = mode == DeviceMode::Mobile;
}

fn print_preboot(version: &str, config: BootConfig, notice: Option<&str>, boot_stamp: &str, show_help: bool) {
    print!("\x1b[2J\x1b[H");
    print_boot_card(version, config, true, notice, boot_stamp, show_help);
    outln(&value(config, "Enter=boot  h=help  e=edge  t=trust  p=vault  i=install  6=device"));
}

fn print_boot_card(version: &str, config: BootConfig, selector: bool, notice: Option<&str>, boot_stamp: &str, show_help: bool) {
    let width = card_width(config);
    outln("");
    outln(&card_top(config, width));
    outln(&card_line(config, width, &console_title(config)));
    outln(&card_line(config, width, &format!("東京-01 // 全サブシステム正常 | boot {boot_stamp}")));
    outln(&card_rule(config, width));
    for row in subsystem_rows(config) { outln(&card_line(config, width, &row)); }
    outln(&card_section(config, width, "STATUS"));
    for row in splash_info(version, config, boot_stamp) { outln(&card_line(config, width, &row)); }
    if selector {
        outln(&card_section(config, width, "BOOT CONFIG"));
        for row in boot_rows(config) { outln(&card_line(config, width, &row)); }
        if show_help {
            outln(&card_section(config, width, "BOOT HELP"));
            for row in boot_help_rows() { outln(&card_line(config, width, row)); }
        }
        if let Some(notice) = notice { outln(&card_line(config, width, &format!("notice    {notice}"))); }
    } else {
        outln(&card_section(config, width, "LIVE OPS"));
        for row in ["help | dash | sysinfo | security | opslog", "theme deck | matrix | audit | ps | lang", "install helper: reboot then press i"] { outln(&card_line(config, width, row)); }
    }
    outln(&card_bottom(config, width));
    outln("");
}

fn print_phase1_splash(config: BootConfig, version: &str, mode: &str) {
    if env_flag("PHASE1_SKIP_BOOT_SPLASH").unwrap_or(false) { return; }
    print!("\x1b[2J\x1b[H");
    let color = config.color && !config.ascii_mode && color_enabled();
    let title = if color { format!("{BOLD}{MAGENTA}PHASE1{RESET}") } else { "PHASE1".to_string() };
    outln("");
    outln(&center_line(48, &title));
    outln(&center_line(48, "Advanced Operator Console"));
    outln(&center_line(48, &format!("edge v{} | stable v4.0.0 | previous v3.10.9", crate::updater::CURRENT_EDGE_VERSION)));
    outln("");
    for (idx, step) in ["preparing kernel interface", "mounting virtual filesystems", "checking trust boundary", "arming developer console"].iter().enumerate() {
        let spinner = ["|", "/", "-", "\\"][idx % 4];
        let line = if color { format!("{CYAN}{spinner}{RESET} {step} [{mode}]") } else { format!("* {step} [{mode}]") };
        outln(&line);
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(55));
    }
    outln(&format!("ready: v{}", display_version(version, config)));
    thread::sleep(Duration::from_millis(80));
}

fn boot_help_rows() -> &'static [&'static str] {
    &[
        "Enter / 1 / boot    start the Phase1 shell",
        "i / d / install     open storage, Git, Rust, and install dock",
        "e                  toggle bleeding-edge channel and edge palette",
        "t                  arm TRUST HOST; SHIELD must be off for host tools",
        "4 / shield          toggle the safe-mode shield",
        "p / vault           toggle persistent state and history",
        "6 / l / w           cycle device UI, force laptop, or force desktop",
        "9 / save / 0        save profile or reset detected defaults",
    ]
}

fn subsystem_rows(config: BootConfig) -> Vec<String> {
    let boundary = if config.safe_mode { "shielded" } else { "host-capable" };
    vec![
        tint(config, "kernel sync        :: nominal"),
        format!("trust boundary     :: {boundary}"),
        "sub-systems       :: all nominal".to_string(),
        format!("ui device          :: {} / vfs / proc / net / audit / lang", device_mode(config).name()),
    ]
}

fn splash_info(version: &str, config: BootConfig, boot_stamp: &str) -> Vec<String> {
    let state_mode = if config.persistent_state { "vault/persistent" } else { "ram/volatile" };
    let security_mode = if config.safe_mode { "safe shield" } else { "host bridge" };
    let channel = if config.bleeding_edge { "bleeding-edge" } else { "release" };
    let workspace = std::env::var("PHASE1_STORAGE_ROOT").unwrap_or_else(|_| "phase1.workspace".to_string());
    vec![
        status_row(config, "version", &format!("v{}", display_version(version, config)), true),
        status_row(config, "stable", "v4.0.0", true),
        status_row(config, "previous", "v3.10.9", true),
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
        option_row(config, "[i] INSTALL DOCK", "storage/git/rust", true),
        option_row(config, "[7] REBOOT", "selector", true),
        option_row(config, "[8] SHUTDOWN", "abort", true),
        option_row(config, "[9] SAVE", "phase1.conf", true),
        option_row(config, "[0] RESET", "defaults", true),
    ]
}

fn option_row(config: BootConfig, label: &str, value_text: &str, bright: bool) -> String {
    format!("{label:<20} {}", if bright { tint(config, value_text) } else { dim(config, value_text) })
}

fn status_row(config: BootConfig, label: &str, value_text: &str, bright: bool) -> String {
    format!("{label:<10} {}", if bright { tint(config, value_text) } else { dim(config, value_text) })
}

fn console_title(config: BootConfig) -> String {
    let title = "Phase1 // Advanced Operator Console";
    if config.color && !config.ascii_mode && color_enabled() { format!("{}{}{}", palette(active_theme_for_config(config)).title, title, RESET) } else { title.to_string() }
}

fn prompt_text(user: &str, path: &str) -> String {
    if ascii_mode() || !color_enabled() {
        format!("phase1://{} {} > ", user, path)
    } else {
        let colors = palette(active_theme());
        format!("{}{}phase1{}{}://{}{}{}{} {}{}{} ❯ ", BOLD, colors.title, RESET, GRAY, RESET, colors.prompt_user, user, RESET, colors.prompt_path, path, RESET)
    }
}

fn prompt_status_bar() -> String {
    let channel = if bleeding_edge_env_enabled() { "edge" } else { "release" };
    let safe = if std::env::var("PHASE1_SAFE_MODE").ok().as_deref() == Some("0") { "host" } else { "safe" };
    let base = format!("HUD {} | ready | Tab completes | {} | {}", static_boot_stamp(), channel, safe);
    if ascii_mode() || !color_enabled() { format!("{base}\n") } else { format!("{}{}{}\n", palette(active_theme()).ready, base, RESET) }
}

fn ready_line(show_hud: bool) {
    if show_hud { outln("HUD ready | Tab completes | type help for commands"); }
}

fn trust_label(config: BootConfig) -> &'static str {
    match (config.host_tools, config.safe_mode) {
        (true, true) => "armed/safe",
        (true, false) => "enabled",
        (false, _) => "off",
    }
}

fn display_mode(config: BootConfig) -> &'static str {
    if config.ascii_mode { "ascii" } else if !config.color { "mono" } else if config.bleeding_edge { "bleeding-edge" } else { active_theme_for_config(config).name() }
}

fn on_off(enabled: bool) -> &'static str { if enabled { "ON" } else { "off" } }

fn card_width(config: BootConfig) -> usize {
    match device_mode(config) {
        DeviceMode::Mobile => MOBILE_WIDTH,
        DeviceMode::Laptop => LAPTOP_WIDTH,
        DeviceMode::Desktop => DESKTOP_WIDTH,
    }
}

fn card_top(config: BootConfig, width: usize) -> String { frame(config, "┌", "─", "┐", width) }
fn card_bottom(config: BootConfig, width: usize) -> String { frame(config, "└", "─", "┘", width) }
fn card_rule(config: BootConfig, width: usize) -> String { frame(config, "├", "─", "┤", width) }

fn card_section(config: BootConfig, width: usize, title: &str) -> String {
    let inner = width.saturating_sub(2);
    let label = format!(" {title} ");
    let fill = inner.saturating_sub(label.chars().count());
    border(config, &format!("├{}{}┤", label, "─".repeat(fill)))
}

fn card_line(config: BootConfig, width: usize, text: &str) -> String {
    let inner = width.saturating_sub(4);
    let clean = single_line(text);
    let clipped = clip_to_width(&clean, inner);
    border(config, &format!("│ {:<inner$} │", clipped))
}

fn frame(config: BootConfig, left: &str, fill: &str, right: &str, width: usize) -> String {
    border(config, &format!("{}{}{}", left, fill.repeat(width.saturating_sub(2)), right))
}

fn border(config: BootConfig, raw: &str) -> String {
    if config.color && !config.ascii_mode && color_enabled() { format!("{}{}{}", palette(active_theme_for_config(config)).border, raw, RESET) } else { raw.to_string() }
}

fn tint(config: BootConfig, raw: &str) -> String {
    if config.color && !config.ascii_mode && color_enabled() { format!("{}{}{}", palette(active_theme_for_config(config)).accent, raw, RESET) } else { raw.to_string() }
}

fn dim(config: BootConfig, raw: &str) -> String {
    if config.color && !config.ascii_mode && color_enabled() { format!("{}{}{}", DIM, raw, RESET) } else { raw.to_string() }
}

fn value(config: BootConfig, raw: &str) -> String { tint(config, raw) }
fn outln(raw: &str) { println!("{raw}"); }

fn command_prompt(config: BootConfig, segment: &str) -> String {
    if config.color && !config.ascii_mode && color_enabled() {
        format!("{}phase1{}://{}{}{} ❯ ", palette(active_theme_for_config(config)).title, RESET, palette(active_theme_for_config(config)).prompt_path, segment, RESET)
    } else {
        format!("phase1://{segment} > ")
    }
}

fn palette(theme: ThemePalette) -> Palette {
    match theme {
        ThemePalette::NeoTokyo => Palette { border: CYAN, title: MAGENTA, accent: GREEN, muted: GRAY, prompt_user: MAGENTA, prompt_path: CYAN, ready: GREEN },
        ThemePalette::Rainbow => Palette { border: CYAN, title: MAGENTA, accent: GREEN, muted: GRAY, prompt_user: CYAN, prompt_path: BLUE, ready: GREEN },
        ThemePalette::Matrix => Palette { border: GREEN, title: GREEN, accent: GREEN, muted: GRAY, prompt_user: GREEN, prompt_path: GREEN, ready: GREEN },
        ThemePalette::Cyber => Palette { border: CYAN, title: MAGENTA, accent: CYAN, muted: GRAY, prompt_user: MAGENTA, prompt_path: CYAN, ready: CYAN },
        ThemePalette::Amber => Palette { border: YELLOW, title: YELLOW, accent: YELLOW, muted: GRAY, prompt_user: YELLOW, prompt_path: YELLOW, ready: YELLOW },
        ThemePalette::Ice => Palette { border: BLUE, title: CYAN, accent: CYAN, muted: GRAY, prompt_user: CYAN, prompt_path: BLUE, ready: CYAN },
        ThemePalette::Synth => Palette { border: MAGENTA, title: MAGENTA, accent: CYAN, muted: GRAY, prompt_user: MAGENTA, prompt_path: CYAN, ready: MAGENTA },
        ThemePalette::Crimson => Palette { border: RED, title: RED, accent: YELLOW, muted: GRAY, prompt_user: RED, prompt_path: YELLOW, ready: RED },
        ThemePalette::BleedingEdge => Palette { border: CYAN, title: MAGENTA, accent: GREEN, muted: GRAY, prompt_user: MAGENTA, prompt_path: CYAN, ready: GREEN },
    }
}

fn active_theme_for_config(config: BootConfig) -> ThemePalette {
    if config.bleeding_edge { return ThemePalette::BleedingEdge; }
    active_theme()
}

fn active_theme() -> ThemePalette {
    std::env::var("PHASE1_THEME").ok().and_then(|raw| ThemePalette::parse(&raw)).unwrap_or(ThemePalette::Cyber)
}

fn device_mode(config: BootConfig) -> DeviceMode {
    std::env::var("PHASE1_DEVICE_MODE").ok().and_then(|raw| DeviceMode::parse(&raw)).unwrap_or(if config.mobile_mode { DeviceMode::Mobile } else { DeviceMode::Desktop })
}

fn set_device_mode(mode: DeviceMode) { std::env::set_var("PHASE1_DEVICE_MODE", mode.name()); }

fn default_large_screen_device() -> DeviceMode {
    let columns = std::env::var("COLUMNS").ok().and_then(|raw| raw.parse::<usize>().ok()).unwrap_or(100);
    if columns < 70 { DeviceMode::Laptop } else { DeviceMode::Desktop }
}

fn default_color_enabled() -> bool { std::env::var_os("NO_COLOR").is_none() && env_flag("PHASE1_NO_COLOR") != Some(true) }
fn default_ascii_mode(device: DeviceMode) -> bool { device == DeviceMode::Mobile || env_flag("PHASE1_ASCII").unwrap_or(false) }
fn ascii_mode() -> bool { env_flag("PHASE1_ASCII").unwrap_or(false) }
fn color_enabled() -> bool { std::env::var_os("NO_COLOR").is_none() && env_flag("PHASE1_NO_COLOR") != Some(true) }
fn bleeding_edge_env_enabled() -> bool { env_flag("PHASE1_BLEEDING_EDGE").unwrap_or(false) }

fn set_bool_env(key: &str, value: bool) { std::env::set_var(key, if value { "1" } else { "0" }); }

fn env_flag(key: &str) -> Option<bool> { std::env::var(key).ok().and_then(|raw| parse_bool(&raw)) }

fn parse_bool(raw: &str) -> Option<bool> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" | "enabled" => Some(true),
        "0" | "false" | "no" | "off" | "disabled" => Some(false),
        _ => None,
    }
}

fn static_boot_stamp() -> String {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() % 86_400;
    format!("{:02}:{:02}:{:02} UTC", secs / 3600, (secs / 60) % 60, secs % 60)
}

fn center_line(width: usize, text: &str) -> String {
    let plain = strip_ansi(text);
    let len = plain.chars().count();
    if len >= width { text.to_string() } else { format!("{}{}", " ".repeat((width - len) / 2), text) }
}

fn single_line(raw: &str) -> String { raw.split_whitespace().collect::<Vec<_>>().join(" ") }

fn clip_to_width(raw: &str, width: usize) -> String {
    if strip_ansi(raw).chars().count() <= width { return raw.to_string(); }
    let take = width.saturating_sub(1);
    let mut out = raw.chars().take(take).collect::<String>();
    out.push('…');
    out
}

fn strip_ansi(raw: &str) -> String {
    let mut out = String::new();
    let mut chars = raw.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            for next in chars.by_ref() { if next == 'm' { break; } }
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{config_path, display_version, parse_bool, prompt_status_bar, BootConfig, ThemePalette};

    fn edge_config() -> BootConfig {
        BootConfig { color: true, ascii_mode: false, safe_mode: true, quick_boot: false, mobile_mode: false, persistent_state: false, bleeding_edge: true, host_tools: false }
    }

    #[test]
    fn bool_parser_handles_common_values() {
        assert_eq!(parse_bool("on"), Some(true));
        assert_eq!(parse_bool("disabled"), Some(false));
        assert_eq!(parse_bool("maybe"), None);
    }

    #[test]
    fn display_version_uses_edge_channel_when_requested() {
        assert_eq!(display_version("4.0.0", edge_config()), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn theme_palette_names_are_available() {
        let names = ThemePalette::all().iter().map(|theme| theme.name()).collect::<Vec<_>>();
        assert!(names.contains(&"cyber"));
        assert!(names.contains(&"bleeding-edge"));
        assert!(ThemePalette::parse("matrix").is_some());
    }

    #[test]
    fn prompt_status_bar_contains_clock_without_overflowing() {
        std::env::set_var("PHASE1_ASCII", "1");
        let out = prompt_status_bar();
        assert!(out.contains("HUD"));
        assert!(out.contains("UTC"));
        assert!(out.contains("Tab completes"));
        assert!(out.lines().next().unwrap_or("").chars().count() < 100);
    }

    #[test]
    fn config_path_is_phase1_conf() {
        assert_eq!(config_path(), "phase1.conf");
    }
}
