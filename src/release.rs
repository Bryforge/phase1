use crate::kernel::VERSION;

pub const RELEASE_VERSION: &str = "3.6.0";
pub const BLEEDING_VERSION: &str = "3.8.3-dev";
pub const CHANNEL: &str = "bleeding-edge";
pub const UPDATE_PROTOCOL_FILE: &str = "UPDATE_PROTOCOL.md";
pub const VERSION_SCHEME: &str = "MAJOR.MINOR.PATCH[-dev]";

const BLEEDING_FEATURES: &[&str] = &[
    "persistent shell history with private-value redaction",
    "structured shell command chains",
    "structured text pipelines",
    "guarded stable-to-bleeding updater",
    "documented update protocol with patch-level SemVer",
    "update protocol patch policy hardening",
    "metadata-backed capability enforcement",
    "WASI-lite plugin runtime with phase1-only sandboxing",
    "selectable UI color palettes with rainbow default",
    "live system tab auto-completion for commands and common arguments",
    "raw-mode input editor with redraw-safe backspace handling",
    "operator sysinfo/theme/banner/tips commands",
];

const ROADMAP_STATUS: &[(&str, &str)] = &[
    ("Persistent shell history", "complete"),
    ("Structured command output and pipelines", "complete"),
    (
        "Update protocol and semantic patch versioning",
        "complete: UPDATE_PROTOCOL.md is the canonical patch-level reference",
    ),
    (
        "Capability enforcement based on command metadata",
        "complete: host tool and network mutation gates are checked from command metadata",
    ),
    (
        "WASM/WASI plugin runtime",
        "complete: WASI-lite plugins run in a phase1 sandbox without host shell access",
    ),
    (
        "Configurable UI color palettes",
        "complete: rainbow remains default; matrix, cyber, amber, ice, synthwave, and crimson are available",
    ),
    (
        "System tab auto-completion",
        "complete: live Tab expands unique command/argument matches and lists ambiguous matches",
    ),
    (
        "Raw input editing",
        "complete: interactive input redraws the prompt and clears ghost characters on backspace",
    ),
    (
        "Full-screen TUI dashboard",
        "planned: compact dashboard and sysinfo are available now",
    ),
];

pub fn version_report(args: &[String]) -> String {
    let compare = args.iter().any(|arg| {
        matches!(
            arg.as_str(),
            "--compare" | "compare" | "--channel" | "channel" | "--bleeding"
        )
    });
    if !compare {
        return format!("phase1 {}\n", VERSION);
    }

    let mut out = String::from("phase1 version report\n");
    out.push_str(&format!("runtime version : {}\n", VERSION));
    out.push_str(&format!("release version : {}\n", RELEASE_VERSION));
    out.push_str(&format!("bleeding edge   : {}\n", BLEEDING_VERSION));
    out.push_str(&format!("version scheme  : {}\n", VERSION_SCHEME));
    out.push_str(&format!("protocol file   : {}\n", UPDATE_PROTOCOL_FILE));
    out.push_str(&format!("channel         : {}\n", CHANNEL));
    out.push_str("\nbleeding-edge additions over release:\n");
    for feature in BLEEDING_FEATURES {
        out.push_str("  - ");
        out.push_str(feature);
        out.push('\n');
    }
    out
}

pub fn roadmap_report() -> String {
    let mut out = String::from("phase1 roadmap status\n");
    out.push_str(&format!("release : v{}\n", RELEASE_VERSION));
    out.push_str(&format!("edge    : {}\n", BLEEDING_VERSION));
    out.push_str(&format!("scheme  : {}\n", VERSION_SCHEME));
    out.push_str(&format!("updates : {}\n\n", UPDATE_PROTOCOL_FILE));
    for (track, status) in ROADMAP_STATUS {
        out.push_str(&format!("{track:<48} {status}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        roadmap_report, version_report, BLEEDING_VERSION, RELEASE_VERSION, UPDATE_PROTOCOL_FILE,
        VERSION_SCHEME,
    };

    #[test]
    fn version_compare_reports_release_and_edge() {
        let out = version_report(&["--compare".to_string()]);
        assert!(out.contains(RELEASE_VERSION));
        assert!(out.contains(BLEEDING_VERSION));
        assert!(out.contains(VERSION_SCHEME));
        assert!(out.contains(UPDATE_PROTOCOL_FILE));
        assert!(out.contains("structured text pipelines"));
        assert!(out.contains("patch-level SemVer"));
        assert!(out.contains("metadata-backed capability enforcement"));
        assert!(out.contains("WASI-lite plugin runtime"));
        assert!(out.contains("selectable UI color palettes"));
        assert!(out.contains("live system tab auto-completion"));
        assert!(out.contains("raw-mode input editor"));
    }

    #[test]
    fn roadmap_reports_pipeline_complete() {
        let out = roadmap_report();
        assert!(out.contains("Structured command output and pipelines"));
        assert!(out.contains("Update protocol and semantic patch versioning"));
        assert!(out.contains("Capability enforcement based on command metadata"));
        assert!(out.contains("WASM/WASI plugin runtime"));
        assert!(out.contains("Configurable UI color palettes"));
        assert!(out.contains("System tab auto-completion"));
        assert!(out.contains("Raw input editing"));
        assert!(out.contains("complete"));
    }
}
