#[test]
fn libreboot_quickstart_defines_safe_grub_first_path() {
    let doc =
        std::fs::read_to_string("base1/LIBREBOOT_QUICKSTART.md").expect("libreboot quickstart");

    assert!(doc.contains("Base1 Libreboot quickstart"), "{doc}");
    assert!(doc.contains("Libreboot"), "{doc}");
    assert!(doc.contains("GRUB first"), "{doc}");
    assert!(doc.contains("Secure Boot: not assumed"), "{doc}");
    assert!(doc.contains("TPM: not assumed"), "{doc}");
    assert!(doc.contains("sh scripts/base1-libreboot-index.sh"), "{doc}");
    assert!(
        doc.contains("sh scripts/base1-libreboot-validate.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-libreboot-report.sh"),
        "{doc}"
    );
    assert!(doc.contains("Do not install GRUB automatically"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("Do not store passwords"), "{doc}");
}

#[test]
fn libreboot_index_links_quickstart() {
    let index = std::fs::read_to_string("base1/LIBREBOOT_COMMAND_INDEX.md")
        .expect("libreboot command index");

    assert!(index.contains("LIBREBOOT_QUICKSTART.md"), "{index}");
    assert!(index.contains("Libreboot quickstart"), "{index}");
}

#[test]
fn readme_links_libreboot_quickstart() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(readme.contains("base1/LIBREBOOT_QUICKSTART.md"), "{readme}");
    assert!(readme.contains("Libreboot quickstart"), "{readme}");
}
