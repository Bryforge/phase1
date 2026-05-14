use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_f5_doc_consistency_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/doc-consistency-ok.txt";

    for field in [
        "name: fyr-doc-consistency",
        "kind: fixture",
        "docs:",
        "required-phrases:",
        "VFS",
        "deterministic",
        "non-claim",
        "100% promotion gate",
        "F5",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn fyr_f5_doc_consistency_fixture_points_to_existing_docs() {
    let fixture = read("docs/fyr/fixtures/doc-consistency-ok.txt");

    for path in [
        "docs/fyr/ROADMAP.md",
        "docs/fyr/LANGUAGE_BOOK.md",
        "docs/fyr/SAFETY_MODEL.md",
        "docs/fyr/SELF_WORKFLOWS.md",
    ] {
        assert!(fixture.contains(path), "fixture should list {path}");
        assert!(
            fs::metadata(path).is_ok(),
            "listed path should exist: {path}"
        );
    }
}

#[test]
fn fyr_f5_docs_preserve_required_consistency_phrases() {
    let docs = [
        "docs/fyr/ROADMAP.md",
        "docs/fyr/LANGUAGE_BOOK.md",
        "docs/fyr/SAFETY_MODEL.md",
        "docs/fyr/SELF_WORKFLOWS.md",
    ];
    let combined = docs
        .iter()
        .map(|path| read(path))
        .collect::<Vec<_>>()
        .join("\n");

    for phrase in [
        "VFS",
        "deterministic",
        "non-claim",
        "100% promotion gate",
        "F5",
    ] {
        assert!(
            combined.contains(phrase),
            "Fyr docs should contain {phrase}"
        );
    }
}

#[test]
fn fyr_self_workflows_doc_links_doc_consistency_fixture() {
    assert_contains(
        "docs/fyr/SELF_WORKFLOWS.md",
        "docs/fyr/fixtures/doc-consistency-ok.txt",
    );
}
