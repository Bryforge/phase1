#[test]
fn b3_gnulinux_stage_doc_defines_scope_and_purpose() {
    let doc =
        std::fs::read_to_string("docs/os/B3_GNULINUX_STAGE.md").expect("B3 GNU/Linux stage doc");

    for text in [
        "Base1 B3 GNU/Linux stage",
        "implementation scaffold present",
        "local GNU/Linux kernel/initrd staging point",
        "local GNU/Linux kernel/initrd -> B3 kernel handoff -> guarded QEMU serial check -> future B3 validation report",
        "does not make Base1 a GNU/Linux distribution",
        "staging scaffold only",
    ] {
        assert!(doc.contains(text), "missing scope/purpose text {text}: {doc}");
    }
}

#[test]
fn b3_gnulinux_stage_doc_lists_commands_and_detection() {
    let doc =
        std::fs::read_to_string("docs/os/B3_GNULINUX_STAGE.md").expect("B3 GNU/Linux stage doc");

    for text in [
        "sh scripts/base1-b3-gnulinux-stage.sh",
        "--kernel /path/to/vmlinuz",
        "--initrd /path/to/initrd.img",
        "--root /path/to/linux-root",
        "--boot /path/to/boot",
        "--prepare",
        "--dry-run",
        "--check",
        "--expect \"custom marker text\"",
        "<root>/boot",
        "vmlinuz-*",
        "bzImage",
        "initrd.img-*",
        "initramfs-*",
    ] {
        assert!(
            doc.contains(text),
            "missing command/detection text {text}: {doc}"
        );
    }
}

#[test]
fn b3_gnulinux_stage_doc_lists_outputs_and_stack_relationship() {
    let doc =
        std::fs::read_to_string("docs/os/B3_GNULINUX_STAGE.md").expect("B3 GNU/Linux stage doc");

    for text in [
        "build/base1-b3-gnulinux-stage/",
        "manifest.env",
        "staging/boot/vmlinuz",
        "staging/boot/initrd.img",
        "base1-sandbox.raw",
        "run-qemu-bundle.sh",
        "reports/qemu-boot.log",
        "reports/qemu-boot-summary.env",
        "scripts/base1-b3-kernel-handoff.sh",
        "scripts/base1-emulator-preview.sh",
        "scripts/base1-qemu-boot-check.sh",
    ] {
        assert!(
            doc.contains(text),
            "missing output/stack text {text}: {doc}"
        );
    }
}

#[test]
fn b3_gnulinux_stage_doc_defines_evidence_model_and_safety_boundary() {
    let doc =
        std::fs::read_to_string("docs/os/B3_GNULINUX_STAGE.md").expect("B3 GNU/Linux stage doc");

    for text in [
        "`--prepare` proves only that a local GNU/Linux kernel/initrd pair was staged",
        "`--dry-run` proves that the staged bundle can be handed to the guarded QEMU checker as a plan",
        "`--check` runs the guarded serial-marker check",
        "A passing `--check` is evidence for a named local emulator stage only.",
        "download a GNU/Linux distribution",
        "install Base1",
        "change host boot settings",
        "partition disks",
        "format host disks",
        "write EFI variables",
        "claim physical hardware validation",
        "claim installer readiness",
        "claim recovery readiness",
        "claim hardening",
        "claim daily-driver readiness",
        "only intended writes are local files under `build/`",
    ] {
        assert!(doc.contains(text), "missing evidence/safety text {text}: {doc}");
    }
}

#[test]
fn b3_gnulinux_stage_doc_links_related_docs_and_preserves_non_claims() {
    let doc =
        std::fs::read_to_string("docs/os/B3_GNULINUX_STAGE.md").expect("B3 GNU/Linux stage doc");

    for text in [
        "B3_KERNEL_INITRD_HANDOFF.md",
        "B3_VM_BOOT_VALIDATION_PLAN.md",
        "B3_VM_BOOT_LOGS.md",
        "B3_VM_BOOT_VALIDATION_LIMITATIONS.md",
        "BOOT_READINESS_STATUS.md",
        "does not make Base1 a GNU/Linux distribution",
        "bootable on physical hardware",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(
            doc.contains(text),
            "missing link/non-claim text {text}: {doc}"
        );
    }
}
