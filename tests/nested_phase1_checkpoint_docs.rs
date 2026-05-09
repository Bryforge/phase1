#[test]
fn readme_mentions_nested_phase1_checkpoint() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(readme.contains("Nested Phase1"), "{readme}");
    assert!(readme.contains("docs/nest/CHECKPOINT.md"), "{readme}");
    assert!(readme.contains("nest spawn <name>"), "{readme}");
    assert!(readme.contains("nest tree"), "{readme}");
}

#[test]
fn nested_phase1_checkpoint_documents_current_surface() {
    let doc = std::fs::read_to_string("docs/nest/CHECKPOINT.md").expect("nested checkpoint");

    assert!(doc.contains("Nested Phase1 checkpoint"), "{doc}");
    assert!(doc.contains("nest status"), "{doc}");
    assert!(doc.contains("nest spawn <name>"), "{doc}");
    assert!(doc.contains("nest enter <name>"), "{doc}");
    assert!(doc.contains("nest destroy <name>"), "{doc}");
    assert!(doc.contains("nest inspect <name>"), "{doc}");
    assert!(doc.contains("nest tree"), "{doc}");
    assert!(doc.contains("metadata-only"), "{doc}");
}
