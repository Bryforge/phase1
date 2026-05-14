use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_validation_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-validation-ok.txt";

    for field in [
        "name: black-arts-validation-fixture",
        "kind: staged-validation-fixture",
        "candidate: phase1-base1-candidate",
        "plan: staged-plan-ok.txt",
        "result: passed",
        "required-checks:",
        "tests",
        "docs",
        "status",
        "non-claims",
        "promotion-state: blocked-until-approval",
        "validation-log",
        "checked-files",
        "status-summary",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn staged_candidate_doc_links_validation_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-validation-ok.txt",
    );
}

#[test]
fn validation_fixture_keeps_promotion_blocked_until_approval() {
    let doc = read("docs/fyr/STAGED_CANDIDATES.md");
    let fixture = read("docs/fyr/fixtures/staged-validation-ok.txt");

    assert!(doc.contains("no promotion without validation"));
    assert!(doc.contains("operator explicitly approves promotion"));
    assert!(fixture.contains("promotion-state: blocked-until-approval"));
}
