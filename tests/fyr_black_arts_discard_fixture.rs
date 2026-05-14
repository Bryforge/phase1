use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_discard_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-discard-ok.txt";

    for field in [
        "name: black-arts-discard-fixture",
        "kind: staged-discard-fixture",
        "candidate: phase1-base1-candidate",
        "reason: failed-validation",
        "discard-mode: explicit-only",
        "required-records:",
        "candidate-id",
        "discard-reason",
        "deletion-log",
        "live-system-untouched",
        "evidence-link",
        "allowed-reasons:",
        "failed-validation",
        "stale-candidate",
        "operator-request",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn staged_candidate_doc_links_discard_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-discard-ok.txt",
    );
}

#[test]
fn discard_fixture_preserves_live_system_untouched_boundary() {
    let doc = read("docs/fyr/STAGED_CANDIDATES.md");
    let fixture = read("docs/fyr/fixtures/staged-discard-ok.txt");

    assert!(doc.contains("live system remains untouched until explicit promotion"));
    assert!(doc.contains("explicit rollback or discard path"));
    assert!(fixture.contains("live-system-untouched"));
    assert!(fixture.contains("discard-mode: explicit-only"));
}
