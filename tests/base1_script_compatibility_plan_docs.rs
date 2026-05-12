#[test]
fn script_compatibility_plan_defines_stable_operator_paths() {
    let plan = std::fs::read_to_string("docs/base1/SCRIPT_COMPATIBILITY_PLAN.md")
        .expect("Base1 script compatibility plan");

    assert!(plan.contains("Base1 script compatibility plan"), "{plan}");
    assert!(
        plan.contains("Current script paths remain the stable operator interface"),
        "{plan}"
    );
    assert!(plan.contains("scripts/base1-preflight.sh"), "{plan}");
    assert!(plan.contains("scripts/base1-doc-integrity.sh"), "{plan}");
    assert!(plan.contains("scripts/base1-*.sh"), "{plan}");
}

#[test]
fn script_compatibility_plan_requires_wrappers_before_moves() {
    let plan = std::fs::read_to_string("docs/base1/SCRIPT_COMPATIBILITY_PLAN.md")
        .expect("Base1 script compatibility plan");

    assert!(
        plan.contains(
            "Do not move Base1 scripts until compatibility wrappers are planned and tested."
        ),
        "{plan}"
    );
    assert!(
        plan.contains("Keep the original `scripts/base1-*.sh` path."),
        "{plan}"
    );
    assert!(
        plan.contains("Preserve command-line arguments exactly."),
        "{plan}"
    );
    assert!(
        plan.contains("Preserve read-only and dry-run wording."),
        "{plan}"
    );
    assert!(
        plan.contains("Add tests proving the old path still works."),
        "{plan}"
    );
}

#[test]
fn script_compatibility_plan_lists_future_candidate_groups() {
    let plan = std::fs::read_to_string("docs/base1/SCRIPT_COMPATIBILITY_PLAN.md")
        .expect("Base1 script compatibility plan");

    for group in [
        "scripts/base1/core/",
        "scripts/base1/dry-run/",
        "scripts/base1/libreboot/",
        "scripts/base1/recovery-usb/",
        "scripts/base1/real-device/",
        "scripts/base1/quality/",
    ] {
        assert!(
            plan.contains(group),
            "missing candidate group {group}: {plan}"
        );
    }
}

#[test]
fn script_compatibility_plan_is_linked_from_base1_indexes() {
    let manual = std::fs::read_to_string("docs/base1/README.md").expect("Base1 manual");
    let map = std::fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md")
        .expect("Base1 documentation map");
    let integrity = std::fs::read_to_string("scripts/base1-doc-integrity.sh")
        .expect("Base1 doc integrity script");

    assert!(manual.contains("SCRIPT_COMPATIBILITY_PLAN.md"), "{manual}");
    assert!(map.contains("SCRIPT_COMPATIBILITY_PLAN.md"), "{map}");
    assert!(
        integrity.contains("SCRIPT_COMPATIBILITY_PLAN.md"),
        "{integrity}"
    );
}

#[test]
fn script_compatibility_plan_preserves_non_claims() {
    let plan = std::fs::read_to_string("docs/base1/SCRIPT_COMPATIBILITY_PLAN.md")
        .expect("Base1 script compatibility plan");

    assert!(plan.contains("does not move scripts"), "{plan}");
    assert!(
        plan.contains("does not make Base1 installer-ready"),
        "{plan}"
    );
    assert!(plan.contains("hardware-validated"), "{plan}");
    assert!(plan.contains("daily-driver ready"), "{plan}");
}
