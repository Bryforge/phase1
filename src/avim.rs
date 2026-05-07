use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::kernel::Vfs;

const MAX_FILE_BYTES: usize = 256 * 1024;
const MAX_UNDO: usize = 64;

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

pub fn edit(vfs: &mut Vfs, args: &[String]) {
    let Some(filename) = args.first() else {
        println!("usage: avim <file>");
        return;
    };
    if !safe_vfs_target(filename) {
        println!("avim: unsafe target path");
        return;
    }
    let existing = vfs.cat(filename).unwrap_or_default();
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
    println!("avim: advanced VFS editor // {}", state.filename);
    println!("easy keys: edit/e replace | before/I insert | after/a append | up/down move | :w save | :wq save+quit | :help");
    render(&state);
    loop {
        print!("{}", avim_prompt(&state));
        let _ = io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => return,
            Ok(_) => {}
            Err(err) => {
                println!("avim: input error: {err}");
                return;
            }
        }
        let input = input.trim_end_matches(['\r', '\n']).to_string();
        if is_escape_input(&input) {
            escape_to_normal(&mut state);
            continue;
        }
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
    let input = input.trim();
    match input {
        "" => render(state),
        "i" | "e" | "edit" | "replace" => enter_insert(state, "type replacement text for the selected line"),
        "a" | "after" | "append" | "o" => {
            push_undo(state);
            let at = (state.cursor + 1).min(state.lines.len());
            state.lines.insert(at, String::new());
            state.cursor = at;
            state.dirty = true;
            enter_insert(state, "type text for the new line after the cursor");
        }
        "I" | "before" | "insert" | "O" => {
            push_undo(state);
            state.lines.insert(state.cursor, String::new());
            state.dirty = true;
            enter_insert(state, "type text for the new line before the cursor");
        }
        "j" | "down" => move_cursor(state, 1),
        "k" | "up" => move_cursor(state, -1),
        "pgdn" | "pagedown" => move_cursor(state, 10),
        "pgup" | "pageup" => move_cursor(state, -10),
        "gg" | "home" | "top" => {
            state.cursor = 0;
            render(state);
        }
        "G" | "end" | "bottom" => {
            state.cursor = state.lines.len().saturating_sub(1);
            render(state);
        }
        "dd" | "delete" | "cut" => delete_line(state),
        "yy" | "copy" | "yank" => yank_line(state),
        "p" | "paste" => paste_line(state),
        "u" | "undo" => undo(state),
        "n" | "next" => repeat_search(state),
        ":" => {
            state.mode = Mode::Command;
            render_status(state, "COMMAND mode: w q wq help n <line> search <text>");
        }
        ":help" | "help" | "?" => print_help(),
        ":w" | "save" => return handle_command(vfs, state, "w"),
        ":wq" | ":x" => return handle_command(vfs, state, "wq"),
        ":q" => return handle_command(vfs, state, "q"),
        ":q!" => return handle_command(vfs, state, "discard"),
        raw if raw.starts_with(':') => return handle_command(vfs, state, &raw[1..]),
        raw if raw.starts_with('/') => search(state, raw.trim_start_matches('/')),
        other => render_status(state, &format!("unknown command: {other}; try edit, after, up, down, :wq, or :help")),
    }
    false
}

fn enter_insert(state: &mut AvimState, message: &str) {
    state.mode = Mode::Insert;
    render_status(state, message);
}

fn handle_insert(state: &mut AvimState, input: &str) {
    push_undo(state);
    if state.lines.is_empty() {
        state.lines.push(input.to_string());
        state.cursor = 0;
    } else {
        state.lines[state.cursor] = input.to_string();
    }
    state.dirty = true;
    state.mode = Mode::Normal;
    render(state);
}

