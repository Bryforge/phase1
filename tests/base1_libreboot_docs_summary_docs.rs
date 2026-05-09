#[test]
fn libreboot_docs_summary_lists_core_docs_and_commands() {
    let doc =
        std::fs::read_to_string("base1/LIBREBOOT_DOCS_SUMMARY.md").expect("libreboot docs summary");

    assert!(doc.contains("Base1 Libreboot docs summary"), "{doc}");
    assert!(doc.contains("LIBREBOOT_QUICKSTART.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_COMMAND_INDEX.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_PROFILE.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_PREFLIGHT.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_GRUB_RECOVERY.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_OPERATOR_CHECKLIST.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_VALIDATION_REPORT.md"), "{doc}");
    assert!(
        doc.contains("sh scripts/base1-libreboot-validate.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-libreboot-report.sh"),
        "{doc}"
    );
}

#[test]
fn libreboot_docs_summary_preserves_read_only_guardrails() {
    let doc =
        std::fs::read_to_string("base1/LIBREBOOT_DOCS_SUMMARY.md").expect("libreboot docs summary");

    assert!(doc.contains("does not flash firmware"), "{doc}");
    assert!(
        doc.contains("does not flash firmware") || doc.contains("Flash firmware"),
        "{doc}"
    );
    assert!(doc.contains("Install GRUB"), "{doc}");
    assert!(doc.contains("Write to /boot"), "{doc}");
    assert!(doc.contains("Modify disks"), "{doc}");
    assert!(doc.contains("Change boot order"), "{doc}");
    assert!(doc.contains("Store secrets"), "{doc}");
    assert!(doc.contains("read-only docs and dry-runs"), "{doc}");
}

#[test]
fn libreboot_index_and_readme_link_docs_summary() {
    let index = std::fs::read_to_string("base1/LIBREBOOT_COMMAND_INDEX.md")
        .expect("libreboot command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(index.contains("LIBREBOOT_DOCS_SUMMARY.md"), "{index}");
    assert!(index.contains("Libreboot docs summary"), "{index}");
    assert!(
        readme.contains("base1/LIBREBOOT_DOCS_SUMMARY.md"),
        "{readme}"
    );
    assert!(readme.contains("Libreboot docs summary"), "{readme}");
}
