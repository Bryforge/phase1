use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::kernel::Vfs;

const MAX_FILE_BYTES: usize = 256 * 1024;
const MAX_UNDO: usize = 64;
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Normal,
    Insert,
    Command,
}

#[derive(Debug)]
struct AvimState {
    filename: String,
    lines: Vec<String>,
    cursor: usize,
    mode: Mode,
    dirty: bool,
    show_numbers: bool,
    yank: Option<String>,
    last_search: Option<String>,
    undo: Vec<Vec<String>>,
}

enum AvimInput {
    Line(String),
    Escape,
    End,
}

pub fn edit(vfs: &mut Vfs, args: &[String]) {
    let Some(filename) = args.first() else {
        println!("usage: avim <file>");
        return;
    };
    if !safe_vfs_target(filename) {
        println!("avim: unsafe target path");
        return;
    }

    let existing = match vfs.cat(filename) {
        Ok(content) => content,
        Err(_) => String::new(),
    };
    if existing.len() > MAX_FILE_BYTES {
        println!("avim: refusing to edit file larger than {MAX_FILE_BYTES} bytes");
        return;
    }

    let mut state = AvimState {
        filename: filename.to_string(),
        lines: content_to_lines(&existing),
        cursor: 0,
        mode: Mode::Normal,
        dirty: false,
        show_numbers: true,
        yank: None,
        last_search: None,
        undo: Vec::new(),
    };

    println!("avim: advanced VFS modal editor // {}", state.filename);
    println!("mode keys: NORMAL i/a/o insert | INSERT Esc normal | NORMAL :wq save+quit | :help");
    render(&state);

    loop {
        let input = match read_avim_input(&state) {
            Ok(AvimInput::Line(input)) => input,
            Ok(AvimInput::Escape) => {
                escape_to_normal(&mut state);
                continue;
            }
            Ok(AvimInput::End) => return,
            Err(err) => {
                println!("avim: input error: {err}");
                return;
            }
        };

        match state.mode {
            Mode::Normal => {
                if handle_normal(vfs, &mut state, &input) {
                    return;
                }
            }
            Mode::Insert => handle_insert(&mut state, &input),
            Mode::Command => {
                state.mode = Mode::Normal;
                if handle_command(vfs, &mut state, &input) {
                    return;
                }
            }
        }
    }
}

fn handle_normal(vfs: &mut Vfs, state: &mut AvimState, input: &str) -> bool {
    match input {
        "" => render(state),
        raw if is_escape_input(raw) => render_status(state, "already in NORMAL mode"),
        "i" => {
            state.mode = Mode::Insert;
            render_status(
                state,
                "INSERT mode: type a line, Enter commits, Esc returns to NORMAL",
            );
        }
        "a" => {
            state.mode = Mode::Insert;
            render_status(
                state,
                "INSERT mode: type a line, Enter commits, Esc returns to NORMAL",
            );
        }
        "o" => {
            push_undo(state);
            let insert_at = (state.cursor + 1).min(state.lines.len());
            state.lines.insert(insert_at, String::new());
            state.cursor = insert_at;
            state.dirty = true;
            state.mode = Mode::Insert;
            render(state);
        }
        "O" => {
            push_undo(state);
            state.lines.insert(state.cursor, String::new());
            state.dirty = true;
            state.mode = Mode::Insert;
            render(state);
        }
        "h" | "l" => render_status(
            state,
            "character movement is line-oriented in this terminal build",
        ),
        "j" => move_cursor(state, 1),
        "k" => move_cursor(state, -1),
        "gg" => {
            state.cursor = 0;
            render(state);
        }
        "G" => {
            state.cursor = state.lines.len().saturating_sub(1);
            render(state);
        }
        "dd" => delete_line(state),
        "yy" => yank_line(state),
        "p" => paste_line(state),
        "u" => undo(state),
        "n" => repeat_search(state),
        ":" => {
            state.mode = Mode::Command;
            render_status(state, "COMMAND mode: w q q! wq help; Esc returns to NORMAL");
        }
        ":q" => return handle_command(vfs, state, "q"),
        ":q!" => return handle_command(vfs, state, "q!"),
        ":w" => return handle_command(vfs, state, "w"),
        ":wq" | ":x" => return handle_command(vfs, state, "wq"),
        ":help" => print_help(),
        raw if raw.starts_with(':') => return handle_command(vfs, state, &raw[1..]),
        raw if raw.starts_with('/') => search(state, raw.trim_start_matches('/')),
        raw if raw.starts_with("set ") => set_option(state, raw.trim_start_matches("set ")),
        other => render_status(
            state,
            &format!("unknown NORMAL command: {other}; use i for INSERT or :help"),
        ),
    }
    false
}

