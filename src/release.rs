use crate::kernel::VERSION;

pub const STABLE_VERSION: &str = "4.0.0";
pub const PREVIOUS_STABLE_VERSION: &str = "3.10.9";
pub const COMPATIBILITY_BASE_VERSION: &str = "3.6.0";
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CHANNEL: &str = "bleeding-edge";
pub const UPDATE_PROTOCOL_FILE: &str = "UPDATE_PROTOCOL.md";
pub const VERSION_SCHEME: &str = "MAJOR.MINOR.PATCH[-dev]";

const BLEEDING_FEATURES: &[&str] = &[
    "edge operator deck with signal, trust, developer, runtime, and command radar panels",
    "boot install dock with guarded local launcher plan",
    "persistent shell history with private-value redaction",
    "structured shell command chains",
    "structured text pipelines",
    "guarded stable-to-bleeding updater",
    "in-system latest self-update flow with explicit host trust",
    "developer test kit for quick/full Rust validation from inside phase1",
    "documented update protocol with patch-level SemVer",
    "update protocol patch policy hardening",
    "metadata-backed command gating",
    "WASI-lite plugin runtime with phase1-only sandboxing",
    "Phase1 Arena game workspace and clean-room text arena",
    "selectable UI color palettes with rainbow default",
    "live system tab auto-completion for commands and common arguments",
    "raw-mode input editor with redraw-safe backspace handling",
    "full-screen operator TUI dashboard with compact fallback",
    "operator sysinfo/theme/banner/tips commands",
    "Neo Tokyo advanced operator console HUD",
    "dynamic command-aware status bar with live clock",
    "avim live mode HUD with real Escape handling",
];

const ROADMAP_STATUS: &[(&str, &str)] = &[
    ("Edge operator deck UI overhaul", "active: edge-only dashboard, trust boundary, developer cockpit, and command radar"),
    ("Boot install dock", "active: guarded phase1-install plan and launcher writer"),
    ("Dynamic version identity", "complete: current, stable, previous stable, and compatibility base are reported together"),
    ("Persistent shell history", "complete"),
    ("Structured command output and pipelines", "complete"),
    ("Update protocol and semantic patch versioning", "complete: UPDATE_PROTOCOL.md is the canonical patch-level reference"),
    ("Capability enforcement based on command metadata", "complete: host tool and network mutation gates are checked from command metadata"),
    ("WASM/WASI plugin runtime", "complete: WASI-lite plugins run in a phase1 sandbox without direct host shell access"),
    ("Phase1 Arena game workspace", "complete: renamed game prototype, isolated game module, focused docs, and game-only test runner"),
    ("Configurable UI color palettes", "complete: neo-tokyo is the default operator HUD; matrix, cyber, amber, ice, synthwave, crimson, and bleeding-edge are available"),
    ("System tab auto-completion", "complete: live Tab expands unique command/argument matches and lists ambiguous matches"),
    ("Raw input editing", "complete: interactive input redraws the prompt and clears ghost characters on backspace"),
    ("Full-screen TUI dashboard", "complete: dash renders a full-screen operator panel set; dash --compact keeps the quick snapshot"),
    ("In-system latest updater", "complete: update now --trust-host fetches, fast-forwards, and rebuilds latest bleeding edge from inside phase1"),
    ("Developer test kit", "complete: update test plans and runs quick/full/smoke/bleeding/game validation suites from inside phase1"),
    ("Dynamic operator HUD", "complete: command-aware prompt HUD and avim mode HUD show live UTC clock and context controls"),
];

pub fn version_report(args: &[String]) -> String {
    let compare = args.iter().any(|arg| {
        matches!(
            arg.as_str(),
            "--compare" | "compare" | "--channel" | "channel" | "--bleeding" | "--stable"
        )
    });
    if !compare {
        return format!("phase1 {}\n", current_version());
    }

    let mut out = String::from("phase1 version report\n");
    out.push_str(&format!("current version      : {}\n", current_version()));
    out.push_str(&format!("stable version       : {}\n", STABLE_VERSION));
    out.push_str(&format!("previous stable      : {}\n", PREVIOUS_STABLE_VERSION));
    out.push_str(&format!("compatibility base   : {}\n", COMPATIBILITY_BASE_VERSION));
    out.push_str(&format!("kernel baseline      : {}\n", VERSION));
    out.push_str(&format!("channel              : {}\n", CHANNEL));
    out.push_str(&format!("version scheme       : {}\n", VERSION_SCHEME));
    out.push_str(&format!("protocol file        : {}\n", UPDATE_PROTOCOL_FILE));
    out.push_str("\ncurrent bleeding-edge additions over stable:\n");
    for feature in BLEEDING_FEATURES {
        out.push_str("  - ");
        out.push_str(feature);
        out.push('\n');
    }
    out
}

