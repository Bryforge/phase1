#[test]
fn b3_kernel_initrd_handoff_doc_defines_scope_and_purpose() {
    let doc = std::fs::read_to_string("docs/os/B3_KERNEL_INITRD_HANDOFF.md")
        .expect("B3 kernel/initrd handoff doc");

    for text in [
        "Base1 B3 kernel/initrd handoff",
        "implementation scaffold present",
        "local QEMU kernel/initrd handoff",
        "existing emulator preview and boot-check scripts",
        "after the UEFI proof-of-life path",
        "emulator-only evidence",
        "does not build or download a kernel",
    ] {
        assert!(doc.contains(text), "missing scope text {text}: {doc}");
    }
}

#[test]
fn b3_kernel_initrd_handoff_doc_lists_commands() {
    let doc = std::fs::read_to_string("docs/os/B3_KERNEL_INITRD_HANDOFF.md")
        .expect("B3 kernel/initrd handoff doc");

    for text in [
        "sh scripts/base1-b3-kernel-handoff.sh",
        "--kernel /path/to/vmlinuz",
        "--initrd /path/to/initrd.img",
        "--prepare",
        "--dry-run",
        "--check",
        "--expect \"custom marker text\"",
        "phase1 6.0.0 ready",
    ] {
        assert!(doc.contains(text), "missing command text {text}: {doc}");
    }
}

#[test]
fn b3_kernel_initrd_handoff_doc_lists_outputs_and_stack_scripts() {
    let doc = std::fs::read_to_string("docs/os/B3_KERNEL_INITRD_HANDOFF.md")
        .expect("B3 kernel/initrd handoff doc");

    for text in [
        "build/base1-b3-kernel-handoff/",
        "manifest.env",
        "staging/boot/vmlinuz",
        "staging/boot/initrd.img",
        "base1-rootfs-preview.tar",
        "base1-sandbox.raw",
        "run-qemu-bundle.sh",
        "reports/qemu-boot.log",
        "reports/qemu-boot-summary.env",
        "scripts/base1-emulator-preview.sh",
        "scripts/base1-qemu-boot-check.sh",
    ] {
        assert!(doc.contains(text), "missing output/stack text {text}: {doc}");
    }
}

#[test]
fn b3_kernel_initrd_handoff_doc_defines_evidence_model_and_inputs() {
    let doc = std::fs::read_to_string("docs/os/B3_KERNEL_INITRD_HANDOFF.md")
        .expect("B3 kernel/initrd handoff doc");

    for text in [
        "`--prepare` proves only that the local bundle was staged.",
        "`--dry-run` proves that the bundle has enough structure",
        "`--check` runs QEMU with serial capture",
        "expected marker appears in the captured log",
        "A passing `--check` is evidence for a named local emulator run only.",
        "Those files must already be available and safe to run in QEMU.",
        "does not build, fetch, sign, verify, or trust them by itself",
    ] {
        assert!(doc.contains(text), "missing evidence/input text {text}: {doc}");
    }
}

#[test]
fn b3_kernel_initrd_handoff_doc_preserves_troubleshooting_and_non_claims() {
    let doc = std::fs::read_to_string("docs/os/B3_KERNEL_INITRD_HANDOFF.md")
        .expect("B3 kernel/initrd handoff doc");

    for text in [
        "Missing kernel or initrd",
        "Missing timeout on macOS",
        "brew install coreutils",
        "Marker missing",
        "write host boot settings",
        "partition disks",
        "format host disks",
        "write EFI variables",
        "download kernels or initrds",
        "claim hardware validation",
        "claim installer readiness",
        "claim recovery readiness",
        "claim hardening",
        "claim daily-driver readiness",
        "only intended writes are local files under `build/`",
        "does not make Base1 bootable on physical hardware",
        "release-candidate ready",
    ] {
        assert!(doc.contains(text), "missing troubleshooting/non-claim text {text}: {doc}");
    }
}

#[test]
fn b3_kernel_initrd_handoff_doc_links_related_docs() {
    let doc = std::fs::read_to_string("docs/os/B3_KERNEL_INITRD_HANDOFF.md")
        .expect("B3 kernel/initrd handoff doc");

    for link in [
        "B3_VM_BOOT_VALIDATION_PLAN.md",
        "B3_VM_BOOT_LOGS.md",
        "B3_VM_BOOT_VALIDATION_LIMITATIONS.md",
        "QEMU_VISUAL_BOOT_PREVIEW.md",
        "BOOT_READINESS_STATUS.md",
    ] {
        assert!(doc.contains(link), "missing related doc link {link}: {doc}");
    }
}
