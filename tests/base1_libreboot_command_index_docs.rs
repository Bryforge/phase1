#[test]
fn libreboot_command_index_lists_docs_and_scripts() {
    let doc = std::fs::read_to_string("base1/LIBREBOOT_COMMAND_INDEX.md")
        .expect("libreboot command index");

    assert!(doc.contains("Base1 Libreboot command index"), "{doc}");
    assert!(doc.contains("LIBREBOOT_PROFILE.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_PREFLIGHT.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_GRUB_RECOVERY.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_OPERATOR_CHECKLIST.md"), "{doc}");
    assert!(
        doc.contains("scripts/base1-libreboot-preflight.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-grub-recovery-dry-run.sh --dry-run"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-libreboot-checklist.sh"),
        "{doc}"
    );
}

#[test]
fn libreboot_command_index_preserves_guardrails() {
    let doc = std::fs::read_to_string("base1/LIBREBOOT_COMMAND_INDEX.md")
        .expect("libreboot command index");

    assert!(doc.contains("Do not flash firmware"), "{doc}");
    assert!(doc.contains("Do not install GRUB automatically"), "{doc}");
    assert!(doc.contains("Do not edit grub.cfg automatically"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("Do not assume systemd-boot"), "{doc}");
    assert!(doc.contains("Do not store passwords"), "{doc}");
}

#[test]
fn libreboot_docs_link_command_index() {
    let profile = std::fs::read_to_string("base1/LIBREBOOT_PROFILE.md").expect("libreboot profile");
    let preflight =
        std::fs::read_to_string("base1/LIBREBOOT_PREFLIGHT.md").expect("libreboot preflight");
    let recovery =
        std::fs::read_to_string("base1/LIBREBOOT_GRUB_RECOVERY.md").expect("libreboot recovery");
    let checklist = std::fs::read_to_string("base1/LIBREBOOT_OPERATOR_CHECKLIST.md")
        .expect("libreboot checklist");

    assert!(profile.contains("LIBREBOOT_COMMAND_INDEX.md"), "{profile}");
    assert!(
        preflight.contains("LIBREBOOT_COMMAND_INDEX.md"),
        "{preflight}"
    );
    assert!(
        recovery.contains("LIBREBOOT_COMMAND_INDEX.md"),
        "{recovery}"
    );
    assert!(
        checklist.contains("LIBREBOOT_COMMAND_INDEX.md"),
        "{checklist}"
    );
}

#[test]
fn readme_links_libreboot_command_index() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("base1/LIBREBOOT_COMMAND_INDEX.md"),
        "{readme}"
    );
    assert!(readme.contains("Libreboot command index"), "{readme}");
}
