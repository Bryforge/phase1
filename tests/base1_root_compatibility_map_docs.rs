#[test]
fn root_compatibility_map_lists_root_and_mirror_paths() {
    let doc = std::fs::read_to_string("docs/base1/ROOT_COMPATIBILITY_MAP.md")
        .expect("root compatibility map");

    assert!(doc.contains("Base1 root compatibility map"), "{doc}");
    assert!(doc.contains("Root path"), "{doc}");
    assert!(doc.contains("Organized mirror"), "{doc}");

    for path in [
        "RELEASE_BASE1_LIBREBOOT_READONLY_V1.md",
        "RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md",
        "RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md",
        "RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md",
        "RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md",
        "RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md",
    ] {
        assert!(doc.contains(path), "missing root compatibility path {path}: {doc}");
        assert!(
            std::path::Path::new(path).is_file(),
            "root compatibility file missing: {path}"
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
        assert!(doc.contains(path), "missing organized mirror path {path}: {doc}");
        assert!(
            std::path::Path::new(path).is_file(),
            "organized mirror file missing: {path}"
        );
    }
}

#[test]
fn documentation_map_and_manual_link_root_compatibility_map() {
    let map = std::fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md")
        .expect("documentation map");
    let manual = std::fs::read_to_string("docs/base1/README.md").expect("base1 manual");

    assert!(map.contains("ROOT_COMPATIBILITY_MAP.md"), "{map}");
    assert!(manual.contains("ROOT_COMPATIBILITY_MAP.md"), "{manual}");
    assert!(
        manual.contains("root-level Base1 checkpoint notes remain compatibility paths"),
        "{manual}"
    );
}

#[test]
fn integrity_gate_checks_root_compatibility_and_release_mirrors() {
    let script = std::fs::read_to_string("scripts/base1-doc-integrity.sh")
        .expect("base1 doc integrity script");

    assert!(script.contains("check_root_compatibility_docs"), "{script}");
    assert!(script.contains("docs/base1/ROOT_COMPATIBILITY_MAP.md"), "{script}");
    assert!(script.contains("docs/base1/releases/README.md"), "{script}");
    assert!(script.contains("RELEASE_BASE1_LIBREBOOT_READONLY_V1.md"), "{script}");
    assert!(script.contains("RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md"), "{script}");
    assert!(script.contains("writes: no"), "{script}");
}
