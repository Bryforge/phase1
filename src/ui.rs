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
}

impl Default for BootConfig {
    fn default() -> Self {
        let mut config = Self::detected_defaults();
        if let Some(saved) = Self::load_saved() {
            config = saved;
        }
        config.apply_env_overrides();
        config
    }
}

impl BootConfig {
    pub fn detected_defaults() -> Self {
        Self {
            color: color_enabled(),
            ascii_mode: ascii_mode(),
            safe_mode: env_flag("PHASE1_SAFE_MODE").unwrap_or(false),
            quick_boot: env_flag("PHASE1_QUICK_BOOT").unwrap_or(false),
            mobile_mode: env_flag("PHASE1_MOBILE_MODE").unwrap_or(false) || detect_mobile(),
            persistent_state: env_flag("PHASE1_PERSISTENT_STATE").unwrap_or(false),
        }
    }

    pub fn apply(self) {
        if self.ascii_mode {
            std::env::set_var("PHASE1_ASCII", "1");
        } else {
            std::env::remove_var("PHASE1_ASCII");
        }

        if self.color {
            std::env::remove_var("PHASE1_NO_COLOR");
        } else {
            std::env::set_var("PHASE1_NO_COLOR", "1");
        }

        if self.safe_mode {
            std::env::set_var("PHASE1_SAFE_MODE", "1");
        } else {
            std::env::remove_var("PHASE1_SAFE_MODE");
        }

        if self.quick_boot {
            std::env::set_var("PHASE1_QUICK_BOOT", "1");
        } else {
            std::env::remove_var("PHASE1_QUICK_BOOT");
        }

        if self.mobile_mode {
            std::env::set_var("PHASE1_MOBILE_MODE", "1");
        } else {
            std::env::remove_var("PHASE1_MOBILE_MODE");
        }

        if self.persistent_state {
            std::env::set_var("PHASE1_PERSISTENT_STATE", "1");
        } else {
            std::env::remove_var("PHASE1_PERSISTENT_STATE");
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

    pub fn profile_name(self) -> &'static str {
        match (self.mobile_mode, self.safe_mode, self.quick_boot) {
            (true, true, true) => "mobile-safe+quick",
            (true, true, false) => "mobile-safe",
            (true, false, true) => "mobile-quick",
            (true, false, false) => "mobile",
            (false, true, true) => "safe+quick",
            (false, true, false) => "safe",
            (false, false, true) => "quick",
            (false, false, false) => "operator",
        }
    }

    fn load_saved() -> Option<Self> {
        let raw = fs::read_to_string(config_path()).ok()?;
        let mut config = Self::detected_defaults();

        for line in raw.lines() {
            let line = line.trim();
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
                _ => {}
            }
        }
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
    }

    fn to_config_string(self) -> String {
        format!(
            "# phase1 boot configuration\ncolor={}\nascii={}\nsafe={}\nquick={}\nmobile={}\npersistent={}\n",
            self.color,
            self.ascii_mode,
            self.safe_mode,
            self.quick_boot,
            self.mobile_mode,
            self.persistent_state
        )
    }
}

pub fn config_path() -> &'static str {
    BOOT_CONFIG_PATH
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
            "p" | "persist" | "persistent" | "persistent-state" => config.persistent_state = !config.persistent_state,
            "7" | "reboot" | "restart" => return BootSelection::Reboot,
            "8" | "x" | "quit" | "exit" | "shutdown" => return BootSelection::Quit,
            "9" | "save" | "write" => match config.save() {
                Ok(()) => pause("Saved boot configuration to phase1.conf."),
                Err(err) => pause(&format!("Could not save phase1.conf: {err}")),
            },
            "0" | "r" | "reset" => {
                config = BootConfig::detected_defaults();
                match BootConfig::remove_saved() {
                    Ok(()) => pause("Reset to detected defaults and removed phase1.conf."),
                    Err(err) => pause(&format!("Reset defaults, but could not remove phase1.conf: {err}")),
                }
            }
            "h" | "help" | "?" => pause("Toggle options, p toggles persistent state, 9 saves, 0 resets saved config, 1 boots, 7 reboots, 8 quits."),
            _ => pause("Unknown boot option. Press Enter to continue."),
        }
    }
}

pub fn print_boot(version: &str) {
    if mobile_mode_enabled() || terminal_width() < 72 {
        print_mobile_boot(version);
    } else if ascii_mode() {
        print_ascii_boot(version);
    } else {
        print_modern_boot(version);
    }
}

