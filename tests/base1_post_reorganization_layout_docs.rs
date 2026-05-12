#[test]
fn post_reorganization_layout_defines_stable_public_paths() {
    let doc = std::fs::read_to_string("docs/base1/POST_REORGANIZATION_LAYOUT.md")
        .expect("Base1 post-reorganization layout");

    assert!(doc.contains("Base1 post-reorganization layout"), "{doc}");
    assert!(doc.contains("proposed stable layout"), "{doc}");
    assert!(doc.contains("Stable public paths"), "{doc}");

    for path in [
        "base1/README.md",
        "docs/base1/README.md",
        "docs/base1/DOCUMENTATION_MAP.md",
        "docs/base1/INVENTORY.md",
        "docs/base1/TEST_INVENTORY.md",
        "docs/base1/MIGRATION_TABLE.md",
        "docs/base1/SCRIPT_COMPATIBILITY_PLAN.md",
        "docs/base1/LINK_CHECK_STRATEGY.md",
        "docs/base1/REORGANIZATION_READINESS.md",
        "docs/base1/RELEASE_ARCHIVE_MAP.md",
    ] {
        assert!(doc.contains(path), "missing stable path {path}: {doc}");
    }
}

#[test]
fn post_reorganization_layout_preserves_public_compatibility_paths() {
    let doc = std::fs::read_to_string("docs/base1/POST_REORGANIZATION_LAYOUT.md")
        .expect("Base1 post-reorganization layout");

    for text in [
        "Root-level checkpoint notes remain compatibility paths.",
        "Current script paths remain the stable operator interface.",
        "scripts/base1-*.sh",
        "tests/base1_*.rs",
        "tests/quality_base1_*.rs",
        "tests/*base1*.rs",
        "Compatibility paths remain valid",
        "Existing public documentation links stay recoverable.",
    ] {
        assert!(
            doc.contains(text),
            "missing compatibility text {text}: {doc}"
        );
    }
}

#[test]
fn post_reorganization_layout_lists_major_layout_sections() {
    let doc = std::fs::read_to_string("docs/base1/POST_REORGANIZATION_LAYOUT.md")
        .expect("Base1 post-reorganization layout");

    for heading in [
        "Core docs layout",
        "Organization docs layout",
        "Release/checkpoint notes layout",
        "Real-device read-only layout",
        "OS-track design slices",
        "Script layout",
        "Test layout",
        "Compatibility policy",
        "Validation before any movement",
    ] {
        assert!(
            doc.contains(heading),
            "missing layout heading {heading}: {doc}"
        );
    }
}

#[test]
fn post_reorganization_layout_is_linked_and_integrity_checked() {
    let manual = std::fs::read_to_string("docs/base1/README.md").expect("Base1 manual");
    let map = std::fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md")
        .expect("Base1 documentation map");
    let integrity =
        std::fs::read_to_string("scripts/base1-doc-integrity.sh").expect("Base1 integrity gate");

    for doc in [&manual, &map, &integrity] {
        assert!(doc.contains("POST_REORGANIZATION_LAYOUT.md"), "{doc}");
    }
}

#[test]
fn post_reorganization_layout_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/base1/POST_REORGANIZATION_LAYOUT.md")
        .expect("Base1 post-reorganization layout");

    assert!(doc.contains("does not make Base1 installer-ready"), "{doc}");
    assert!(doc.contains("hardware-validated"), "{doc}");
    assert!(doc.contains("daily-driver ready"), "{doc}");
}
