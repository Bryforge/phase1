use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_lifecycle_example_has_required_rows() {
    let path = "docs/fyr/fixtures/staged-lifecycle-example.txt";

    for row in [
        "black_arts staged lifecycle",
        "codename      : black_arts",
        "candidate     : phase1-base1-candidate",
        "workspace     : .phase1/staged-candidates/phase1-base1-candidate",
        "1 plan        : docs/fyr/fixtures/staged-plan-example.txt",
        "2 create      : docs/fyr/fixtures/staged-create-example.txt",
        "3 apply       : docs/fyr/fixtures/staged-apply-example.txt",
        "4 validate    : docs/fyr/fixtures/staged-validate-example.txt",
        "5 promote     : docs/fyr/fixtures/staged-promote-example.txt",
        "6 discard     : docs/fyr/fixtures/staged-discard-example.txt",
        "guards        : candidate-only, evidence-recorded, claim-boundary, operator-approval",
        "live-system   : untouched-until-explicit-promotion",
        "claim-boundary: fixture-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_candidate_doc_links_lifecycle_example() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-lifecycle-example.txt",
    );
}

#[test]
fn lifecycle_example_links_existing_example_fixtures() {
    let lifecycle = read("docs/fyr/fixtures/staged-lifecycle-example.txt");

    for path in [
        "docs/fyr/fixtures/staged-plan-example.txt",
        "docs/fyr/fixtures/staged-create-example.txt",
        "docs/fyr/fixtures/staged-apply-example.txt",
        "docs/fyr/fixtures/staged-validate-example.txt",
        "docs/fyr/fixtures/staged-promote-example.txt",
        "docs/fyr/fixtures/staged-discard-example.txt",
    ] {
        assert!(lifecycle.contains(path), "lifecycle should list {path}");
        assert!(
            fs::metadata(path).is_ok(),
            "listed fixture should exist: {path}"
        );
    }
}
