#[test]
fn boot_readiness_race_plan_defines_goal_and_boundary() {
    let plan = std::fs::read_to_string("docs/os/BOOT_READINESS_RACE_PLAN.md")
        .expect("boot readiness race plan");

    assert!(plan.contains("Base1 boot readiness race plan"), "{plan}");
    assert!(
        plan.contains("fastest safe path toward boot readiness"),
        "{plan}"
    );
    assert!(
        plan.contains("speed without unsafe claims"),
        "{plan}"
    );
    assert!(
        plan.contains("Phase1 is still a terminal-first virtual OS console"),
        "{plan}"
    );
}

#[test]
fn boot_readiness_race_plan_defines_readiness_ladder() {
    let plan = std::fs::read_to_string("docs/os/BOOT_READINESS_RACE_PLAN.md")
        .expect("boot readiness race plan");

    for level in [
        "B0",
        "Documentation ready",
        "B1",
        "Read-only detection ready",
        "B2",
        "Dry-run assembly ready",
        "B3",
        "VM boot validated",
        "B4",
        "Recovery validated",
        "B5",
        "Physical target validated",
        "B6",
        "Release candidate",
        "Do not skip levels when strengthening claims.",
    ] {
        assert!(plan.contains(level), "missing boot readiness level {level}: {plan}");
    }
}

#[test]
fn boot_readiness_race_plan_defines_fast_sprints() {
    let plan = std::fs::read_to_string("docs/os/BOOT_READINESS_RACE_PLAN.md")
        .expect("boot readiness race plan");

    for sprint in [
        "Sprint 1: readiness inventory",
        "Sprint 2: read-only detection",
        "Sprint 3: boot parameter inventory",
        "Sprint 4: image and initrd path",
        "Sprint 5: VM validation",
        "Sprint 6: recovery and rollback",
        "Sprint 7: hardware validation",
        "Sprint 8: release-candidate evidence",
    ] {
        assert!(plan.contains(sprint), "missing boot readiness sprint {sprint}: {plan}");
    }
}

#[test]
fn boot_readiness_race_plan_lists_required_artifacts() {
    let plan = std::fs::read_to_string("docs/os/BOOT_READINESS_RACE_PLAN.md")
        .expect("boot readiness race plan");

    for artifact in [
        "ROADMAP.md",
        "X86_64_BOOT_SUPPORT_ROADMAP.md",
        "BASE1_IMAGE_BUILDER.md",
        "INSTALLER_RECOVERY.md",
        "BASE1_DRY_RUN_COMMANDS.md",
        "BASE1_RECOVERY_COMMAND.md",
        "BASE1_STORAGE_LAYOUT_CHECKER.md",
        "BASE1_ROLLBACK_METADATA.md",
        "../../base1/HARDWARE_TARGETS.md",
        "../../base1/LIBREBOOT_PROFILE.md",
    ] {
        assert!(plan.contains(artifact), "missing boot artifact {artifact}: {plan}");
    }
}

#[test]
fn boot_readiness_race_plan_preserves_checklist_and_safety_rules() {
    let plan = std::fs::read_to_string("docs/os/BOOT_READINESS_RACE_PLAN.md")
        .expect("boot readiness race plan");

    for text in [
        "target name is explicit",
        "architecture is explicit",
        "firmware mode is explicit",
        "boot loader path is explicit",
        "boot parameters are documented",
        "image build path is documented",
        "Phase1 autostart path is documented",
        "emergency shell fallback is documented",
        "recovery path is documented",
        "rollback path is documented",
        "validation report exists",
        "Read-only before dry-run.",
        "Dry-run before mutation.",
        "VM validation before broad hardware claims.",
        "Recovery before installer claims.",
        "Evidence before hardening claims.",
    ] {
        assert!(plan.contains(text), "missing checklist/safety text {text}: {plan}");
    }
}

#[test]
fn boot_readiness_race_plan_is_linked_from_os_roadmap_and_readme() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");
    let readme = std::fs::read_to_string("README.md").expect("README");

    assert!(roadmap.contains("BOOT_READINESS_RACE_PLAN.md"), "{roadmap}");
    assert!(roadmap.contains("boot readiness ladder"), "{roadmap}");
    assert!(
        readme.contains("BOOT_READINESS_RACE_PLAN.md") || readme.contains("boot readiness"),
        "README should mention or link boot readiness: {readme}"
    );
}

#[test]
fn boot_readiness_race_plan_preserves_non_claims() {
    let plan = std::fs::read_to_string("docs/os/BOOT_READINESS_RACE_PLAN.md")
        .expect("boot readiness race plan");

    for text in [
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "daily-driver ready",
    ] {
        assert!(plan.contains(text), "missing boot readiness non-claim {text}: {plan}");
    }
}
