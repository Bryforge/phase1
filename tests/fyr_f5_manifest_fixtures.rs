use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_f5_repo_manifest_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/repo-manifest-ok.txt";

    for field in [
        "name: phase1-fixture",
        "root: /phase1",
        "kind: fixture",
        "files:",
        "checks:",
        "deterministic",
        "vfs-first",
        "bounded",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn fyr_f5_repo_manifest_fixture_points_to_existing_docs() {
    let fixture = read("docs/fyr/fixtures/repo-manifest-ok.txt");

    for path in [
        "docs/fyr/SELF_WORKFLOWS.md",
        "docs/fyr/ROADMAP.md",
        "docs/fyr/SAFETY_MODEL.md",
    ] {
        assert!(fixture.contains(path), "fixture should list {path}");
        assert!(
            fs::metadata(path).is_ok(),
            "listed path should exist: {path}"
        );
    }
}

#[test]
fn fyr_f5_self_workflows_doc_links_manifest_fixture() {
    assert_contains(
        "docs/fyr/SELF_WORKFLOWS.md",
        "docs/fyr/fixtures/repo-manifest-ok.txt",
    );
}
