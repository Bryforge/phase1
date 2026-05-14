use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_discard_example_has_required_output_rows() {
    let path = "docs/fyr/fixtures/staged-discard-example.txt";

    for row in [
        "fyr staged discard phase1-base1-candidate",
        "codename      : black_arts",
        "candidate     : phase1-base1-candidate",
        "reason        : failed-validation",
        "discard-mode  : explicit-only",
        "removed       : .phase1/staged-candidates/phase1-base1-candidate",
        "record        : discard.log",
        "evidence      : docs/fyr/fixtures/staged-discard-ok.txt",
        "live-system   : untouched",
        "claim-boundary: fixture-only",
        "result        : candidate-discard-record-ready",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_candidate_doc_links_discard_example() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-discard-example.txt",
    );
}

#[test]
fn discard_example_preserves_live_system_boundary() {
    let example = read("docs/fyr/fixtures/staged-discard-example.txt");

    assert!(example.contains("discard-mode  : explicit-only"));
    assert!(example.contains("removed       : .phase1/staged-candidates/phase1-base1-candidate"));
    assert!(example.contains("live-system   : untouched"));
    assert!(example.contains("claim-boundary: fixture-only"));
}
