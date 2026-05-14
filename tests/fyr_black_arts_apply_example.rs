use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_apply_example_has_required_output_rows() {
    let path = "docs/fyr/fixtures/staged-apply-example.txt";

    for row in [
        "fyr staged apply phase1-base1-candidate docs/fyr/fixtures/staged-plan-ok.txt",
        "codename      : black_arts",
        "candidate     : phase1-base1-candidate",
        "plan          : docs/fyr/fixtures/staged-plan-ok.txt",
        "change-mode   : candidate-only",
        "applied       : config-update, feature-toggle, docs-update",
        "rejected      : live-system-write, undeclared-path, missing-evidence",
        "records       : changes.log, candidate.toml",
        "write-scope   : .phase1/staged-candidates/phase1-base1-candidate",
        "live-system   : untouched",
        "next          : fyr staged validate phase1-base1-candidate",
        "claim-boundary: fixture-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_candidate_doc_links_apply_example() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-apply-example.txt",
    );
}

#[test]
fn apply_example_preserves_rejected_operation_visibility() {
    let example = read("docs/fyr/fixtures/staged-apply-example.txt");

    assert!(example.contains("change-mode   : candidate-only"));
    assert!(example.contains("rejected      : live-system-write, undeclared-path, missing-evidence"));
    assert!(example.contains("live-system   : untouched"));
}
