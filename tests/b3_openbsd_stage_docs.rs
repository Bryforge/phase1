#[test]
fn b3_openbsd_stage_doc_defines_scope_and_purpose() {
    let doc = std::fs::read_to_string("docs/os/B3_OPENBSD_STAGE.md").expect("B3 OpenBSD stage doc");

    for text in [
        "Base1 B3 OpenBSD stage",
        "implementation scaffold present",
        "local OpenBSD ISO/image staging point",
        "OpenBSD is intentionally separate from the GNU/Linux kernel/initrd handoff path",
        "local OpenBSD ISO/image -> guarded QEMU serial check -> future B3 validation report",
        "staging scaffold only",
    ] {
        assert!(
            doc.contains(text),
            "missing scope/purpose text {text}: {doc}"
        );
    }
}

#[test]
fn b3_openbsd_stage_doc_lists_commands_and_marker() {
    let doc = std::fs::read_to_string("docs/os/B3_OPENBSD_STAGE.md").expect("B3 OpenBSD stage doc");

    for text in [
        "sh scripts/base1-b3-openbsd-stage.sh",
        "--iso /path/to/install.iso",
        "--img /path/to/install.img",
        "--prepare",
        "--dry-run",
        "--check",
        "--expect \"OpenBSD/amd64\"",
        "OpenBSD",
    ] {
        assert!(
            doc.contains(text),
            "missing command/marker text {text}: {doc}"
        );
    }
}

#[test]
fn b3_openbsd_stage_doc_lists_outputs_and_evidence_model() {
    let doc = std::fs::read_to_string("docs/os/B3_OPENBSD_STAGE.md").expect("B3 OpenBSD stage doc");

    for text in [
        "build/base1-b3-openbsd-stage/",
        "openbsd-stage.env",
        "reports/openbsd-qemu-boot.log",
        "reports/openbsd-qemu-summary.env",
        "`--prepare` proves only that a local OpenBSD artifact path was accepted",
        "`--dry-run` proves that the staged artifact can be converted into a guarded QEMU command plan",
        "`--check` runs QEMU with serial capture",
        "expected marker appears in the captured log",
        "A passing `--check` is evidence for a named local emulator stage only.",
    ] {
        assert!(doc.contains(text), "missing output/evidence text {text}: {doc}");
    }
}

#[test]
fn b3_openbsd_stage_doc_preserves_safety_boundary_and_non_claims() {
    let doc = std::fs::read_to_string("docs/os/B3_OPENBSD_STAGE.md").expect("B3 OpenBSD stage doc");

    for text in [
        "download OpenBSD",
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
        "does not make Base1 an OpenBSD distribution",
        "bootable on physical hardware",
        "release-candidate ready",
    ] {
        assert!(
            doc.contains(text),
            "missing safety/non-claim text {text}: {doc}"
        );
    }
}

#[test]
fn b3_openbsd_stage_doc_links_related_docs() {
    let doc = std::fs::read_to_string("docs/os/B3_OPENBSD_STAGE.md").expect("B3 OpenBSD stage doc");

    for link in [
        "B3_VM_BOOT_VALIDATION_PLAN.md",
        "B3_GNULINUX_STAGE.md",
        "B3_KERNEL_INITRD_HANDOFF.md",
        "B3_VM_BOOT_LOGS.md",
        "B3_VM_BOOT_VALIDATION_LIMITATIONS.md",
        "BOOT_READINESS_STATUS.md",
    ] {
        assert!(doc.contains(link), "missing related doc link {link}: {doc}");
    }
}
