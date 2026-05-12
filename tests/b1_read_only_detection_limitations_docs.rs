#[test]
fn b1_limitations_note_defines_scope_and_command() {
    let doc = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_LIMITATIONS.md")
        .expect("B1 read-only detection limitations note");

    assert!(
        doc.contains("Base1 B1 read-only detection limitations"),
        "{doc}"
    );
    assert!(
        doc.contains("known limitations for `scripts/base1-x86_64-detect.sh --dry-run`"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-x86_64-detect.sh --dry-run"),
        "{doc}"
    );
    assert!(
        doc.contains("gathers hints only and writes nothing"),
        "{doc}"
    );
}

#[test]
fn b1_limitations_note_lists_what_b1_can_report() {
    let doc = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_LIMITATIONS.md")
        .expect("B1 read-only detection limitations note");

    for text in [
        "machine architecture",
        "UEFI directory presence",
        "EFI variable directory presence",
        "limited GRUB/systemd-boot directory hints",
        "redacted kernel command-line availability",
        "virtualization/container hints when visible",
        "storage layout hints through `lsblk` or `/proc/mounts`",
        "basic recovery/emergency-mode hints",
        "unknown fields and next planned read-only check",
        "`writes: no`",
    ] {
        assert!(doc.contains(text), "missing can-report text {text}: {doc}");
    }
}

#[test]
fn b1_limitations_note_lists_what_b1_cannot_prove() {
    let doc = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_LIMITATIONS.md")
        .expect("B1 read-only detection limitations note");

    for text in [
        "that Base1 is bootable",
        "that the current machine can safely install Base1",
        "that a boot loader is configured correctly",
        "that a kernel command line is complete or correct",
        "that secure boot is supported or enabled correctly",
        "that measured boot, TPM, or lockdown modes are available or safe",
        "that recovery media exists or is usable",
        "that rollback is complete",
        "that hardware is validated",
        "that the system is hardened",
        "that the system is daily-driver ready",
    ] {
        assert!(
            doc.contains(text),
            "missing cannot-prove text {text}: {doc}"
        );
    }
}

#[test]
fn b1_limitations_note_preserves_known_limitations_table() {
    let doc = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_LIMITATIONS.md")
        .expect("B1 read-only detection limitations note");

    for text in [
        "Architecture",
        "Firmware",
        "Boot loader",
        "Kernel command line",
        "Storage",
        "Virtualization",
        "Recovery",
        "Cross-platform hosts",
        "Directory hints do not prove active boot-loader configuration.",
        "Non-Linux hosts may produce more unknown values.",
    ] {
        assert!(
            doc.contains(text),
            "missing known limitation text {text}: {doc}"
        );
    }
}

#[test]
fn b1_limitations_note_preserves_required_non_mutating_behavior() {
    let doc = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_LIMITATIONS.md")
        .expect("B1 read-only detection limitations note");

    for text in [
        "require `--dry-run`",
        "report `writes: no`",
        "avoid network access",
        "avoid boot-loader mutation",
        "avoid partition mutation",
        "avoid package installation",
        "avoid writing to `/boot`, `/etc`, EFI variables, initramfs files, or partitions",
        "keep unknown states visible",
    ] {
        assert!(
            doc.contains(text),
            "missing required behavior text {text}: {doc}"
        );
    }
}

#[test]
fn b1_limitations_note_defines_b1_completion_requirements() {
    let doc = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_LIMITATIONS.md")
        .expect("B1 read-only detection limitations note");

    for text in [
        "script test suite passing in CI or local validation",
        "status tracker links updated after implementation",
        "secret-redaction review",
        "non-mutation source review",
        "limitations documented in this file",
        "README or roadmap visibility for the B1 boundary",
    ] {
        assert!(
            doc.contains(text),
            "missing completion requirement {text}: {doc}"
        );
    }
}

#[test]
fn b1_limitations_note_is_linked_from_status_tracker() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    assert!(
        status.contains("B1_READ_ONLY_DETECTION_LIMITATIONS.md"),
        "{status}"
    );
    assert!(
        status.contains("B1 known limitations are documented."),
        "{status}"
    );
}

#[test]
fn b1_limitations_note_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_LIMITATIONS.md")
        .expect("B1 read-only detection limitations note");

    for text in [
        "does not make Base1 bootable",
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