fn handle_insert(state: &mut AvimState, input: &str) {
    if is_escape_input(input) {
        escape_to_normal(state);
        return;
    }
    if matches!(input.trim(), ":w" | ":wq" | ":q" | ":q!" | ":x") {
        render_status(
            state,
            "still in INSERT mode; press Esc first, then run :wq or :q! from NORMAL mode",
        );
        return;
    }

    push_undo(state);
    if state.lines.is_empty() {
        state.lines.push(input.to_string());
        state.cursor = 0;
    } else {
        state.lines[state.cursor] = input.to_string();
        state.cursor = (state.cursor + 1).min(state.lines.len());
        if state.cursor == state.lines.len() {
            state.lines.push(String::new());
        }
    }
    state.dirty = true;
    render(state);
}

fn handle_command(vfs: &mut Vfs, state: &mut AvimState, command: &str) -> bool {
    let command = command.trim();
    if is_escape_input(command) {
        escape_to_normal(state);
        return false;
    }
    match command {
        "q" => {
            if state.dirty {
                render_status(state, "unsaved changes; use :wq to save or :q! to discard");
                false
            } else {
                println!("avim: closed {}", state.filename);
                true
            }
        }
        "q!" => {
            println!("avim: discarded changes to {}", state.filename);
            true
        }
        "w" => {
            save(vfs, state);
            false
        }
        "wq" | "x" => {
            save(vfs, state);
            !state.dirty
        }
        "help" | "h" => {
            print_help();
            false
        }
        "security" => {
            print_security();
            false
        }
        "set number" | "set nu" => {
            state.show_numbers = true;
            render(state);
            false
        }
        "set nonumber" | "set nonu" => {
            state.show_numbers = false;
            render(state);
            false
        }
        "u" | "undo" => {
            undo(state);
            false
        }
        raw if raw.starts_with("r ") => {
            read_file(vfs, state, raw.trim_start_matches("r ").trim());
            false
        }
        raw if raw.starts_with("e ") => {
            render_status(
                state,
                &format!(
                    "opening another file is disabled in this avim session: {}",
                    raw.trim_start_matches("e ").trim()
                ),
            );
            false
        }
        raw if raw.starts_with("%s/") => {
            substitute(state, raw);
            false
        }
        raw if raw.starts_with('/') => {
            search(state, raw.trim_start_matches('/'));
            false
        }
        other => {
            render_status(state, &format!("unknown command: :{other}"));
            false
        }
    }
}

fn save(vfs: &mut Vfs, state: &mut AvimState) {
    let content = lines_to_content(&state.lines);
    if content.len() > MAX_FILE_BYTES {
        println!("avim: save blocked; file would exceed {MAX_FILE_BYTES} bytes");
        return;
    }
    match vfs.write_file(&state.filename, &content, false) {
        Ok(()) => {
            state.dirty = false;
            println!("avim: wrote {} ({} bytes)", state.filename, content.len());
        }
        Err(err) => println!("avim: write failed: {err}"),
    }
}

fn move_cursor(state: &mut AvimState, delta: isize) {
    let len = state.lines.len().max(1);
    let next = if delta.is_negative() {
        state.cursor.saturating_sub(delta.unsigned_abs())
    } else {
        state.cursor.saturating_add(delta as usize).min(len - 1)
    };
    state.cursor = next;
    render(state);
}

fn delete_line(state: &mut AvimState) {
    push_undo(state);
    if state.lines.is_empty() {
        render_status(state, "nothing to delete");
        return;
    }
    let removed = state.lines.remove(state.cursor);
    state.yank = Some(removed);
    if state.lines.is_empty() {
        state.lines.push(String::new());
    }
    state.cursor = state.cursor.min(state.lines.len() - 1);
    state.dirty = true;
    render(state);
}

fn yank_line(state: &mut AvimState) {
    if let Some(line) = state.lines.get(state.cursor) {
        state.yank = Some(line.clone());
        render_status(state, "yanked 1 line");
    }
}

