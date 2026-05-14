use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn visual_runtime_checklist_names_required_banner_rows() {
    let path = "docs/fyr/STAGED_VISUAL_RUNTIME_CHECKLIST.md";

    for row in [
        "☠ FYR black_arts // STAGED CANDIDATE MODE",
        "candidate     : phase1-base1-candidate",
        "workspace     : .phase1/staged-candidates/phase1-base1-candidate",
        "state         : fixture-backed",
        "live-system   : untouched",
        "promotion     : blocked-until-validation-and-approval",
        "boundary      : candidate-only | non-live | evidence-bound | claim-boundary",
        "claim-boundary: fixture-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn visual_runtime_checklist_requires_ascii_fallback() {
    assert_contains(
        "docs/fyr/STAGED_VISUAL_RUNTIME_CHECKLIST.md",
        "[BLACK_ARTS] FYR staged candidate mode",
    );
}

#[test]
fn visual_runtime_checklist_names_source_wiring_target() {
    let path = "docs/fyr/STAGED_VISUAL_RUNTIME_CHECKLIST.md";

    for row in [
        "Add `fyr_staged_visual() -> String` or equivalent.",
        "Call it from `fyr_staged` for the no-argument/status path.",
        "Keep unknown actions no-op and help-guided.",
        "Avoid adding candidate writes or host commands.",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn visual_runtime_checklist_blocks_unsafe_first_implementation_scope() {
    let path = "docs/fyr/STAGED_VISUAL_RUNTIME_CHECKLIST.md";

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
fn visual_runtime_checklist_links_operator_visual_fixture() {
    assert_contains(
        "docs/fyr/STAGED_VISUAL_RUNTIME_CHECKLIST.md",
        "docs/fyr/fixtures/staged-operator-visual-ok.txt",
    );
    assert!(
        fs::metadata("docs/fyr/fixtures/staged-operator-visual-ok.txt").is_ok(),
        "operator visual fixture should exist"
    );
}
