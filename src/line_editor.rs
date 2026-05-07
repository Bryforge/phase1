use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::autocomplete::{self, TabCompletion};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const HISTORY_LIMIT: usize = 200;
const IDLE_ENTER_GUARD_DEFAULT_SECS: u64 = 30;

static SESSION_HISTORY: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

#[derive(Clone, Debug, Eq, PartialEq)]
struct EditorState {
    line: String,
    cursor: usize,
    history: Vec<String>,
    history_index: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EscapeKey {
    Escape,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    Delete,
    Unknown,
}

pub fn read_shell_line(prompt: &str) -> io::Result<Option<String>> {
    if std::env::var_os("PHASE1_COOKED_INPUT").is_some() {
        let line = read_cooked_line(prompt)?;
        if let Some(line) = &line {
            push_session_history(line);
        }
        return Ok(line);
    }

    let Some(_guard) = RawModeGuard::enter() else {
        let line = read_cooked_line(prompt)?;
        if let Some(line) = &line {
            push_session_history(line);
        }
        return Ok(line);
    };

    let mut stdout = io::stdout();
    let mut stdin = io::stdin().lock();
    let mut bytes = [0_u8; 1];
    let mut editor = EditorState {
        line: String::with_capacity(256),
        cursor: 0,
        history: session_history_snapshot(),
        history_index: None,
    };
    let mut last_status = command_status_line(&editor.line);
    let mut last_input = Instant::now();

    init_frame(prompt, &editor, &mut stdout)?;

    loop {
        match stdin.read(&mut bytes) {
            Ok(0) => {
                let status = command_status_line(&editor.line);
                if status != last_status {
                    redraw_frame(prompt, &editor, &mut stdout)?;
                    last_status = status;
                }
                continue;
            }
            Ok(_) => {}
            Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(err) => return Err(err),
        }

        let byte = bytes[0];
        let idle_for = last_input.elapsed();
        if matches!(byte, b'\r' | b'\n') && idle_enter_guard_triggered(idle_for) {
            clear_status_and_newline(&mut stdout)?;
            writeln!(
                stdout,
                "idle-enter guard: ignored Enter after {}s idle; press Enter again to run",
                idle_for.as_secs()
            )?;
            redraw_frame(prompt, &editor, &mut stdout)?;
            last_status = command_status_line(&editor.line);
            last_input = Instant::now();
            continue;
        }
        last_input = Instant::now();

        match byte {
            b'\r' | b'\n' => {
                finish_frame(prompt, &editor, &mut stdout)?;
                push_session_history(&editor.line);
                return Ok(Some(editor.line));
            }
            b'\t' => {
                handle_tab(prompt, &mut editor, &mut stdout)?;
                last_status = command_status_line(&editor.line);
            }
            0x7f | 0x08 => {
                if delete_before_cursor(&mut editor.line, &mut editor.cursor) {
                    editor.history_index = None;
                    redraw_frame(prompt, &editor, &mut stdout)?;
                    last_status = command_status_line(&editor.line);
                }
            }
            0x03 => {
                clear_status_and_newline(&mut stdout)?;
                stdout.write_all(b"^C\r\n")?;
                stdout.flush()?;
                return Ok(Some(String::new()));
            }
            0x04 => {
                if editor.line.is_empty() {
                    finish_frame(prompt, &editor, &mut stdout)?;
                    return Ok(None);
                }
            }
            0x01 => {
                editor.cursor = 0;
                redraw_frame(prompt, &editor, &mut stdout)?;
            }
            0x05 => {
                editor.cursor = char_len(&editor.line);
                redraw_frame(prompt, &editor, &mut stdout)?;
            }
            0x0b => {
                kill_to_end(&mut editor.line, editor.cursor);
                editor.history_index = None;
                redraw_frame(prompt, &editor, &mut stdout)?;
                last_status = command_status_line(&editor.line);
            }
            0x0c => {
                stdout.write_all(b"\x1b[2J\x1b[H")?;
                redraw_frame(prompt, &editor, &mut stdout)?;
            }
            0x15 => {
                editor.line.clear();
                editor.cursor = 0;
                editor.history_index = None;
                redraw_frame(prompt, &editor, &mut stdout)?;
                last_status = command_status_line(&editor.line);
            }
            0x17 => {
                delete_previous_word(&mut editor.line, &mut editor.cursor);
                editor.history_index = None;
                redraw_frame(prompt, &editor, &mut stdout)?;
                last_status = command_status_line(&editor.line);
            }
            0x1b => {
                handle_escape_key(prompt, &mut editor, &mut stdin, &mut stdout)?;
                last_status = command_status_line(&editor.line);
            }
            byte if byte.is_ascii_control() => {}
            byte if byte.is_ascii() => {
                insert_char(&mut editor.line, &mut editor.cursor, byte as char);
                editor.history_index = None;
                redraw_frame(prompt, &editor, &mut stdout)?;
                last_status = command_status_line(&editor.line);
            }
            _ => {}
        }
    }
}

fn read_cooked_line(prompt: &str) -> io::Result<Option<String>> {
    print!("{prompt}");
    io::stdout().flush()?;
    let prompt_started = Instant::now();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => Ok(None),
        Ok(_) => {
            let line = input.trim_end_matches(['\r', '\n']);
            if line.trim().is_empty() && idle_enter_guard_triggered(prompt_started.elapsed()) {
                println!(
                    "idle-enter guard: ignored blank Enter after {}s idle; press Enter again to run",
                    prompt_started.elapsed().as_secs()
                );
                return Ok(Some(String::new()));
            }
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

fn handle_tab(prompt: &str, editor: &mut EditorState, stdout: &mut io::Stdout) -> io::Result<()> {
    match autocomplete::complete_input_prefix(&editor.line) {
        TabCompletion::Unchanged(_) => {}
        TabCompletion::Completed(completed) => {
            editor.line = completed;
            editor.cursor = char_len(&editor.line);
            editor.history_index = None;
            redraw_frame(prompt, editor, stdout)?;
        }
        TabCompletion::Suggestions { prefix, matches } => {
            clear_status_and_newline(stdout)?;
            writeln!(stdout, "tab matches for '{prefix}': {}", matches.join(" "))?;
            redraw_frame(prompt, editor, stdout)?;
        }
        TabCompletion::NoMatch { prefix } => {
            clear_status_and_newline(stdout)?;
            writeln!(stdout, "tab complete: no matches for '{prefix}'")?;
            redraw_frame(prompt, editor, stdout)?;
        }
    }
    Ok(())
}

fn init_frame(prompt: &str, editor: &EditorState, stdout: &mut io::Stdout) -> io::Result<()> {
    write!(stdout, "\r\x1b[2K\x1b[1A\r\x1b[2K")?;
    redraw_frame(prompt, editor, stdout)
}

fn redraw_frame(prompt: &str, editor: &EditorState, stdout: &mut io::Stdout) -> io::Result<()> {
    write!(
        stdout,
        "\r\x1b[2K{}{}\r\n\x1b[2K{}\x1b[1A\r",
        prompt,
        editor.line,
        command_status_line(&editor.line)
    )?;
    move_cursor_to_prompt_position(prompt, editor, stdout)?;
    stdout.flush()
}

fn finish_frame(prompt: &str, editor: &EditorState, stdout: &mut io::Stdout) -> io::Result<()> {
    write!(stdout, "\r\x1b[2K{}{}\x1b[1B\r\x1b[2K", prompt, editor.line)?;
    stdout.flush()
}

fn clear_status_and_newline(stdout: &mut io::Stdout) -> io::Result<()> {
    write!(stdout, "\x1b[1B\r\x1b[2K\r\n")?;
    stdout.flush()
}

fn move_cursor_to_prompt_position(
    prompt: &str,
    editor: &EditorState,
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    let column = visible_len(prompt) + editor.cursor;
    if column > 0 {
        write!(stdout, "\x1b[{column}C")?;
    }
    Ok(())
}

fn handle_escape_key<R: Read>(
    prompt: &str,
    editor: &mut EditorState,
    stdin: &mut R,
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    match read_escape_key(stdin)? {
        EscapeKey::Up => history_up(editor),
        EscapeKey::Down => history_down(editor),
        EscapeKey::Left => {
            editor.cursor = editor.cursor.saturating_sub(1);
        }
        EscapeKey::Right => {
            editor.cursor = (editor.cursor + 1).min(char_len(&editor.line));
        }
        EscapeKey::Home => editor.cursor = 0,
        EscapeKey::End => editor.cursor = char_len(&editor.line),
        EscapeKey::Delete => {
            let _ = delete_at_cursor(&mut editor.line, editor.cursor);
        }
        EscapeKey::Escape | EscapeKey::Unknown => {}
    }
    redraw_frame(prompt, editor, stdout)
}

fn read_escape_key<R: Read>(stdin: &mut R) -> io::Result<EscapeKey> {
    let mut seq = [0_u8; 1];
    if stdin.read(&mut seq)? == 0 {
        return Ok(EscapeKey::Escape);
    }
    match seq[0] {
        b'[' => {
            if stdin.read(&mut seq)? == 0 {
                return Ok(EscapeKey::Unknown);
            }
            match seq[0] {
                b'A' => Ok(EscapeKey::Up),
                b'B' => Ok(EscapeKey::Down),
                b'C' => Ok(EscapeKey::Right),
                b'D' => Ok(EscapeKey::Left),
                b'H' => Ok(EscapeKey::Home),
                b'F' => Ok(EscapeKey::End),
                b'3' => {
                    let _ = stdin.read(&mut seq)?;
                    Ok(EscapeKey::Delete)
                }
                _ => Ok(EscapeKey::Unknown),
            }
        }
        b'O' => {
            if stdin.read(&mut seq)? == 0 {
                return Ok(EscapeKey::Unknown);
            }
            match seq[0] {
                b'H' => Ok(EscapeKey::Home),
                b'F' => Ok(EscapeKey::End),
                _ => Ok(EscapeKey::Unknown),
            }
        }
        _ => Ok(EscapeKey::Unknown),
    }
}

fn history_up(editor: &mut EditorState) {
    if editor.history.is_empty() {
        return;
    }
    let next = match editor.history_index {
        Some(idx) => idx.saturating_sub(1),
        None => editor.history.len().saturating_sub(1),
    };
    editor.history_index = Some(next);
    editor.line = editor.history[next].clone();
    editor.cursor = char_len(&editor.line);
}

fn history_down(editor: &mut EditorState) {
    let Some(idx) = editor.history_index else {
        return;
    };
    if idx + 1 < editor.history.len() {
        let next = idx + 1;
        editor.history_index = Some(next);
        editor.line = editor.history[next].clone();
    } else {
        editor.history_index = None;
        editor.line.clear();
    }
    editor.cursor = char_len(&editor.line);
}

fn insert_char(line: &mut String, cursor: &mut usize, ch: char) {
    let idx = byte_index_for_char(line, *cursor);
    line.insert(idx, ch);
    *cursor += 1;
}

fn delete_before_cursor(line: &mut String, cursor: &mut usize) -> bool {
    if *cursor == 0 {
        return false;
    }
    *cursor -= 1;
    delete_at_cursor(line, *cursor)
}

fn delete_at_cursor(line: &mut String, cursor: usize) -> bool {
    if cursor >= char_len(line) {
        return false;
    }
    let start = byte_index_for_char(line, cursor);
    let end = byte_index_for_char(line, cursor + 1);
    line.replace_range(start..end, "");
    true
}

fn kill_to_end(line: &mut String, cursor: usize) {
    let start = byte_index_for_char(line, cursor);
    line.truncate(start);
}

fn delete_previous_word(line: &mut String, cursor: &mut usize) {
    while *cursor > 0 && char_at(line, *cursor - 1).is_some_and(char::is_whitespace) {
        delete_before_cursor(line, cursor);
    }
    while *cursor > 0 && char_at(line, *cursor - 1).is_some_and(|ch| !ch.is_whitespace()) {
        delete_before_cursor(line, cursor);
    }
}

fn char_at(line: &str, idx: usize) -> Option<char> {
    line.chars().nth(idx)
}

fn char_len(line: &str) -> usize {
    line.chars().count()
}

fn byte_index_for_char(line: &str, char_idx: usize) -> usize {
    line.char_indices()
        .nth(char_idx)
        .map(|(idx, _)| idx)
        .unwrap_or(line.len())
}

fn session_history() -> &'static Mutex<Vec<String>> {
    SESSION_HISTORY.get_or_init(|| Mutex::new(Vec::new()))
}

fn session_history_snapshot() -> Vec<String> {
    session_history()
        .lock()
        .map(|history| history.clone())
        .unwrap_or_default()
}

fn push_session_history(line: &str) {
    let line = line.trim_end();
    if line.trim().is_empty() {
        return;
    }
    if let Ok(mut history) = session_history().lock() {
        if history.last().is_some_and(|last| last == line) {
            return;
        }
        history.push(line.to_string());
        if history.len() > HISTORY_LIMIT {
            let overflow = history.len() - HISTORY_LIMIT;
            history.drain(0..overflow);
        }
    }
}

fn idle_enter_guard_triggered(idle_for: Duration) -> bool {
    let Some(threshold) = idle_enter_guard_duration() else {
        return false;
    };
    idle_for >= threshold
}

fn idle_enter_guard_duration() -> Option<Duration> {
    let seconds = std::env::var("PHASE1_IDLE_ENTER_GUARD_SECONDS")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .unwrap_or(IDLE_ENTER_GUARD_DEFAULT_SECS);
    if seconds == 0 {
        None
    } else {
        Some(Duration::from_secs(seconds))
    }
}

fn command_status_line(input: &str) -> String {
    let width = terminal_width().clamp(32, 72);
    let raw = format!("HUD {} | {}", short_clock_utc(), command_hint(input));
    let clipped = clip(&raw, width);
    let padded = format!(
        "{clipped}{}",
        " ".repeat(width.saturating_sub(clipped.len()))
    );

    if color_enabled() {
        let color = match first_word(input) {
            Some("avim") => MAGENTA,
            Some("lang" | "python" | "gcc" | "wasm") => CYAN,
            Some("rm" | "kill" | "update") => YELLOW,
            Some("security" | "audit" | "bootcfg") => BLUE,
            Some("exit" | "shutdown" | "reboot") => RED,
            _ => GREEN,
        };
        format!("{BOLD}{color}{padded}{RESET}")
    } else {
        padded
    }
}

fn command_hint(input: &str) -> String {
    let trimmed = input.trim_start();
    let Some(cmd) = first_word(trimmed) else {
        return "ready | Tab completes | ↑ history | Ctrl-L clear".to_string();
    };

    match cmd {
        "avim" => "avim | Esc=NORMAL | i=INSERT | :wq=save".to_string(),
        "lang" => "lang | run/status/detect | host tools guarded".to_string(),
        "python" | "gcc" => format!("{cmd} | shield off + trust host required"),
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
        "update" => "update | boot trust + --trust-host required".to_string(),
        "history" => "history | sanitized command log".to_string(),
        "fastfetch" | "sysinfo" => "sysinfo | simulated machine report".to_string(),
        "clear" => "clear | redraw terminal".to_string(),
        "help" | "man" | "complete" | "capabilities" => {
            "help | docs/completion/capabilities".to_string()
        }
        "exit" | "shutdown" => "exit | shutdown Phase1 shell".to_string(),
        "reboot" => "reboot | return to boot dock".to_string(),
        other => format!("{other} | Enter runs | Tab completes | ↑ history"),
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
    use super::{
        char_len, command_hint, command_status_line, delete_at_cursor, delete_before_cursor,
        delete_previous_word, history_down, history_up, idle_enter_guard_triggered, insert_char,
        EditorState,
    };
    use std::time::Duration;

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

    #[test]
    fn editing_keys_modify_line_at_cursor() {
        let mut line = "cat radme.txt".to_string();
        let mut cursor = 5;
        insert_char(&mut line, &mut cursor, 'e');
        assert_eq!(line, "cat readme.txt");
        assert_eq!(cursor, 6);
        assert!(delete_before_cursor(&mut line, &mut cursor));
        assert_eq!(line, "cat radme.txt");
        assert!(delete_at_cursor(&mut line, cursor));
        assert_eq!(line, "cat rdme.txt");
    }

    #[test]
    fn ctrl_w_deletes_previous_word() {
        let mut line = "python hello.py".to_string();
        let mut cursor = char_len(&line);
        delete_previous_word(&mut line, &mut cursor);
        assert_eq!(line, "python ");
        assert_eq!(cursor, 7);
    }

    #[test]
    fn history_navigation_recalls_last_command() {
        let mut editor = EditorState {
            line: String::new(),
            cursor: 0,
            history: vec!["ls".to_string(), "cat readme.txt".to_string()],
            history_index: None,
        };
        history_up(&mut editor);
        assert_eq!(editor.line, "cat readme.txt");
        history_up(&mut editor);
        assert_eq!(editor.line, "ls");
        history_down(&mut editor);
        assert_eq!(editor.line, "cat readme.txt");
    }

    #[test]
    fn idle_enter_guard_blocks_stale_enter_and_can_be_disabled() {
        std::env::remove_var("PHASE1_IDLE_ENTER_GUARD_SECONDS");
        assert!(!idle_enter_guard_triggered(Duration::from_secs(29)));
        assert!(idle_enter_guard_triggered(Duration::from_secs(30)));

        std::env::set_var("PHASE1_IDLE_ENTER_GUARD_SECONDS", "0");
        assert!(!idle_enter_guard_triggered(Duration::from_secs(999)));

        std::env::set_var("PHASE1_IDLE_ENTER_GUARD_SECONDS", "5");
        assert!(!idle_enter_guard_triggered(Duration::from_secs(4)));
        assert!(idle_enter_guard_triggered(Duration::from_secs(5)));
        std::env::remove_var("PHASE1_IDLE_ENTER_GUARD_SECONDS");
    }
}
