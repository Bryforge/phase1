#[test]
fn b2_limitations_note_defines_scope_and_command() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md")
        .expect("B2 dry-run assembly limitations note");

    assert!(doc.contains("Base1 B2 dry-run assembly limitations"), "{doc}");
    assert!(
        doc.contains("known limitations for `scripts/base1-b2-assembly-dry-run.sh --dry-run --profile <profile>`"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation"),
        "{doc}"
    );
    assert!(
        doc.contains("connects boot-readiness planning pieces into a no-write assembly preview"),
        "{doc}"
    );
}

#[test]
fn b2_limitations_note_lists_what_b2_can_report() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md")
        .expect("B2 dry-run assembly limitations note");

    for text in [
        "`writes: no`",
        "selected boot-readiness level: `B2`",
        "selected profile",
        "B1 detector command reference",
        "basic machine and firmware hints",
        "profile assumptions",
        "image-builder preview status",
        "boot handoff preview status",
        "installer preview status",
        "recovery preview status",
        "rollback preview status",
        "validation bundle planned path",
        "known limitations",
        "next validation step",
    ] {
        assert!(doc.contains(text), "missing can-report text {text}: {doc}");
    }
}

#[test]
fn b2_limitations_note_lists_what_b2_cannot_prove() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md")
        .expect("B2 dry-run assembly limitations note");

    for text in [
        "that a Base1 image can be built",
        "that a Base1 image can boot",
        "that a boot loader is configured correctly",
        "that the selected profile is valid for real hardware",
        "that kernel or initramfs handoff will work",
        "that Phase1 autostart will work after boot",
        "that emergency shell fallback is usable",
        "that installer flow is safe beyond dry-run planning",
        "that recovery media exists or works",
        "that rollback is complete",
        "that VM validation has passed",
        "that hardware is validated",
        "that the system is hardened",
        "that the system is release-candidate ready",
        "that the system is daily-driver ready",
    ] {
        assert!(doc.contains(text), "missing cannot-prove text {text}: {doc}");
    }
}

#[test]
fn b2_limitations_note_preserves_known_limitations_table() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md")
        .expect("B2 dry-run assembly limitations note");

    for text in [
        "Detection",
        "B2 uses only lightweight local hints and does not consume a formal B1 report yet.",
        "Profile selection",
        "Image builder",
        "Boot handoff",
        "Installer",
        "Recovery",
        "Rollback",
        "Validation bundle",
        "VM validation",
        "Hardware",
        "No physical hardware support claim is allowed from B2.",
    ] {
        assert!(doc.contains(text), "missing known limitation text {text}: {doc}");
    }
}

#[test]
fn b2_limitations_note_preserves_required_non_mutating_behavior() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md")
        .expect("B2 dry-run assembly limitations note");

    for text in [
        "require `--dry-run`",
        "require `--profile`",
        "reject unsupported profiles",
        "report `writes: no`",
        "report `mutation: no`",
        "report `network: no`",
        "avoid image writes",
        "avoid boot-loader mutation",
        "avoid partition mutation",
        "avoid package installation",
        "avoid network access",
        "avoid writing to `/boot`, `/etc`, EFI variables, initramfs files, or partitions",
        "keep all boot, recovery, rollback, hardening, hardware, and release-candidate claims explicitly unclaimed",
    ] {
        assert!(doc.contains(text), "missing required behavior text {text}: {doc}");
    }
}

#[test]
fn b2_limitations_note_defines_b2_completion_requirements() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md")
        .expect("B2 dry-run assembly limitations note");

    for text in [
        "script test suite passing in CI or local validation",
        "status tracker links updated after implementation",
        "source review confirming no mutation paths",
        "output review confirming no secret leakage",
        "limitations documented in this file",
        "validation report documenting review results",
        "README or roadmap visibility for the B2 boundary",
    ] {
        assert!(doc.contains(text), "missing completion requirement {text}: {doc}");
    }
}

#[test]
fn b2_limitations_note_is_linked_from_status_tracker() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    assert!(
        status.contains("B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md"),
        "{status}"
    );
    assert!(status.contains("B2 known limitations are documented."), "{status}");
}

#[test]
fn b2_limitations_note_links_related_docs() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md")
        .expect("B2 dry-run assembly limitations note");

    for link in [
        "BOOT_READINESS_STATUS.md",
        "BOOT_READINESS_RACE_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_PLAN.md",
        "B1_READ_ONLY_DETECTION_PLAN.md",
        "B1_READ_ONLY_DETECTION_LIMITATIONS.md",
        "X86_64_BOOT_SUPPORT_ROADMAP.md",
    ] {
        assert!(doc.contains(link), "missing related doc link {link}: {doc}");
    }
}

#[test]
fn b2_limitations_note_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md")
        .expect("B2 dry-run assembly limitations note");

    for text in [
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "VM-validated",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(doc.contains(text), "missing non-claim {text}: {doc}");
    }
}
