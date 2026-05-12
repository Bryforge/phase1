const LIBREBOOT_V1_1_RELEASE_NOTES: &str =
    "docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md";

#[test]
fn libreboot_patch_release_notes_record_v1_1_checkpoint() {
    let doc = std::fs::read_to_string(LIBREBOOT_V1_1_RELEASE_NOTES)
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
fn readme_and_release_index_link_v1_1_patch_release_notes() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");
    let releases = std::fs::read_to_string("docs/base1/releases/README.md")
        .expect("base1 release notes index");

    assert!(
        readme.contains("RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md"),
        "{readme}"
    );
    assert!(
        releases.contains("RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md"),
        "{releases}"
    );
    assert!(
        readme.contains("Libreboot read-only checkpoint v1.1 release notes"),
        "{readme}"
    );
}
