use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::autocomplete::{self, TabCompletion};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";

pub fn read_shell_line(prompt: &str) -> io::Result<Option<String>> {
    if std::env::var_os("PHASE1_COOKED_INPUT").is_some() {
        return read_cooked_line(prompt);
    }

    let Some(_guard) = RawModeGuard::enter() else {
        return read_cooked_line(prompt);
    };

    let mut stdout = io::stdout();
    let mut stdin = io::stdin().lock();
    let mut line = String::with_capacity(256);
    let mut bytes = [0_u8; 1];
    let mut last_status = command_status_line(&line);

    redraw_dynamic(prompt, &line, &mut stdout)?;

    loop {
        match stdin.read(&mut bytes) {
            Ok(0) => {
                let status = command_status_line(&line);
                if status != last_status {
                    redraw_dynamic(prompt, &line, &mut stdout)?;
                    last_status = status;
                }
                continue;
            }
            Ok(_) => {}
            Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(err) => return Err(err),
        }

        match bytes[0] {
            b'\r' | b'\n' => {
                stdout.write_all(b"\r\n")?;
                stdout.flush()?;
                return Ok(Some(line));
            }
            b'\t' => handle_tab(prompt, &mut line, &mut stdout)?,
            0x7f | 0x08 => {
                if !line.is_empty() {
                    line.pop();
                    redraw_dynamic(prompt, &line, &mut stdout)?;
                    last_status = command_status_line(&line);
                }
            }
            0x03 => {
                stdout.write_all(b"^C\r\n")?;
                stdout.flush()?;
                line.clear();
                return Ok(Some(line));
            }
            0x04 => {
                if line.is_empty() {
                    stdout.write_all(b"\r\n")?;
                    stdout.flush()?;
                    return Ok(None);
                }
            }
            0x1b => {}
            byte if byte.is_ascii_control() => {}
            byte => {
                line.push(byte as char);
                redraw_dynamic(prompt, &line, &mut stdout)?;
                last_status = command_status_line(&line);
            }
        }
    }
}

fn read_cooked_line(prompt: &str) -> io::Result<Option<String>> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => Ok(None),
        Ok(_) => {
            let line = input.trim_end_matches(['\r', '\n']);
            if line.contains('\t') {
                return Ok(Some(complete_cooked_line(line, prompt)?));
            }
            Ok(Some(line.to_string()))
        }
        Err(err) => Err(err),
    }
}

fn complete_cooked_line(line: &str, prompt: &str) -> io::Result<String> {
    match autocomplete::complete_tab_line(line) {
        TabCompletion::Unchanged(line) => Ok(line),
        TabCompletion::Completed(line) => {
            println!("tab complete: {line}");
            Ok(line)
        }
        TabCompletion::Suggestions { prefix, matches } => {
            println!("tab matches for '{prefix}': {}", matches.join(" "));
            print!("{prompt}");
            io::stdout().flush()?;
            Ok(String::new())
        }
        TabCompletion::NoMatch { prefix } => {
            println!("tab complete: no matches for '{prefix}'");
            print!("{prompt}");
            io::stdout().flush()?;
            Ok(String::new())
        }
    }
}

fn handle_tab(prompt: &str, line: &mut String, stdout: &mut io::Stdout) -> io::Result<()> {
    match autocomplete::complete_input_prefix(line) {
        TabCompletion::Unchanged(_) => {}
        TabCompletion::Completed(completed) => {
            *line = completed;
            redraw_dynamic(prompt, line, stdout)?;
        }
        TabCompletion::Suggestions { prefix, matches } => {
            stdout.write_all(b"\r\n")?;
            writeln!(stdout, "tab matches for '{prefix}': {}", matches.join(" "))?;
            write!(stdout, "{}{}{line}", command_status_line(line), prompt)?;
            stdout.flush()?;
        }
        TabCompletion::NoMatch { prefix } => {
            stdout.write_all(b"\r\n")?;
            writeln!(stdout, "tab complete: no matches for '{prefix}'")?;
            write!(stdout, "{}{}{line}", command_status_line(line), prompt)?;
            stdout.flush()?;
        }
    }
    Ok(())
}

fn redraw_dynamic(prompt: &str, line: &str, stdout: &mut io::Stdout) -> io::Result<()> {
    write!(
        stdout,
        "\r\x1b[2K\x1b[1A\r\x1b[2K{}{}{line}",
        command_status_line(line),
        prompt
    )?;
    stdout.flush()
}

