#[test]
fn release_pre_move_checks_define_candidate_group() {
    let doc = std::fs::read_to_string("docs/base1/releases/PRE_MOVE_CHECKS.md")
        .expect("Base1 release pre-move checks");

    assert!(
        doc.contains("Base1 release/checkpoint pre-move checks"),
        "{doc}"
    );
    assert!(doc.contains("Candidate group"), "{doc}");
    assert!(doc.contains("release/checkpoint notes"), "{doc}");
    assert!(
        doc.contains("organized mirrors already exist under `docs/base1/releases/`"),
        "{doc}"
    );
}

#[test]
fn release_pre_move_checks_preserve_root_and_mirror_paths() {
    let doc = std::fs::read_to_string("docs/base1/releases/PRE_MOVE_CHECKS.md")
        .expect("Base1 release pre-move checks");

    for path in [
        "RELEASE_BASE1_LIBREBOOT_READONLY_V1.md",
        "RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md",
        "RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md",
        "RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md",
        "RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md",
        "RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md",
        "docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md",
    ] {
        assert!(doc.contains(path), "missing release path {path}: {doc}");
    }
}

#[test]
fn release_pre_move_checks_require_release_note_tests() {
    let doc = std::fs::read_to_string("docs/base1/releases/PRE_MOVE_CHECKS.md")
        .expect("Base1 release pre-move checks");

    for test in [
        "base1_root_compatibility_map_docs",
        "base1_libreboot_release_notes_docs",
        "base1_libreboot_patch_release_notes_docs",
        "base1_recovery_usb_hardware_release_notes_docs",
        "base1_recovery_usb_target_release_notes_docs",
        "base1_recovery_usb_image_release_notes_docs",
        "base1_recovery_usb_emergency_shell_release_notes_docs",
        "base1_link_check_script",
        "quality_base1_docs_gate",
    ] {
        assert!(doc.contains(test), "missing required release test {test}: {doc}");
    }
}

#[test]
fn release_pre_move_checks_block_unsafe_release_moves() {
    let doc = std::fs::read_to_string("docs/base1/releases/PRE_MOVE_CHECKS.md")
        .expect("Base1 release pre-move checks");

    for blocker in [
        "Root compatibility files are missing.",
        "Organized mirror files are missing.",
        "docs/base1/ROOT_COMPATIBILITY_MAP.md",
        "docs/base1/releases/README.md",
        "Local link checking fails.",
        "Test inventory verification fails.",
        "Non-claims are weakened.",
        "deleting root checkpoint files to look clean.",
    ] {
        assert!(doc.contains(blocker), "missing blocker {blocker}: {doc}");
    }
}

#[test]
fn release_pre_move_checks_are_linked_and_integrity_checked() {
    let releases = std::fs::read_to_string("docs/base1/releases/README.md")
        .expect("Base1 releases index");
    let integrity = std::fs::read_to_string("scripts/base1-doc-integrity.sh")
        .expect("Base1 integrity gate");

    assert!(releases.contains("PRE_MOVE_CHECKS.md"), "{releases}");
    assert!(integrity.contains("docs/base1/releases/PRE_MOVE_CHECKS.md"), "{integrity}");
    assert!(
        integrity.contains("No root release/checkpoint file should be removed"),
        "{integrity}"
    );
}

#[test]
fn release_pre_move_checks_preserve_non_claims() {
    let doc = std::fs::read_to_string("docs/base1/releases/PRE_MOVE_CHECKS.md")
        .expect("Base1 release pre-move checks");

    assert!(doc.contains("does not make Base1 installer-ready"), "{doc}");
    assert!(doc.contains("hardware-validated"), "{doc}");
    assert!(doc.contains("daily-driver ready"), "{doc}");
}
