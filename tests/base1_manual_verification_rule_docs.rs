#[test]
fn manual_documents_base1_verification_rule() {
    let manual = std::fs::read_to_string("docs/base1/README.md").expect("Base1 manual");

    assert!(manual.contains("Verification rule"), "{manual}");
    assert!(
        manual.contains("sh scripts/quality-check.sh base1-docs"),
        "{manual}"
    );
    assert!(
        manual.contains("sh scripts/base1-reorganization-verify.sh"),
        "{manual}"
    );
}

#[test]
fn manual_explains_reorganization_verifier_scope() {
    let manual = std::fs::read_to_string("docs/base1/README.md").expect("Base1 manual");

    for text in [
        "The verification bundle is read-only.",
        "Base1 integrity gate",
        "local link checker",
        "test-inventory verifier",
        "cargo test --all-targets",
        "when Cargo is available",
    ] {
        assert!(
            manual.contains(text),
            "missing verifier scope {text}: {manual}"
        );
    }
}

#[test]
fn manual_keeps_organization_guardrails_visible() {
    let manual = std::fs::read_to_string("docs/base1/README.md").expect("Base1 manual");

    for text in [
        "RELEASE_ARCHIVE_MAP.md",
        "INVENTORY.md",
        "TEST_INVENTORY.md",
        "MIGRATION_TABLE.md",
        "SCRIPT_COMPATIBILITY_PLAN.md",
        "LINK_CHECK_STRATEGY.md",
        "POST_REORGANIZATION_LAYOUT.md",
        "PRE_MOVE_CHECKLIST.md",
        "REORGANIZATION_READINESS.md",
    ] {
        assert!(
            manual.contains(text),
            "missing guardrail link {text}: {manual}"
        );
    }
}

#[test]
fn manual_preserves_base1_non_claims() {
    let manual = std::fs::read_to_string("docs/base1/README.md").expect("Base1 manual");

    assert!(
        manual.contains("not currently documented here as a released bootable daily-driver image"),
        "{manual}"
    );
    assert!(
        manual.contains("destructive installer-ready system"),
        "{manual}"
    );
    assert!(
        manual.contains("does not validate boot, hardware, recovery"),
        "{manual}"
    );
}
