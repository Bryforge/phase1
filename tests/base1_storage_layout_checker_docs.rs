#[test]
fn storage_layout_checker_doc_exists_and_defines_read_only_surface() {
    let doc = std::fs::read_to_string("docs/os/BASE1_STORAGE_LAYOUT_CHECKER.md")
        .expect("storage layout checker doc");

    assert!(doc.contains("Base1 storage layout checker design"), "{doc}");
    assert!(doc.contains("documentation-only"), "{doc}");
    assert!(
        doc.contains("base1 storage check --dry-run --target <disk>"),
        "{doc}"
    );
    assert!(doc.contains("read-only previews"), "{doc}");
    assert!(doc.contains("Do not partition disks"), "{doc}");
    assert!(doc.contains("Do not format disks"), "{doc}");
    assert!(doc.contains("rollback metadata"), "{doc}");
}

#[test]
fn readme_links_storage_layout_checker_design() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("docs/os/BASE1_STORAGE_LAYOUT_CHECKER.md"),
        "{readme}"
    );
    assert!(
        readme.contains("Base1 storage layout checker design"),
        "{readme}"
    );
}

#[test]
fn os_roadmap_links_storage_layout_checker_design() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("os roadmap");

    assert!(
        roadmap.contains("BASE1_STORAGE_LAYOUT_CHECKER.md"),
        "{roadmap}"
    );
    assert!(
        roadmap.contains("storage layout checker design"),
        "{roadmap}"
    );
}
