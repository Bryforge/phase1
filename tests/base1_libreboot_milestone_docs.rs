#[test]
fn libreboot_milestone_records_read_only_checkpoint() {
    let doc = std::fs::read_to_string("base1/LIBREBOOT_MILESTONE.md").expect("libreboot milestone");

    assert!(
        doc.contains("Base1 Libreboot milestone checkpoint"),
        "{doc}"
    );
    assert!(doc.contains("read-only"), "{doc}");
    assert!(doc.contains("Libreboot documented"), "{doc}");
    assert!(doc.contains("GRUB first documented"), "{doc}");
    assert!(doc.contains("documentation and read-only scripts"), "{doc}");
    assert!(doc.contains("does not flash firmware"), "{doc}");
    assert!(
        doc.contains("does not flash firmware") || doc.contains("flash firmware"),
        "{doc}"
    );
    assert!(doc.contains("does not claim"), "{doc}");
    assert!(doc.contains("Bootable Base1 image readiness"), "{doc}");
    assert!(
        doc.contains("Rollback validation on real hardware"),
        "{doc}"
    );
}

#[test]
fn libreboot_milestone_lists_current_docs_and_scripts() {
    let doc = std::fs::read_to_string("base1/LIBREBOOT_MILESTONE.md").expect("libreboot milestone");

    assert!(doc.contains("LIBREBOOT_DOCS_SUMMARY.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_QUICKSTART.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_COMMAND_INDEX.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_VALIDATION_REPORT.md"), "{doc}");
    assert!(doc.contains("scripts/base1-libreboot-docs.sh"), "{doc}");
    assert!(doc.contains("scripts/base1-libreboot-validate.sh"), "{doc}");
    assert!(doc.contains("scripts/base1-libreboot-report.sh"), "{doc}");
}

#[test]
fn libreboot_indexes_link_milestone_checkpoint() {
    let summary =
        std::fs::read_to_string("base1/LIBREBOOT_DOCS_SUMMARY.md").expect("libreboot docs summary");
    let index = std::fs::read_to_string("base1/LIBREBOOT_COMMAND_INDEX.md")
        .expect("libreboot command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(summary.contains("LIBREBOOT_MILESTONE.md"), "{summary}");
    assert!(
        summary.contains("Libreboot milestone checkpoint"),
        "{summary}"
    );
    assert!(index.contains("LIBREBOOT_MILESTONE.md"), "{index}");
    assert!(index.contains("Libreboot milestone checkpoint"), "{index}");
    assert!(readme.contains("base1/LIBREBOOT_MILESTONE.md"), "{readme}");
    assert!(
        readme.contains("Libreboot milestone checkpoint"),
        "{readme}"
    );
}
