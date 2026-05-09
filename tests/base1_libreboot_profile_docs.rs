#[test]
fn libreboot_profile_doc_exists_and_defines_boundary() {
    let doc = std::fs::read_to_string("base1/LIBREBOOT_PROFILE.md").expect("libreboot profile doc");

    assert!(doc.contains("Base1 Libreboot profile"), "{doc}");
    assert!(doc.contains("documentation-only"), "{doc}");
    assert!(doc.contains("ThinkPad X200-class"), "{doc}");
    assert!(doc.contains("Do not flash firmware"), "{doc}");
    assert!(
        doc.contains("Do not change boot order automatically"),
        "{doc}"
    );
    assert!(doc.contains("Do not assume Secure Boot"), "{doc}");
    assert!(doc.contains("Do not assume TPM"), "{doc}");
    assert!(doc.contains("sh scripts/base1-preflight.sh"), "{doc}");
}

#[test]
fn hardware_targets_link_libreboot_profile() {
    let targets =
        std::fs::read_to_string("base1/HARDWARE_TARGETS.md").expect("hardware targets doc");

    assert!(targets.contains("LIBREBOOT_PROFILE.md"), "{targets}");
    assert!(targets.contains("Libreboot profile"), "{targets}");
}

#[test]
fn readme_links_libreboot_profile() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(readme.contains("base1/LIBREBOOT_PROFILE.md"), "{readme}");
    assert!(readme.contains("Libreboot profile"), "{readme}");
}
