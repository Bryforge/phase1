use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_promote_example_has_required_output_rows() {
    let path = "docs/fyr/fixtures/staged-promote-example.txt";

    for row in [
        "fyr staged promote phase1-base1-candidate",
        "codename      : black_arts",
        "candidate     : phase1-base1-candidate",
        "validation    : passed",
        "approval      : approved-by-operator",
        "promotion     : explicit-only",
        "rollback      : docs/fyr/fixtures/staged-approval-ok.txt",
        "evidence      : docs/fyr/fixtures/staged-validation-ok.txt",
        "claim-boundary: fixture-only",
        "live-system   : untouched-before-promotion",
        "result        : promotion-record-ready",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_candidate_doc_links_promote_example() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-promote-example.txt",
    );
}

#[test]
fn promote_example_preserves_explicit_approval_boundary() {
    let example = read("docs/fyr/fixtures/staged-promote-example.txt");

    assert!(example.contains("validation    : passed"));
    assert!(example.contains("approval      : approved-by-operator"));
    assert!(example.contains("promotion     : explicit-only"));
    assert!(example.contains("rollback      : docs/fyr/fixtures/staged-approval-ok.txt"));
    assert!(example.contains("evidence      : docs/fyr/fixtures/staged-validation-ok.txt"));
}
