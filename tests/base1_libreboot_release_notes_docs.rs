const LIBREBOOT_V1_RELEASE_NOTES: &str = "docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md";

#[test]
fn libreboot_release_notes_record_checkpoint_status() {
    let doc = std::fs::read_to_string(LIBREBOOT_V1_RELEASE_NOTES)
        .expect("libreboot release notes");

    assert!(
        doc.contains("Base1 Libreboot read-only validation checkpoint v1"),
        "{doc}"
    );
    assert!(
        doc.contains("checkpoint/base1-libreboot-readonly-v1"),
        "{doc}"
    );
    assert!(doc.contains("base1-libreboot-readonly-v1"), "{doc}");
    assert!(doc.contains("Firmware profile: Libreboot"), "{doc}");
    assert!(doc.contains("Bootloader expectation: GRUB first"), "{doc}");
    assert!(doc.contains("documentation and read-only scripts"), "{doc}");
}

#[test]
fn libreboot_release_notes_list_surfaces_and_non_claims() {
    let doc = std::fs::read_to_string(LIBREBOOT_V1_RELEASE_NOTES)
        .expect("libreboot release notes");

    assert!(doc.contains("base1/LIBREBOOT_MILESTONE.md"), "{doc}");
    assert!(
        doc.contains("scripts/base1-libreboot-milestone.sh"),
        "{doc}"
    );
    assert!(doc.contains("scripts/base1-libreboot-validate.sh"), "{doc}");
    assert!(doc.contains("Bootable Base1 image readiness"), "{doc}");
    assert!(doc.contains("Hardware recovery validation"), "{doc}");
    assert!(
        doc.contains("Rollback validation on real hardware"),
        "{doc}"
    );
}

#[test]
fn libreboot_indexes_link_release_notes() {
    let milestone =
        std::fs::read_to_string("base1/LIBREBOOT_MILESTONE.md").expect("libreboot milestone");
    let summary =
        std::fs::read_to_string("base1/LIBREBOOT_DOCS_SUMMARY.md").expect("libreboot docs summary");
    let readme = std::fs::read_to_string("README.md").expect("README.md");
    let releases = std::fs::read_to_string("docs/base1/releases/README.md")
        .expect("base1 release notes index");

    assert!(
        milestone.contains(LIBREBOOT_V1_RELEASE_NOTES),
        "{milestone}"
    );
    assert!(summary.contains(LIBREBOOT_V1_RELEASE_NOTES), "{summary}");
    assert!(
        readme.contains("docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md"),
        "{readme}"
    );
    assert!(
        releases.contains("RELEASE_BASE1_LIBREBOOT_READONLY_V1.md"),
        "{releases}"
    );
}
