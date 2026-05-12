#[test]
fn documentation_map_lists_verification_commands() {
    let map = std::fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md")
        .expect("Base1 documentation map");

    for command in [
        "sh scripts/base1-doc-integrity.sh",
        "sh scripts/base1-link-check.sh",
        "sh scripts/base1-test-inventory-verify.sh",
        "sh scripts/base1-reorganization-verify.sh",
    ] {
        assert!(
            map.contains(command),
            "missing verification command {command}: {map}"
        );
    }
}

#[test]
fn documentation_map_links_organization_guardrails() {
    let map = std::fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md")
        .expect("Base1 documentation map");

    for link in [
        "INVENTORY.md",
        "TEST_INVENTORY.md",
        "MIGRATION_TABLE.md",
        "SCRIPT_COMPATIBILITY_PLAN.md",
        "LINK_CHECK_STRATEGY.md",
        "POST_REORGANIZATION_LAYOUT.md",
        "PRE_MOVE_CHECKLIST.md",
        "REORGANIZATION_READINESS.md",
        "REORGANIZATION_VERIFICATION_REPORT_TEMPLATE.md",
        "ROOT_COMPATIBILITY_MAP.md",
    ] {
        assert!(map.contains(link), "missing guardrail link {link}: {map}");
    }
}

#[test]
fn documentation_map_links_release_pre_move_checks() {
    let map = std::fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md")
        .expect("Base1 documentation map");

    assert!(map.contains("Release/checkpoint pre-move checks"), "{map}");
    assert!(map.contains("releases/PRE_MOVE_CHECKS.md"), "{map}");
    assert!(
        map.contains("Root checkpoint-note files remain compatibility paths"),
        "{map}"
    );
}

#[test]
fn documentation_map_preserves_non_claims() {
    let map = std::fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md")
        .expect("Base1 documentation map");

    assert!(map.contains("Not installer-ready"), "{map}");
    assert!(map.contains("Not hardware-validated"), "{map}");
    assert!(map.contains("Not daily-driver ready"), "{map}");
    assert!(map.contains("No destructive disk writes"), "{map}");
    assert!(map.contains("No real-device write path"), "{map}");
}
