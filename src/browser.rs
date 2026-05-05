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

    pub fn browse(&self, url: &str) -> String {
        let url = url.trim();

        if url.is_empty() || matches!(url, "about" | "about:blank") {
            return self.internal_about();
        }

        if url == "phase1" || url.contains("bryforge/phase1") {
            return self.internal_phase1();
        }

        if !is_allowed_url(url) {
            return "browser: only http:// and https:// URLs are allowed. Refusing local files and unsupported schemes.".to_string();
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
            "phase1-browser/3.3.2",
            url,
        ]);

        match run_with_timeout(cmd, Duration::from_secs(12)) {
            Ok(output) if output.status.success() => {
                let content = String::from_utf8_lossy(&output.stdout);
                self.strip_html(&content)
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!(
                    "Request failed with status: {}\n{}",
                    output.status,
                    stderr.trim()
                )
            }
            Err(err) => format!("Failed to fetch URL: {}", err),
        }
    }

    fn internal_about(&self) -> String {
        format!(
            "phase1 Terminal Browser v{VERSION}\n\
             ======================================\n\
             Usage:\n\
               browser <https-url>\n\
               browser phase1\n\
               browser about\n\n\
             Fetching is performed through curl with scheme, timeout, and size limits."
        )
    }

    fn internal_phase1(&self) -> String {
        format!(
            "phase1 v{VERSION}\n\
             =================\n\
             Educational OS simulator with in-memory VFS, process scheduler,\n\
             PCIe simulation, Python plugins, C compilation, networking, and a browser.\n\
             Type 'help' or 'man <command>' for details."
        )
    }

    pub fn strip_html(&self, html: &str) -> String {
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
                    let normalized = normalized.trim_start_matches('/').trim();

                    if tag.trim_start().starts_with('/') {
                        if normalized.starts_with("script") {
                            in_script = false;
                        } else if normalized.starts_with("style") {
                            in_style = false;
                        }
                    } else if normalized.starts_with("script") {
                        in_script = true;
                    } else if normalized.starts_with("style") {
                        in_style = true;
                    } else if matches!(
                        normalized,
                        "br" | "hr" | "p" | "div" | "li" | "tr" | "h1" | "h2" | "h3"
                    ) {
                        text.push('\n');
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

        let decoded = decode_html_entities(&text);
        let mut cleaned = String::new();
        let mut last_blank = false;

        for line in decoded.lines().map(str::trim) {
            if line.is_empty() {
                if !last_blank {
                    cleaned.push('\n');
                }
                last_blank = true;
            } else {
                cleaned.push_str(line);
                cleaned.push('\n');
                last_blank = false;
            }
        }

        let cleaned = cleaned.trim();
        if cleaned.is_empty() {
            format!(
                "Fetched content from URL, but no readable text was found.\nRaw preview:\n{}",
                html.chars().take(600).collect::<String>()
            )
        } else {
            cleaned.to_string()
        }
    }
}

fn is_allowed_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}

fn decode_html_entities(input: &str) -> String {
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
    let started = Instant::now();

    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output();
        }

        if started.elapsed() >= timeout {
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
    fn strips_script_and_style() {
        let text = Browser::new().strip_html("<style>x</style><h1>Hello</h1><script>bad()</script><p>World &amp; all</p>");
        assert!(text.contains("Hello"));
        assert!(text.contains("World & all"));
        assert!(!text.contains("bad"));
    }
}
