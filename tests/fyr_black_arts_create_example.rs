use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_create_example_has_required_output_rows() {
    let path = "docs/fyr/fixtures/staged-create-example.txt";

    for row in [
        "fyr staged create phase1-base1-candidate",
        "codename      : black_arts",
        "candidate     : phase1-base1-candidate",
        "source        : known-good",
        "workspace     : .phase1/staged-candidates/phase1-base1-candidate",
        "created       : candidate.toml, plan.fyr, changes.log, validation.log, approval.toml, discard.log",
        "write-scope   : candidate-only",
        "live-system   : untouched",
        "next          : fyr staged apply phase1-base1-candidate docs/fyr/fixtures/staged-plan-ok.txt",
        "claim-boundary: fixture-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_candidate_doc_links_create_example() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-create-example.txt",
    );
}

#[test]
fn create_example_preserves_candidate_workspace_boundary() {
    let example = read("docs/fyr/fixtures/staged-create-example.txt");

    assert!(example.contains("write-scope   : candidate-only"));
    assert!(example.contains("live-system   : untouched"));
    assert!(example.contains("workspace     : .phase1/staged-candidates/phase1-base1-candidate"));
}
