use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_f6_text_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/stdlib-text-ok.txt";

    for field in [
        "name: fyr-stdlib-text",
        "kind: standard-library-module-fixture",
        "module: text",
        "allowed-operations:",
        "trim",
        "split-lines",
        "contains",
        "replace",
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
fn fyr_f6_stdlib_doc_links_text_fixture() {
    assert_contains("docs/fyr/STDLIB.md", "docs/fyr/fixtures/stdlib-text-ok.txt");
}

#[test]
fn fyr_f6_stdlib_doc_keeps_text_module_non_claiming() {
    let doc = read("docs/fyr/STDLIB.md");

    assert!(doc.contains("`text`"), "stdlib doc should list text module");
    assert!(
        doc.contains("not implemented as a complete standard library yet"),
        "stdlib doc should keep non-claim boundary"
    );
}

#[test]
fn fyr_roadmap_tracks_text_module_fixture_evidence() {
    assert_contains("docs/fyr/ROADMAP.md", "F6 `text` module fixture evidence");
}