fn command_status_line(input: &str) -> String {
    let width = terminal_width().clamp(32, 72);
    let raw = format!(
        "HUD {} | {}",
        short_clock_utc(),
        command_hint(input)
    );
    let clipped = clip(&raw, width);
    let padded = format!("{clipped}{}", " ".repeat(width.saturating_sub(clipped.len())));

    if color_enabled() {
        let color = match first_word(input) {
            Some("avim") => MAGENTA,
            Some("lang" | "python" | "gcc" | "wasm") => CYAN,
            Some("rm" | "kill" | "update") => YELLOW,
            Some("security" | "audit" | "bootcfg") => BLUE,
            Some("exit" | "shutdown" | "reboot") => RED,
            _ => GREEN,
        };
        format!("{BOLD}{color}{padded}{RESET}\r\n")
    } else {
        format!("{padded}\r\n")
    }
}

fn command_hint(input: &str) -> String {
    let trimmed = input.trim_start();
    let Some(cmd) = first_word(trimmed) else {
        return "ready | type command | Tab completes".to_string();
    };

    match cmd {
        "avim" => "avim | Esc=NORMAL | i=INSERT | :wq=save".to_string(),
        "lang" => "lang | run/status/detect | host tools guarded".to_string(),
        "python" | "gcc" => format!("{cmd} | safe-mode blocks host execution"),
        "wasm" => "wasm | sandboxed plugins | list/run/inspect".to_string(),
        "ls" | "cd" | "pwd" | "cat" | "tree" => "fs | VFS navigation/read".to_string(),
        "mkdir" | "touch" | "rm" | "cp" | "mv" | "echo" => "fs write | VFS mutation".to_string(),
        "grep" | "wc" | "head" | "tail" | "find" | "pipeline" => {
            "text | filters/search pipelines".to_string()
        }
        "ps" | "top" | "spawn" | "jobs" | "fg" | "bg" | "kill" | "nice" => {
            "proc | simulated scheduler".to_string()
        }
        "ifconfig" | "iwconfig" | "wifi-scan" | "wifi-connect" | "ping" | "nmcli" => {
            "net | safe loopback by default".to_string()
        }
        "dash" => "dash | system dashboard | --compact".to_string(),
        "matrix" => "matrix | visual rain | Ctrl-C exits".to_string(),
        "theme" => "theme | list/set palettes".to_string(),
        "security" => "security | guard report".to_string(),
        "audit" => "audit | kernel event log".to_string(),
        "bootcfg" => "bootcfg | show/save/reset boot profile".to_string(),
        "update" => "update | plan/test guarded workflow".to_string(),
        "history" => "history | sanitized command log".to_string(),
        "fastfetch" | "sysinfo" => "sysinfo | simulated machine report".to_string(),
        "clear" => "clear | redraw terminal".to_string(),
        "help" | "man" | "complete" | "capabilities" => "help | docs/completion/capabilities".to_string(),
        "exit" | "shutdown" => "exit | shutdown Phase1 shell".to_string(),
        "reboot" => "reboot | return to boot dock".to_string(),
        other => format!("{other} | Enter runs | Tab completes"),
    }
}

fn first_word(input: &str) -> Option<&str> {
    input.split_whitespace().next()
}

fn terminal_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(40)
}

fn clip(text: &str, width: usize) -> String {
    text.chars().take(width).collect()
}

fn short_clock_utc() -> String {
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

fn color_enabled() -> bool {
    std::env::var_os("NO_COLOR").is_none()
        && std::env::var("PHASE1_NO_COLOR").ok().as_deref() != Some("1")
}

struct RawModeGuard {
    original: Option<String>,
}

impl RawModeGuard {
    fn enter() -> Option<Self> {
        let original = stty(&["-g"]).ok().filter(|raw| !raw.trim().is_empty())?;
        if stty(&["raw", "-echo", "-icanon", "min", "0", "time", "10"]).is_err() {
            return None;
        }
        Some(Self {
            original: Some(original.trim().to_string()),
        })
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        if let Some(original) = self.original.take() {
            let _ = stty(&[&original]);
        }
    }
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
    use super::{command_hint, command_status_line};

    #[test]
    fn avim_status_explains_modes() {
        let hint = command_hint("avim hello.py");
        assert!(hint.contains("Esc=NORMAL"));
        assert!(hint.contains(":wq"));
    }

    #[test]
    fn command_status_has_clock_and_stays_one_line() {
        std::env::set_var("NO_COLOR", "1");
        std::env::set_var("COLUMNS", "40");
        let status = command_status_line("lang run python hi.py");
        assert!(status.contains("HUD"));
        assert!(status.contains("UTC"));
        assert_eq!(status.lines().count(), 1);
        std::env::remove_var("NO_COLOR");
        std::env::remove_var("COLUMNS");
    }
}
