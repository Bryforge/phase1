#[test]
fn libreboot_validation_report_template_defines_safe_target_summary() {
    let doc = std::fs::read_to_string("base1/LIBREBOOT_VALIDATION_REPORT.md")
        .expect("libreboot validation report");

    assert!(doc.contains("Base1 Libreboot validation report"), "{doc}");
    assert!(doc.contains("Firmware profile: Libreboot"), "{doc}");
    assert!(doc.contains("Hardware profile: X200-class"), "{doc}");
    assert!(doc.contains("Bootloader expectation: GRUB first"), "{doc}");
    assert!(doc.contains("Secure Boot: not assumed"), "{doc}");
    assert!(doc.contains("TPM: not assumed"), "{doc}");
    assert!(
        doc.contains("sh scripts/base1-libreboot-validate.sh"),
        "{doc}"
    );
    assert!(doc.contains("Do not store secrets"), "{doc}");
}

#[test]
fn libreboot_command_index_links_validation_report() {
    let index = std::fs::read_to_string("base1/LIBREBOOT_COMMAND_INDEX.md")
        .expect("libreboot command index");

    assert!(index.contains("LIBREBOOT_VALIDATION_REPORT.md"), "{index}");
    assert!(index.contains("Libreboot validation report"), "{index}");
}

#[test]
fn readme_links_validation_report() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("base1/LIBREBOOT_VALIDATION_REPORT.md"),
        "{readme}"
    );
    assert!(readme.contains("Libreboot validation report"), "{readme}");
}
