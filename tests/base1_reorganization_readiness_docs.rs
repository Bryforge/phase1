#[test]
fn reorganization_readiness_records_current_state() {
    let doc = std::fs::read_to_string("docs/base1/REORGANIZATION_READINESS.md")
        .expect("Base1 reorganization readiness doc");

    assert!(
        doc.contains("Base1 is not ready for a full reorganization yet."),
        "{doc}"
    );
    assert!(
        doc.contains("safe incremental organization only"),
        "{doc}"
    );
}

#[test]
fn reorganization_readiness_lists_current_safeguards() {
    let doc = std::fs::read_to_string("docs/base1/REORGANIZATION_READINESS.md")
        .expect("Base1 reorganization readiness doc");

    for item in [
        "docs/base1/DOCUMENTATION_MAP.md",
        "docs/base1/INVENTORY.md",
        "docs/base1/TEST_INVENTORY.md",
        "scripts/base1-test-inventory.sh",
        "docs/base1/MIGRATION_TABLE.md",
        "docs/base1/SCRIPT_COMPATIBILITY_PLAN.md",
        "docs/base1/LINK_CHECK_STRATEGY.md",
        "scripts/base1-link-check.sh",
        "docs/base1/POST_REORGANIZATION_LAYOUT.md",
        "docs/base1/ROOT_COMPATIBILITY_MAP.md",
        "scripts/base1-doc-integrity.sh",
        "sh scripts/quality-check.sh base1-docs",
    ] {
        assert!(doc.contains(item), "missing safeguard {item}: {doc}");
    }
}

#[test]
fn reorganization_readiness_lists_remaining_blockers() {
    let doc = std::fs::read_to_string("docs/base1/REORGANIZATION_READINESS.md")
        .expect("Base1 reorganization readiness doc");

    for blocker in [
        "complete repository-wide Base1 test listing confirmed against reporter output",
        "Tests for every major moved group before that group is moved",
        "final integrity pass through `sh scripts/quality-check.sh base1-docs` and `cargo test --all-targets`",
    ] {
        assert!(doc.contains(blocker), "missing blocker {blocker}: {doc}");
    }
}

#[test]
fn reorganization_readiness_requires_compatibility_and_validation() {
    let doc = std::fs::read_to_string("docs/base1/REORGANIZATION_READINESS.md")
        .expect("Base1 reorganization readiness doc");

    for requirement in [
        "Every old path has a compatibility decision.",
        "Every inbound link is accounted for.",
        "Every release/checkpoint note remains recoverable.",
        "No compatibility path is removed without explicit future approval.",
        "Any script relocation has a wrapper or compatibility command plan.",
        "Markdown path movement is protected by a local, read-only link checker or equivalent validation.",
        "The post-reorganization layout names the stable public paths.",
    ] {
        assert!(doc.contains(requirement), "missing requirement {requirement}: {doc}");
    }
}

#[test]
fn reorganization_readiness_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/base1/REORGANIZATION_READINESS.md")
        .expect("Base1 reorganization readiness doc");

    assert!(doc.contains("does not make Base1 installer-ready"), "{doc}");
    assert!(doc.contains("hardware-validated"), "{doc}");
    assert!(doc.contains("daily-driver ready"), "{doc}");
}