fn handle_command(vfs: &mut Vfs, state: &mut AvimState, command: &str) -> bool {
    let command = command.trim();
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
        "q!" | "discard" => {
            println!("avim: discarded changes to {}", state.filename);
            true
        }
        "w" | "save" => {
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
        raw if raw.starts_with("n ") || raw.starts_with("line ") || raw.starts_with("goto ") => {
            goto_line(state, raw.split_whitespace().nth(1).unwrap_or_default());
            false
        }
        raw if raw.parse::<usize>().is_ok() => {
            goto_line(state, raw);
            false
        }
        raw if raw.starts_with("search ") => {
            search(state, raw.trim_start_matches("search "));
            false
        }
        raw if raw.starts_with('/') => {
            search(state, raw.trim_start_matches('/'));
            false
        }
        raw if raw.starts_with("%s/") => {
            substitute(state, raw);
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
    state.cursor = if delta.is_negative() {
        state.cursor.saturating_sub(delta.unsigned_abs())
    } else {
        state.cursor.saturating_add(delta as usize).min(len - 1)
    };
    render(state);
}

fn goto_line(state: &mut AvimState, raw: &str) {
    match raw.trim().parse::<usize>() {
        Ok(line) if line > 0 => {
            state.cursor = (line - 1).min(state.lines.len().saturating_sub(1));
            render(state);
        }
        _ => render_status(state, "expected a 1-based line number"),
    }
}

fn delete_line(state: &mut AvimState) {
    if state.lines.is_empty() {
        render_status(state, "nothing to delete");
        return;
    }
    push_undo(state);
    state.yank = Some(state.lines.remove(state.cursor));
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
        render_status(state, "copied current line");
    }
}

fn paste_line(state: &mut AvimState) {
    let Some(line) = state.yank.clone() else {
        render_status(state, "nothing copied");
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
    state.lines.iter().enumerate().skip(start).find_map(|(idx, line)| line.contains(needle).then_some(idx))
}

fn substitute(state: &mut AvimState, raw: &str) {
    let parts = raw.trim_start_matches("%s/").split('/').collect::<Vec<_>>();
    if parts.len() < 2 || parts[0].is_empty() {
        render_status(state, "usage: :%s/old/new/[g]");
        return;
    }
    push_undo(state);
    let old = parts[0];
    let new = parts[1];
    let global = parts.get(2).is_some_and(|flags| flags.contains('g'));
    let mut changed = 0usize;
    for line in &mut state.lines {
        let before = line.clone();
        *line = if global { line.replace(old, new) } else { line.replacen(old, new, 1) };
        if *line != before {
            changed += 1;
        }
    }
    state.dirty |= changed > 0;
    render_status(state, &format!("substitute changed {changed} lines"));
}

fn push_undo(state: &mut AvimState) {
    if state.undo.len() == MAX_UNDO {
        state.undo.remove(0);
    }
    state.undo.push(state.lines.clone());
}

fn render(state: &AvimState) {
    println!("--- avim {}{} ---", state.filename, if state.dirty { " [+]" } else { "" });
    let width = terminal_width().saturating_sub(8).clamp(24, 120);
    let start = state.cursor.saturating_sub(6);
    let end = (state.cursor + 7).min(state.lines.len());
    for idx in start..end {
        let marker = if idx == state.cursor { '>' } else { ' ' };
        let chunks = wrap_line(state.lines.get(idx).map(String::as_str).unwrap_or(""), width);
        println!("{marker}{:>4} {}", idx + 1, chunks[0]);
        for chunk in chunks.iter().skip(1) {
            println!("     | {chunk}");
        }
    }
    println!("--- line {}/{} mode={} ---", state.cursor + 1, state.lines.len().max(1), mode_name(state.mode));
}

fn render_status(state: &AvimState, message: &str) {
    println!("avim: {message}");
    println!("line {}/{} mode={} | {}", state.cursor + 1, state.lines.len().max(1), mode_name(state.mode), mode_hint(state.mode));
}

fn avim_prompt(state: &AvimState) -> String {
    format!("{}avim[{}] {}:{}> ", avim_status_line(state), mode_tag(state.mode), state.filename, state.cursor + 1)
}

fn avim_status_line(state: &AvimState) -> String {
    let raw = format!("AVIM {} {} L{}/{} | {} | {}", mode_name(state.mode), if state.dirty { "dirty" } else { "clean" }, state.cursor + 1, state.lines.len().max(1), short_clock_utc(), mode_hint(state.mode));
    format!("{}\n", raw.chars().take(terminal_width().clamp(32, 96)).collect::<String>())
}

fn mode_tag(mode: Mode) -> &'static str {
    match mode { Mode::Normal => "N", Mode::Insert => "I", Mode::Command => ":" }
}

fn mode_name(mode: Mode) -> &'static str {
    match mode { Mode::Normal => "NORMAL", Mode::Insert => "INSERT", Mode::Command => "COMMAND" }
}

