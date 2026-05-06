use crate::kernel::{Vfs, VfsNode};
use std::path::{Path, PathBuf};

pub fn grep(vfs: &Vfs, args: &[String]) -> String {
    let mut ignore_case = false;
    let mut line_numbers = false;
    let mut count_only = false;
    let mut positional = Vec::new();

    for arg in args {
        match arg.as_str() {
            "-i" | "--ignore-case" => ignore_case = true,
            "-n" | "--line-number" => line_numbers = true,
            "-c" | "--count" => count_only = true,
            "-h" | "--help" => return grep_help(),
            _ => positional.push(arg.as_str()),
        }
    }

    if positional.len() < 2 {
        return grep_help();
    }

    let pattern = positional[0];
    let query = if ignore_case {
        pattern.to_ascii_lowercase()
    } else {
        pattern.to_string()
    };
    let files = &positional[1..];
    let show_file = files.len() > 1;
    let mut out = String::new();

    for file in files {
        match vfs.cat(file) {
            Ok(content) => {
                let mut matches = 0usize;
                for (idx, line) in content.lines().enumerate() {
                    let haystack = if ignore_case {
                        line.to_ascii_lowercase()
                    } else {
                        line.to_string()
                    };
                    if haystack.contains(&query) {
                        matches += 1;
                        if !count_only {
                            if show_file {
                                out.push_str(file);
                                out.push(':');
                            }
                            if line_numbers {
                                out.push_str(&(idx + 1).to_string());
                                out.push(':');
                            }
                            out.push_str(line);
                            out.push('\n');
                        }
                    }
                }
                if count_only {
                    if show_file {
                        out.push_str(file);
                        out.push(':');
                    }
                    out.push_str(&matches.to_string());
                    out.push('\n');
                }
            }
            Err(err) => out.push_str(&format!("grep: {file}: {err}\n")),
        }
    }

    out
}

pub fn wc(vfs: &Vfs, args: &[String]) -> String {
    let mut show_lines = false;
    let mut show_words = false;
    let mut show_bytes = false;
    let mut files = Vec::new();

    for arg in args {
        match arg.as_str() {
            "-l" | "--lines" => show_lines = true,
            "-w" | "--words" => show_words = true,
            "-c" | "--bytes" => show_bytes = true,
            "-h" | "--help" => return wc_help(),
            _ => files.push(arg.as_str()),
        }
    }

    if files.is_empty() {
        return wc_help();
    }
    if !show_lines && !show_words && !show_bytes {
        show_lines = true;
        show_words = true;
        show_bytes = true;
    }

    let mut out = String::new();
    let mut totals = Counts::default();
    let mut counted = 0usize;

    for file in files {
        match vfs.cat(file) {
            Ok(content) => {
                let counts = Counts::from_text(&content);
                totals.add(counts);
                counted += 1;
                out.push_str(&format_counts(counts, show_lines, show_words, show_bytes, file));
            }
            Err(err) => out.push_str(&format!("wc: {file}: {err}\n")),
        }
    }

    if counted > 1 {
        out.push_str(&format_counts(totals, show_lines, show_words, show_bytes, "total"));
    }

    out
}

pub fn head(vfs: &Vfs, args: &[String]) -> String {
    lines_window(vfs, args, 10, true)
}

pub fn tail(vfs: &Vfs, args: &[String]) -> String {
    lines_window(vfs, args, 10, false)
}