pub fn print_quick_boot(version: &str, config: BootConfig) {
    print_fastfetch_splash(version, config);
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
    print_fastfetch_splash_with_config(version, config);
    let width = preboot_width();
    println!("{}", panel_line(config, "BOOT", width));
    println!("  1  boot system       {}", value(config, "save+start"));
    println!("  2  color output      {}", flag(config, config.color));
    println!("  3  ascii compatible  {}", flag(config, config.ascii_mode));
    println!("  4  safe mode         {}", flag(config, config.safe_mode));
    println!("  5  quick boot        {}", flag(config, config.quick_boot));
    println!("  6  mobile mode       {}", flag(config, config.mobile_mode));
    println!("  p  persistent state  {}", flag(config, config.persistent_state));
    println!("  7  reboot selector");
    println!("  8  quit boot");
    println!("  9  save config");
    println!("  0  reset saved config");
    println!();
    println!("{}", value(config, "Enter=save+boot  p=persist  h=help"));
}

fn print_fastfetch_splash(version: &str, config: BootConfig) {
    print_fastfetch_splash_with_config(version, config);
}

fn print_fastfetch_splash_with_config(version: &str, config: BootConfig) {
    let compact = config.mobile_mode || terminal_width() < 72;
    let art = phase1_art();
    let info = splash_info(version, config, compact);

    println!();
    if compact {
        for (idx, line) in art.iter().enumerate() {
            println!("{}", rainbow(idx, line, config));
        }
        println!();
        for row in info {
            println!("{}", value(config, &row));
        }
        println!("{}", value(config, "fastfetch boot // cyberdeck ready"));
    } else {
        for (idx, line) in art.iter().enumerate() {
            let art_line = rainbow(idx, line, config);
            let info_line = info.get(idx).map(String::as_str).unwrap_or("");
            println!("{art_line}  {}", value(config, info_line));
        }
        println!("{}", value(config, "      ▀▀ fastfetch boot selector // cyberdeck ready ▀▀"));
    }
    println!();
}

fn splash_info(version: &str, config: BootConfig, compact: bool) -> Vec<String> {
    let state_mode = if config.persistent_state { "persistent" } else { "volatile" };
    if compact {
        vec![
            format!("os      phase1 v{version}"),
            format!("profile {}", config.profile_name()),
            format!("device  {}", if config.mobile_mode { "mobile" } else { "desktop" }),
            format!("display {}", if config.color { "rainbow" } else { "mono" }),
            format!("state   {state_mode}"),
            format!("config  {}", config_path()),
        ]
    } else {
        vec![
            format!("os        phase1 terminal-os-sim v{version}"),
            format!("profile   {}", config.profile_name()),
            format!("device    {}", if config.mobile_mode { "mobile" } else { "desktop" }),
            format!("display   {}", if config.color { "retro rainbow" } else { "mono" }),
            format!("state     {state_mode}"),
            format!("config    {}", config_path()),
        ]
    }
}

fn phase1_art() -> [&'static str; 5] {
    [
        " ____  _                     __ ",
        "|  _ \\| |__   __ _ ___  ___ /_ |",
        "| |_) | '_ \\ / _` / __|/ _ \\ | |",
        "|  __/| | | | (_| \\__ \\  __/ | |",
        "|_|   |_| |_|\\__,_|___/\\___| |_|",
    ]
}

fn prompt_text(user: &str, path: &str) -> String {
    if ascii_mode() || !color_enabled() {
        format!("phase1://{} {} > ", user, path)
    } else {
        format!(
            "{}phase1{}{}://{}{}{}{} {}{}{} ❯ ",
            BOLD, RESET, GRAY, RESET, CYAN, user, RESET, BLUE, path, RESET
        )
    }
}