pub fn roadmap_report() -> String {
    let mut out = String::from("phase1 roadmap status\n");
    out.push_str(&format!("current        : v{}\n", current_version()));
    out.push_str(&format!("stable         : v{}\n", STABLE_VERSION));
    out.push_str(&format!("previous       : v{}\n", PREVIOUS_STABLE_VERSION));
    out.push_str(&format!("compatibility  : v{}\n", COMPATIBILITY_BASE_VERSION));
    out.push_str(&format!("channel        : {}\n", CHANNEL));
    out.push_str(&format!("scheme         : {}\n", VERSION_SCHEME));
    out.push_str(&format!("updates        : {}\n\n", UPDATE_PROTOCOL_FILE));
    for (track, status) in ROADMAP_STATUS {
        out.push_str(&format!("{track:<48} {status}\n"));
    }
    out
}

fn current_version() -> String {
    CURRENT_VERSION.to_string()
}

#[cfg(test)]
mod tests {
    use super::{
        roadmap_report, version_report, COMPATIBILITY_BASE_VERSION, CURRENT_VERSION,
        PREVIOUS_STABLE_VERSION, STABLE_VERSION, UPDATE_PROTOCOL_FILE, VERSION_SCHEME,
    };

    #[test]
    fn version_compare_reports_stable_and_current() {
        let out = version_report(&["--compare".to_string()]);
        assert!(out.contains(STABLE_VERSION));
        assert!(out.contains(PREVIOUS_STABLE_VERSION));
        assert!(out.contains(COMPATIBILITY_BASE_VERSION));
        assert!(out.contains(CURRENT_VERSION));
        assert!(out.contains("current version"));
        assert!(out.contains("stable version"));
        assert!(out.contains("previous stable"));
        assert!(out.contains("compatibility base"));
        assert!(out.contains(VERSION_SCHEME));
        assert!(out.contains(UPDATE_PROTOCOL_FILE));
        assert!(out.contains("edge operator deck"));
        assert!(out.contains("boot install dock"));
        assert!(out.contains("structured text pipelines"));
        assert!(out.contains("patch-level SemVer"));
        assert!(out.contains("WASI-lite plugin runtime"));
        assert!(out.contains("Phase1 Arena game workspace"));
        assert!(out.contains("selectable UI color palettes"));
        assert!(out.contains("live system tab auto-completion"));
        assert!(out.contains("raw-mode input editor"));
        assert!(out.contains("full-screen operator TUI dashboard"));
        assert!(out.contains("in-system latest self-update"));
        assert!(out.contains("developer test kit"));
        assert!(out.contains("dynamic command-aware status bar"));
    }

    #[test]
    fn bare_version_uses_current_package_version() {
        std::env::remove_var("PHASE1_DISPLAY_VERSION");
        let out = version_report(&[]);
        assert!(out.contains(CURRENT_VERSION));
    }

    #[test]
    fn roadmap_reports_pipeline_complete() {
        let out = roadmap_report();
        assert!(out.contains("current"));
        assert!(out.contains("stable"));
        assert!(out.contains("previous"));
        assert!(out.contains("compatibility"));
        assert!(out.contains("Edge operator deck UI overhaul"));
        assert!(out.contains("Boot install dock"));
        assert!(out.contains("Structured command output and pipelines"));
        assert!(out.contains("Update protocol and semantic patch versioning"));
        assert!(out.contains("Capability enforcement based on command metadata"));
        assert!(out.contains("WASM/WASI plugin runtime"));
        assert!(out.contains("Phase1 Arena game workspace"));
        assert!(out.contains("Configurable UI color palettes"));
        assert!(out.contains("System tab auto-completion"));
        assert!(out.contains("Raw input editing"));
        assert!(out.contains("Full-screen TUI dashboard"));
        assert!(out.contains("In-system latest updater"));
        assert!(out.contains("Developer test kit"));
        assert!(out.contains("Dynamic operator HUD"));
        assert!(out.contains("complete"));
    }
}
