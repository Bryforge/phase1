use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_plan_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-plan-ok.txt";

    for field in [
        "name: black-arts-plan-fixture",
        "kind: staged-plan-fixture",
        "plan-version: 0.1.0",
        "source: known-good",
        "candidate: phase1-base1-candidate",
        "scope: candidate-only",
        "changes:",
        "validation:",
        "promotion:",
        "discard:",
        "operator-approval",
        "rollback-path",
        "evidence-link",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn staged_candidate_doc_links_plan_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-plan-ok.txt",
    );
}

#[test]
fn black_arts_plan_keeps_promotion_guarded() {
    let doc = read("docs/fyr/STAGED_CANDIDATES.md");
    let fixture = read("docs/fyr/fixtures/staged-plan-ok.txt");

    assert!(doc.contains("no promotion without validation"));
    assert!(doc.contains("operator explicitly approves promotion"));
    assert!(fixture.contains("operator-approval"));
    assert!(fixture.contains("rollback-path"));
}