pub fn find(vfs: &Vfs, args: &[String]) -> String {
    let mut root = ".";
    let mut name_pattern: Option<&str> = None;
    let mut type_filter: Option<NodeKind> = None;
    let mut max_depth: Option<usize> = None;
    let mut idx = 0;

    if let Some(first) = args.first() {
        if !first.starts_with('-') {
            root = first;
            idx = 1;
        }
    }

    while idx < args.len() {
        match args[idx].as_str() {
            "-name" => {
                idx += 1;
                let Some(value) = args.get(idx) else {
                    return "find: missing value for -name\n".to_string();
                };
                name_pattern = Some(value);
            }
            "-type" => {
                idx += 1;
                type_filter = match args.get(idx).map(String::as_str) {
                    Some("f") => Some(NodeKind::File),
                    Some("d") => Some(NodeKind::Dir),
                    Some(other) => return format!("find: unsupported type '{other}'\n"),
                    None => return "find: missing value for -type\n".to_string(),
                };
            }
            "-maxdepth" => {
                idx += 1;
                let Some(value) = args.get(idx) else {
                    return "find: missing value for -maxdepth\n".to_string();
                };
                match value.parse::<usize>() {
                    Ok(value) => max_depth = Some(value),
                    Err(_) => return format!("find: invalid maxdepth '{value}'\n"),
                }
            }
            "-h" | "--help" => return find_help(),
            other => return format!("find: unknown option '{other}'\n{}", find_help()),
        }
        idx += 1;
    }

    let root_path = vfs.resolve_path(root);
    let Some(node) = vfs.get_node(&root_path) else {
        return format!("find: {root}: no such file or directory\n");
    };

    let mut out = String::new();
    walk_find(
        &root_path,
        node,
        0,
        name_pattern,
        type_filter,
        max_depth,
        &mut out,
    );
    out
}

#[derive(Clone, Copy, Debug, Default)]
struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
}

impl Counts {
    fn from_text(text: &str) -> Self {
        Self {
            lines: text.lines().count(),
            words: text.split_whitespace().count(),
            bytes: text.len(),
        }
    }

