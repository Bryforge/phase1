#[test]
fn progress_checkpoint_disclosure_defines_plain_language_state() {
    let doc = std::fs::read_to_string("docs/os/PROGRESS_CHECKPOINT_DISCLOSURE.md")
        .expect("progress checkpoint disclosure");

    for text in [
        "Phase1/Base1 progress checkpoint disclosure",
        "checkpoint disclosure",
        "current repository state, local evidence state, claim boundaries, and next blockers",
        "not a finished operating system",
        "installer",
        "recovery system",
        "hardened release",
        "hardware-validated system",
        "release candidate",
        "daily-driver system",
        "evidence-bound boot-readiness scaffolding",
    ] {
        assert!(doc.contains(text), "missing plain-language state text {text}: {doc}");
    }
}

#[test]
fn progress_checkpoint_disclosure_records_current_readiness_and_evidence() {
    let doc = std::fs::read_to_string("docs/os/PROGRESS_CHECKPOINT_DISCLOSURE.md")
        .expect("progress checkpoint disclosure");

    for text in [
        "Current level: B2 local dry-run validation evidence present",
        "Target next level: B3 VM boot validated",
        "B3 claim: not_claimed",
        "B2 focused suite",
        "build/base1-b2-test-suite/b2-test-suite-summary.env",
        "B3 UEFI proof",
        "build/base1-b3-uefi-proof/reports/b3-summary.env",
        "B3 kernel/initrd handoff",
        "build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env",
        "B3 GNU/Linux stage",
        "build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env",
        "B3 OpenBSD stage",
        "build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env",
        "OpenBSD serial marker",
        "docs/os/B3_OPENBSD_SERIAL_LIMITATION.md",
    ] {
        assert!(doc.contains(text), "missing readiness/evidence text {text}: {doc}");
    }
}

#[test]
fn progress_checkpoint_disclosure_lists_evidence_commands() {
    let doc = std::fs::read_to_string("docs/os/PROGRESS_CHECKPOINT_DISCLOSURE.md")
        .expect("progress checkpoint disclosure");

    for text in [
        "sh scripts/base1-b2-test-suite-check.sh --check --write-report",
        "sh scripts/base1-b3-kernel-handoff.sh",
        "--kernel build/linux/alpine-netboot/vmlinuz",
        "--initrd build/linux/alpine-netboot/initrd.img",
        "--boot-profile hardened",
        "--expect \"Linux version\"",
        "sh scripts/base1-b3-gnulinux-stage.sh",
        "sh scripts/base1-b3-openbsd-stage.sh",
        "--img build/openbsd/7.8/miniroot78.img",
        "--check-mode launch",
        "sh scripts/base1-b3-vm-validate.sh",
        "--profile x86_64-vm-validation",
        "--write-report",
    ] {
        assert!(doc.contains(text), "missing evidence command text {text}: {doc}");
    }
}

#[test]
fn progress_checkpoint_disclosure_distinguishes_proven_from_not_proven() {
    let doc = std::fs::read_to_string("docs/os/PROGRESS_CHECKPOINT_DISCLOSURE.md")
        .expect("progress checkpoint disclosure");

    for text in [
        "What is proven",
        "B1 detector exists",
        "focused B2 test suite can pass locally",
        "B3 UEFI proof path can produce local proof-of-life evidence",
        "stage a local kernel/initrd pair and detect `Linux version`",
        "B3 OpenBSD stage can launch a local OpenBSD boot artifact",
        "What is not proven",
        "Base1 boots as a complete operating system",
        "Phase1 launches inside a complete Base1 boot path",
        "GNU/Linux reaches a complete userspace boot",
        "OpenBSD reaches installer or userland",
        "hardened mode is actually enforced or verified",
        "recovery works",
        "an installer works",
        "physical hardware works",
    ] {
        assert!(doc.contains(text), "missing proven/not-proven text {text}: {doc}");
    }
}

#[test]
fn progress_checkpoint_disclosure_preserves_limitations_and_remaining_blockers() {
    let doc = std::fs::read_to_string("docs/os/PROGRESS_CHECKPOINT_DISCLOSURE.md")
        .expect("progress checkpoint disclosure");

    for text in [
        "GNU/Linux stage limitation",
        "Linux kernel-start output, not a complete userspace boot",
        "OpenBSD stage limitation",
        "serial-marker mode has not yet captured",
        "Hardening limitation",
        "request-only evidence",
        "B3 validation limitation",
        "still a scaffold",
        "B3 validation report reviewed against captured logs",
        "VM profile explicitly recorded",
        "VM runtime explicitly recorded",
        "boot artifacts explicitly identified",
        "Phase1 launch result recorded",
        "non-claims preserved",
    ] {
        assert!(doc.contains(text), "missing limitation/blocker text {text}: {doc}");
    }
}

#[test]
fn progress_checkpoint_disclosure_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/os/PROGRESS_CHECKPOINT_DISCLOSURE.md")
        .expect("progress checkpoint disclosure");

    for text in [
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
        "local B1/B2/B3 progress, evidence, limitations, and remaining blockers",
    ] {
        assert!(doc.contains(text), "missing non-claim text {text}: {doc}");
    }
}
