use crate::registry;
use std::io::{self, Write};

const PANEL_WIDTH: usize = 62;
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
pub struct BootConfig {
    pub color: bool,
    pub ascii_mode: bool,
    pub safe_mode: bool,
    pub quick_boot: bool,
}

impl Default for BootConfig {
    fn default() -> Self {
        Self {
            color: color_enabled(),
            ascii_mode: ascii_mode(),
            safe_mode: std::env::var("PHASE1_SAFE_MODE").ok().as_deref() == Some("1"),
            quick_boot: std::env::var("PHASE1_QUICK_BOOT").ok().as_deref() == Some("1"),
        }
    }
}

impl BootConfig {
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
    }

    pub fn profile_name(self) -> &'static str {
        match (self.safe_mode, self.quick_boot) {
            (true, true) => "safe+quick",
            (true, false) => "safe",
            (false, true) => "quick",
            (false, false) => "operator",
        }
    }
}

pub fn configure_boot(version: &str) -> BootConfig {
    let mut config = BootConfig::default();
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print_preboot(version, config);
        print!("boot> ");
        let _ = io::stdout().flush();

        input.clear();
        match stdin.read_line(&mut input) {
            Ok(0) | Err(_) => return config,
            Ok(_) => {}
        }

        match input.trim().to_ascii_lowercase().as_str() {
            "" | "1" | "b" | "boot" | "start" => return config,
            "2" | "c" | "color" | "colour" => config.color = !config.color,
            "3" | "a" | "ascii" => config.ascii_mode = !config.ascii_mode,
            "4" | "s" | "safe" | "safe-mode" => config.safe_mode = !config.safe_mode,
            "5" | "q" | "quick" | "quick-boot" => config.quick_boot = !config.quick_boot,
            "0" | "r" | "reset" => config = BootConfig::default(),
            "h" | "help" | "?" => pause("Toggle options, then press Enter or choose 1 to boot."),
            _ => pause("Unknown boot option. Press Enter to continue."),
        }
    }
}

pub fn print_boot(version: &str) {
    if ascii_mode() {
        print_ascii_boot(version);
    } else {
        print_modern_boot(version);
    }
}

pub fn print_quick_boot(version: &str, config: BootConfig) {
    print_fastfetch_splash(version, config);
    println!("[quick] boot matrix skipped :: profile={} :: shell armed", config.profile_name());
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
    println!("{}", panel_line(config, "BOOT OPTIONS"));
    println!("  1  boot system       {}", value(config, "ready"));
    println!("  2  color output      {}", flag(config, config.color));
    println!("  3  ascii compatible  {}", flag(config, config.ascii_mode));
    println!("  4  safe mode         {}", flag(config, config.safe_mode));
    println!("  5  quick boot        {}", flag(config, config.quick_boot));
    println!("  0  reset defaults");
    println!();
    println!("{}", value(config, "Enter=boot  h=help  toggle by number or name"));
}

fn print_fastfetch_splash(version: &str, config: BootConfig) {
    print_fastfetch_splash_with_config(version, config);
}

fn print_fastfetch_splash_with_config(version: &str, config: BootConfig) {
    let art = [
        "        __                 ___ ",
        "   ___ / /  ___ ____ ___  <  / ",
        "  / _ \\ _ \\/ _ `(_-</ -_) / /  ",
        " / .__/_.__/\\_,_/___/\\__/ /_/   ",
        "/_/                             ",
    ];
    let info = [
        format!("os        phase1 terminal-os-sim v{version}"),
        format!("profile   {}", config.profile_name()),
        format!("display   {}", if config.color { "retro rainbow" } else { "mono" }),
        format!("charset   {}", if config.ascii_mode { "ascii" } else { "unicode" }),
        format!("guards    {}", if config.safe_mode { "host integrations locked" } else { "standard audited" }),
    ];

    println!();
    for (idx, line) in art.iter().enumerate() {
        let art_line = rainbow(idx, line, config);
        let info_line = info.get(idx).map(String::as_str).unwrap_or("");
        println!("{art_line}  {}", value(config, info_line));
    }
    println!("{}", value(config, "      ▀▀ fastfetch boot selector // cyberdeck ready ▀▀"));
    println!();
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
    line("integrity=nominal  shell=registry-backed  ui=mobile-aware");
    mid("QUICK ACTIONS");
    line("help        complete p      audit        ps        ls /");
    line("man browser browser phase1  ifconfig     tree      version");
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
    println!("| help  complete p  audit  ps  ls /  browser phase1            |");
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

fn panel_line(config: BootConfig, label: &str) -> String {
    if config.color && !config.ascii_mode {
        format!("{CYAN}── {label} {}{RESET}", "─".repeat(42))
    } else {
        format!("-- {label} {}", "-".repeat(42))
    }
}

fn rainbow(idx: usize, text: &str, config: BootConfig) -> String {
    if !config.color || config.ascii_mode {
        return text.to_string();
    }
    let colors = [RED, YELLOW, GREEN, CYAN, BLUE, MAGENTA];
    format!("{}{}{}{}", BOLD, colors[idx % colors.len()], text, RESET)
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
    use super::{clip, prompt_text, BootConfig, PANEL_WIDTH};

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
        assert_eq!(BootConfig { color: true, ascii_mode: false, safe_mode: false, quick_boot: false }.profile_name(), "operator");
        assert_eq!(BootConfig { color: true, ascii_mode: false, safe_mode: true, quick_boot: true }.profile_name(), "safe+quick");
    }
}