fn print_mobile_boot(version: &str) {
    println!("\x1b[2J\x1b[H");
    let config = BootConfig::default();
    println!("{}", accent(config, &format!("PHASE1 // OPERATOR v{version}")));
    println!("{}", value(config, "virtual kernel • sandbox • cyberdeck"));
    println!("{}", panel_line(config, "BOOT MATRIX", preboot_width()));

    for (name, state) in [
        ("core    kernel", "online"),
        ("vfs     mounted", "ready"),
        ("proc    scheduler", "active"),
        ("net     inspection", "linked"),
        ("hw      pcie model", "ready"),
        ("sec     audit log", "tracking"),
    ] {
        println!("{}  {}", accent(config, name), value(config, state));
    }

    println!("{}", panel_line(config, "SESSION", preboot_width()));
    println!("{}", value(config, "user=root  mode=operator"));
    println!("{}", value(config, "shell=registry  ui=mobile"));
    println!("{}", value(config, &format!("state={}", if config.persistent_state { "persistent" } else { "volatile" })));

    println!("{}", panel_line(config, "QUICK", preboot_width()));
    println!("{}", value(config, "help  audit  ps  ls /"));
    println!("{}", value(config, "matrix  browser phase1  version"));

    if color_enabled() {
        println!("{GREEN}[ready]{RESET} all subsystems nominal");
    } else {
        println!("[ready] all subsystems nominal");
    }
    println!();
}

fn print_modern_boot(version: &str) {
    println!("\x1b[2J\x1b[H");
    top();
    center(&format!("PHASE1 // ADVANCED OPERATOR CONSOLE  v{version}"));
    center("virtual kernel • secure sandbox • terminal control deck");
    mid("BOOT MATRIX");
    boot_row("CORE", "kernel orchestration", "ONLINE");
    boot_row("VFS", "virtual filesystem", "MOUNTED");
    boot_row("PROC", "scheduler + process table", "ACTIVE");
    boot_row("NET", "network inspection layer", "LINKED");
    boot_row("HW", "pcie + memory model", "READY");
    boot_row("SEC", "audit telemetry pipeline", "TRACKING");
    mid("SESSION");
    line("user=root  tty=phase1  mode=operator  runtime=std-only");
    line(&format!("integrity=nominal  shell=registry-backed  state={}", if env_flag("PHASE1_PERSISTENT_STATE").unwrap_or(false) { "persistent" } else { "volatile" }));
    mid("QUICK ACTIONS");
    line("help        complete p      audit        ps        ls /");
    line("matrix      browser phase1  ifconfig     tree      version");
    bottom();
    if color_enabled() {
        println!("{GREEN}[ready]{RESET} all subsystems nominal {GRAY}:: operator shell armed{RESET}");
    } else {
        println!("[ready] all subsystems nominal :: operator shell armed");
    }
    println!();
}

fn print_ascii_boot(version: &str) {
    println!("+--------------------------------------------------------------+");
    println!("| PHASE1 // ADVANCED OPERATOR CONSOLE  v{version:<22}|");
    println!("| virtual kernel | secure sandbox | terminal control deck      |");
    println!("+--------------------------- BOOT MATRIX ----------------------+");
    println!("| CORE kernel orchestration                         ONLINE     |");
    println!("| VFS  virtual filesystem                           MOUNTED    |");
    println!("| PROC scheduler + process table                    ACTIVE     |");
    println!("| NET  network inspection layer                     LINKED     |");
    println!("| HW   pcie + memory model                          READY      |");
    println!("| SEC  audit telemetry pipeline                     TRACKING   |");
    println!("+--------------------------- QUICK ACTIONS --------------------+");
    println!("| help  complete p  audit  ps  ls /  matrix                    |");
    println!("+--------------------------------------------------------------+");
    println!("[ready] all subsystems nominal :: operator shell armed");
    println!();
}

fn top() {
    if color_enabled() {
        println!("{CYAN}╭{}╮{RESET}", "─".repeat(PANEL_WIDTH));
    } else {
        println!("+{}+", "-".repeat(PANEL_WIDTH));
    }
}

fn bottom() {
    if color_enabled() {
        println!("{CYAN}╰{}╯{RESET}", "─".repeat(PANEL_WIDTH));
    } else {
        println!("+{}+", "-".repeat(PANEL_WIDTH));
    }
}

fn mid(label: &str) {
    let marker = format!(" {} ", label);
    let fill = PANEL_WIDTH.saturating_sub(marker.chars().count());
    if color_enabled() {
        println!("{CYAN}├{marker}{}┤{RESET}", "─".repeat(fill));
    } else {
        println!("+{marker}{}+", "-".repeat(fill));
    }
}

fn center(text: &str) {
    let clipped = clip(text, PANEL_WIDTH);
    let visible = clipped.chars().count();
    let left = PANEL_WIDTH.saturating_sub(visible) / 2;
    let right = PANEL_WIDTH.saturating_sub(visible + left);
    framed(&format!("{}{}{}", " ".repeat(left), clipped, " ".repeat(right)));
}

