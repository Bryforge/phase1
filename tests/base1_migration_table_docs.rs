#[test]
fn migration_table_lists_required_groups() {
    let table =
        std::fs::read_to_string("docs/base1/MIGRATION_TABLE.md").expect("Base1 migration table");

    for heading in [
        "Core docs",
        "Organization docs",
        "Release/checkpoint notes",
        "Recovery USB docs",
        "Libreboot docs",
        "Scripts",
        "Tests",
        "Readiness impact",
    ] {
        assert!(
            table.contains(heading),
            "missing migration table heading {heading}: {table}"
        );
    }
}

#[test]
fn migration_table_preserves_compatibility_decisions() {
    let table =
        std::fs::read_to_string("docs/base1/MIGRATION_TABLE.md").expect("Base1 migration table");

    assert!(table.contains("Compatibility decision"), "{table}");
    assert!(table.contains("keep root compatibility path"), "{table}");
    assert!(table.contains("canonical"), "{table}");
    assert!(table.contains("mirrored"), "{table}");
    assert!(table.contains("pending review"), "{table}");
}

#[test]
fn migration_table_lists_key_current_and_target_paths() {
    let table =
        std::fs::read_to_string("docs/base1/MIGRATION_TABLE.md").expect("Base1 migration table");

    for path in [
        "base1/README.md",
        "docs/base1/DOCUMENTATION_MAP.md",
        "docs/base1/INVENTORY.md",
        "docs/base1/TEST_INVENTORY.md",
        "docs/base1/REORGANIZATION_READINESS.md",
        "RELEASE_BASE1_LIBREBOOT_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md",
        "RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md",
        "scripts/base1-doc-integrity.sh",
        "scripts/base1/",
        "tests/base1_*.rs",
        "tests/quality_base1_*.rs",
    ] {
        assert!(
            table.contains(path),
            "missing migration path {path}: {table}"
        );
    }
}

#[test]
fn migration_table_blocks_broad_moves_until_safety_work_exists() {
    let table =
        std::fs::read_to_string("docs/base1/MIGRATION_TABLE.md").expect("Base1 migration table");

    assert!(
        table.contains("No move until links, tests, and compatibility shims are planned."),
        "{table}"
    );
    assert!(
        table.contains("Complete repository-wide test listing."),
        "{table}"
    );
    assert!(table.contains("Compatibility shim plan"), "{table}");
    assert!(
        table.contains("Link checker or equivalent validation"),
        "{table}"
    );
}

#[test]
fn migration_table_preserves_non_claims() {
    let table =
        std::fs::read_to_string("docs/base1/MIGRATION_TABLE.md").expect("Base1 migration table");

    assert!(
        table.contains("does not make Base1 installer-ready"),
        "{table}"
    );
    assert!(table.contains("hardware-validated"), "{table}");
    assert!(table.contains("daily-driver ready"), "{table}");
}
