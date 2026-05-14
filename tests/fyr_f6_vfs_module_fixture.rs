use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_f6_vfs_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/stdlib-vfs-ok.txt";

    for field in [
        "name: fyr-stdlib-vfs",
        "kind: standard-library-module-fixture",
        "module: vfs",
        "allowed-operations:",
        "read-vfs-file",
        "write-vfs-file",
        "list-vfs-directory",
        "required-evidence:",
        "smoke-test",
        "failure-mode-test",
        "docs-example",
        "safety-note",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn fyr_f6_stdlib_doc_links_vfs_fixture() {
    assert_contains("docs/fyr/STDLIB.md", "docs/fyr/fixtures/stdlib-vfs-ok.txt");
}

#[test]
fn fyr_f6_stdlib_doc_keeps_vfs_module_non_claiming() {
    let doc = read("docs/fyr/STDLIB.md");

    assert!(doc.contains("`vfs`"), "stdlib doc should list vfs module");
    assert!(
        doc.contains("not implemented as a complete standard library yet"),
        "stdlib doc should keep non-claim boundary"
    );
}

#[test]
fn fyr_roadmap_tracks_vfs_module_fixture_evidence() {
    assert_contains("docs/fyr/ROADMAP.md", "F6 `vfs` module fixture evidence");
}
