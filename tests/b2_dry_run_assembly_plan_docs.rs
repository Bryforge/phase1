#[test]
fn b2_dry_run_assembly_plan_defines_scope_and_command() {
    let plan = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_PLAN.md")
        .expect("B2 dry-run assembly plan");

    assert!(plan.contains("Base1 B2 dry-run assembly plan"), "{plan}");
    assert!(
        plan.contains("B2 boot-readiness dry-run assembly path"),
        "{plan}"
    );
    assert!(
        plan.contains("sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation"),
        "{plan}"
    );
    assert!(
        plan.contains("The command must require `--dry-run` and a named profile."),
        "{plan}"
    );
}

#[test]
fn b2_dry_run_assembly_plan_defines_goal_and_flow() {
    let plan = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_PLAN.md")
        .expect("B2 dry-run assembly plan");

    for text in [
        "B1 detection facts",
        "boot profile selection",
        "image-builder preview",
        "kernel/initramfs handoff preview",
        "Phase1 autostart preview",
        "emergency shell fallback preview",
        "installer dry-run preview",
        "recovery dry-run preview",
        "rollback metadata dry-run preview",
        "validation bundle summary",
        "known limitations",
    ] {
        assert!(plan.contains(text), "missing B2 flow text {text}: {plan}");
    }
}

#[test]
fn b2_dry_run_assembly_plan_lists_required_output() {
    let plan = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_PLAN.md")
        .expect("B2 dry-run assembly plan");

    for text in [
        "`writes: no`",
        "selected boot-readiness level: `B2`",
        "selected profile",
        "architecture and firmware facts from B1 or explicit unknowns",
        "boot profile assumptions",
        "image-builder preview status",
        "kernel/initramfs handoff preview",
        "Phase1 autostart preview",
        "emergency shell fallback preview",
        "installer preview status",
        "recovery preview status",
        "rollback metadata preview status",
        "validation bundle path or planned path",
        "next recommended validation step",
    ] {
        assert!(plan.contains(text), "missing B2 output text {text}: {plan}");
    }
}

#[test]
fn b2_dry_run_assembly_plan_defines_profiles() {
    let plan = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_PLAN.md")
        .expect("B2 dry-run assembly plan");

    for profile in [
        "x86_64-uefi-generic",
        "x86_64-bios-generic",
        "x86_64-libreboot-grub",
        "x86_64-vm-validation",
        "x86_64-recovery-usb",
    ] {
        assert!(plan.contains(profile), "missing B2 profile {profile}: {plan}");
    }

    assert!(
        plan.contains("The first implementation should prefer `x86_64-vm-validation`"),
        "{plan}"
    );
}

#[test]
fn b2_dry_run_assembly_plan_preserves_stages() {
    let plan = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_PLAN.md")
        .expect("B2 dry-run assembly plan");

    for text in [
        "Detect",
        "Profile",
        "Image",
        "Boot handoff",
        "Recovery",
        "Rollback",
        "Validation bundle",
        "B1 detector summary or unknowns",
        "Kernel/initramfs/Phase1 autostart preview",
        "Rollback metadata preview",
    ] {
        assert!(plan.contains(text), "missing B2 stage text {text}: {plan}");
    }
}

#[test]
fn b2_dry_run_assembly_plan_preserves_non_mutation_rules() {
    let plan = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_PLAN.md")
        .expect("B2 dry-run assembly plan");

    for text in [
        "write images",
        "write partitions",
        "format disks",
        "mount filesystems read-write",
        "edit `/boot`, `/etc`, EFI variables, initramfs files, or partitions",
        "call mutating boot-loader commands",
        "install packages",
        "require network access",
        "mark hardware as validated",
        "claim a bootable release candidate",
    ] {
        assert!(plan.contains(text), "missing B2 non-mutation rule {text}: {plan}");
    }
}

#[test]
fn b2_dry_run_assembly_plan_links_required_integration_points() {
    let plan = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_PLAN.md")
        .expect("B2 dry-run assembly plan");

    for link in [
        "BOOT_READINESS_STATUS.md",
        "BOOT_READINESS_RACE_PLAN.md",
        "B1_READ_ONLY_DETECTION_PLAN.md",
        "B1_READ_ONLY_DETECTION_LIMITATIONS.md",
        "B1_READ_ONLY_DETECTION_VALIDATION.md",
        "X86_64_BOOT_SUPPORT_ROADMAP.md",
        "BASE1_IMAGE_BUILDER.md",
        "INSTALLER_RECOVERY.md",
        "BASE1_DRY_RUN_COMMANDS.md",
        "BASE1_ROLLBACK_METADATA.md",
    ] {
        assert!(plan.contains(link), "missing B2 integration link {link}: {plan}");
    }
}

#[test]
fn b2_dry_run_assembly_plan_defines_test_expectations_and_completion() {
    let plan = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_PLAN.md")
        .expect("B2 dry-run assembly plan");

    for text in [
        "B2 tests should verify",
        "B2 requires `--dry-run`",
        "B2 requires a named profile",
        "B2 reports `writes: no`",
        "profile, image, boot handoff, recovery, rollback, and validation bundle sections",
        "B2 dry-run assembly script exists",
        "B2 script tests exist",
        "B2 plan tests exist",
        "B2 known limitations are documented",
        "B2 validation report exists",
        "README and OS roadmap reflect the B2 boundary",
    ] {
        assert!(plan.contains(text), "missing B2 test/completion text {text}: {plan}");
    }
}

#[test]
fn b2_dry_run_assembly_plan_is_linked_from_status_and_race_and_roadmap() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status");
    let race = std::fs::read_to_string("docs/os/BOOT_READINESS_RACE_PLAN.md")
        .expect("boot readiness race plan");
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");

    for doc in [&status, &race, &roadmap] {
        assert!(doc.contains("B2_DRY_RUN_ASSEMBLY_PLAN.md"), "{doc}");
    }
}

#[test]
fn b2_dry_run_assembly_plan_preserves_non_claims() {
    let plan = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_PLAN.md")
        .expect("B2 dry-run assembly plan");

    for text in [
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(plan.contains(text), "missing B2 non-claim {text}: {plan}");
    }
}
