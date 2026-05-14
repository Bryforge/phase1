use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_status_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-status-ok.txt";

    for field in [
        "name: black-arts-status-fixture",
        "kind: staged-status-fixture",
        "command: fyr staged status",
        "candidate-count: 1",
        "phase1-base1-candidate",
        "required-fields:",
        "workspace-root",
        "candidate-id",
        "validation-state",
        "promotion-state",
        "claim-boundary",
        "evidence-link",
        "states:",
        "planned",
        "created",
        "changed",
        "validated",
        "approved",
        "discarded",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn staged_candidate_doc_links_status_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-status-ok.txt",
    );
}

#[test]
fn status_fixture_preserves_status_contract_boundaries() {
    let doc = read("docs/fyr/STAGED_CANDIDATES.md");
    let fixture = read("docs/fyr/fixtures/staged-status-ok.txt");

    assert!(doc.contains("fyr staged status"));
    assert!(doc.contains("visible non-claims in every report"));
    assert!(fixture.contains("claim-boundary"));
    assert!(fixture.contains("evidence-link"));
}
