use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_claim_boundary_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-claim-boundary-ok.txt";

    for field in [
        "name: black-arts-claim-boundary-fixture",
        "kind: staged-claim-boundary-fixture",
        "candidate: phase1-base1-candidate",
        "required-boundaries:",
        "not-production",
        "not-release-ready",
        "not-live-update",
        "not-auto-promote",
        "not-daily-driver",
        "report-fields:",
        "candidate-id",
        "evidence-link",
        "validation-state",
        "promotion-state",
        "claim-boundary",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn staged_candidate_doc_links_claim_boundary_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-claim-boundary-ok.txt",
    );
}

#[test]
fn staged_candidate_doc_preserves_claim_boundaries() {
    let doc = read("docs/fyr/STAGED_CANDIDATES.md");

    assert!(doc.contains("not a production updater"));
    assert!(doc.contains("not a release candidate process"));
    assert!(doc.contains("does not change the live system"));
    assert!(doc.contains("does not promote candidates without validation evidence"));
}
