use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_status_example_has_required_output_rows() {
    let path = "docs/fyr/fixtures/staged-status-example.txt";

    for row in [
        "fyr staged status",
        "codename      : black_arts",
        "workspace     : .phase1/staged-candidates",
        "candidates    : 1",
        "candidate     : phase1-base1-candidate",
        "state         : validated",
        "validation    : passed",
        "promotion     : blocked-until-approval",
        "claim-boundary: fixture-only",
        "evidence      : docs/fyr/fixtures/staged-validation-ok.txt",
        "non-claims    : not-production, not-release-ready, not-live-update",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_candidate_doc_links_status_example() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-status-example.txt",
    );
}

#[test]
fn status_example_preserves_promotion_block() {
    let example = read("docs/fyr/fixtures/staged-status-example.txt");

    assert!(example.contains("validation    : passed"));
    assert!(example.contains("promotion     : blocked-until-approval"));
    assert!(example.contains("claim-boundary: fixture-only"));
}
