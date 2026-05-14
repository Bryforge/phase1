use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_f5_checkpoint_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/checkpoint-metadata-ok.txt";

    for field in [
        "name: fyr-checkpoint-fixture",
        "kind: checkpoint-metadata",
        "checkpoint: F5-fixture-001",
        "claim: not-complete",
        "result: fixture-only",
        "artifacts:",
        "checks:",
        "deterministic",
        "scoped",
        "reviewable",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn fyr_f5_checkpoint_fixture_points_to_existing_artifacts() {
    let fixture = read("docs/fyr/fixtures/checkpoint-metadata-ok.txt");

    for path in ["docs/fyr/SELF_WORKFLOWS.md", "docs/fyr/ROADMAP.md"] {
        assert!(fixture.contains(path), "fixture should list {path}");
        assert!(fs::metadata(path).is_ok(), "listed path should exist: {path}");
    }
}

#[test]
fn fyr_self_workflows_doc_links_checkpoint_fixture() {
    assert_contains(
        "docs/fyr/SELF_WORKFLOWS.md",
        "docs/fyr/fixtures/checkpoint-metadata-ok.txt",
    );
}
