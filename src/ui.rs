use crate::registry;

const PANEL_WIDTH: usize = 62;
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const GRAY: &str = "\x1b[90m";

pub fn print_boot(version: &str) {
    if ascii_mode() {
        print_ascii_boot(version);
    } else {
        print_modern_boot(version);
    }
}

pub fn print_help() {
    print!("{}", registry::command_map());
}

pub fn print_prompt(user: &str, path: &str) {
    print!("{}", prompt_text(user, path));
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
    use super::{clip, prompt_text, PANEL_WIDTH};

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
}
