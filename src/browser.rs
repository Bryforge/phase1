<<<<<<< HEAD
// src/browser.rs — Terminal-based browser for phase1 v2.0.1
=======
// src/browser.rs — Terminal-based browser for phase1 v3.0.0 Codename Blue
// Real HTTP fetching via host curl with robust HTML-to-text conversion.
// Internal pages for offline demonstration. No external crates. Clean,
// production-grade output suitable for terminal education and demonstration.
>>>>>>> 63d5bbc (update v3.0.0)

use std::process::Command;

pub struct Browser;

impl Browser {
    pub fn new() -> Self {
        Browser
    }

    pub fn browse(&self, url: &str) -> String {
        let url = url.trim();

<<<<<<< HEAD
        // Internal pages
=======
        // Internal simulator pages
>>>>>>> 63d5bbc (update v3.0.0)
        if url.is_empty() || url == "about" || url == "about:blank" {
            return self.internal_about();
        }
        if url == "phase1" || url.contains("bryforge/phase1") {
            return self.internal_phase1();
        }

<<<<<<< HEAD
        // Real web fetch
        let output = Command::new("curl")
            .args(&[
                "-L", "--silent", "--max-time", "10",
                "--user-agent", "phase1-browser/2.0.1",
=======
        // Real web fetch using host curl
        let output = Command::new("curl")
            .args(&[
                "-L", "--silent", "--max-time", "15",
                "--user-agent", "phase1-browser/3.0.0 (Codename Blue)",
                "--connect-timeout", "10",
>>>>>>> 63d5bbc (update v3.0.0)
                url
            ])
            .output();

        match output {
            Ok(o) if o.status.success() => {
                let content = String::from_utf8_lossy(&o.stdout);
                self.strip_html(&content)
            }
<<<<<<< HEAD
            Ok(o) => format!("Request failed with status: {:?}\nTry a different URL or check network.", o.status),
=======
            Ok(o) => format!("Request failed with status: {:?}\nURL: {}\nCheck network connectivity or try a different address.", o.status, url),
>>>>>>> 63d5bbc (update v3.0.0)
            Err(e) => format!("Failed to execute curl: {}", e),
        }
    }

    fn internal_about(&self) -> String {
<<<<<<< HEAD
        r#"phase1 Terminal Browser v2.0.1
================================

Lightweight web viewer integrated into the phase1 OS simulator.
=======
        r#"phase1 Terminal Browser v3.0.0 — Codename Blue
==============================================

Integrated web viewer for the phase1 educational OS simulator.

Features:
  • Real HTTP/HTTPS fetching via host curl
  • Automatic HTML-to-readable-text conversion
  • Built-in offline pages (about, phase1)
  • Safe, sandboxed operation within the simulator
>>>>>>> 63d5bbc (update v3.0.0)

Usage:
  browser <url>
  browser phase1
  browser about

<<<<<<< HEAD
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
=======
All network activity is performed by the host system's curl binary.
Content is stripped of markup for clean terminal display."#.to_string()
    }

    fn internal_phase1(&self) -> String {
        r#"phase1 v3.0.0 — Codename Blue
===============================

https://github.com/Bryforge/phase1

Educational operating system simulator built entirely in safe Rust.

Core demonstrations:
  • In-memory Virtual File System with full POSIX-like operations
  • Preemptive multitasking scheduler with process lifecycle
  • Hardware abstraction (PCIe enumeration, CR3/CR4/PCIDE)
  • Cross-platform networking stack (Linux + macOS)
  • Extensible Python plugin architecture
  • Built-in editors (nano/vi) and C compiler integration
  • Terminal web browser with real HTTP support

This entire environment runs in userspace with zero elevated privileges.
Type 'help' for the complete command reference."#.to_string()
>>>>>>> 63d5bbc (update v3.0.0)
    }

    fn strip_html(&self, html: &str) -> String {
        let mut result = String::with_capacity(html.len());
<<<<<<< HEAD

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
=======
        let mut in_tag = false;
        let mut in_script = false;
        let mut in_style = false;
        let mut in_comment = false;

        let lower = html.to_lowercase();
        let chars: Vec<char> = html.chars().collect();
        let lower_chars: Vec<char> = lower.chars().collect();

        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];

            // Script/style detection
            if !in_tag && i + 7 < lower_chars.len() {
                if lower_chars[i..].starts_with(&['<','s','c','r','i','p','t']) {
                    in_script = true;
                }
                if lower_chars[i..].starts_with(&['<','/','s','c','r','i','p','t']) {
                    in_script = false;
                }
                if lower_chars[i..].starts_with(&['<','s','t','y','l','e']) {
                    in_style = true;
                }
                if lower_chars[i..].starts_with(&['<','/','s','t','y','l','e']) {
                    in_style = false;
                }
            }

            // Comment handling
            if !in_tag && i + 3 < lower_chars.len() && lower_chars[i..].starts_with(&['<','!','-','-']) {
                in_comment = true;
            }
            if in_comment && i + 2 < lower_chars.len() && lower_chars[i..].starts_with(&['-','-','>']) {
                in_comment = false;
                i += 3;
                continue;
            }

            if in_script || in_style || in_comment {
                i += 1;
>>>>>>> 63d5bbc (update v3.0.0)
                continue;
            }

            match c {
<<<<<<< HEAD
                '<' => in_tag = true,
                '>' if in_tag => in_tag = false,
=======
                '<' => {
                    in_tag = true;
                }
                '>' if in_tag => {
                    in_tag = false;
                }
>>>>>>> 63d5bbc (update v3.0.0)
                _ if !in_tag => {
                    result.push(c);
                }
                _ => {}
            }
<<<<<<< HEAD
        }

        // Clean excessive whitespace
=======
            i += 1;
        }

        // Clean whitespace and normalize output
>>>>>>> 63d5bbc (update v3.0.0)
        let cleaned: String = result
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        let final_text = cleaned
            .replace("  ", " ")
            .replace(" \n", "\n")
<<<<<<< HEAD
            .replace("\n\n\n", "\n\n");

        if final_text.trim().is_empty() {
            format!("Fetched content from URL (raw preview):\n{}", &html.chars().take(600).collect::<String>())
=======
            .replace("\n\n\n", "\n\n")
            .replace("\n\n\n\n", "\n\n");

        if final_text.trim().is_empty() {
            format!("Fetched content from {}\n\n(raw preview — {} bytes)\n{}", 
                html.chars().take(80).collect::<String>().replace('\n', " "),
                html.len(),
                &html.chars().take(600).collect::<String>())
>>>>>>> 63d5bbc (update v3.0.0)
        } else {
            final_text
        }
    }
}