fn paste_line(state: &mut AvimState) {
    let Some(line) = state.yank.clone() else {
        render_status(state, "nothing yanked");
        return;
    };
    push_undo(state);
    let idx = (state.cursor + 1).min(state.lines.len());
    state.lines.insert(idx, line);
    state.cursor = idx;
    state.dirty = true;
    render(state);
}

fn undo(state: &mut AvimState) {
    match state.undo.pop() {
        Some(previous) => {
            state.lines = previous;
            state.cursor = state.cursor.min(state.lines.len().saturating_sub(1));
            state.dirty = true;
            render(state);
        }
        None => render_status(state, "nothing to undo"),
    }
}

fn search(state: &mut AvimState, needle: &str) {
    if needle.is_empty() {
        render_status(state, "empty search");
        return;
    }
    state.last_search = Some(needle.to_string());
    let start = (state.cursor + 1).min(state.lines.len());
    if let Some(idx) = find_from(state, needle, start).or_else(|| find_from(state, needle, 0)) {
        state.cursor = idx;
        render(state);
    } else {
        render_status(state, &format!("pattern not found: {needle}"));
    }
}

fn repeat_search(state: &mut AvimState) {
    match state.last_search.clone() {
        Some(needle) => search(state, &needle),
        None => render_status(state, "no previous search"),
    }
}

fn find_from(state: &AvimState, needle: &str, start: usize) -> Option<usize> {
    state
        .lines
        .iter()
        .enumerate()
        .skip(start)
        .find_map(|(idx, line)| line.contains(needle).then_some(idx))
}

fn substitute(state: &mut AvimState, raw: &str) {
    let parts = raw.trim_start_matches("%s/").split('/').collect::<Vec<_>>();
    if parts.len() < 2 {
        render_status(state, "usage: :%s/old/new/[g]");
        return;
    }
    let old = parts[0];
    let new = parts[1];
    if old.is_empty() {
        render_status(state, "substitute pattern must not be empty");
        return;
    }
    push_undo(state);
    let global = parts.get(2).is_some_and(|flags| flags.contains('g'));
    let mut changed = 0usize;
    for line in &mut state.lines {
        if line.contains(old) {
            let before = line.clone();
            *line = if global {
                line.replace(old, new)
            } else {
                line.replacen(old, new, 1)
            };
            if *line != before {
                changed += 1;
            }
        }
    }
    if changed > 0 {
        state.dirty = true;
    }
    render_status(state, &format!("substitute changed {changed} lines"));
}

fn read_file(vfs: &mut Vfs, state: &mut AvimState, path: &str) {
    if !safe_vfs_target(path) {
        render_status(state, "read blocked: unsafe VFS path");
        return;
    }
    match vfs.cat(path) {
        Ok(content) if content.len() <= MAX_FILE_BYTES => {
            push_undo(state);
            let mut incoming = content_to_lines(&content);
            let idx = (state.cursor + 1).min(state.lines.len());
            state.lines.splice(idx..idx, incoming.drain(..));
            state.dirty = true;
            render(state);
        }
        Ok(_) => render_status(state, "read blocked: file too large"),
        Err(err) => render_status(state, &format!("read failed: {err}")),
    }
}

fn set_option(state: &mut AvimState, option: &str) {
    match option {
        "number" | "nu" => state.show_numbers = true,
        "nonumber" | "nonu" => state.show_numbers = false,
        _ => render_status(state, &format!("unknown option: {option}")),
    }
    render(state);
}

fn push_undo(state: &mut AvimState) {
    if state.undo.len() == MAX_UNDO {
        state.undo.remove(0);
    }
    state.undo.push(state.lines.clone());
}

fn render(state: &AvimState) {
    println!(
        "--- avim {}{} ---",
        state.filename,
        if state.dirty { " [+]" } else { "" }
    );
    let start = state.cursor.saturating_sub(6);
    let end = (state.cursor + 7).min(state.lines.len());
    for idx in start..end {
        let marker = if idx == state.cursor { '>' } else { ' ' };
        if state.show_numbers {
            println!(
                "{marker}{:>4} {}",
                idx + 1,
                state.lines.get(idx).map(String::as_str).unwrap_or("")
            );
        } else {
            println!(
                "{marker} {}",
                state.lines.get(idx).map(String::as_str).unwrap_or("")
            );
        }
    }
    println!(
        "--- line {}/{} mode={} ---",
        state.cursor + 1,
        state.lines.len().max(1),
        mode_name(state.mode)
    );
}