    fn add(&mut self, other: Self) {
        self.lines += other.lines;
        self.words += other.words;
        self.bytes += other.bytes;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NodeKind {
    File,
    Dir,
}

fn format_counts(
    counts: Counts,
    show_lines: bool,
    show_words: bool,
    show_bytes: bool,
    label: &str,
) -> String {
    let mut out = String::new();
    if show_lines {
        out.push_str(&format!("{:>5}", counts.lines));
    }
    if show_words {
        out.push_str(&format!("{:>5}", counts.words));
    }
    if show_bytes {
        out.push_str(&format!("{:>5}", counts.bytes));
    }
    out.push(' ');
    out.push_str(label);
    out.push('\n');
    out
}

fn lines_window(vfs: &Vfs, args: &[String], default_lines: usize, from_start: bool) -> String {
    let mut limit = default_lines;
    let mut files = Vec::new();
    let mut idx = 0;

    while idx < args.len() {
        let arg = &args[idx];
        if arg == "-n" || arg == "--lines" {
            idx += 1;
            let Some(value) = args.get(idx) else {
                return format!("{}: missing line count\n", if from_start { "head" } else { "tail" });
            };
            match value.parse::<usize>() {
                Ok(value) => limit = value,
                Err(_) => return format!("{}: invalid line count '{value}'\n", if from_start { "head" } else { "tail" }),
            }
        } else if let Some(raw) = arg.strip_prefix('-') {
            if raw.chars().all(|ch| ch.is_ascii_digit()) && !raw.is_empty() {
                if let Ok(value) = raw.parse::<usize>() {
                    limit = value;
                }
            } else if arg == "-h" || arg == "--help" {
                return if from_start { head_help() } else { tail_help() };
            } else {
                return format!("{}: unknown option '{arg}'\n", if from_start { "head" } else { "tail" });
            }
        } else {
            files.push(arg.as_str());
        }
        idx += 1;
    }

    if files.is_empty() {
        return if from_start { head_help() } else { tail_help() };
    }

    let mut out = String::new();
    let show_headers = files.len() > 1;
    for file in files {
        match vfs.cat(file) {
            Ok(content) => {
                if show_headers {
                    out.push_str(&format!("==> {file} <==\n"));
                }
                let lines: Vec<_> = content.lines().collect();
                let selected: Vec<_> = if from_start {
                    lines.into_iter().take(limit).collect()
                } else {
                    let start = lines.len().saturating_sub(limit);
                    lines.into_iter().skip(start).collect()
                };
                for line in selected {
                    out.push_str(line);
                    out.push('\n');
                }
            }
            Err(err) => out.push_str(&format!("{}: {file}: {err}\n", if from_start { "head" } else { "tail" })),
        }
    }
    out
}

fn walk_find(
    path: &Path,
    node: &VfsNode,
    depth: usize,
    name_pattern: Option<&str>,
    type_filter: Option<NodeKind>,
    max_depth: Option<usize>,
    out: &mut String,
) {
    if max_depth.is_some_and(|max| depth > max) {
        return;
    }

    let node_kind = match node {
        VfsNode::File { .. } => NodeKind::File,
        VfsNode::Dir { .. } => NodeKind::Dir,
    };
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("/");
    let name_matches = name_pattern.is_none_or(|pattern| wildcard_match(pattern, name));
    let type_matches = type_filter.is_none_or(|expected| expected == node_kind);

    if name_matches && type_matches {
        out.push_str(&display_path(path));
        out.push('\n');
    }

    if let VfsNode::Dir { children, .. } = node {
        let mut names: Vec<_> = children.keys().cloned().collect();
        names.sort();
        for child in names {
            walk_find(
                &path.join(&child),
                &children[&child],
                depth + 1,
                name_pattern,
                type_filter,
                max_depth,
                out,
            );
        }
    }
}

fn wildcard_match(pattern: &str, value: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some((prefix, suffix)) = pattern.split_once('*') {
        value.starts_with(prefix) && value.ends_with(suffix)
    } else {
        pattern == value
    }
}

fn display_path(path: &Path) -> String {
    if path == Path::new("") {
        "/".to_string()
    } else {
        let raw = PathBuf::from(path).display().to_string();
        if raw.is_empty() {
            "/".to_string()
        } else {
            raw
        }
    }
}

fn grep_help() -> String {
    "usage: grep [-i] [-n] [-c] <pattern> <file>...\n".to_string()
}

fn wc_help() -> String {
    "usage: wc [-l] [-w] [-c] <file>...\n".to_string()
}

fn head_help() -> String {
    "usage: head [-n count|-count] <file>...\n".to_string()
}

fn tail_help() -> String {
    "usage: tail [-n count|-count] <file>...\n".to_string()
}

fn find_help() -> String {
    "usage: find [path] [-name pattern] [-type f|d] [-maxdepth n]\n".to_string()
}

#[cfg(test)]
mod tests {
    use super::{find, grep, head, tail, wc};
    use crate::kernel::Vfs;

    #[test]
    fn grep_wc_head_tail_read_vfs_files() {
        let mut vfs = Vfs::new();
        vfs.write_file("/home/log.txt", "alpha\nbeta\nalpha beta\n", false)
            .unwrap();
        assert!(grep(&vfs, &["alpha".to_string(), "/home/log.txt".to_string()]).contains("alpha"));
        assert!(grep(&vfs, &["-c".to_string(), "alpha".to_string(), "/home/log.txt".to_string()]).contains("2"));
        assert!(wc(&vfs, &["/home/log.txt".to_string()]).contains("/home/log.txt"));
        assert_eq!(head(&vfs, &["-1".to_string(), "/home/log.txt".to_string()]), "alpha\n");
        assert_eq!(tail(&vfs, &["-1".to_string(), "/home/log.txt".to_string()]), "alpha beta\n");
    }

    #[test]
    fn find_supports_name_and_type_filters() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/home/docs").unwrap();
        vfs.write_file("/home/docs/a.txt", "ok", false).unwrap();
        let out = find(
            &vfs,
            &[
                "/home".to_string(),
                "-name".to_string(),
                "*.txt".to_string(),
                "-type".to_string(),
                "f".to_string(),
            ],
        );
        assert!(out.contains("/home/docs/a.txt"));
    }
}
