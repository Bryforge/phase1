#[test]
fn rollback_metadata_doc_exists_and_defines_safe_contract() {
    let doc = std::fs::read_to_string("docs/os/BASE1_ROLLBACK_METADATA.md")
        .expect("rollback metadata doc");

    assert!(doc.contains("Base1 rollback metadata design"), "{doc}");
    assert!(doc.contains("documentation-only"), "{doc}");
    assert!(doc.contains("base1 rollback metadata"), "{doc}");
    assert!(doc.contains("phase1_version"), "{doc}");
    assert!(doc.contains("stable_version"), "{doc}");
    assert!(doc.contains("Do not store secrets"), "{doc}");
    assert!(doc.contains("Do not store credentials"), "{doc}");
    assert!(
        doc.contains("Do not imply rollback was tested unless it was actually tested"),
        "{doc}"
    );
}

#[test]
fn readme_links_rollback_metadata_design() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("docs/os/BASE1_ROLLBACK_METADATA.md"),
        "{readme}"
    );
    assert!(
        readme.contains("Base1 rollback metadata design"),
        "{readme}"
    );
}

#[test]
fn os_roadmap_links_rollback_metadata_design() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("os roadmap");

    assert!(roadmap.contains("BASE1_ROLLBACK_METADATA.md"), "{roadmap}");
    assert!(roadmap.contains("rollback metadata design"), "{roadmap}");
}
