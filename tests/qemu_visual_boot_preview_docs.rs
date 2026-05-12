#[test]
fn qemu_visual_boot_preview_doc_defines_scope_and_purpose() {
    let doc = std::fs::read_to_string("docs/os/QEMU_VISUAL_BOOT_PREVIEW.md")
        .expect("QEMU visual boot preview doc");

    assert!(doc.contains("Base1 QEMU visual boot preview"), "{doc}");
    assert!(
        doc.contains("local QEMU visual boot splash preview using `assets/phase1-splash.png`"),
        "{doc}"
    );
    assert!(
        doc.contains("demos, screenshots, and presentation recordings"),
        "{doc}"
    );
}

#[test]
fn qemu_visual_boot_preview_doc_lists_commands() {
    let doc = std::fs::read_to_string("docs/os/QEMU_VISUAL_BOOT_PREVIEW.md")
        .expect("QEMU visual boot preview doc");

    for command in [
        "sh scripts/base1-qemu-visual-boot-preview.sh --build",
        "sh scripts/base1-qemu-visual-boot-preview.sh --build --run",
        "sh scripts/base1-qemu-visual-boot-preview.sh --run --fullscreen",
    ] {
        assert!(doc.contains(command), "missing command {command}: {doc}");
    }
}

#[test]
fn qemu_visual_boot_preview_doc_lists_artifacts_and_assets() {
    let doc = std::fs::read_to_string("docs/os/QEMU_VISUAL_BOOT_PREVIEW.md")
        .expect("QEMU visual boot preview doc");

    for text in [
        "build/base1-qemu-visual-boot-preview.img",
        "assets/phase1-splash.png",
        "/EFI/BOOT/BOOTX64.EFI",
        "/boot/grub/phase1-splash.png",
        "/boot/grub/fonts/phase1.pf2",
        "The image is a local showcase artifact.",
        "It is not a release image and should not be committed.",
    ] {
        assert!(
            doc.contains(text),
            "missing artifact/asset text {text}: {doc}"
        );
    }
}

#[test]
fn qemu_visual_boot_preview_doc_lists_required_tools() {
    let doc = std::fs::read_to_string("docs/os/QEMU_VISUAL_BOOT_PREVIEW.md")
        .expect("QEMU visual boot preview doc");

    for text in [
        "brew install qemu xorriso mtools x86_64-elf-grub",
        "x86_64-elf-grub-mkstandalone",
        "mformat",
        "mmd",
        "mcopy",
        "qemu-system-x86_64",
        "QEMU UEFI firmware from Homebrew",
    ] {
        assert!(doc.contains(text), "missing tool text {text}: {doc}");
    }
}

#[test]
fn qemu_visual_boot_preview_doc_preserves_inspection_commands() {
    let doc = std::fs::read_to_string("docs/os/QEMU_VISUAL_BOOT_PREVIEW.md")
        .expect("QEMU visual boot preview doc");

    for command in [
        "mdir -i build/base1-qemu-visual-boot-preview.img ::/EFI/BOOT",
        "mdir -i build/base1-qemu-visual-boot-preview.img ::/boot/grub",
        "mdir -i build/base1-qemu-visual-boot-preview.img ::/boot/grub/fonts",
    ] {
        assert!(
            doc.contains(command),
            "missing inspection command {command}: {doc}"
        );
    }
}

#[test]
fn qemu_visual_boot_preview_doc_preserves_troubleshooting() {
    let doc = std::fs::read_to_string("docs/os/QEMU_VISUAL_BOOT_PREVIEW.md")
        .expect("QEMU visual boot preview doc");

    for text in [
        "No boot device",
        "Black screen",
        "Garbled text",
        "Backslash mistakes in manual commands",
        "Do not type a space after the backslash.",
    ] {
        assert!(
            doc.contains(text),
            "missing troubleshooting text {text}: {doc}"
        );
    }
}

#[test]
fn qemu_visual_boot_preview_doc_preserves_safety_boundary() {
    let doc = std::fs::read_to_string("docs/os/QEMU_VISUAL_BOOT_PREVIEW.md")
        .expect("QEMU visual boot preview doc");

    for text in [
        "install Base1",
        "write host boot settings",
        "partition disks",
        "format host disks",
        "write EFI variables",
        "call network tools",
        "claim boot readiness",
        "claim VM validation",
        "claim hardware validation",
        "claim hardening",
        "The only intended writes are local files under `build/`.",
    ] {
        assert!(
            doc.contains(text),
            "missing safety-boundary text {text}: {doc}"
        );
    }
}

#[test]
fn qemu_visual_boot_preview_doc_links_related_docs() {
    let doc = std::fs::read_to_string("docs/os/QEMU_VISUAL_BOOT_PREVIEW.md")
        .expect("QEMU visual boot preview doc");

    for link in [
        "BOOT_READINESS_STATUS.md",
        "B2_DRY_RUN_ASSEMBLY_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md",
        "B3_VM_BOOT_VALIDATION_PLAN.md",
    ] {
        assert!(doc.contains(link), "missing related doc link {link}: {doc}");
    }
}

#[test]
fn qemu_visual_boot_preview_doc_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/os/QEMU_VISUAL_BOOT_PREVIEW.md")
        .expect("QEMU visual boot preview doc");

    for text in [
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "VM-validated",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
        "local visual showcase helper only",
    ] {
        assert!(doc.contains(text), "missing non-claim {text}: {doc}");
    }
}
