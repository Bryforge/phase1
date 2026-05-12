#[test]
fn pre_move_checklist_defines_required_before_move_items() {
    let doc = std::fs::read_to_string("docs/base1/PRE_MOVE_CHECKLIST.md")
        .expect("Base1 pre-move checklist");

    assert!(doc.contains("Base1 pre-move checklist"), "{doc}");
    assert!(doc.contains("Required before any group move"), "{doc}");

    for required in [
        "docs/base1/INVENTORY.md",
        "docs/base1/TEST_INVENTORY.md",
        "docs/base1/MIGRATION_TABLE.md",
        "docs/base1/POST_REORGANIZATION_LAYOUT.md",
        "docs/base1/ROOT_COMPATIBILITY_MAP.md",
        "docs/base1/SCRIPT_COMPATIBILITY_PLAN.md",
        "scripts/base1-link-check.sh",
        "sh scripts/quality-check.sh base1-docs",
    ] {
        assert!(
            doc.contains(required),
            "missing pre-move requirement {required}: {doc}"
        );
    }
}

#[test]
fn pre_move_checklist_requires_tests_before_moves() {
    let doc = std::fs::read_to_string("docs/base1/PRE_MOVE_CHECKLIST.md")
        .expect("Base1 pre-move checklist");

    assert!(doc.contains("Pre-move test requirement"), "{doc}");

    for required in [
        "Old compatibility paths are still present or wrapped.",
        "New organized paths are present.",
        "Indexes link to the new organized paths.",
        "Existing public paths remain recoverable.",
        "Non-claims remain visible.",
        "Dry-run and read-only wording remains visible where relevant.",
    ] {
        assert!(
            doc.contains(required),
            "missing test requirement {required}: {doc}"
        );
    }
}

#[test]
fn pre_move_checklist_preserves_first_safe_candidate_limits() {
    let doc = std::fs::read_to_string("docs/base1/PRE_MOVE_CHECKLIST.md")
        .expect("Base1 pre-move checklist");

    assert!(doc.contains("First safe candidate group"), "{doc}");
    assert!(doc.contains("release/checkpoint notes"), "{doc}");
    assert!(
        doc.contains("no root checkpoint file should be removed"),
        "{doc}"
    );
}

#[test]
fn pre_move_checklist_blocks_unsafe_moves() {
    let doc = std::fs::read_to_string("docs/base1/PRE_MOVE_CHECKLIST.md")
        .expect("Base1 pre-move checklist");

    for blocked in [
        "The reporter and test inventory disagree.",
        "Link checking fails.",
        "The migration table lacks a compatibility decision.",
        "A script move lacks a wrapper plan.",
        "A public/root path would disappear.",
        "Non-claims would be weakened.",
        "The move would require deletion to look clean.",
    ] {
        assert!(
            doc.contains(blocked),
            "missing blocked condition {blocked}: {doc}"
        );
    }
}

#[test]
fn pre_move_checklist_is_linked_and_integrity_checked() {
    let manual = std::fs::read_to_string("docs/base1/README.md").expect("Base1 manual");
    let map = std::fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md")
        .expect("Base1 documentation map");
    let integrity =
        std::fs::read_to_string("scripts/base1-doc-integrity.sh").expect("Base1 integrity gate");

    for doc in [&manual, &map, &integrity] {
        assert!(doc.contains("PRE_MOVE_CHECKLIST.md"), "{doc}");
    }
}

#[test]
fn pre_move_checklist_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/base1/PRE_MOVE_CHECKLIST.md")
        .expect("Base1 pre-move checklist");

    assert!(doc.contains("does not make Base1 installer-ready"), "{doc}");
    assert!(doc.contains("hardware-validated"), "{doc}");
    assert!(doc.contains("daily-driver ready"), "{doc}");
}
