use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::PathBuf;

pub const HISTORY_LIMIT: usize = 512;
pub const DEFAULT_HISTORY_PATH: &str = "phase1.history";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HistoryStore {
    Disabled,
    Disk(PathBuf),
}

impl HistoryStore {
    pub fn from_env(persistent_state: bool) -> Self {
        match std::env::var("PHASE1_HISTORY") {
            Ok(value) if value.eq_ignore_ascii_case("off") => Self::Disabled,
            Ok(value) if !value.trim().is_empty() => Self::Disk(PathBuf::from(value)),
            _ if persistent_state => Self::Disk(PathBuf::from(DEFAULT_HISTORY_PATH)),
            _ => Self::Disabled,
        }
    }

    pub fn describe(&self) -> String {
        match self {
            Self::Disabled => "off".to_string(),
            Self::Disk(path) => path.display().to_string(),
        }
    }

    pub fn load(&self, history: &mut VecDeque<String>) -> io::Result<usize> {
        let Self::Disk(path) = self else {
            return Ok(0);
        };
        if !path.exists() {
            return Ok(0);
        }
        let raw = fs::read_to_string(path)?;
        let mut loaded = 0;
        for line in raw.lines().filter(|line| !line.trim().is_empty()) {
            let command = if let Some(encoded) = line.strip_prefix("H\t") {
                let bytes = decode_hex(encoded)
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
                String::from_utf8(bytes)
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?
            } else if line.starts_with('#') {
                continue;
            } else {
                line.to_string()
            };
            push_bounded(history, &command);
            loaded += 1;
        }
        Ok(loaded)
    }

    pub fn save(&self, history: &VecDeque<String>) -> io::Result<usize> {
        let Self::Disk(path) = self else {
            return Ok(0);
        };
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent)?;
        }
        let mut out = String::from("# phase1 persistent history v1\n");
        out.push_str(
            "# command history is sanitized before write; do not type secrets into commands\n",
        );
        let mut written = 0;
        for line in history.iter().filter(|line| !line.trim().is_empty()) {
            out.push_str("H\t");
            out.push_str(&encode_hex(sanitize_line(line).as_bytes()));
            out.push('\n');
            written += 1;
        }
        fs::write(path, out)?;
        Ok(written)
    }

    pub fn clear(&self, history: &mut VecDeque<String>) -> io::Result<()> {
        history.clear();
        match self {
            Self::Disabled => Ok(()),
            Self::Disk(path) if path.exists() => fs::write(path, ""),
            Self::Disk(_) => Ok(()),
        }
    }
}

pub fn push_bounded(history: &mut VecDeque<String>, line: &str) {
    if line.trim().is_empty() {
        return;
    }
    history.push_back(line.to_string());
    while history.len() > HISTORY_LIMIT {
        history.pop_front();
    }
}

pub fn status(history_len: usize, store: &HistoryStore) -> String {
    format!(
        "history entries     : {}\npersistent history  : {}\nhistory file        : {}\nprivacy             : persisted history is sanitized; password/token/secret-like commands are redacted\n",
        history_len,
        if matches!(store, HistoryStore::Disk(_)) { "on" } else { "off" },
        store.describe()
    )
}

pub fn list(history: &VecDeque<String>, limit: Option<usize>) -> String {
    let limit = limit.unwrap_or(HISTORY_LIMIT).min(HISTORY_LIMIT);
    let start = history.len().saturating_sub(limit);
    let mut out = String::new();
    for (idx, line) in history.iter().enumerate().skip(start) {
        out.push_str(&format!("{:>4} {}\n", idx + 1, line));
    }
    if out.is_empty() {
        out.push_str("history: empty\n");
    }
    out
}

pub fn sanitize_line(line: &str) -> String {
    let trimmed = line.trim();
    let lower = trimmed.to_ascii_lowercase();
    let risky_words = [
        "password",
        "passwd",
        "token",
        "secret",
        "credential",
        "cookie",
        "recovery",
        "apikey",
        "api_key",
        "private_key",
        "github_pat_",
        "ghp_",
        "gho_",
        "ghu_",
        "ghs_",
        "ghr_",
    ];
    if risky_words.iter().any(|word| lower.contains(word)) {
        return "[redacted-sensitive-command]".to_string();
    }

    let mut sanitized = Vec::new();
    for part in trimmed.split_whitespace() {
        let lower_part = part.to_ascii_lowercase();
        if lower_part.contains("authorization:")
            || lower_part.starts_with("bearer=")
            || lower_part.starts_with("key=")
            || lower_part.starts_with("pass=")
            || lower_part.starts_with("pwd=")
        {
            sanitized.push("[redacted]".to_string());
        } else {
            sanitized.push(part.to_string());
        }
    }
    sanitized.join(" ")
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn decode_hex(raw: &str) -> Result<Vec<u8>, String> {
    if !raw.len().is_multiple_of(2) {
        return Err("hex payload has odd length".to_string());
    }
    let mut bytes = Vec::with_capacity(raw.len() / 2);
    let chars: Vec<_> = raw.bytes().collect();
    for pair in chars.chunks(2) {
        let high = hex_value(pair[0]).ok_or_else(|| "invalid hex payload".to_string())?;
        let low = hex_value(pair[1]).ok_or_else(|| "invalid hex payload".to_string())?;
        bytes.push((high << 4) | low);
    }
    Ok(bytes)
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{list, push_bounded, sanitize_line, status, HistoryStore, DEFAULT_HISTORY_PATH, HISTORY_LIMIT};
    use std::collections::VecDeque;
    use std::path::PathBuf;

    #[test]
    fn bounded_history_keeps_recent_entries() {
        let mut history = VecDeque::new();
        for idx in 0..(HISTORY_LIMIT + 3) {
            push_bounded(&mut history, &format!("cmd-{idx}"));
        }
        assert_eq!(history.len(), HISTORY_LIMIT);
        assert_eq!(history.front().map(String::as_str), Some("cmd-3"));
    }

    #[test]
    fn disabled_history_describes_as_off() {
        assert_eq!(HistoryStore::Disabled.describe(), "off");
    }

    #[test]
    fn persistent_state_enables_default_disk_history() {
        std::env::remove_var("PHASE1_HISTORY");
        assert_eq!(
            HistoryStore::from_env(true),
            HistoryStore::Disk(PathBuf::from(DEFAULT_HISTORY_PATH))
        );
    }

    #[test]
    fn sanitizer_redacts_secret_like_commands() {
        assert_eq!(
            sanitize_line("wifi-connect home password123"),
            "[redacted-sensitive-command]"
        );
        assert_eq!(
            sanitize_line("export API_TOKEN=abc"),
            "[redacted-sensitive-command]"
        );
        assert_eq!(sanitize_line("echo hello world"), "echo hello world");
    }

    #[test]
    fn shell_free_status_and_list_work() {
        let mut history = VecDeque::new();
        push_bounded(&mut history, "help");
        assert!(status(history.len(), &HistoryStore::Disabled).contains("history entries"));
        assert!(list(&history, None).contains("help"));
    }
}
