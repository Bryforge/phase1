use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_help_fixture_has_required_output_rows() {
    let path = "docs/fyr/fixtures/staged-help-ok.txt";

    for row in [
        "fyr staged help",
        "codename      : black_arts",
        "status        : fixture-backed design help",
        "usage         : fyr staged <status|plan|create|apply|validate|promote|discard>",
        "commands      : status, plan, create, apply, validate, promote, discard",
        "workspace     : .phase1/staged-candidates",
        "boundaries    : candidate-only, non-live, evidence-bound, claim-boundary",
        "promotion     : validation-and-approval-required",
        "implementation: pending",
        "claim-boundary: fixture-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_candidate_doc_links_help_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-help-ok.txt",
    );
}

#[test]
fn help_fixture_keeps_staged_commands_guarded() {
    let help = read("docs/fyr/fixtures/staged-help-ok.txt");

    assert!(help.contains("candidate-only"));
    assert!(help.contains("non-live"));
    assert!(help.contains("evidence-bound"));
    assert!(help.contains("validation-and-approval-required"));
}
