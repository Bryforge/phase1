use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

use crate::autocomplete::{self, TabCompletion};

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

    loop {
        match stdin.read(&mut bytes) {
            Ok(0) => return Ok(None),
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
                    redraw(prompt, &line, &mut stdout)?;
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
                stdout.write_all(&[byte])?;
                stdout.flush()?;
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
            redraw(prompt, line, stdout)?;
        }
        TabCompletion::Suggestions { prefix, matches } => {
            stdout.write_all(b"\r\n")?;
            writeln!(stdout, "tab matches for '{prefix}': {}", matches.join(" "))?;
            write!(stdout, "{prompt}{line}")?;
            stdout.flush()?;
        }
        TabCompletion::NoMatch { prefix } => {
            stdout.write_all(b"\r\n")?;
            writeln!(stdout, "tab complete: no matches for '{prefix}'")?;
            write!(stdout, "{prompt}{line}")?;
            stdout.flush()?;
        }
    }
    Ok(())
}

fn redraw(prompt: &str, line: &str, stdout: &mut io::Stdout) -> io::Result<()> {
    write!(stdout, "\r\x1b[2K{prompt}{line}")?;
    stdout.flush()
}

struct RawModeGuard {
    original: Option<String>,
}

impl RawModeGuard {
    fn enter() -> Option<Self> {
        let original = stty(&["-g"]).ok().filter(|raw| !raw.trim().is_empty())?;
        if stty(&["raw", "-echo", "-icanon", "min", "1", "time", "0"]).is_err() {
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
