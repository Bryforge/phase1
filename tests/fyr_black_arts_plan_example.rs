use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_plan_example_has_required_output_rows() {
    let path = "docs/fyr/fixtures/staged-plan-example.txt";

    for row in [
        "fyr staged plan",
        "codename      : black_arts",
        "plan          : docs/fyr/fixtures/staged-plan-ok.txt",
        "version       : 0.1.0",
        "source        : known-good",
        "candidate     : phase1-base1-candidate",
        "scope         : candidate-only",
        "changes       : config-update, feature-toggle, docs-update",
        "validation    : tests, docs, status",
        "promotion     : operator-approval, rollback-path, evidence-link",
        "discard       : stale-candidate, failed-validation",
        "claim-boundary: fixture-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_candidate_doc_links_plan_example() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-plan-example.txt",
    );
}

#[test]
fn plan_example_preserves_candidate_only_scope() {
    let example = read("docs/fyr/fixtures/staged-plan-example.txt");

    assert!(example.contains("scope         : candidate-only"));
    assert!(example.contains("promotion     : operator-approval, rollback-path, evidence-link"));
    assert!(example.contains("claim-boundary: fixture-only"));
}
