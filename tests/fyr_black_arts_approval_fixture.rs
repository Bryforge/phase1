use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_approval_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-approval-ok.txt";

    for field in [
        "name: black-arts-approval-fixture",
        "kind: staged-approval-fixture",
        "candidate: phase1-base1-candidate",
        "validation: staged-validation-ok.txt",
        "approval-state: approved-by-operator",
        "promotion-mode: explicit-only",
        "required-approval:",
        "operator",
        "evidence-link",
        "rollback-path",
        "status-boundary-check",
        "promotion-blockers:",
        "missing-validation",
        "missing-approval",
        "missing-rollback",
        "failed-status-boundary",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn staged_candidate_doc_links_approval_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-approval-ok.txt",
    );
}

#[test]
fn approval_fixture_preserves_explicit_promotion_boundary() {
    let doc = read("docs/fyr/STAGED_CANDIDATES.md");
    let fixture = read("docs/fyr/fixtures/staged-approval-ok.txt");

    assert!(doc.contains("operator explicitly approves promotion"));
    assert!(doc.contains("Promote only after validation"));
    assert!(fixture.contains("promotion-mode: explicit-only"));
    assert!(fixture.contains("missing-approval"));
}
