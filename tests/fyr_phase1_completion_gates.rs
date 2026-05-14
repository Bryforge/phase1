use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_phase1_completion_gate_defines_evidence_bound_100_percent() {
    let path = "docs/status/FYR_PHASE1_100_COMPLETION_GATES.md";

    assert_contains(path, "# Fyr and Phase1 100% completion gates");
    assert_contains(path, "The feature is implemented in source, not only described in docs.");
    assert_contains(path, "The behavior has deterministic tests");
    assert_contains(path, "Public docs describe exactly what works and what does not.");
    assert_contains(path, "Do not edit `site/status.json` or public percentage values manually to reach 100%.");
    assert_contains(path, "All F0-F7 gates are satisfied.");
    assert_contains(path, "Phase1 guarded host workspace v2");
}

#[test]
fn fyr_roadmap_points_to_completion_gate() {
    assert_contains(
        "docs/fyr/ROADMAP.md",
        "../status/FYR_PHASE1_100_COMPLETION_GATES.md",
    );
    assert_contains("docs/fyr/ROADMAP.md", "## 100% promotion gate");
    assert_contains(
        "docs/fyr/ROADMAP.md",
        "Fyr is not promoted to 100% because the name, command, or docs exist.",
    );
}

#[test]
fn focus_policy_allows_only_phase1_aligned_fyr_work() {
    assert_contains("docs/project/FOCUS.md", "## Fyr completion exception");
    assert_contains(
        "docs/project/FOCUS.md",
        "Fyr language work is allowed only when it directly advances Phase1/Base1 completion.",
    );
    assert_contains("docs/project/FOCUS.md", "production-language claims");
    assert_contains("docs/project/FOCUS.md", "host shell access outside guarded Phase1 policy");
}
