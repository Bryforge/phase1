#[test]
fn reorganization_plan_defines_minimalist_target_structure() {
    let plan = std::fs::read_to_string("docs/REORGANIZATION_PLAN.md").expect("reorganization plan");

    assert!(
        plan.contains("Phase1 repository reorganization plan"),
        "{plan}"
    );
    assert!(plan.contains("minimalist visible structure"), "{plan}");

    for path in [
        ".github/",
        "assets/",
        "base1/",
        "docs/",
        "examples/",
        "scripts/",
        "tests/",
        "tools/",
        "src/",
        "phase1-core/",
        "xtask/",
    ] {
        assert!(plan.contains(path), "missing target path {path}: {plan}");
    }
}

#[test]
fn reorganization_plan_preserves_create_a_place_policy() {
    let plan = std::fs::read_to_string("docs/REORGANIZATION_PLAN.md").expect("reorganization plan");

    assert!(plan.contains("Create-a-place checklist"), "{plan}");

    for category in [
        "docs;",
        "source;",
        "tests;",
        "scripts;",
        "assets;",
        "examples;",
        "tools;",
        "release notes;",
        "compatibility path;",
        "GitHub automation.",
    ] {
        assert!(
            plan.contains(category),
            "missing destination category {category}: {plan}"
        );
    }

    assert!(
        plan.contains(
            "If none fits, create an index or destination folder first, then move later."
        ),
        "{plan}"
    );
}

#[test]
fn reorganization_plan_preserves_first_rules() {
    let plan = std::fs::read_to_string("docs/REORGANIZATION_PLAN.md").expect("reorganization plan");

    for rule in [
        "Do not delete or move public/root compatibility files until all of these exist:",
        "documented old path -> new path map;",
        "compatibility shim or mirror decision;",
        "link-check coverage;",
        "tests for required links;",
        "rollback plan;",
        "maintainer approval.",
        "When in doubt, add an index before moving a file.",
    ] {
        assert!(
            plan.contains(rule),
            "missing preserve-first rule {rule}: {plan}"
        );
    }
}

#[test]
fn reorganization_plan_defines_destination_map_and_root_policy() {
    let plan = std::fs::read_to_string("docs/REORGANIZATION_PLAN.md").expect("reorganization plan");

    for text in [
        "Proposed destination map",
        "Public entry point",
        "Contribution rules",
        "Security policy",
        "Quality system",
        "Release notes",
        "Security and crypto docs",
        "Community/support docs",
        "Base1 organized docs",
        "Base1 compatibility docs",
        "Examples",
        "Scripts",
        "Internal helper tools",
        "Root-level policy",
        "Root should stay readable and small.",
    ] {
        assert!(
            plan.contains(text),
            "missing destination/root policy text {text}: {plan}"
        );
    }
}

#[test]
fn reorganization_plan_defines_special_handling_for_base1_and_crypto() {
    let plan = std::fs::read_to_string("docs/REORGANIZATION_PLAN.md").expect("reorganization plan");

    for text in [
        "Base1 compatibility policy",
        "Base1 is special because it has many root-level and release/checkpoint references.",
        "keep existing Base1 compatibility paths unless a tested wrapper or mirror exists;",
        "sh scripts/quality-check.sh base1-docs",
        "sh scripts/quality-check.sh base1-reorg",
        "Security and crypto organization policy",
        "Security and crypto docs stay under:",
        "sh scripts/quality-check.sh security-crypto-docs",
        "Do not move or rename security/crypto files without updating the integrity gate and tests.",
    ] {
        assert!(plan.contains(text), "missing Base1/crypto policy text {text}: {plan}");
    }
}

#[test]
fn reorganization_plan_defines_phases_move_map_and_rollback() {
    let plan = std::fs::read_to_string("docs/REORGANIZATION_PLAN.md").expect("reorganization plan");

    for text in [
        "Phase 1: navigation and indexes",
        "Phase 2: destination folders",
        "Phase 3: docs mirrors and indexes",
        "Phase 4: low-risk moves",
        "Phase 5: assets and examples",
        "Phase 6: script/tool boundary",
        "Phase 7: broad cleanup",
        "Move map template",
        "Current path",
        "Proposed path",
        "Compatibility method",
        "Rollback rule",
        "Every reorganization PR should be reversible.",
    ] {
        assert!(
            plan.contains(text),
            "missing phase/move/rollback text {text}: {plan}"
        );
    }
}

#[test]
fn reorganization_plan_is_linked_from_docs_index_and_navigation() {
    let docs_index = std::fs::read_to_string("docs/README.md").expect("docs index");
    let nav = std::fs::read_to_string("docs/REPOSITORY_NAVIGATION.md")
        .expect("repository navigation guide");

    for doc in [&docs_index, &nav] {
        assert!(doc.contains("REORGANIZATION_PLAN.md"), "{doc}");
    }
}

#[test]
fn reorganization_plan_preserves_non_claims() {
    let plan = std::fs::read_to_string("docs/REORGANIZATION_PLAN.md").expect("reorganization plan");

    assert!(plan.contains("does not move files by itself"), "{plan}");
    assert!(plan.contains("prove repository quality"), "{plan}");
    assert!(plan.contains("remove compatibility obligations"), "{plan}");
    assert!(
        plan.contains("make Phase1, Base1, or Fyr production-ready"),
        "{plan}"
    );
}
