use crate::registry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TabCompletion {
    Unchanged(String),
    Completed(String),
    Suggestions { prefix: String, matches: Vec<String> },
    NoMatch { prefix: String },
}

pub fn complete_tab_line(line: &str) -> TabCompletion {
    let Some(tab_idx) = line.find('\t') else {
        return TabCompletion::Unchanged(line.to_string());
    };

    let before = &line[..tab_idx];
    let after = line[tab_idx + 1..].replace('\t', "");
    let token_start = current_token_start(before);
    let prefix = &before[token_start..];
    let command_position = is_command_position(before, token_start);
    let matches = if command_position {
        command_matches(prefix)
    } else {
        let command = command_for_context(before, token_start).unwrap_or("");
        argument_matches(command, prefix)
    };

    match matches.len() {
        0 => TabCompletion::NoMatch {
            prefix: prefix.to_string(),
        },
        1 => TabCompletion::Completed(format!(
            "{}{}{}",
            &before[..token_start], matches[0], after
        )),
        _ => TabCompletion::Suggestions {
            prefix: prefix.to_string(),
            matches,
        },
    }
}

fn current_token_start(input: &str) -> usize {
    input
        .char_indices()
        .rev()
        .find(|(_, ch)| ch.is_whitespace() || matches!(*ch, '|' | ';' | '&'))
        .map(|(idx, ch)| idx + ch.len_utf8())
        .unwrap_or(0)
}

fn segment_start(input: &str) -> usize {
    let mut start = 0;
    for (idx, ch) in input.char_indices() {
        if matches!(ch, ';' | '|' | '&') {
            start = idx + ch.len_utf8();
        }
    }
    start
}

fn is_command_position(before: &str, token_start: usize) -> bool {
    let start = segment_start(before);
    before[start..token_start].trim().is_empty()
}

fn command_for_context(before: &str, token_start: usize) -> Option<&str> {
    let start = segment_start(before);
    before[start..token_start].split_whitespace().next()
}

fn command_matches(prefix: &str) -> Vec<String> {
    registry::completions(prefix)
        .into_iter()
        .map(str::to_string)
        .collect()
}

fn argument_matches(command: &str, prefix: &str) -> Vec<String> {
    let canonical = registry::canonical_name(command).unwrap_or(command);
    let options: &[&str] = match canonical {
        "theme" => &[
            "show",
            "list",
            "rainbow",
            "matrix",
            "cyber",
            "amber",
            "ice",
            "synthwave",
            "crimson",
            "mono",
            "ascii",
            "reset",
        ],
        "banner" => &[
            "mobile",
            "desktop",
            "mono",
            "rainbow",
            "matrix",
            "cyber",
            "amber",
            "ice",
            "synthwave",
            "crimson",
            "ascii",
            "safe",
            "host",
            "persist",
            "edge",
            "bleeding-edge",
        ],
        "update" => &[
            "plan",
            "check",
            "status",
            "execute",
            "--execute",
            "protocol",
            "bleeding",
            "edge",
            "stable",
            "release",
            "--build",
            "--no-build",
        ],
        "wasm" => &["list", "inspect", "run", "validate", "hello-wasi"],
        "history" => &["list", "status", "path", "save", "clear"],
        "bootcfg" => &["show", "save", "reset", "defaults", "path", "state", "help"],
        "matrix" => &["0", "forever", "--speed", "--density", "--chars"],
        "grep" => &["-i", "-n", "-c", "--help"],
        "wc" => &["-l", "-w", "-c", "--help"],
        "head" | "tail" => &["-n", "--lines", "--help"],
        "man" | "complete" => return command_matches(prefix),
        _ => &[],
    };
    matches_from(options, prefix)
}

fn matches_from(options: &[&str], prefix: &str) -> Vec<String> {
    let mut matches = options
        .iter()
        .copied()
        .filter(|candidate| candidate.starts_with(prefix))
        .map(str::to_string)
        .collect::<Vec<_>>();
    matches.sort();
    matches.dedup();
    matches
}

#[cfg(test)]
mod tests {
    use super::{complete_tab_line, TabCompletion};

    #[test]
    fn tab_completes_unique_command_prefix() {
        assert_eq!(
            complete_tab_line("vers\t --compare"),
            TabCompletion::Completed("version --compare".to_string())
        );
    }

    #[test]
    fn tab_completes_common_argument_prefixes() {
        assert_eq!(
            complete_tab_line("theme ma\t"),
            TabCompletion::Completed("theme matrix".to_string())
        );
        assert_eq!(
            complete_tab_line("update pr\t"),
            TabCompletion::Completed("update protocol".to_string())
        );
    }

    #[test]
    fn tab_completes_commands_after_pipeline_separator() {
        assert_eq!(
            complete_tab_line("echo hi | gr\t"),
            TabCompletion::Completed("echo hi | grep".to_string())
        );
    }

    #[test]
    fn tab_lists_ambiguous_prefixes() {
        match complete_tab_line("w\t") {
            TabCompletion::Suggestions { matches, .. } => {
                assert!(matches.contains(&"wasm".to_string()));
                assert!(matches.contains(&"wc".to_string()));
            }
            other => panic!("expected suggestions, got {other:?}"),
        }
    }
}
