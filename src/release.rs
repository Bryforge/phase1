use crate::kernel::VERSION;

pub const RELEASE_VERSION: &str = "3.6.0";
pub const BLEEDING_VERSION: &str = "3.7.0-dev";
pub const CHANNEL: &str = "bleeding-edge";

const BLEEDING_FEATURES: &[&str] = &[
    "persistent shell history with secret redaction",
    "structured shell command chains",
    "structured text pipelines",
    "guarded stable-to-bleeding updater",
    "operator sysinfo/theme/banner/tips commands",
];

const ROADMAP_STATUS: &[(&str, &str)] = &[
    ("Persistent shell history", "complete"),
    ("Structured command output and pipelines", "complete"),
    ("Capability enforcement based on command metadata", "partial: enforced for host tools and network mutation gates"),
    ("WASM/WASI plugin runtime", "planned: Python plugin runtime remains guarded behind host-tool opt-in"),
    ("Full-screen TUI dashboard", "planned: compact dashboard and sysinfo are available now"),
];

pub fn version_report(args: &[String]) -> String {
    let compare = args
        .iter()
        .any(|arg| matches!(arg.as_str(), "--compare" | "compare" | "--channel" | "channel" | "--bleeding"));
    if !compare {
        return format!("phase1 {}\n", VERSION);
    }

    let mut out = String::from("phase1 version report\n");
    out.push_str(&format!("runtime version : {}\n", VERSION));
    out.push_str(&format!("release version : {}\n", RELEASE_VERSION));
    out.push_str(&format!("bleeding edge   : {}\n", BLEEDING_VERSION));
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
    out.push_str(&format!("edge    : {}\n\n", BLEEDING_VERSION));
    for (track, status) in ROADMAP_STATUS {
        out.push_str(&format!("{track:<48} {status}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{roadmap_report, version_report, BLEEDING_VERSION, RELEASE_VERSION};

    #[test]
    fn version_compare_reports_release_and_edge() {
        let out = version_report(&["--compare".to_string()]);
        assert!(out.contains(RELEASE_VERSION));
        assert!(out.contains(BLEEDING_VERSION));
        assert!(out.contains("structured text pipelines"));
    }

    #[test]
    fn roadmap_reports_pipeline_complete() {
        let out = roadmap_report();
        assert!(out.contains("Structured command output and pipelines"));
        assert!(out.contains("complete"));
    }
}
