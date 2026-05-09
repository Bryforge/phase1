#[test]
fn libreboot_operator_checklist_defines_grub_first_readiness_path() {
    let doc = std::fs::read_to_string("base1/LIBREBOOT_OPERATOR_CHECKLIST.md")
        .expect("libreboot operator checklist");

    assert!(doc.contains("Base1 Libreboot operator checklist"), "{doc}");
    assert!(doc.contains("Libreboot"), "{doc}");
    assert!(doc.contains("GRUB first"), "{doc}");
    assert!(doc.contains("external USB recommended"), "{doc}");
    assert!(doc.contains("emergency shell required"), "{doc}");
    assert!(
        doc.contains("sh scripts/base1-libreboot-preflight.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-grub-recovery-dry-run.sh --dry-run"),
        "{doc}"
    );
    assert!(doc.contains("Do not install GRUB automatically"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("Do not store passwords"), "{doc}");
}

#[test]
fn libreboot_docs_link_operator_checklist() {
    let profile = std::fs::read_to_string("base1/LIBREBOOT_PROFILE.md").expect("libreboot profile");
    let preflight =
        std::fs::read_to_string("base1/LIBREBOOT_PREFLIGHT.md").expect("libreboot preflight");
    let recovery = std::fs::read_to_string("base1/LIBREBOOT_GRUB_RECOVERY.md")
        .expect("libreboot grub recovery");

    assert!(
        profile.contains("LIBREBOOT_OPERATOR_CHECKLIST.md"),
        "{profile}"
    );
    assert!(
        preflight.contains("LIBREBOOT_OPERATOR_CHECKLIST.md"),
        "{preflight}"
    );
    assert!(
        recovery.contains("LIBREBOOT_OPERATOR_CHECKLIST.md"),
        "{recovery}"
    );
}

#[test]
fn readme_links_operator_checklist() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("base1/LIBREBOOT_OPERATOR_CHECKLIST.md"),
        "{readme}"
    );
    assert!(readme.contains("Libreboot operator checklist"), "{readme}");
}
