use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_f6_json_lite_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/stdlib-json-lite-ok.txt";

    for field in [
        "name: fyr-stdlib-json-lite",
        "kind: standard-library-module-fixture",
        "module: json-lite",
        "allowed-operations:",
        "read-object",
        "read-string",
        "read-number",
        "write-object",
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
fn fyr_f6_stdlib_doc_links_json_lite_fixture() {
    assert_contains(
        "docs/fyr/STDLIB.md",
        "docs/fyr/fixtures/stdlib-json-lite-ok.txt",
    );
}

#[test]
fn fyr_f6_stdlib_doc_keeps_json_lite_module_non_claiming() {
    let doc = read("docs/fyr/STDLIB.md");

    assert!(doc.contains("`json-lite`"), "stdlib doc should list json-lite module");
    assert!(
        doc.contains("not implemented as a complete standard library yet"),
        "stdlib doc should keep non-claim boundary"
    );
}

#[test]
fn fyr_f6_stdlib_doc_marks_json_lite_fixture_backed() {
    let doc = read("docs/fyr/STDLIB.md");

    assert!(
        doc.contains("`json-lite` | Minimal deterministic JSON-like reading/writing. | fixture"),
        "stdlib doc should mark json-lite as fixture-backed"
    );
}
