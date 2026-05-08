const SENSITIVE_KEYS: &[&str] = &[
    "password",
    "passwd",
    "pwd",
    "secret",
    "token",
    "api_key",
    "api-key",
    "apikey",
    "access_token",
    "refresh_token",
    "client_secret",
    "private_key",
    "aws_access_key_id",
    "aws_secret_access_key",
    "cookie",
    "session",
    "authorization",
];

const KNOWN_TOKEN_PREFIXES: &[&str] = &[
    "github_pat_",
    "ghp_",
    "gho_",
    "ghu_",
    "ghs_",
    "ghr_",
];

const SECRET_FLAGS: &[&str] = &[
    "--password",
    "--passwd",
    "--secret",
    "--token",
    "--api-key",
    "--api_key",
    "--access-token",
    "--refresh-token",
    "--client-secret",
    "--private-key",
    "-p",
];

pub fn has_sensitive_marker(raw: &str) -> bool {
    let lower = raw.to_ascii_lowercase();
    SENSITIVE_KEYS
        .iter()
        .any(|key| lower.contains(key))
        || KNOWN_TOKEN_PREFIXES
            .iter()
            .any(|prefix| lower.contains(prefix))
        || lower.contains("bearer ")
        || lower.contains("authorization:")
        || lower.contains("-----begin ") && lower.contains("private key-----")
        || contains_url_credentials(raw)
}

pub fn redact_multiline(raw: &str) -> String {
    if raw.is_empty() {
        return String::new();
    }
    let mut out = raw.lines().map(redact_line).collect::<Vec<_>>().join("\n");
    if raw.ends_with('\n') {
        out.push('\n');
    }
    out
}

pub fn redact_line(raw: &str) -> String {
    let no_controls = raw.chars().filter(|ch| !ch.is_control() || *ch == '\t').collect::<String>();
    if contains_private_key_marker(&no_controls) {
        return "[redacted-private-key]".to_string();
    }
    redact_tokens(&redact_url_credentials(&no_controls))
}

pub fn redact_url_credentials(line: &str) -> String {
    let mut output = String::new();
    let mut rest = line;

    while let Some(scheme_index) = rest.find("://") {
        let credential_start = scheme_index + 3;
        let after_scheme = &rest[credential_start..];
        let Some(at_index) = after_scheme.find('@') else {
            break;
        };

        let credential_candidate = &after_scheme[..at_index];
        let looks_like_credentials = credential_candidate.contains(':')
            && credential_candidate
                .chars()
                .all(|ch| !ch.is_whitespace() && ch != '/');

        if looks_like_credentials {
            output.push_str(&rest[..credential_start]);
            output.push_str("[redacted]@");
            rest = &after_scheme[at_index + 1..];
        } else {
            let copy_end = credential_start + at_index + 1;
            output.push_str(&rest[..copy_end]);
            rest = &rest[copy_end..];
        }
    }

    output.push_str(rest);
    output
}

fn redact_tokens(line: &str) -> String {
    let mut out = Vec::new();
    let mut redact_next = false;
    for token in line.split_whitespace() {
        if redact_next {
            out.push("[redacted-secret]".to_string());
            redact_next = false;
            continue;
        }

        let lower = token.to_ascii_lowercase();
        if is_secret_flag(&lower) {
            out.push(token.to_string());
            redact_next = true;
            continue;
        }
        if lower == "bearer" {
            out.push("Bearer".to_string());
            redact_next = true;
            continue;
        }
        if lower == "authorization:" || lower == "authorization" {
            out.push("Authorization:".to_string());
            redact_next = true;
            continue;
        }

        out.push(redact_token(token));
    }
    out.join(" ")
}

fn redact_token(token: &str) -> String {
    let lower = token.to_ascii_lowercase();
    if contains_private_key_marker(&lower) {
        return "[redacted-private-key]".to_string();
    }
    if KNOWN_TOKEN_PREFIXES
        .iter()
        .any(|prefix| lower.contains(prefix))
    {
        return "[redacted-token]".to_string();
    }
    if has_sensitive_assignment(&lower) {
        return redact_assignment(token);
    }
    token.to_string()
}

