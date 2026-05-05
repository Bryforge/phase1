use std::io;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::kernel::VERSION;

pub struct Browser;

impl Browser {
    pub fn new() -> Self {
        Self
    }

    pub fn browse(&self, raw_url: &str) -> String {
        let url = raw_url.trim();
        if url.is_empty() || matches!(url, "about" | "about:blank") {
            return self.about();
        }
        if url == "phase1" || url.contains("Bryforge/phase1") || url.contains("bryforge/phase1") {
            return self.phase1_page();
        }
        if !allowed_url(url) {
            return "browser: only http:// and https:// URLs are allowed".to_string();
        }

        let mut cmd = Command::new("curl");
        cmd.args([
            "--location",
            "--fail",
            "--show-error",
            "--silent",
            "--max-time",
            "10",
            "--max-filesize",
            "1048576",
            "--proto",
            "=http,https",
            "--user-agent",
            "phase1-browser/3.5.0",
            url,
        ]);

        match run_with_timeout(cmd, Duration::from_secs(12)) {
            Ok(output) if output.status.success() => {
                let html = String::from_utf8_lossy(&output.stdout);
                self.render_text(&html)
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if stderr.trim().is_empty() {
                    format!("browser: request failed with status {}", output.status)
                } else {
                    format!("browser: {}", stderr.trim())
                }
            }
            Err(err) => format!("browser: {}", err),
        }
    }

    fn about(&self) -> String {
        format!(
            "phase1 browser v{}\n\ncommands:\n  browser about\n  browser phase1\n  browser https://example.com\n\nsafety:\n  schemes: http, https only\n  timeout: 12s\n  max download: 1 MiB",
            VERSION
        )
    }

    fn phase1_page(&self) -> String {
        format!(
            "phase1 v{}\nTerminal-first virtual OS console.\nUse: help, man browser, ps, audit, ls /",
            VERSION
        )
    }

    fn render_text(&self, html: &str) -> String {
        let mut text = String::with_capacity(html.len());
        let mut tag = String::new();
        let mut in_tag = false;
        let mut in_script = false;
        let mut in_style = false;

        for ch in html.chars() {
            if ch == '<' {
                in_tag = true;
                tag.clear();
                continue;
            }
            if in_tag {
                if ch == '>' {
                    let normalized = tag.trim().to_ascii_lowercase();
                    let closing = normalized.starts_with('/');
                    let name = normalized.trim_start_matches('/').split_whitespace().next().unwrap_or("");
                    match (closing, name) {
                        (false, "script") => in_script = true,
                        (true, "script") => in_script = false,
                        (false, "style") => in_style = true,
                        (true, "style") => in_style = false,
                        (_, "br" | "p" | "div" | "li" | "tr" | "h1" | "h2" | "h3") => text.push('\n'),
                        _ => {}
                    }
                    in_tag = false;
                } else {
                    tag.push(ch);
                }
                continue;
            }
            if !in_script && !in_style {
                text.push(ch);
            }
        }

        let decoded = decode_entities(&text);
        let mut cleaned = String::new();
        let mut previous_blank = false;
        for line in decoded.lines().map(str::trim) {
            if line.is_empty() {
                if !previous_blank {
                    cleaned.push('\n');
                }
                previous_blank = true;
            } else {
                cleaned.push_str(line);
                cleaned.push('\n');
                previous_blank = false;
            }
        }
        let trimmed = cleaned.trim();
        if trimmed.is_empty() {
            "browser: fetched page but no readable text was found".to_string()
        } else {
            trimmed.to_string()
        }
    }
}

impl Default for Browser {
    fn default() -> Self {
        Self::new()
    }
}

fn allowed_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}

fn decode_entities(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

fn run_with_timeout(mut cmd: Command, timeout: Duration) -> io::Result<Output> {
    let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let start = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output();
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::new(io::ErrorKind::TimedOut, "command timed out"));
        }
        thread::sleep(Duration::from_millis(25));
    }
}

#[cfg(test)]
mod tests {
    use super::Browser;

    #[test]
    fn browser_strips_script_text() {
        let text = Browser::new().render_text("<h1>Hello</h1><script>bad()</script><p>World &amp; ok</p>");
        assert!(text.contains("Hello"));
        assert!(text.contains("World & ok"));
        assert!(!text.contains("bad"));
    }
}
