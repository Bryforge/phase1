#[test]
fn libreboot_preflight_doc_exists_and_defines_read_only_checks() {
    let doc =
        std::fs::read_to_string("base1/LIBREBOOT_PREFLIGHT.md").expect("libreboot preflight doc");

    assert!(doc.contains("Base1 Libreboot preflight notes"), "{doc}");
    assert!(doc.contains("read-only checks"), "{doc}");
    assert!(
        doc.contains("Firmware profile: Libreboot expected"),
        "{doc}"
    );
    assert!(doc.contains("Bootloader expectation: GRUB first"), "{doc}");
    assert!(doc.contains("Do not flash firmware"), "{doc}");
    assert!(doc.contains("Do not install GRUB automatically"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("sh scripts/base1-preflight.sh"), "{doc}");
}

#[test]
fn libreboot_profile_links_preflight_notes() {
    let profile =
        std::fs::read_to_string("base1/LIBREBOOT_PROFILE.md").expect("libreboot profile doc");

    assert!(profile.contains("LIBREBOOT_PREFLIGHT.md"), "{profile}");
    assert!(profile.contains("Libreboot preflight notes"), "{profile}");
}

#[test]
fn readme_links_libreboot_preflight_notes() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(readme.contains("base1/LIBREBOOT_PREFLIGHT.md"), "{readme}");
    assert!(readme.contains("Libreboot preflight notes"), "{readme}");
}