fn render_status(state: &AvimState, message: &str) {
    println!("avim: {message}");
    println!(
        "line {}/{} mode={} | {}",
        state.cursor + 1,
        state.lines.len().max(1),
        mode_name(state.mode),
        mode_hint(state.mode)
    );
}

fn read_avim_input(state: &AvimState) -> io::Result<AvimInput> {
    if std::env::var_os("PHASE1_COOKED_INPUT").is_some() {
        print_avim_prompt(state);
        let _ = io::stdout().flush();
        let mut input = String::new();
        return match io::stdin().read_line(&mut input) {
            Ok(0) => Ok(AvimInput::End),
            Ok(_) => {
                let input = input.trim_end_matches(['\r', '\n']).to_string();
                if is_escape_input(&input) {
                    Ok(AvimInput::Escape)
                } else {
                    Ok(AvimInput::Line(input))
                }
            }
            Err(err) => Err(err),
        };
    }

    let Some(_guard) = RawModeGuard::enter() else {
        print_avim_prompt(state);
        let _ = io::stdout().flush();
        let mut input = String::new();
        return match io::stdin().read_line(&mut input) {
            Ok(0) => Ok(AvimInput::End),
            Ok(_) => {
                let input = input.trim_end_matches(['\r', '\n']).to_string();
                if is_escape_input(&input) {
                    Ok(AvimInput::Escape)
                } else {
                    Ok(AvimInput::Line(input))
                }
            }
            Err(err) => Err(err),
        };
    };

    let mut stdout = io::stdout();
    let mut stdin = io::stdin().lock();
    let mut input = String::new();
    let mut bytes = [0_u8; 1];
    let mut last_status = avim_status_line(state);

    write!(stdout, "{}{}", last_status, avim_prompt(state))?;
    stdout.flush()?;

    loop {
        match stdin.read(&mut bytes) {
            Ok(0) => {
                let status = avim_status_line(state);
                if status != last_status {
                    redraw_avim_input(state, &input, &mut stdout)?;
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
                return Ok(AvimInput::Line(input));
            }
            0x1b => {
                stdout.write_all(b"\r\n")?;
                stdout.flush()?;
                return Ok(AvimInput::Escape);
            }
            0x7f | 0x08 => {
                if !input.is_empty() {
                    input.pop();
                    redraw_avim_input(state, &input, &mut stdout)?;
                    last_status = avim_status_line(state);
                }
            }
            0x03 => {
                stdout.write_all(b"^C\r\n")?;
                stdout.flush()?;
                return Ok(AvimInput::Escape);
            }
            0x04 => {
                if input.is_empty() {
                    stdout.write_all(b"\r\n")?;
                    stdout.flush()?;
                    return Ok(AvimInput::End);
                }
            }
            byte if byte.is_ascii_control() => {}
            byte => {
                input.push(byte as char);
                redraw_avim_input(state, &input, &mut stdout)?;
                last_status = avim_status_line(state);
            }
        }
    }
}

fn redraw_avim_input(state: &AvimState, input: &str, stdout: &mut io::Stdout) -> io::Result<()> {
    write!(
        stdout,
        "\r\x1b[2K\x1b[1A\r\x1b[2K{}{}{}",
        avim_status_line(state),
        avim_prompt(state),
        input
    )?;
    stdout.flush()
}

fn print_avim_prompt(state: &AvimState) {
    print!("{}{}", avim_status_line(state), avim_prompt(state));
}

fn avim_prompt(state: &AvimState) -> String {
    let marker = match state.mode {
        Mode::Normal => "N",
        Mode::Insert => "I",
        Mode::Command => ":",
    };
    format!("avim[{marker}] {}:{}> ", state.filename, state.cursor + 1)
}

fn avim_status_line(state: &AvimState) -> String {
    let width = terminal_width().clamp(32, 72);
    let dirty = if state.dirty { "dirty" } else { "clean" };
    let raw = format!(
        "AVIM {} {} L{}/{} | {} | {}",
        mode_name(state.mode),
        dirty,
        state.cursor + 1,
        state.lines.len().max(1),
        short_clock_utc(),
        mode_hint(state.mode)
    );
    let clipped: String = raw.chars().take(width).collect();
    let padded = format!(
        "{clipped}{}",
        " ".repeat(width.saturating_sub(clipped.len()))
    );

    if color_enabled() {
        let color = match state.mode {
            Mode::Normal => CYAN,
            Mode::Insert => GREEN,
            Mode::Command => YELLOW,
        };
        format!("{BOLD}{color}{padded}{RESET}\r\n")
    } else {
        format!("{padded}\r\n")
    }
}

fn mode_name(mode: Mode) -> &'static str {
    match mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
        Mode::Command => "COMMAND",
    }
}

