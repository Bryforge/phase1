// src/browser.rs — Terminal-based browser for phase1 v3.0.0 Codename Blue

use std::process::Command;

pub struct Browser;

impl Browser {
    pub fn new() -> Self {
        Browser
    }

    pub fn browse(&self, url: &str) -> String {
        let url = url.trim();

        if url.is_empty() || url == "about" || url == "about:blank" {
            return self.internal_about();
        }
        if url == "phase1" || url.contains("bryforge/phase1") {
            return self.internal_phase1();
        }

        let output = Command::new("curl")
            .args(&[
                "-L", "--silent", "--max-time", "10",
                "--user-agent", "phase1-browser/3.0.0",
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
        r#"phase1 Terminal Browser v3.0.0 Codename Blue
======================================

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
        r#"phase1 v3.0.0 - Codename Blue
=======================================

Educational OS Simulator

Core demonstrated concepts:
- In-memory Virtual File System with full POSIX-like operations
- Preemptive multitasking scheduler with process lifecycle
- Hardware abstraction (PCIe enumeration, CR3/CR4/PCIDE)
- Cross-platform networking stack (Linux + macOS)
- Extensible Python plugin architecture
- Built-in editors (nano/vi) and C compiler integration
- Terminal web browser with real HTTP support

Everything runs entirely in memory for safe, portable learning.

Type 'help' for the complete command reference."#.to_string()
    }

    fn strip_html(&self, html: &str) -> String {
        let mut result = String::with_capacity(html.len());

        let mut in_tag = false;
        let mut in_script = false;
        let mut in_style = false;

        let lower_html = html.to_lowercase();

        for (i, c) in html.chars().enumerate() {
            let remaining = &lower_html[i..];
            if remaining.starts_with("<script") {
                in_script = true;
            }
            if remaining.starts_with("</script") {
                in_script = false;
            }
            if remaining.starts_with("<style") {
                in_style = true;
            }
            if remaining.starts_with("</style") {
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
            format!("Fetched content from URL (raw preview):\n{}", html.chars().take(600).collect::<String>())
        } else {
            final_text
        }
    }
}
