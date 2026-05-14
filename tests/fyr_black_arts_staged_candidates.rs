use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn staged_candidate_doc_preserves_black_arts_boundary() {
    let path = "docs/fyr/STAGED_CANDIDATES.md";

    assert_contains(path, "Codename: `black_arts`");
    assert_contains(path, "experimental design track");
    assert_contains(path, "candidate-only writes");
    assert_contains(path, "no promotion without validation");
    assert_contains(path, "operator explicitly approves promotion");
}

#[test]
fn staged_candidate_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-candidate-ok.txt";

    for field in [
        "name: black-arts-candidate-fixture",
        "kind: staged-candidate-fixture",
        "candidate: phase1-base1-candidate",
        "source: known-good",
        "status: fixture-only",
        "plan-shape",
        "candidate-metadata",
        "validation-result",
        "promotion-approval",
        "discard-result",
        "deterministic",
        "candidate-only",
        "evidence-bound",
        "non-claim-aware",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn staged_candidate_track_is_not_claimed_as_live_update_system() {
    let doc = read("docs/fyr/STAGED_CANDIDATES.md");

    assert!(doc.contains("not a production updater"));
    assert!(doc.contains("live system remains untouched until explicit promotion"));
    assert!(doc.contains("does not change the live system"));
}
