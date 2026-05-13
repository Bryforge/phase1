#[test]
fn release_archive_map_lists_former_root_and_archived_paths() {
    let doc = std::fs::read_to_string("docs/base1/RELEASE_ARCHIVE_MAP.md")
        .expect("release archive map");

    assert!(doc.contains("Base1 release archive map"), "{doc}");
    assert!(doc.contains("Former root path"), "{doc}");
    assert!(doc.contains("Archived path"), "{doc}");
    assert!(
        doc.contains("repository no longer requires those files to exist at the repository root"),
        "{doc}"
    );

    for path in [
        "RELEASE_BASE1_LIBREBOOT_READONLY_V1.md",
        "RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md",
        "RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md",
        "RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md",
        "RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md",
        "RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md",
    ] {
        assert!(
            doc.contains(path),
            "missing former root compatibility name {path}: {doc}"
        );
    }

    for path in [
        "docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md",
        "docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md",
    ] {
        assert!(
            doc.contains(path),
            "missing archived path {path}: {doc}"
        );
        assert!(
            std::path::Path::new(path).is_file(),
            "organized archive file missing: {path}"
        );
    }
}

#[test]
fn documentation_map_and_manual_link_release_archive_map() {
    let map =
        std::fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md").expect("documentation map");
    let manual = std::fs::read_to_string("docs/base1/README.md").expect("base1 manual");

    assert!(map.contains("RELEASE_ARCHIVE_MAP.md"), "{map}");
    assert!(manual.contains("RELEASE_ARCHIVE_MAP.md"), "{manual}");
    assert!(
        manual.contains("Base1 release/checkpoint notes are archived under"),
        "{manual}"
    );
    assert!(
        manual.contains("current repository does not require root-level release files to exist"),
        "{manual}"
    );
}

#[test]
fn integrity_gate_checks_release_archive_map_and_release_archives() {
    let script = std::fs::read_to_string("scripts/base1-doc-integrity.sh")
        .expect("base1 doc integrity script");

    assert!(
        script.contains("check_release_compatibility_docs"),
        "{script}"
    );
    assert!(
        script.contains("docs/base1/RELEASE_ARCHIVE_MAP.md"),
        "{script}"
    );
    assert!(script.contains("docs/base1/releases/README.md"), "{script}");
    assert!(
        script.contains("RELEASE_BASE1_LIBREBOOT_READONLY_V1.md"),
        "{script}"
    );
    assert!(
        script.contains("RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md"),
        "{script}"
    );
    assert!(
        script.contains("root Base1 release compatibility files are archived under docs/base1/releases"),
        "{script}"
    );
    assert!(script.contains("writes: no"), "{script}");
}
