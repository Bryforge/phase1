// src/browser.rs — Terminal-based browser for phase1 v2.0.1

use std::process::Command;

pub struct Browser;

impl Browser {
    pub fn new() -> Self {
        Browser
    }

    pub fn browse(&self, url: &str) -> String {
        let url = url.trim();

        // Internal pages
        if url.is_empty() || url == "about" || url == "about:blank" {
            return self.internal_about();
        }
        if url == "phase1" || url.contains("bryforge/phase1") {
            return self.internal_phase1();
        }

        // Real web fetch
        let output = Command::new("curl")
            .args(&[
                "-L", "--silent", "--max-time", "10",
                "--user-agent", "phase1-browser/2.0.1",
                url
            ])
            .output();

        match output {
            Ok(o) if o.status.success() => {
                let content = String::from_utf8_lossy(&o.stdout);
                self.strip_html(&content)
            }
            Ok(o) => format!("Request failed with status: {:?}\nTry a different URL or check network.", o.status),
            Err(e) => format!("Failed to execute curl: {}", e),
        }
    }

    fn internal_about(&self) -> String {
        r#"phase1 Terminal Browser v2.0.1
================================

Lightweight web viewer integrated into the phase1 OS simulator.

Usage:
  browser <url>
  browser phase1
  browser about

HTTP fetching is performed via curl on the host system.
Content is automatically converted from HTML to readable terminal text.
All activity stays within the educational simulator environment."#.to_string()
    }

    fn internal_phase1(&self) -> String {
        r#"phase1 v2.0.1 — Educational OS Simulator
=======================================

https://github.com/Bryforge/phase1

Built as a research project demonstrating core OS concepts safely in userspace:
  ## In-memory Virtual File System (VFS)
  ## Preemptive multitasking scheduler
  ## Process management and job control
  ## Hardware simulation (PCIe, CR3/CR4/PCID)
  ## Cross-platform networking stack
  ## Extensible Python plugin system
  ## Built-in editors and C compiler support
  ## Terminal web browser

Everything runs entirely in memory for safe, portable learning.

Type 'help' for the full command reference."#.to_string()
    }

    fn strip_html(&self, html: &str) -> String {
        let mut result = String::with_capacity(html.len());

        let mut in_tag = false;
        let mut in_script = false;
        let mut in_style = false;

        let lower = html.to_lowercase();

        for (i, c) in html.chars().enumerate() {
            if lower[i..].starts_with("<script") {
                in_script = true;
            }
            if lower[i..].starts_with("</script") {
                in_script = false;
            }
            if lower[i..].starts_with("<style") {
                in_style = true;
            }
            if lower[i..].starts_with("</style") {
                in_style = false;
            }

            if in_script || in_style {
                continue;
            }

            match c {
                '<' => in_tag = true,
                '>' if in_tag => in_tag = false,
                _ if !in_tag => {
                    result.push(c);
                }
                _ => {}
            }
        }

        // Clean excessive whitespace
        let cleaned: String = result
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        let final_text = cleaned
            .replace("  ", " ")
            .replace(" \n", "\n")
            .replace("\n\n\n", "\n\n");

        if final_text.trim().is_empty() {
            format!("Fetched content from URL (raw preview):\n{}", &html.chars().take(600).collect::<String>())
        } else {
            final_text
        }
    }
}
