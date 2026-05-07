const SECRET_KEY_MARKERS: &[&str] = &[
    "authorization",
    "auth_token",
    "access_token",
    "refresh_token",
    "id_token",
    "api_key",
    "apikey",
    "client_secret",
    "password",
    "passwd",
    "secret",
    "token",
    "credential",
    "private_key",
    "ssh_key",
];

const TOKEN_PREFIXES: &[&str] = &[
    "ghp_",
    "gho_",
    "ghu_",
    "ghs_",
    "ghr_",
    "github_pat_",
    "glpat-",
    "sk-",
    "xoxb-",
    "xoxp-",
    "xoxa-",
];

pub fn sanitize_line(raw: &str) -> String {
    raw.split_whitespace()
        .map(sanitize_token)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn sanitize_multiline(raw: &str) -> String {
    let mut out = raw.lines().map(sanitize_line).collect::<Vec<_>>().join("\n");
    if raw.ends_with('\n') || !out.is_empty() {
        out.push('\n');
    }
    out
}

pub fn sanitize_token(token: &str) -> String {
    if token.is_empty() {
        return token.to_string();
    }

    if let Some(redacted) = sanitize_url_credentials(token) {
        return redacted;
    }

    if looks_like_assignment_secret(token) || looks_like_header_secret(token) {
        return "[redacted-secret]".to_string();
    }

    let lower = token.to_ascii_lowercase();
    if TOKEN_PREFIXES.iter().any(|prefix| lower.starts_with(prefix)) {
        return "[redacted-token]".to_string();
    }

    if looks_like_long_bearer(token) {
        return "[redacted-token]".to_string();
    }

    token.to_string()
}

pub fn sanitize_env_value(key: &str, value: &str) -> String {
    let key_lower = key.to_ascii_lowercase();
    if SECRET_KEY_MARKERS
        .iter()
        .any(|marker| key_lower.contains(marker))
    {
        "[redacted-secret]".to_string()
    } else {
        sanitize_line(value)
    }
}

fn looks_like_assignment_secret(token: &str) -> bool {
    let Some((key, value)) = token.split_once('=') else {
        return false;
    };
    !value.is_empty()
        && SECRET_KEY_MARKERS
            .iter()
            .any(|marker| key.to_ascii_lowercase().contains(marker))
}

fn looks_like_header_secret(token: &str) -> bool {
    let lower = token.to_ascii_lowercase();
    lower.starts_with("authorization:")
        || lower.starts_with("x-api-key:")
        || lower.starts_with("proxy-authorization:")
        || lower.starts_with("cookie:")
        || lower.starts_with("set-cookie:")
}

fn looks_like_long_bearer(token: &str) -> bool {
    token.len() >= 32
        && token
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.' | '=' | '/'))
        && token.chars().any(|ch| ch.is_ascii_uppercase())
        && token.chars().any(|ch| ch.is_ascii_lowercase())
        && token.chars().any(|ch| ch.is_ascii_digit())
}

fn sanitize_url_credentials(token: &str) -> Option<String> {
    let proto_pos = token.find("://")?;
    let auth_start = proto_pos + 3;
    let at_offset = token[auth_start..].find('@')?;
    let at = auth_start + at_offset;
    if at == auth_start {
        return None;
    }
    let mut redacted = token.to_string();
    redacted.replace_range(auth_start..at, "[redacted-credential]");
    Some(redacted)
}

#[cfg(test)]
mod tests {
    use super::{sanitize_env_value, sanitize_line, sanitize_multiline, sanitize_token};

    #[test]
    fn redacts_assignment_and_header_secrets() {
        assert_eq!(sanitize_token("token=example"), "[redacted-secret]");
        assert_eq!(sanitize_token("PASSWORD=hunter2"), "[redacted-secret]");
        assert_eq!(sanitize_token("Authorization:"), "[redacted-secret]");
    }

    #[test]
    fn redacts_common_token_prefixes_and_url_credentials() {
        assert_eq!(sanitize_token("ghp_example"), "[redacted-token]");
        assert_eq!(sanitize_token("github_pat_example"), "[redacted-token]");
        assert_eq!(
            sanitize_token("https://user:pass@example.com/repo.git"),
            "https://[redacted-credential]@example.com/repo.git"
        );
    }

    #[test]
    fn redacts_sensitive_environment_values_by_key() {
        assert_eq!(
            sanitize_env_value("GITHUB_TOKEN", "abc123"),
            "[redacted-secret]"
        );
        assert_eq!(sanitize_env_value("PHASE1_THEME", "matrix"), "matrix");
    }

    #[test]
    fn sanitizes_lines_and_multiline_text() {
        assert_eq!(sanitize_line("ok token=secret"), "ok [redacted-secret]");
        let out = sanitize_multiline("ok\nAuthorization: bearer nope\n");
        assert!(out.contains("ok"));
        assert!(out.contains("[redacted-secret] bearer nope"));
        assert!(!out.contains("Authorization:"));
    }
}
