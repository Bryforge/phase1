#[test]
fn b3_vm_boot_validation_plan_defines_scope_and_entry_gate() {
    let plan = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_PLAN.md")
        .expect("B3 VM boot validation plan");

    assert!(plan.contains("Base1 B3 VM boot validation plan"), "{plan}");
    assert!(
        plan.contains("evidence needed before Base1 can claim VM boot validation for a named profile"),
        "{plan}"
    );
    assert!(
        plan.contains("B3 validation should not start until the focused B2 test suite has passed locally or in CI."),
        "{plan}"
    );
    assert!(plan.contains("B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md"), "{plan}");
}

#[test]
fn b3_vm_boot_validation_plan_defines_initial_profile_and_command_shape() {
    let plan = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_PLAN.md")
        .expect("B3 VM boot validation plan");

    for text in [
        "x86_64-vm-validation",
        "sh scripts/base1-b3-vm-validate.sh --dry-run --profile x86_64-vm-validation",
        "The first command surface should be dry-run only.",
        "Any real VM run should come later with a separate validation report and captured logs.",
    ] {
        assert!(plan.contains(text), "missing B3 command/profile text {text}: {plan}");
    }
}

#[test]
fn b3_vm_boot_validation_plan_lists_required_evidence() {
    let plan = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_PLAN.md")
        .expect("B3 VM boot validation plan");

    for text in [
        "selected VM profile",
        "VM runtime used",
        "architecture and firmware mode",
        "boot artifact identifier",
        "command used for the VM run",
        "boot result",
        "Phase1 launch result",
        "emergency fallback result or known limitation",
        "captured logs path",
        "known limitations",
        "explicit non-claims",
    ] {
        assert!(plan.contains(text), "missing B3 evidence text {text}: {plan}");
    }
}

#[test]
fn b3_vm_boot_validation_plan_preserves_checklist() {
    let plan = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_PLAN.md")
        .expect("B3 VM boot validation plan");

    for text in [
        "B2 test suite has passed locally or in CI.",
        "VM profile is explicit.",
        "VM runtime is explicit.",
        "Boot artifact is explicit.",
        "Boot command is documented.",
        "Logs are captured.",
        "Phase1 launch result is recorded.",
        "Emergency fallback result or limitation is recorded.",
        "Known limitations are documented.",
        "VM result is not generalized to physical hardware.",
        "Non-claims are preserved.",
    ] {
        assert!(plan.contains(text), "missing B3 checklist text {text}: {plan}");
    }
}

#[test]
fn b3_vm_boot_validation_plan_links_related_docs() {
    let plan = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_PLAN.md")
        .expect("B3 VM boot validation plan");

    for link in [
        "BOOT_READINESS_STATUS.md",
        "BOOT_READINESS_RACE_PLAN.md",
        "ROADMAP.md",
        "X86_64_BOOT_SUPPORT_ROADMAP.md",
        "B2_DRY_RUN_ASSEMBLY_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md",
    ] {
        assert!(plan.contains(link), "missing B3 related doc link {link}: {plan}");
    }
}

#[test]
fn b3_vm_boot_validation_plan_is_linked_from_status_tracker() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    assert!(status.contains("B3_VM_BOOT_VALIDATION_PLAN.md"), "{status}");
    assert!(status.contains("B3 VM boot validation planning is now present"), "{status}");
    assert!(
        status.contains("B3 is planning-only until VM validation evidence exists"),
        "{status}"
    );
}

#[test]
fn b3_vm_boot_validation_plan_preserves_non_claims() {
    let plan = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_PLAN.md")
        .expect("B3 VM boot validation plan");

    for text in [
        "does not make Base1 bootable on physical hardware",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(plan.contains(text), "missing B3 non-claim {text}: {plan}");
    }
}
