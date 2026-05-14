use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_workspace_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-workspace-ok.txt";

    for field in [
        "name: black-arts-workspace-fixture",
        "kind: staged-workspace-fixture",
        "workspace-root: .phase1/staged-candidates",
        "candidate-root: .phase1/staged-candidates/phase1-base1-candidate",
        "required-paths:",
        "plan.fyr",
        "candidate.toml",
        "changes.log",
        "validation.log",
        "approval.toml",
        "discard.log",
        "isolation-rules:",
        "candidate-only-writes",
        "live-system-untouched",
        "declared-workspace",
        "evidence-recorded",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn staged_candidate_doc_links_workspace_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-workspace-ok.txt",
    );
}

#[test]
fn workspace_fixture_preserves_candidate_only_boundary() {
    let doc = read("docs/fyr/STAGED_CANDIDATES.md");
    let fixture = read("docs/fyr/fixtures/staged-workspace-ok.txt");

    assert!(doc.contains("candidate-only writes"));
    assert!(doc.contains("declared workspace first"));
    assert!(doc.contains("live system remains untouched until explicit promotion"));
    assert!(fixture.contains("candidate-only-writes"));
    assert!(fixture.contains("live-system-untouched"));
}
