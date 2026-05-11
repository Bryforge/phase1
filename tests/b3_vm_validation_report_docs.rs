#[test]
fn b3_vm_validation_report_defines_scope_and_summary() {
    let report = std::fs::read_to_string("docs/os/B3_VM_VALIDATION_REPORT.md")
        .expect("B3 VM validation report");

    for text in [
        "Base1 B3 VM validation report",
        "initial local-evidence report scaffold",
        "B3 emulator evidence collected from local proof/stage runs",
        "BASE1_B3_EVIDENCE_STATE=evidence-present",
        "BASE1_B3_VALIDATION_CLAIM=not_claimed",
        "does not prove Base1 is fully bootable",
    ] {
        assert!(report.contains(text), "missing report scope/summary text {text}: {report}");
    }
}

#[test]
fn b3_vm_validation_report_records_evidence_items() {
    let report = std::fs::read_to_string("docs/os/B3_VM_VALIDATION_REPORT.md")
        .expect("B3 VM validation report");

    for text in [
        "B3 UEFI proof",
        "build/base1-b3-uefi-proof/reports/b3-summary.env",
        "B3 GNU/Linux stage",
        "build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env",
        "B3 OpenBSD stage",
        "build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env",
        "B3 kernel/initrd handoff",
        "build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env",
        "Hardened-profile request and Linux kernel-start evidence only.",
        "Launch-check evidence only until serial marker routing is tuned.",
    ] {
        assert!(report.contains(text), "missing evidence item text {text}: {report}");
    }
}

#[test]
fn b3_vm_validation_report_lists_commands() {
    let report = std::fs::read_to_string("docs/os/B3_VM_VALIDATION_REPORT.md")
        .expect("B3 VM validation report");

    for text in [
        "sh scripts/base1-b3-uefi-proof.sh --build --check",
        "sh scripts/base1-b3-gnulinux-stage.sh",
        "--kernel build/linux/alpine-netboot/vmlinuz",
        "--initrd build/linux/alpine-netboot/initrd.img",
        "sh scripts/base1-b3-openbsd-stage.sh",
        "--img build/openbsd/7.8/miniroot78.img",
        "--check-mode launch",
        "sh scripts/base1-b3-vm-validate.sh",
        "--profile x86_64-vm-validation",
        "--write-report",
    ] {
        assert!(report.contains(text), "missing command text {text}: {report}");
    }
}

#[test]
fn b3_vm_validation_report_preserves_interpretation_boundaries() {
    let report = std::fs::read_to_string("docs/os/B3_VM_VALIDATION_REPORT.md")
        .expect("B3 VM validation report");

    for text in [
        "Linux version",
        "Linux kernel started",
        "does not prove a complete GNU/Linux userspace boot",
        "hardened profile is request-only",
        "marker",
        "launch",
        "requires the expected serial marker",
        "does not prove OpenBSD booted to installer or userland",
        "inside the local evidence boundary",
    ] {
        assert!(report.contains(text), "missing interpretation boundary text {text}: {report}");
    }
}

#[test]
fn b3_vm_validation_report_lists_remaining_requirements_and_non_claims() {
    let report = std::fs::read_to_string("docs/os/B3_VM_VALIDATION_REPORT.md")
        .expect("B3 VM validation report");

    for text in [
        "B2 focused test suite pass record",
        "known-good local kernel/initrd handoff check",
        "OpenBSD serial marker routing or a documented limitation",
        "reviewed B3 log bundle",
        "explicit VM profile",
        "explicit VM runtime",
        "explicit boot artifact identifiers",
        "Phase1 launch result",
        "non-claims preserved",
        "does not make Base1 bootable on physical hardware",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(report.contains(text), "missing remaining requirement/non-claim text {text}: {report}");
    }
}
