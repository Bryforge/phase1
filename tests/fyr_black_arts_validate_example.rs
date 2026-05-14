use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_validate_example_has_required_output_rows() {
    let path = "docs/fyr/fixtures/staged-validate-example.txt";

    for row in [
        "fyr staged validate phase1-base1-candidate",
        "codename      : black_arts",
        "candidate     : phase1-base1-candidate",
        "checks        : tests, docs, status, non-claims",
        "result        : passed",
        "evidence      : docs/fyr/fixtures/staged-validation-ok.txt",
        "promotion     : blocked-until-approval",
        "approval      : required",
        "rollback      : required",
        "claim-boundary: fixture-only",
        "next          : fyr staged promote phase1-base1-candidate",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_candidate_doc_links_validate_example() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-validate-example.txt",
    );
}

#[test]
fn validate_example_keeps_promotion_blocked() {
    let example = read("docs/fyr/fixtures/staged-validate-example.txt");

    assert!(example.contains("result        : passed"));
    assert!(example.contains("promotion     : blocked-until-approval"));
    assert!(example.contains("approval      : required"));
    assert!(example.contains("rollback      : required"));
}
