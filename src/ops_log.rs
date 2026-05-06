use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::panic;
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

pub const LOG_PATH: &str = "phase1.log";
const ROTATED_LOG_PATH: &str = "phase1.log.1";
const MAX_LOG_BYTES: u64 = 256 * 1024;
static PANIC_HOOK: Once = Once::new();

pub fn install_panic_hook() {
    PANIC_HOOK.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            let location = info
                .location()
                .map(|loc| format!("{}:{}", loc.file(), loc.line()))
                .unwrap_or_else(|| "unknown".to_string());
            let payload = info
                .payload()
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
                .unwrap_or("panic payload unavailable");
            log_error("panic", &format!("{location} {payload}"));
            previous(info);
        }));
    });
}

pub fn log_event(kind: &str, detail: &str) {
    let _ = append(kind, detail);
}

pub fn log_error(kind: &str, detail: &str) {
    let _ = append(&format!("error.{kind}"), detail);
}

pub fn log_command(command: &str) {
    let trimmed = command.trim();
    if !trimmed.is_empty() {
        let _ = append("command", trimmed);
    }
}

pub fn run(args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status") => status(),
        Some("path") => format!("{LOG_PATH}\n"),
        Some("tail") | Some("show") | Some("read") => {
            let count = args
                .get(1)
                .and_then(|raw| raw.parse::<usize>().ok())
                .unwrap_or(25)
                .clamp(1, 200);
            tail(count)
        }
        Some("clear") | Some("reset") => match clear() {
            Ok(()) => "ops log: cleared phase1.log\n".to_string(),
            Err(err) => format!("ops log: clear failed: {err}\n"),
        },
        Some("help") | Some("-h") | Some("--help") => help(),
        Some(other) => format!("ops log: unknown option '{other}'\n{}", help()),
    }
}

fn append(kind: &str, detail: &str) -> io::Result<()> {
    rotate_if_needed()?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)?;
    writeln!(
        file,
        "{} [{}] {}",
        timestamp(),
        sanitize(kind),
        sanitize(detail)
    )
}

fn rotate_if_needed() -> io::Result<()> {
    let Ok(meta) = fs::metadata(LOG_PATH) else {
        return Ok(());
    };
    if meta.len() <= MAX_LOG_BYTES {
        return Ok(());
    }
    let _ = fs::remove_file(ROTATED_LOG_PATH);
    fs::rename(LOG_PATH, ROTATED_LOG_PATH)
}

fn status() -> String {
    let size = fs::metadata(LOG_PATH).map(|meta| meta.len()).unwrap_or(0);
    let rotated = fs::metadata(ROTATED_LOG_PATH)
        .map(|meta| format!("{} bytes", meta.len()))
        .unwrap_or_else(|_| "none".to_string());
    format!(
        "phase1 ops log\npath       : {LOG_PATH}\nsize       : {size} bytes\nrotation   : {ROTATED_LOG_PATH} ({rotated})\ncommands   : opslog tail | opslog clear | opslog path\nprivacy    : credential-like strings are redacted before write\n"
    )
}

fn tail(count: usize) -> String {
    let Ok(raw) = fs::read_to_string(LOG_PATH) else {
        return format!("phase1 ops log\npath       : {LOG_PATH}\nstatus     : no log file yet\n");
    };
    let lines = raw.lines().collect::<Vec<_>>();
    let start = lines.len().saturating_sub(count);
    let mut out = format!("phase1 ops log tail // last {} lines\n", lines.len() - start);
    for line in &lines[start..] {
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn clear() -> io::Result<()> {
    match fs::remove_file(LOG_PATH) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}

fn help() -> String {
    "usage: opslog [status|tail [n]|path|clear]\n\nThe ops log records boot selections, shell commands, guarded local operations, and panic summaries in phase1.log. It is local-only and redacts common credential-like values.\n".to_string()
}

fn timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("unix:{now}")
}

fn sanitize(raw: &str) -> String {
    raw.split_whitespace()
        .map(sanitize_token)
        .collect::<Vec<_>>()
        .join(" ")
}

fn sanitize_token(token: &str) -> String {
    let lower = token.to_ascii_lowercase();
    if lower.contains("password=")
        || lower.contains("passwd=")
        || lower.contains("secret=")
        || lower.contains("token=")
        || lower.contains("api_key=")
        || lower.contains("apikey=")
    {
        return "[redacted-secret]".to_string();
    }
    let blocked_prefixes = ["ghp_", "gho_", "ghu_", "ghs_", "ghr_", "github_pat_"];
    if blocked_prefixes.iter().any(|prefix| lower.starts_with(prefix)) {
        return "[redacted-token]".to_string();
    }
    if let Some(proto_pos) = token.find("://") {
        let auth_start = proto_pos + 3;
        if let Some(at_offset) = token[auth_start..].find('@') {
            let at = auth_start + at_offset;
            let mut redacted = token.to_string();
            redacted.replace_range(auth_start..at, "[redacted-credential]");
            return redacted;
        }
    }
    token.to_string()
}

#[cfg(test)]
mod tests {
    use super::{run, sanitize_token};

    #[test]
    fn redacts_secret_like_tokens() {
        assert_eq!(sanitize_token("token=example"), "[redacted-secret]");
        assert_eq!(sanitize_token("ghp_example"), "[redacted-token]");
        assert_eq!(
            sanitize_token("https://user:pass@example.com/repo.git"),
            "https://[redacted-credential]@example.com/repo.git"
        );
    }

    #[test]
    fn status_mentions_local_log_path() {
        let out = run(&["status".to_string()]);
        assert!(out.contains("phase1 ops log"));
        assert!(out.contains("phase1.log"));
    }
}