fn mode_hint(mode: Mode) -> &'static str {
    match mode {
        Mode::Normal => "edit/e | after/a | up/down | :wq save | :help",
        Mode::Insert => "type line text | Esc NORMAL",
        Mode::Command => "w q wq help n <line> | Esc NORMAL",
    }
}

fn escape_to_normal(state: &mut AvimState) {
    state.mode = Mode::Normal;
    render_status(state, "NORMAL mode; use edit/e, after/a, up/down, :wq, or :help");
}

fn is_escape_input(input: &str) -> bool {
    matches!(input.trim(), "\u{1b}" | "^[" | "<esc>" | "<ESC>" | "ESC" | "esc")
}

fn print_help() {
    println!("avim help");
    println!("  easy  : edit/e replace selected line, before/I insert before, after/a append after");
    println!("  move  : up/down, j/k, pgup/pgdn, gg/G, :n 12, :12");
    println!("  edit  : delete/cut, copy/yank, paste, undo");
    println!("  search: /text, n repeat, :search text");
    println!("  save  : :w writes the file; :wq writes and exits; :q refuses dirty exit; :q! discards");
}

fn content_to_lines(content: &str) -> Vec<String> {
    let mut lines = content.lines().map(ToOwned::to_owned).collect::<Vec<_>>();
    if lines.is_empty() { lines.push(String::new()); }
    lines
}

fn lines_to_content(lines: &[String]) -> String {
    let mut out = lines.join("\n");
    out.push('\n');
    out
}

fn safe_vfs_target(path: &str) -> bool {
    !path.trim().is_empty() && !path.contains('\0') && !path.contains("../") && !path.ends_with("/..") && path.len() <= 240
}

fn terminal_width() -> usize {
    std::env::var("COLUMNS").ok().and_then(|raw| raw.parse().ok()).unwrap_or(72)
}

fn short_clock_utc() -> String {
    let seconds = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() % 86_400;
    format!("{:02}:{:02}:{:02} UTC", seconds / 3_600, (seconds % 3_600) / 60, seconds % 60)
}

fn wrap_line(line: &str, width: usize) -> Vec<String> {
    let width = width.max(8);
    if line.is_empty() { return vec![String::new()]; }
    let mut out = Vec::new();
    let mut current = String::new();
    for ch in line.chars() {
        if current.chars().count() >= width {
            out.push(current);
            current = String::new();
        }
        current.push(ch);
    }
    if !current.is_empty() { out.push(current); }
    out
}

#[cfg(test)]
mod tests {
    use super::{avim_status_line, content_to_lines, is_escape_input, lines_to_content, safe_vfs_target, wrap_line, AvimState, Mode};

    fn state(mode: Mode) -> AvimState {
        AvimState { filename: "hello.py".to_string(), lines: vec!["print('hi')".to_string()], cursor: 0, mode, dirty: false, show_numbers: true, yank: None, last_search: None, undo: Vec::new() }
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

    #[test]
    fn avim_wraps_long_lines() {
        let wrapped = wrap_line("abcdefghijklmnop", 5);
        assert_eq!(wrapped, vec!["abcdefgh", "ijklmnop"]);
    }
}
