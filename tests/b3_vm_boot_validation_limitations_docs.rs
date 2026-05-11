#[test]
fn b3_limitations_note_defines_scope_and_command() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_LIMITATIONS.md")
        .expect("B3 VM boot validation limitations note");

    assert!(doc.contains("Base1 B3 VM boot validation limitations"), "{doc}");
    assert!(
        doc.contains("known limits for the future B3 VM boot validation path"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-b3-vm-validate.sh --dry-run --profile x86_64-vm-validation"),
        "{doc}"
    );
    assert!(
        doc.contains("The first B3 command surface should remain dry-run only"),
        "{doc}"
    );
}

#[test]
fn b3_limitations_note_lists_what_b3_can_eventually_prove() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_LIMITATIONS.md")
        .expect("B3 VM boot validation limitations note");

    for text in [
        "the VM profile is explicit",
        "the VM runtime is explicit",
        "the boot artifact is explicit",
        "the boot command is documented",
        "logs were captured",
        "the observed boot result is recorded",
        "the observed Phase1 launch result is recorded",
        "the emergency fallback result or limitation is recorded",
        "known limitations are documented",
    ] {
        assert!(doc.contains(text), "missing can-prove text {text}: {doc}");
    }
}

#[test]
fn b3_limitations_note_lists_what_b3_cannot_prove() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_LIMITATIONS.md")
        .expect("B3 VM boot validation limitations note");

    for text in [
        "physical hardware support",
        "installer readiness",
        "recovery completion",
        "rollback completion",
        "hardened status",
        "release-candidate readiness",
        "daily-driver readiness",
        "support for all x86_64 systems",
        "support for secure boot, measured boot, TPM, or lockdown modes",
    ] {
        assert!(doc.contains(text), "missing cannot-prove text {text}: {doc}");
    }
}

#[test]
fn b3_limitations_note_preserves_required_behavior() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_LIMITATIONS.md")
        .expect("B3 VM boot validation limitations note");

    for text in [
        "require B2 validation before B3 claims",
        "use a named VM profile",
        "keep logs and reports explicit",
        "avoid generalizing VM results to physical hardware",
        "keep limitations visible",
        "preserve non-claims",
    ] {
        assert!(doc.contains(text), "missing required behavior text {text}: {doc}");
    }
}

#[test]
fn b3_limitations_note_links_related_docs() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_LIMITATIONS.md")
        .expect("B3 VM boot validation limitations note");

    for link in [
        "BOOT_READINESS_STATUS.md",
        "BOOT_READINESS_RACE_PLAN.md",
        "B3_VM_BOOT_VALIDATION_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md",
        "X86_64_BOOT_SUPPORT_ROADMAP.md",
    ] {
        assert!(doc.contains(link), "missing related doc link {link}: {doc}");
    }
}

#[test]
fn b3_limitations_note_is_linked_from_status_tracker() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    assert!(
        status.contains("B3_VM_BOOT_VALIDATION_LIMITATIONS.md"),
        "{status}"
    );
    assert!(status.contains("B3 limitations note exists."), "{status}");
}

#[test]
fn b3_limitations_note_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_VALIDATION_LIMITATIONS.md")
        .expect("B3 VM boot validation limitations note");

    for text in [
        "does not make Base1 bootable on physical hardware",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(doc.contains(text), "missing non-claim {text}: {doc}");
    }
}