fn redact_assignment(token: &str) -> String {
    let Some((delimiter_index, delimiter)) = find_assignment_delimiter(token) else {
        return "[redacted-secret]".to_string();
    };
    let key = token[..delimiter_index]
        .trim_matches(|ch: char| matches!(ch, '"' | '\'' | '{' | '[' | ',' | ' '))
        .to_ascii_lowercase();
    if !is_sensitive_key(&key) {
        return token.to_string();
    }

    let prefix = &token[..delimiter_index + delimiter.len_utf8()];
    let suffix = token
        .chars()
        .rev()
        .take_while(|ch| matches!(ch, ',' | ';' | ')' | '}' | ']' | '"' | '\''))
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    format!("{prefix}[redacted-secret]{suffix}")
}

fn find_assignment_delimiter(token: &str) -> Option<(usize, char)> {
    let eq = token.find('=');
    let colon = token.find(':');
    match (eq, colon) {
        (Some(a), Some(b)) if a < b => Some((a, '=')),
        (Some(_), Some(b)) => Some((b, ':')),
        (Some(a), None) => Some((a, '=')),
        (None, Some(b)) => Some((b, ':')),
        (None, None) => None,
    }
}

fn has_sensitive_assignment(lower: &str) -> bool {
    let Some((delimiter_index, _)) = find_assignment_delimiter(lower) else {
        return false;
    };
    let key = lower[..delimiter_index]
        .trim_matches(|ch: char| matches!(ch, '"' | '\'' | '{' | '[' | ',' | ' '));
    is_sensitive_key(key)
}

fn is_sensitive_key(key: &str) -> bool {
    SENSITIVE_KEYS.iter().any(|candidate| key.ends_with(candidate))
}

fn is_secret_flag(lower: &str) -> bool {
    SECRET_FLAGS.iter().any(|flag| lower == *flag || lower.starts_with(&format!("{flag}=")))
}

fn contains_private_key_marker(raw: &str) -> bool {
    let lower = raw.to_ascii_lowercase();
    lower.contains("-----begin ") && lower.contains("private key-----")
}

fn contains_url_credentials(line: &str) -> bool {
    let mut rest = line;
    while let Some(scheme_index) = rest.find("://") {
        let credential_start = scheme_index + 3;
        let after_scheme = &rest[credential_start..];
        let Some(at_index) = after_scheme.find('@') else {
            return false;
        };
        let credential_candidate = &after_scheme[..at_index];
        if credential_candidate.contains(':')
            && credential_candidate
                .chars()
                .all(|ch| !ch.is_whitespace() && ch != '/')
        {
            return true;
        }
        rest = &after_scheme[at_index + 1..];
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{has_sensitive_marker, redact_line, redact_multiline, redact_url_credentials};

    #[test]
    fn redacts_assignments_json_tokens_and_headers() {
        let line = "token=abc \"password\":\"hunter2\", Authorization: Bearer ghp_secret";
        let out = redact_line(line);
        assert!(out.contains("token=[redacted-secret]"));
        assert!(out.contains("\"password\":[redacted-secret]\","));
        assert!(out.contains("Authorization: [redacted-secret]"));
        assert!(!out.contains("hunter2"));
        assert!(!out.contains("ghp_secret"));
    }

    #[test]
    fn redacts_cli_flag_values_and_url_credentials() {
        let out = redact_line("login --token ghp_secret clone https://user:pass@example.com/repo.git");
        assert!(out.contains("--token [redacted-secret]"));
        assert!(out.contains("https://[redacted]@example.com/repo.git"));
        assert!(!out.contains("ghp_secret"));
        assert!(!out.contains("user:pass"));
    }

    #[test]
    fn redacts_private_key_markers_without_echoing_material() {
        let out = redact_multiline("ok\n-----BEGIN PRIVATE KEY-----\nsecret-body\n");
        assert!(out.contains("ok"));
        assert!(out.contains("[redacted-private-key]"));
        assert!(!out.contains("BEGIN PRIVATE KEY"));
    }

    #[test]
    fn detects_sensitive_markers() {
        assert!(has_sensitive_marker("Authorization: Bearer abc"));
        assert!(has_sensitive_marker("https://user:pass@example.com/repo.git"));
        assert!(!has_sensitive_marker("phase1 sysinfo"));
    }

    #[test]
    fn redacts_url_credentials_without_hiding_safe_urls() {
        assert_eq!(
            redact_url_credentials("https://user:pass@example.com/repo.git"),
            "https://[redacted]@example.com/repo.git"
        );
        assert_eq!(
            redact_url_credentials("https://github.com/Bryforge/phase1.git"),
            "https://github.com/Bryforge/phase1.git"
        );
    }
}
