use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_100_plan_preserves_non_claim_boundary() {
    let path = "docs/fyr/FYR_100_COMPLETION_PLAN.md";

    for row in [
        "Non-claim: Fyr is not 100% complete yet.",
        "Documentation-only work may prepare a gate, but it must not claim the runtime behavior exists until implementation and tests land.",
        "Do not raise Fyr percentage for plan-only changes.",
        "Do not close #317 until the runtime source and tests are merged.",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn fyr_100_plan_requires_runtime_source_wiring_evidence() {
    let path = "docs/fyr/FYR_100_COMPLETION_PLAN.md";

    for row in [
        "Gate F100-1: first staged runtime stub",
        "`src/main.rs` contains `Some(\"staged\") => fyr_staged(&args[1..])` inside `fyr_command`.",
        "`src/main.rs` contains `fyr_staged`, `fyr_staged_visual`, `fyr_staged_help`, and `fyr_staged_unknown` helpers.",
        "Integration tests run Phase1 commands for staged, staged status, staged help, and unknown staged actions.",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn fyr_100_plan_keeps_dangerous_runtime_scope_blocked() {
    let path = "docs/fyr/FYR_100_COMPLETION_PLAN.md";

    for row in [
        "candidate creation",
        "candidate apply/change behavior",
        "validation execution",
        "promotion execution",
        "discard execution",
        "host command execution",
        "network access",
        "live-system writes",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn fyr_100_plan_links_existing_fyr_trackers() {
    let path = "docs/fyr/FYR_100_COMPLETION_PLAN.md";

    for row in [
        "issue #317",
        "issue #97",
        "issue #101",
        "issue #103",
        "issue #108",
        "issue #110",
        "issue #112",
        "issue #113",
    ] {
        assert_contains(path, row);
    }
}
