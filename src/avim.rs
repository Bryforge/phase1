use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::kernel::Vfs;

const MAX_FILE_BYTES: usize = 256 * 1024;
const MAX_UNDO: usize = 64;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const BLINK: &str = "\x1b[5m";
const NO_BLINK: &str = "\x1b[25m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Normal,
    Insert,
    Command,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EditAction {
    ReplaceLine,
    InsertAtCursor,
    AppendAtCursor,
    NewLineBefore,
    NewLineAfter,
}

#[derive(Debug)]
struct AvimState {
    filename: String,
    lines: Vec<String>,
    cursor: usize,
    column: usize,
    mode: Mode,
    pending_edit: Option<EditAction>,
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
        column: 0,
        mode: Mode::Normal,
        pending_edit: None,
        dirty: false,
        show_numbers: true,
        yank: None,
        last_search: None,
        undo: Vec::new(),
    };
    clamp_cursor(&mut state);
    println!(
        "{}",
        paint(
            "avim: advanced VFS editor // simple modal editing",
            BOLD,
            CYAN
        )
    );
    println!(
        "{}",
        paint(
            "quick: i text insert | a text append | e text replace | h/l cursor | o/O lines | :wq save+quit | Tab completes",
            DIM,
            CYAN,
        )
    );
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
        if input.contains('\t') {
            complete_avim_input(&state, &input);
            continue;
        }
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
    if input.is_empty() {
        render(state);
        return false;
    }
    if input == ":" {
        state.mode = Mode::Command;
        render_status(state, "COMMAND mode: w q wq help read search set", YELLOW);
        return false;
    }
    if let Some(command) = input.strip_prefix(':') {
        return handle_command(vfs, state, command);
    }
    if input.starts_with('/') {
        search(state, input.trim_start_matches('/'));
        return false;
    }

    let (command, rest) = split_command(input);
    let lower = command.to_ascii_lowercase();
    match lower.as_str() {
        "h" | "left" => move_column(state, -1),
        "l" | "right" => move_column(state, 1),
        "0" | "home" | "bol" => set_column(state, 0),
        "$" | "end" | "eol" => set_column(state, current_line_len(state)),
        "w" | "word" => move_word_forward(state),
        "b" | "back" => move_word_back(state),
        "j" | "down" => move_cursor(state, 1),
        "k" | "up" => move_cursor(state, -1),
        "pgdn" | "pagedown" => move_cursor(state, 10),
        "pgup" | "pageup" => move_cursor(state, -10),
        "gg" | "top" => {
            state.cursor = 0;
            clamp_cursor(state);
            render(state);
        }
        "g" | "bottom" if command == "G" || lower == "bottom" => {
            state.cursor = state.lines.len().saturating_sub(1);
            clamp_cursor(state);
            render(state);
        }
        "dd" | "delete" | "cut" => delete_line(state),
        "yy" | "copy" | "yank" => yank_line(state),
        "p" | "paste" => paste_line(state),
        "u" | "undo" => undo(state),
        "x" | "delete-char" | "del" => delete_at_cursor_char(state),
        "backspace" | "bs" => delete_before_cursor_char(state),
        "n" | "next" => repeat_search(state),
        "tab" | "complete" => print_avim_completion_help(state.mode),
        "help" | "?" => print_help(),
        "security" | "safety" => print_security_model(),
        "save" => return handle_command(vfs, state, "w"),
        "wq" | "savequit" => return handle_command(vfs, state, "wq"),
        "q" | "quit" | "exit" => return handle_command(vfs, state, "q"),
        "discard" | "q!" => return handle_command(vfs, state, "q!"),
        _ => {
            if handle_edit_action(state, command, rest) {
                return false;
            }
            if let Ok(line) = input.parse::<usize>() {
                goto_line(state, &line.to_string());
                return false;
            }
            render_status(
                state,
                &format!(
                    "unknown command: {input}; try i text, a text, e text, h/l, up/down, :wq, Tab, or :help"
                ),
                RED,
            );
        }
    }
    false
}