fn mode_hint(mode: Mode) -> &'static str {
    match mode {
        Mode::Normal => "i INSERT | :wq save | :q! quit",
        Mode::Insert => "Esc NORMAL | Enter commit line",
        Mode::Command => "w q q! wq help | Esc NORMAL",
    }
}

fn escape_to_normal(state: &mut AvimState) {
    state.mode = Mode::Normal;
    render_status(
        state,
        "NORMAL mode; use i for INSERT, :wq to save+quit, :help for keys",
    );
}

fn is_escape_input(input: &str) -> bool {
    matches!(
        input.trim(),
        "\u{1b}" | "^[" | "<esc>" | "<ESC>" | "ESC" | "esc"
    )
}

fn print_help() {
    println!("avim help");
    println!("  mode  : NORMAL accepts commands, INSERT writes text, COMMAND runs : commands");
    println!("  switch: NORMAL i/a/o enters INSERT; Esc returns to NORMAL from any mode");
    println!("  normal: o/O open line, j/k move, gg/G top/bottom");
    println!("  edit  : dd delete, yy yank, p paste, u undo");
    println!("  search: /text, n repeat");
    println!("  cmd   : :w, :q, :q!, :wq, :set number, :set nonumber, :%s/old/new/g, :r file");
    println!("  insert: type replacement text for current line; Enter commits current line; Esc returns to NORMAL");
}

fn print_security() {
    println!("avim security model");
    println!("  edits phase1 VFS files only");
    println!("  shell escapes, external filters, modelines, plugins, and host file paths are not implemented");
    println!("  file size is capped at {MAX_FILE_BYTES} bytes");
    println!("  save uses the existing VFS write path so normal phase1 persistence rules apply");
}

fn content_to_lines(content: &str) -> Vec<String> {
    let mut lines = content.lines().map(ToOwned::to_owned).collect::<Vec<_>>();
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn lines_to_content(lines: &[String]) -> String {
    let mut out = lines.join("\n");
    out.push('\n');
    out
}

fn safe_vfs_target(path: &str) -> bool {
    !path.trim().is_empty()
        && !path.contains('\0')
        && !path.contains("../")
        && !path.ends_with("/..")
        && path.len() <= 240
}

fn terminal_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(40)
}

fn color_enabled() -> bool {
    std::env::var_os("NO_COLOR").is_none()
        && std::env::var("PHASE1_NO_COLOR").ok().as_deref() != Some("1")
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
        avim_status_line, content_to_lines, is_escape_input, lines_to_content, safe_vfs_target,
        AvimState, Mode,
    };

    fn state(mode: Mode) -> AvimState {
        AvimState {
            filename: "hello.py".to_string(),
            lines: vec!["print('hi')".to_string()],
            cursor: 0,
            mode,
            dirty: false,
            show_numbers: true,
            yank: None,
            last_search: None,
            undo: Vec::new(),
        }
    }

    #[test]
    fn line_round_trip_preserves_terminal_newline() {
        let lines = content_to_lines("alpha\nbeta\n");
        assert_eq!(lines, vec!["alpha".to_string(), "beta".to_string()]);
        assert_eq!(lines_to_content(&lines), "alpha\nbeta\n");
    }

    #[test]
    fn target_validation_blocks_traversal() {
        assert!(safe_vfs_target("/home/app.rs"));
        assert!(!safe_vfs_target("../host"));
        assert!(!safe_vfs_target("/home/../host"));
    }

    #[test]
    fn escape_aliases_switch_modes() {
        assert!(is_escape_input("\u{1b}"));
        assert!(is_escape_input("^["));
        assert!(is_escape_input("esc"));
        assert!(is_escape_input("<ESC>"));
    }

    #[test]
    fn avim_status_explains_insert_escape() {
        std::env::set_var("NO_COLOR", "1");
        std::env::set_var("COLUMNS", "72");
        let line = avim_status_line(&state(Mode::Insert));
        assert!(line.contains("INSERT"));
        assert!(line.contains("Esc NORMAL"));
        assert!(line.contains("UTC"));
        std::env::remove_var("NO_COLOR");
        std::env::remove_var("COLUMNS");
    }
}