fn line(text: &str) {
    let clipped = clip(text, PANEL_WIDTH);
    framed(&format!("{clipped:<width$}", width = PANEL_WIDTH));
}

fn boot_row(code: &str, name: &str, state: &str) {
    let content = format!("{code:<5} {name:<38} {state:>12}");
    if color_enabled() {
        let clipped = clip(&content, PANEL_WIDTH);
        println!("{CYAN}│{RESET}{clipped:<width$}{CYAN}│{RESET}", width = PANEL_WIDTH);
    } else {
        line(&content);
    }
}

fn framed(content: &str) {
    if color_enabled() {
        println!("{CYAN}│{RESET}{content}{CYAN}│{RESET}");
    } else {
        println!("|{content}|");
    }
}

fn panel_line(config: BootConfig, label: &str, width: usize) -> String {
    let fill = width.saturating_sub(label.chars().count() + 4);
    if config.color && !config.ascii_mode {
        format!("{CYAN}-- {label} {}{RESET}", "─".repeat(fill))
    } else {
        format!("-- {label} {}", "-".repeat(fill))
    }
}

fn rainbow(idx: usize, text: &str, config: BootConfig) -> String {
    if !config.color || config.ascii_mode {
        return text.to_string();
    }
    let colors = [RED, YELLOW, GREEN, CYAN, BLUE, MAGENTA];
    format!("{}{}{}{}", BOLD, colors[idx % colors.len()], text, RESET)
}

fn accent(config: BootConfig, text: &str) -> String {
    if config.color && !config.ascii_mode {
        format!("{GREEN}{text}{RESET}")
    } else {
        text.to_string()
    }
}

fn value(config: BootConfig, text: &str) -> String {
    if config.color && !config.ascii_mode {
        format!("{GRAY}{text}{RESET}")
    } else {
        text.to_string()
    }
}

fn flag(config: BootConfig, enabled: bool) -> String {
    let label = if enabled { "on" } else { "off" };
    if !config.color || config.ascii_mode {
        return label.to_string();
    }
    if enabled {
        format!("{GREEN}{label}{RESET}")
    } else {
        format!("{GRAY}{label}{RESET}")
    }
}

fn pause(message: &str) {
    println!("{message}");
    let _ = io::stdout().flush();
    let mut ignored = String::new();
    let _ = io::stdin().read_line(&mut ignored);
}

fn preboot_width() -> usize {
    terminal_width().clamp(32, MOBILE_WIDTH)
}

fn terminal_width() -> usize {
    std::env::var("COLUMNS").ok().and_then(|raw| raw.parse().ok()).unwrap_or(MOBILE_WIDTH)
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

fn env_flag(name: &str) -> Option<bool> {
    std::env::var(name).ok().and_then(|value| parse_bool(&value))
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn clip(text: &str, width: usize) -> String {
    text.chars().take(width).collect()
}

fn color_enabled() -> bool {
    std::env::var_os("NO_COLOR").is_none() && std::env::var("PHASE1_NO_COLOR").ok().as_deref() != Some("1")
}

fn ascii_mode() -> bool {
    std::env::var("PHASE1_ASCII").ok().as_deref() == Some("1")
}

#[cfg(test)]
mod tests {
    use super::{clip, parse_bool, phase1_art, prompt_text, BootConfig, PANEL_WIDTH};

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
    fn boot_profile_names_cover_modes() {
        assert_eq!(
            BootConfig { color: true, ascii_mode: false, safe_mode: false, quick_boot: false, mobile_mode: false, persistent_state: false }.profile_name(),
            "operator"
        );
        assert_eq!(
            BootConfig { color: true, ascii_mode: false, safe_mode: true, quick_boot: true, mobile_mode: true, persistent_state: true }.profile_name(),
            "mobile-safe+quick"
        );
    }

    #[test]
    fn phase1_art_is_mobile_width() {
        assert!(phase1_art().iter().all(|line| line.chars().count() <= 40));
    }

    #[test]
    fn parse_bool_accepts_config_values() {
        assert_eq!(parse_bool("true"), Some(true));
        assert_eq!(parse_bool("off"), Some(false));
        assert_eq!(parse_bool("maybe"), None);
    }
}