fn handle_edit_action(state: &mut AvimState, command: &str, rest: &str) -> bool {
    let lower = command.to_ascii_lowercase();
    if command == "I" || command == "O" || matches!(lower.as_str(), "before") {
        if rest.is_empty() {
            enter_insert(
                state,
                EditAction::NewLineBefore,
                "type a new line before the cursor; Esc cancels",
            );
        } else {
            insert_line(state, state.cursor, rest.to_string());
        }
        return true;
    }

    match lower.as_str() {
        "i" | "insert" => {
            if rest.is_empty() {
                enter_insert(
                    state,
                    EditAction::InsertAtCursor,
                    "type text to insert at the pink cursor; Esc cancels",
                );
            } else {
                insert_text_at_cursor(state, rest, false);
            }
            true
        }
        "a" | "append" => {
            if rest.is_empty() {
                enter_insert(
                    state,
                    EditAction::AppendAtCursor,
                    "type text to append after the pink cursor; Esc cancels",
                );
            } else {
                insert_text_at_cursor(state, rest, true);
            }
            true
        }
        "e" | "edit" | "r" | "replace" => {
            if rest.is_empty() {
                enter_insert(
                    state,
                    EditAction::ReplaceLine,
                    "type replacement text for the selected line; Esc cancels",
                );
            } else {
                replace_current_line(state, rest.to_string());
            }
            true
        }
        "o" | "after" | "open" => {
            let idx = (state.cursor + 1).min(state.lines.len());
            if rest.is_empty() {
                state.cursor = idx;
                state.column = 0;
                enter_insert(
                    state,
                    EditAction::NewLineAfter,
                    "type a new line after the cursor; Esc cancels",
                );
            } else {
                insert_line(state, idx, rest.to_string());
            }
            true
        }
        _ => false,
    }
}

fn enter_insert(state: &mut AvimState, action: EditAction, message: &str) {
    state.mode = Mode::Insert;
    state.pending_edit = Some(action);
    render_status(state, message, MAGENTA);
}

fn handle_insert(state: &mut AvimState, input: &str) {
    let Some(action) = state.pending_edit.take() else {
        state.mode = Mode::Normal;
        render_status(state, "no pending edit action", YELLOW);
        return;
    };
    match action {
        EditAction::ReplaceLine => replace_current_line(state, input.to_string()),
        EditAction::InsertAtCursor => insert_text_at_cursor(state, input, false),
        EditAction::AppendAtCursor => insert_text_at_cursor(state, input, true),
        EditAction::NewLineBefore => insert_line(state, state.cursor, input.to_string()),
        EditAction::NewLineAfter => insert_line(
            state,
            (state.cursor + 1).min(state.lines.len()),
            input.to_string(),
        ),
    }
    state.mode = Mode::Normal;
}

