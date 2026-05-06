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
    complete_at(before, &after)
}

pub fn complete_input_prefix(line: &str) -> TabCompletion {
    complete_at(line, "")
}

fn complete_at(before: &str, after: &str) -> TabCompletion {
    let token_start = current_token_start(before);
    let prefix = &before[token_start..];
    let command_position = is_command_position(before, token_start);
    let matches = if command_position {
        command_matches(prefix)
    } else {
        let command = command_for_context(before, token_start).unwrap_or("");
        argument_matches(command, prefix)
    };

    completion_result(before, after, token_start, prefix, matches)
}

fn completion_result(
    before: &str,
    after: &str,
    token_start: usize,
    prefix: &str,
    matches: Vec<String>,
) -> TabCompletion {
    match matches.len() {
        0 => TabCompletion::NoMatch {
            prefix: prefix.to_string(),
        },
        1 => TabCompletion::Completed(format!(
            "{}{}{}",
            &before[..token_start], matches[0], after
        )),
        _ => {
            let common = common_prefix(&matches);
            if common.len() > prefix.len() {
                TabCompletion::Completed(format!("{}{}{}", &before[..token_start], common, after))
            } else {
                TabCompletion::Suggestions {
                    prefix: prefix.to_string(),
                    matches,
                }
            }
        }
    }
}

fn common_prefix(matches: &[String]) -> String {
    let Some(first) = matches.first() else {
        return String::new();
    };
    let mut end = first.len();
    for candidate in matches.iter().skip(1) {
        end = first[..end]
            .char_indices()
            .map(|(idx, _)| idx)
            .chain(std::iter::once(end))
            .take_while(|idx| candidate.starts_with(&first[..*idx]))
            .last()
            .unwrap_or(0);
    }
    first[..end].to_string()
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
    let mut matches = registry::completions(prefix)
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    for builtin in ["reboot", "arena", "game", "doom"] {
        if builtin.starts_with(prefix) {
            matches.push(builtin.to_string());
        }
    }
    matches.sort();
    matches.dedup();
    matches
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
            "latest",
            "now",
            "self-update",
            "bleeding",
            "edge",
            "stable",
            "release",
            "test",
            "tests",
            "devtest",
            "validate",
            "verify",
            "qa",
            "quick",
            "full",
            "smoke",
            "game",
            "fmt",
            "cargo-check",
            "clippy",
            "doctor",
            "--build",
            "--no-build",
            "--trust-host",
        ],
        "wasm" => &["list", "inspect", "run", "validate", "hello-wasi", "arena", "game"],
        "arena" | "doom" => &[
            "start",
            "play",
            "demo",
            "script",
            "roadmap",
            "dev",
            "test-plan",
            "help",
            "quit",
        ],
        "game" => &["status", "files", "roadmap", "test-plan", "version", "help", "arena"],
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
    use super::{complete_input_prefix, complete_tab_line, TabCompletion};

    #[test]
    fn tab_completes_unique_command_prefix() {
        assert_eq!(
            complete_tab_line("vers\t --compare"),
            TabCompletion::Completed("version --compare".to_string())
        );
        assert_eq!(
            complete_input_prefix("spa"),
            TabCompletion::Completed("spawn".to_string())
        );
        assert_eq!(
            complete_input_prefix("rebo"),
            TabCompletion::Completed("reboot".to_string())
        );
        assert_eq!(
            complete_input_prefix("are"),
            TabCompletion::Completed("arena".to_string())
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
        assert_eq!(
            complete_tab_line("update lat\t"),
            TabCompletion::Completed("update latest".to_string())
        );
        assert_eq!(
            complete_tab_line("update test q\t"),
            TabCompletion::Completed("update test quick".to_string())
        );
        assert_eq!(
            complete_tab_line("update doct\t"),
            TabCompletion::Completed("update doctor".to_string())
        );
        assert_eq!(
            complete_tab_line("update --trust\t"),
            TabCompletion::Completed("update --trust-host".to_string())
        );
        assert_eq!(
            complete_tab_line("arena dem\t"),
            TabCompletion::Completed("arena demo".to_string())
        );
        match complete_tab_line("arena de\t") {
            TabCompletion::Suggestions { prefix, matches } => {
                assert_eq!(prefix, "de");
                assert!(matches.contains(&"demo".to_string()));
                assert!(matches.contains(&"dev".to_string()));
            }
            other => panic!("expected arena de suggestions, got {other:?}"),
        }
        assert_eq!(
            complete_tab_line("game test-p\t"),
            TabCompletion::Completed("game test-plan".to_string())
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
