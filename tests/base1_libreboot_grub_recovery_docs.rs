#[test]
fn libreboot_grub_recovery_notes_define_read_only_recovery_path() {
    let doc = std::fs::read_to_string("base1/LIBREBOOT_GRUB_RECOVERY.md")
        .expect("libreboot grub recovery notes");

    assert!(doc.contains("Base1 Libreboot GRUB recovery notes"), "{doc}");
    assert!(doc.contains("read-only"), "{doc}");
    assert!(doc.contains("GRUB first"), "{doc}");
    assert!(doc.contains("emergency shell required"), "{doc}");
    assert!(doc.contains("external USB recommended"), "{doc}");
    assert!(
        doc.contains("Do not run grub-install automatically"),
        "{doc}"
    );
    assert!(doc.contains("Do not edit grub.cfg automatically"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("Do not assume systemd-boot"), "{doc}");
    assert!(doc.contains("Do not assume EFI-only boot"), "{doc}");
}

#[test]
fn libreboot_docs_link_grub_recovery_notes() {
    let profile = std::fs::read_to_string("base1/LIBREBOOT_PROFILE.md").expect("libreboot profile");
    let preflight =
        std::fs::read_to_string("base1/LIBREBOOT_PREFLIGHT.md").expect("libreboot preflight");

    assert!(profile.contains("LIBREBOOT_GRUB_RECOVERY.md"), "{profile}");
    assert!(
        preflight.contains("LIBREBOOT_GRUB_RECOVERY.md"),
        "{preflight}"
    );
}

#[test]
fn readme_links_grub_recovery_notes() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("base1/LIBREBOOT_GRUB_RECOVERY.md"),
        "{readme}"
    );
    assert!(readme.contains("Libreboot GRUB recovery notes"), "{readme}");
}