fn handle_command(vfs: &mut Vfs, state: &mut AvimState, command: &str) -> bool {
    let command = command.trim();
    if command.contains('\t') {
        complete_avim_input(state, command);
        return false;
    }
    let (head, rest) = split_command(command);
    let lower = head.to_ascii_lowercase();
    match command {
        "q" | "quit" | "exit" => {
            if state.dirty {
                render_status(
                    state,
                    "unsaved changes; use :wq to save or :q! to discard",
                    YELLOW,
                );
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
        "wq" | "x" | "savequit" => {
            save(vfs, state);
            !state.dirty
        }
        "help" | "h" => {
            print_help();
            false
        }
        "security" | "safety" => {
            print_security_model();
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
        _ if lower == "read" || lower == "r" => {
            read_file_below_cursor(vfs, state, rest);
            false
        }
        _ if handle_edit_action(state, head, rest) => false,
        other => {
            render_status(state, &format!("unknown command: :{other}"), RED);
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
            render_status(
                state,
                &format!("wrote {} ({} bytes)", state.filename, content.len()),
                GREEN,
            );
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
    clamp_cursor(state);
    render(state);
}

fn move_column(state: &mut AvimState, delta: isize) {
    let max = current_line_len(state);
    state.column = if delta.is_negative() {
        state.column.saturating_sub(delta.unsigned_abs())
    } else {
        state.column.saturating_add(delta as usize).min(max)
    };
    render(state);
}

fn set_column(state: &mut AvimState, column: usize) {
    state.column = column.min(current_line_len(state));
    render(state);
}

fn move_word_forward(state: &mut AvimState) {
    let Some(line) = state.lines.get(state.cursor) else {
        return;
    };
    let mut seen_space = false;
    let mut next = current_line_len(state);
    for (idx, ch) in line
        .chars()
        .enumerate()
        .skip(state.column.saturating_add(1))
    {
        if ch.is_whitespace() {
            seen_space = true;
        } else if seen_space {
            next = idx;
            break;
        }
    }
    set_column(state, next);
}

fn move_word_back(state: &mut AvimState) {
    let Some(line) = state.lines.get(state.cursor) else {
        return;
    };
    let chars = line.chars().collect::<Vec<_>>();
    if state.column == 0 || chars.is_empty() {
        set_column(state, 0);
        return;
    }
    let mut idx = state
        .column
        .saturating_sub(1)
        .min(chars.len().saturating_sub(1));
    while idx > 0 && chars[idx].is_whitespace() {
        idx -= 1;
    }
    while idx > 0 && !chars[idx - 1].is_whitespace() {
        idx -= 1;
    }
    set_column(state, idx);
}

fn goto_line(state: &mut AvimState, raw: &str) {
    match raw.trim().parse::<usize>() {
        Ok(line) if line > 0 => {
            state.cursor = (line - 1).min(state.lines.len().saturating_sub(1));
            clamp_cursor(state);
            render(state);
        }
        _ => render_status(state, "expected a 1-based line number", YELLOW),
    }
}

fn insert_line(state: &mut AvimState, idx: usize, text: String) {
    push_undo(state);
    let idx = idx.min(state.lines.len());
    state.lines.insert(idx, text);
    state.cursor = idx;
    state.column = current_line_len(state);
    state.dirty = true;
    render(state);
}

fn replace_current_line(state: &mut AvimState, text: String) {
    push_undo(state);
    if state.lines.is_empty() {
        state.lines.push(text);
        state.cursor = 0;
    } else {
        state.lines[state.cursor] = text;
    }
    state.column = current_line_len(state);
    state.dirty = true;
    render(state);
}

fn insert_text_at_cursor(state: &mut AvimState, text: &str, after_cursor: bool) {
    push_undo(state);
    if state.lines.is_empty() {
        state.lines.push(String::new());
        state.cursor = 0;
        state.column = 0;
    }
    let line = &mut state.lines[state.cursor];
    let len = char_len(line);
    let insert_at = if after_cursor && state.column < len {
        state.column + 1
    } else {
        state.column
    }
    .min(len);
    let byte_idx = byte_index_for_char(line, insert_at);
    line.insert_str(byte_idx, text);
    state.column = insert_at + char_len(text);
    state.dirty = true;
    render(state);
}

fn delete_at_cursor_char(state: &mut AvimState) {
    if state.lines.is_empty() || state.column >= current_line_len(state) {
        render_status(state, "no character under cursor", YELLOW);
        return;
    }
    push_undo(state);
    let line = &mut state.lines[state.cursor];
    let start = byte_index_for_char(line, state.column);
    let end = byte_index_for_char(line, state.column + 1);
    line.replace_range(start..end, "");
    state.column = state.column.min(char_len(line));
    state.dirty = true;
    render(state);
}

fn delete_before_cursor_char(state: &mut AvimState) {
    if state.column == 0 {
        render_status(state, "cursor is at start of line", YELLOW);
        return;
    }
    state.column = state.column.saturating_sub(1);
    delete_at_cursor_char(state);
}

fn delete_line(state: &mut AvimState) {
    if state.lines.is_empty() {
        render_status(state, "nothing to delete", YELLOW);
        return;
    }
    push_undo(state);
    state.yank = Some(state.lines.remove(state.cursor));
    if state.lines.is_empty() {
        state.lines.push(String::new());
    }
    state.cursor = state.cursor.min(state.lines.len() - 1);
    clamp_cursor(state);
    state.dirty = true;
    render(state);
}

fn yank_line(state: &mut AvimState) {
    if let Some(line) = state.lines.get(state.cursor) {
        state.yank = Some(line.clone());
        render_status(state, "copied current line", CYAN);
    }
}

fn paste_line(state: &mut AvimState) {
    let Some(line) = state.yank.clone() else {
        render_status(state, "nothing copied", YELLOW);
        return;
    };
    insert_line(state, (state.cursor + 1).min(state.lines.len()), line);
}

fn undo(state: &mut AvimState) {
    match state.undo.pop() {
        Some(previous) => {
            state.lines = previous;
            clamp_cursor(state);
            state.dirty = true;
            render(state);
        }
        None => render_status(state, "nothing to undo", YELLOW),
    }
}

fn search(state: &mut AvimState, needle: &str) {
    if needle.is_empty() {
        render_status(state, "empty search", YELLOW);
        return;
    }
    state.last_search = Some(needle.to_string());
    let start = (state.cursor + 1).min(state.lines.len());
    if let Some(idx) = find_from(state, needle, start).or_else(|| find_from(state, needle, 0)) {
        state.cursor = idx;
        if let Some(found_at) = state.lines[idx].find(needle) {
            state.column = state.lines[idx][..found_at].chars().count();
        }
        render(state);
    } else {
        render_status(state, &format!("pattern not found: {needle}"), YELLOW);
    }
}

fn repeat_search(state: &mut AvimState) {
    match state.last_search.clone() {
        Some(needle) => search(state, &needle),
        None => render_status(state, "no previous search", YELLOW),
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
    if parts.len() < 2 || parts[0].is_empty() {
        render_status(state, "usage: :%s/old/new/[g]", YELLOW);
        return;
    }
    push_undo(state);
    let old = parts[0];
    let new = parts[1];
    let global = parts.get(2).is_some_and(|flags| flags.contains('g'));
    let mut changed = 0usize;
    for line in &mut state.lines {
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
    state.dirty |= changed > 0;
    clamp_cursor(state);
    render_status(state, &format!("substitute changed {changed} lines"), CYAN);
}

fn read_file_below_cursor(vfs: &Vfs, state: &mut AvimState, path: &str) {
    if path.trim().is_empty() {
        render_status(state, "usage: :read <vfs-file>", YELLOW);
        return;
    }
    if !safe_vfs_target(path) {
        render_status(state, "read blocked: unsafe VFS path", RED);
        return;
    }
    match vfs.cat(path) {
        Ok(content) if content.len() <= MAX_FILE_BYTES => {
            push_undo(state);
            let mut lines = content_to_lines(&content);
            let idx = (state.cursor + 1).min(state.lines.len());
            for (offset, line) in lines.drain(..).enumerate() {
                state.lines.insert(idx + offset, line);
            }
            state.cursor = idx;
            state.column = 0;
            state.dirty = true;
            render(state);
        }
        Ok(_) => render_status(state, "read blocked: source file is too large", RED),
        Err(err) => render_status(state, &format!("read failed: {err}"), RED),
    }
}

fn push_undo(state: &mut AvimState) {
    if state.undo.len() == MAX_UNDO {
        state.undo.remove(0);
    }
    state.undo.push(state.lines.clone());
}

fn render(state: &AvimState) {
    let title = format!(
        "AVIM {} {} {} L{}/{} C{} | {}",
        mode_name(state.mode),
        if state.dirty { "dirty" } else { "clean" },
        state.filename,
        state.cursor + 1,
        state.lines.len().max(1),
        state.column + 1,
        short_clock_utc()
    );
    println!(
        "{}",
        paint(&format!("╭─ {title}"), BOLD, mode_color(state.mode))
    );
    println!(
        "{}",
        paint(&format!("│ {}", mode_hint(state.mode)), DIM, CYAN)
    );
    let width = terminal_width().saturating_sub(12).clamp(24, 120);
    let start = state.cursor.saturating_sub(6);
    let end = (state.cursor + 7).min(state.lines.len());
    for idx in start..end {
        let is_current = idx == state.cursor;
        let marker = if is_current { "▶" } else { " " };
        let line = state.lines.get(idx).map(String::as_str).unwrap_or("");
        let rendered = if is_current {
            render_line_with_cursor(line, state.column)
        } else {
            wrap_line(line, width).first().cloned().unwrap_or_default()
        };
        let line_no = if state.show_numbers {
            format!("{:>4}", idx + 1)
        } else {
            "    ".to_string()
        };
        if is_current {
            println!(
                "{} {} {}",
                paint(marker, BOLD, MAGENTA),
                paint(&line_no, DIM, MAGENTA),
                rendered
            );
        } else {
            println!(
                "{marker} {} {}",
                paint(&line_no, DIM, BLUE),
                paint(&rendered, DIM, RESET)
            );
        }
    }
    println!(
        "{}",
        paint(
            "╰─ Tab completes · Esc/escape returns NORMAL · :help lists commands",
            DIM,
            CYAN
        )
    );
}

fn render_status(state: &AvimState, message: &str, color: &str) {
    println!("{}", paint(&format!("avim: {message}"), BOLD, color));
    println!(
        "{}",
        paint(
            &format!(
                "line {}/{} col {} mode={} | {}",
                state.cursor + 1,
                state.lines.len().max(1),
                state.column + 1,
                mode_name(state.mode),
                mode_hint(state.mode)
            ),
            DIM,
            color,
        )
    );
}

fn avim_prompt(state: &AvimState) -> String {
    format!(
        "{}avim[{}] {}:{}:{} {}> ",
        avim_status_line(state),
        mode_tag(state.mode),
        state.filename,
        state.cursor + 1,
        state.column + 1,
        cursor_glyph()
    )
}

fn avim_status_line(state: &AvimState) -> String {
    let raw = format!(
        "AVIM {} {} L{}/{} C{} | {} | {}",
        mode_name(state.mode),
        if state.dirty { "dirty" } else { "clean" },
        state.cursor + 1,
        state.lines.len().max(1),
        state.column + 1,
        short_clock_utc(),
        mode_hint(state.mode)
    );
    let clipped = raw
        .chars()
        .take(terminal_width().clamp(32, 108))
        .collect::<String>();
    format!("{}\n", paint(&clipped, BOLD, mode_color(state.mode)))
}

fn render_line_with_cursor(line: &str, column: usize) -> String {
    let mut out = String::new();
    let len = char_len(line);
    if line.is_empty() {
        out.push_str(&cursor_glyph());
        out.push_str(&paint("·", DIM, BLUE));
        return out;
    }
    for (idx, ch) in line.chars().enumerate() {
        if idx == column.min(len) {
            out.push_str(&cursor_glyph());
        }
        out.push(ch);
    }
    if column >= len {
        out.push_str(&cursor_glyph());
    }
    out
}

fn complete_avim_input(state: &AvimState, input: &str) {
    let before_tab = input.split('\t').next().unwrap_or_default();
    let prefix = before_tab
        .split_whitespace()
        .last()
        .unwrap_or_default()
        .trim_start_matches(':');
    let matches = avim_completion_matches(prefix);
    match matches.as_slice() {
        [] => render_status(
            state,
            &format!("tab complete: no matches for '{prefix}'"),
            YELLOW,
        ),
        [only] => render_status(state, &format!("tab complete: {only}"), CYAN),
        _ => render_status(
            state,
            &format!("tab matches for '{prefix}': {}", matches.join(" ")),
            CYAN,
        ),
    }
}

fn avim_completion_matches(prefix: &str) -> Vec<String> {
    let options = [
        "i",
        "insert",
        "a",
        "append",
        "e",
        "edit",
        "replace",
        "before",
        "after",
        "left",
        "right",
        "up",
        "down",
        "home",
        "end",
        "word",
        "back",
        "delete",
        "delete-char",
        "copy",
        "yank",
        "paste",
        "undo",
        "search",
        "next",
        "save",
        "quit",
        "exit",
        "discard",
        "help",
        "security",
        "set",
        "set number",
        "set nonumber",
        ":w",
        ":wq",
        ":q",
        ":q!",
        ":help",
        ":read",
        ":%s/old/new/g",
    ];
    let mut matches = options
        .iter()
        .copied()
        .filter(|candidate| {
            candidate.starts_with(prefix) || candidate.trim_start_matches(':').starts_with(prefix)
        })
        .map(str::to_string)
        .collect::<Vec<_>>();
    matches.sort();
    matches.dedup();
    matches
}

fn print_avim_completion_help(mode: Mode) {
    println!("avim tab completion ({})", mode_name(mode));
    println!("  commands: {}", avim_completion_matches("").join(" "));
}

fn mode_tag(mode: Mode) -> &'static str {
    match mode {
        Mode::Normal => "N",
        Mode::Insert => "I",
        Mode::Command => ":",
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
        Mode::Normal => "i/a insert text | e replace line | h/l cursor | up/down lines | :wq save | Tab commands",
        Mode::Insert => "Esc/escape cancels | type text for pending edit",
        Mode::Command => "w q wq help read search set | Esc returns NORMAL | Tab commands",
    }
}

fn mode_color(mode: Mode) -> &'static str {
    match mode {
        Mode::Normal => CYAN,
        Mode::Insert => MAGENTA,
        Mode::Command => YELLOW,
    }
}

fn escape_to_normal(state: &mut AvimState) {
    state.mode = Mode::Normal;
    state.pending_edit = None;
    render_status(
        state,
        "NORMAL mode; use i text, a text, e text, h/l, up/down, :wq, or :help",
        CYAN,
    );
}

fn is_escape_input(input: &str) -> bool {
    let trimmed = input.trim();
    let lower = trimmed.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "\u{1b}" | "^[" | "<esc>" | "esc" | "escape" | "normal" | "cancel"
    )
}

fn print_help() {
    println!("{}", paint("avim help", BOLD, CYAN));
    println!("  cursor: h/left, l/right, 0/home, $/end, w next word, b previous word");
    println!(
        "  edit  : i text inserts at cursor, a text appends after cursor, e text replaces line"
    );
    println!("  lines : o text opens below, O text or before text opens above, dd deletes, yy copies, p pastes");
    println!("  fixup : x deletes char under cursor, backspace deletes before cursor, u undo");
    println!("  move  : up/down, j/k, pgup/pgdn, gg/G, :n 12, :12");
    println!("  search: /text, n repeat, :search text, :%s/old/new/g substitute");
    println!("  save  : :w writes; :wq writes and exits; :q refuses dirty exit; :q! discards");
    println!(
        "  tab   : press Tab in avim for command suggestions; shell Tab completes avim file names"
    );
}

fn print_security_model() {
    println!("{}", paint("avim security model", BOLD, BLUE));
    println!("  edits stay inside the Phase1 VFS");
    println!("  no shell escapes, host editor launch, modelines, plugins, network fetches, or background jobs");
    println!(
        "  file size is capped; unsafe traversal paths are rejected; save/discard is explicit"
    );
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
        .unwrap_or(72)
}

fn short_clock_utc() -> String {
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

fn wrap_line(line: &str, width: usize) -> Vec<String> {
    let width = width.max(8);
    if line.is_empty() {
        return vec![String::new()];
    }
    let mut out = Vec::new();
    let mut current = String::new();
    for ch in line.chars() {
        if current.chars().count() >= width {
            out.push(current);
            current = String::new();
        }
        current.push(ch);
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

fn split_command(input: &str) -> (&str, &str) {
    let trimmed = input.trim();
    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or_default();
    let rest = parts.next().unwrap_or_default().trim_start();
    (command, rest)
}

fn current_line_len(state: &AvimState) -> usize {
    state
        .lines
        .get(state.cursor)
        .map_or(0, |line| char_len(line))
}

fn clamp_cursor(state: &mut AvimState) {
    if state.lines.is_empty() {
        state.lines.push(String::new());
    }
    state.cursor = state.cursor.min(state.lines.len().saturating_sub(1));
    state.column = state.column.min(current_line_len(state));
}

fn char_len(text: &str) -> usize {
    text.chars().count()
}

fn byte_index_for_char(text: &str, char_idx: usize) -> usize {
    text.char_indices()
        .nth(char_idx)
        .map(|(idx, _)| idx)
        .unwrap_or(text.len())
}

fn cursor_glyph() -> String {
    if color_enabled() {
        format!("{BOLD}{MAGENTA}{BLINK}▌{NO_BLINK}{RESET}")
    } else {
        "|".to_string()
    }
}

fn paint(text: &str, style: &str, color: &str) -> String {
    if color_enabled() {
        let color = if color == RESET { "" } else { color };
        format!("{style}{color}{text}{RESET}")
    } else {
        text.to_string()
    }
}

fn color_enabled() -> bool {
    std::env::var_os("NO_COLOR").is_none()
        && std::env::var("PHASE1_NO_COLOR").ok().as_deref() != Some("1")
}

#[cfg(test)]
mod tests {
    use super::{
        avim_completion_matches, avim_status_line, content_to_lines, handle_edit_action,
        is_escape_input, lines_to_content, render_line_with_cursor, safe_vfs_target, wrap_line,
        AvimState, Mode,
    };

    fn state(mode: Mode) -> AvimState {
        AvimState {
            filename: "hello.py".to_string(),
            lines: vec!["print('hi')".to_string()],
            cursor: 0,
            column: 5,
            mode,
            pending_edit: None,
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
        assert!(is_escape_input("Esc"));
        assert!(is_escape_input("escape"));
        assert!(is_escape_input("<ESC>"));
        assert!(is_escape_input("normal"));
    }

    #[test]
    fn avim_status_explains_insert_escape() {
        std::env::set_var("NO_COLOR", "1");
        std::env::set_var("COLUMNS", "72");
        let line = avim_status_line(&state(Mode::Insert));
        assert!(line.contains("INSERT"));
        assert!(line.contains("Esc"));
        assert!(line.contains("UTC"));
        std::env::remove_var("NO_COLOR");
        std::env::remove_var("COLUMNS");
    }

    #[test]
    fn avim_wraps_long_lines() {
        let wrapped = wrap_line("abcdefghijklmnop", 5);
        assert_eq!(wrapped, vec!["abcdefgh", "ijklmnop"]);
    }

    #[test]
    fn avim_cursor_render_has_visible_fallback() {
        std::env::set_var("NO_COLOR", "1");
        std::env::set_var("PHASE1_NO_COLOR", "1");
        assert_eq!(render_line_with_cursor("abc", 1), "a|bc");
        assert_eq!(render_line_with_cursor("", 0), "|·");
        std::env::remove_var("NO_COLOR");
        std::env::remove_var("PHASE1_NO_COLOR");
    }

    #[test]
    fn avim_inline_insert_and_replace_are_editable() {
        std::env::set_var("NO_COLOR", "1");
        let mut state = state(Mode::Normal);
        assert!(handle_edit_action(&mut state, "i", "_fixed"));
        assert_eq!(state.lines[0], "print_fixed('hi')");
        assert!(state.dirty);
        assert!(handle_edit_action(&mut state, "e", "print('hello')"));
        assert_eq!(state.lines[0], "print('hello')");
        std::env::remove_var("NO_COLOR");
    }

    #[test]
    fn avim_completion_lists_editor_commands() {
        let matches = avim_completion_matches("app");
        assert!(matches.contains(&"append".to_string()));
        let matches = avim_completion_matches("w");
        assert!(matches.contains(&":w".to_string()));
        assert!(matches.contains(&":wq".to_string()));
    }
}
