use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_change_log_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-change-log-ok.txt";

    for field in [
        "name: black-arts-change-log-fixture",
        "kind: staged-change-log-fixture",
        "candidate: phase1-base1-candidate",
        "plan: staged-plan-ok.txt",
        "change-mode: candidate-only",
        "applied-changes:",
        "config-update",
        "feature-toggle",
        "docs-update",
        "rejected-operations:",
        "live-system-write",
        "undeclared-path",
        "missing-evidence",
        "required-records:",
        "file-list",
        "change-summary",
        "rejected-operations",
        "evidence-link",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn staged_candidate_doc_links_change_log_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-change-log-ok.txt",
    );
}

#[test]
fn change_log_fixture_preserves_rejected_operation_visibility() {
    let doc = read("docs/fyr/STAGED_CANDIDATES.md");
    let fixture = read("docs/fyr/fixtures/staged-change-log-ok.txt");

    assert!(doc.contains("rejected operations"));
    assert!(doc.contains("candidate-only writes"));
    assert!(fixture.contains("rejected-operations"));
    assert!(fixture.contains("live-system-write"));
}
