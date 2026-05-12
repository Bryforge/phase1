#[test]
fn base1_inventory_lists_required_groups() {
    let inventory = std::fs::read_to_string("docs/base1/INVENTORY.md").expect("Base1 inventory");

    for heading in [
        "Core Base1 docs",
        "Base1 organization and validation docs",
        "OS-track Base1 design slices",
        "Libreboot docs",
        "Recovery USB docs",
        "Real-device read-only docs",
        "Release and checkpoint notes",
        "Base1 scripts",
        "Test groups",
    ] {
        assert!(
            inventory.contains(heading),
            "missing inventory heading {heading}: {inventory}"
        );
    }
}

#[test]
fn base1_inventory_preserves_key_paths() {
    let inventory = std::fs::read_to_string("docs/base1/INVENTORY.md").expect("Base1 inventory");

    for path in [
        "base1/README.md",
        "base1/SECURITY_MODEL.md",
        "base1/PHASE1_COMPATIBILITY.md",
        "docs/base1/DOCUMENTATION_MAP.md",
        "docs/base1/ROOT_COMPATIBILITY_MAP.md",
        "docs/base1/REORGANIZATION_READINESS.md",
        "docs/base1/TEST_INVENTORY.md",
        "docs/os/BASE1_DRY_RUN_COMMANDS.md",
        "base1/RECOVERY_USB_COMMAND_INDEX.md",
        "scripts/base1-doc-integrity.sh",
        "scripts/base1-network-lockdown-dry-run.sh",
        "RELEASE_BASE1_LIBREBOOT_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md",
    ] {
        assert!(
            inventory.contains(path),
            "missing inventoried path {path}: {inventory}"
        );
    }
}

#[test]
fn base1_test_inventory_lists_required_test_groups() {
    let tests =
        std::fs::read_to_string("docs/base1/TEST_INVENTORY.md").expect("Base1 test inventory");

    for heading in [
        "Core and OS-track tests",
        "Organization and quality tests",
        "Libreboot tests",
        "Recovery USB hardware tests",
        "Recovery USB target-selection tests",
        "Recovery USB image-provenance tests",
        "Recovery USB emergency-shell tests",
        "Recovery USB shared tests",
        "Real-device read-only tests",
    ] {
        assert!(
            tests.contains(heading),
            "missing test inventory heading {heading}: {tests}"
        );
    }
}

#[test]
fn base1_test_inventory_preserves_key_test_paths() {
    let tests =
        std::fs::read_to_string("docs/base1/TEST_INVENTORY.md").expect("Base1 test inventory");

    for path in [
        "tests/base1_foundation.rs",
        "tests/base1_network_lockdown_docs.rs",
        "tests/base1_root_compatibility_map_docs.rs",
        "tests/quality_base1_docs_gate.rs",
        "tests/base1_libreboot_release_notes_docs.rs",
        "tests/base1_recovery_usb_command_index_docs.rs",
        "tests/base1_recovery_usb_image_release_notes_docs.rs",
        "tests/base1_recovery_usb_emergency_shell_release_notes_docs.rs",
    ] {
        assert!(
            tests.contains(path),
            "missing inventoried test path {path}: {tests}"
        );
    }
}

#[test]
fn base1_inventory_docs_keep_non_claims() {
    let inventory = std::fs::read_to_string("docs/base1/INVENTORY.md").expect("Base1 inventory");
    let tests =
        std::fs::read_to_string("docs/base1/TEST_INVENTORY.md").expect("Base1 test inventory");

    for doc in [inventory, tests] {
        assert!(doc.contains("does not make Base1 installer-ready"), "{doc}");
        assert!(doc.contains("hardware-validated"), "{doc}");
        assert!(doc.contains("daily-driver ready"), "{doc}");
    }
}
