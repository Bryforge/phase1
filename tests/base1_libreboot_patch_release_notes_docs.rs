#[test]
fn libreboot_patch_release_notes_record_v1_1_checkpoint() {
    let doc = std::fs::read_to_string("RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md")
        .expect("libreboot v1.1 release notes");

    assert!(
        doc.contains("Base1 Libreboot read-only validation checkpoint v1.1"),
        "{doc}"
    );
    assert!(
        doc.contains("checkpoint/base1-libreboot-readonly-v1.1"),
        "{doc}"
    );
    assert!(doc.contains("base1-libreboot-readonly-v1.1"), "{doc}");
    assert!(
        doc.contains("Previous checkpoint tag: base1-libreboot-readonly-v1"),
        "{doc}"
    );
    assert!(
        doc.contains("prints the GRUB recovery report once"),
        "{doc}"
    );
    assert!(doc.contains("Expected count: 1"), "{doc}");
}

#[test]
fn readme_links_v1_1_patch_release_notes() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md"),
        "{readme}"
    );
    assert!(
        readme.contains("Libreboot read-only checkpoint v1.1 release notes"),
        "{readme}"
    );
}
