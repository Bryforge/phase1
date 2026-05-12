#[test]
fn x86_64_boot_support_roadmap_defines_scope_and_safety_boundary() {
    let roadmap = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");

    assert!(
        roadmap.contains("Base1 x86_64 boot support roadmap"),
        "{roadmap}"
    );
    assert!(
        roadmap
            .contains("automatic x86_64 support, boot parameter discovery, firmware mode handling"),
        "{roadmap}"
    );
    assert!(
        roadmap.contains("Current boot readiness is tracked in [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)."),
        "{roadmap}"
    );
    assert!(
        roadmap.contains("dry-run-only, read-only/non-mutating, and evidence-bound"),
        "{roadmap}"
    );
}

#[test]
fn x86_64_boot_support_roadmap_defines_target_systems_and_boot_modes() {
    let roadmap = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");

    for text in [
        "generic UEFI x86_64 laptops and desktops",
        "legacy BIOS x86_64 systems where supported",
        "ThinkPad X200-class Libreboot/GRUB systems",
        "virtual machines used for validation",
        "removable recovery media boot paths",
        "UEFI",
        "Legacy BIOS",
        "Libreboot/GRUB",
        "Virtual machine",
        "Recovery USB",
    ] {
        assert!(
            roadmap.contains(text),
            "missing target/boot-mode text {text}: {roadmap}"
        );
    }
}

#[test]
fn x86_64_boot_support_roadmap_lists_detection_goals() {
    let roadmap = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");

    for text in [
        "CPU architecture: `x86_64`",
        "firmware mode: UEFI, BIOS, or Libreboot/GRUB path",
        "boot loader: GRUB, systemd-boot, EFI stub, or unknown",
        "kernel command-line source",
        "root device strategy",
        "initramfs availability",
        "storage layout",
        "display/input availability",
        "network availability",
        "recovery media availability",
        "virtualization status",
        "secure boot status where applicable",
        "TPM presence where applicable",
        "Unknown or unsupported states should fail closed",
    ] {
        assert!(
            roadmap.contains(text),
            "missing detection goal {text}: {roadmap}"
        );
    }
}

#[test]
fn x86_64_boot_support_roadmap_preserves_boot_parameter_inventory() {
    let roadmap = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");

    for text in [
        "Root filesystem",
        "`root=`",
        "Init process",
        "`init=`",
        "Initramfs",
        "Console",
        "`console=`",
        "Graphics",
        "`nomodeset`",
        "Storage",
        "Network",
        "Recovery",
        "Security posture",
        "Debugging",
    ] {
        assert!(
            roadmap.contains(text),
            "missing boot parameter inventory text {text}: {roadmap}"
        );
    }
}

#[test]
fn x86_64_boot_support_roadmap_defines_boot_profiles() {
    let roadmap = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");

    for profile in [
        "x86_64-uefi-generic",
        "x86_64-bios-generic",
        "x86_64-libreboot-grub",
        "x86_64-vm-validation",
        "x86_64-recovery-usb",
    ] {
        assert!(
            roadmap.contains(profile),
            "missing boot profile {profile}: {roadmap}"
        );
    }
}

#[test]
fn x86_64_boot_support_roadmap_documents_b1_script_and_tests() {
    let roadmap = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");

    for text in [
        "sh scripts/base1-x86_64-detect.sh --dry-run",
        "cargo test -p phase1 --test base1_x86_64_detect_script",
        "B1_READ_ONLY_DETECTION_PLAN.md",
        "B1_READ_ONLY_DETECTION_LIMITATIONS.md",
        "B1_READ_ONLY_DETECTION_VALIDATION.md",
        "Initial script: `scripts/base1-x86_64-detect.sh`.",
        "Initial tests: `tests/base1_x86_64_detect_script.rs`.",
        "architecture hints;",
        "firmware hints;",
        "boot-loader hints;",
        "virtualization hints;",
        "storage-layout hints;",
        "recovery availability hints;",
        "`writes: no`.",
    ] {
        assert!(
            roadmap.contains(text),
            "missing B1 script/test text {text}: {roadmap}"
        );
    }
}

#[test]
fn x86_64_boot_support_roadmap_documents_b2_script_and_tests() {
    let roadmap = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");

    for text in [
        "sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation",
        "cargo test -p phase1 --test b2_dry_run_assembly_plan_docs",
        "cargo test -p phase1 --test base1_b2_assembly_dry_run_script",
        "cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs",
        "cargo test -p phase1 --test b2_dry_run_assembly_validation_docs",
        "B2_DRY_RUN_ASSEMBLY_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md",
        "B2_DRY_RUN_ASSEMBLY_VALIDATION.md",
        "selected profile;",
        "B1 detection summary;",
        "image-builder preview;",
        "boot handoff preview;",
        "installer preview;",
        "recovery preview;",
        "rollback preview;",
        "validation bundle preview;",
    ] {
        assert!(
            roadmap.contains(text),
            "missing B2 script/test text {text}: {roadmap}"
        );
    }
}

#[test]
fn x86_64_boot_support_roadmap_preserves_hardening_and_safety_rules() {
    let roadmap = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");

    for text in [
        "Hardening is a valid goal for this track.",
        "read-only base image",
        "writable user/data layer separation",
        "signed or verifiable boot metadata",
        "measured boot planning where hardware supports it",
        "No silent boot-loader mutation.",
        "No silent partition mutation.",
        "No automatic destructive install path.",
        "No unsupported hardening claims.",
        "No hardware support claim without validation report.",
        "No secure boot, measured boot, TPM, or lockdown claim without evidence.",
    ] {
        assert!(
            roadmap.contains(text),
            "missing hardening/safety text {text}: {roadmap}"
        );
    }
}

#[test]
fn x86_64_boot_support_roadmap_links_required_docs() {
    let roadmap = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");

    for link in [
        "ROADMAP.md",
        "BOOT_READINESS_RACE_PLAN.md",
        "BOOT_READINESS_STATUS.md",
        "B1_READ_ONLY_DETECTION_PLAN.md",
        "B1_READ_ONLY_DETECTION_LIMITATIONS.md",
        "B1_READ_ONLY_DETECTION_VALIDATION.md",
        "B2_DRY_RUN_ASSEMBLY_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md",
        "B2_DRY_RUN_ASSEMBLY_VALIDATION.md",
        "BASE1_IMAGE_BUILDER.md",
        "INSTALLER_RECOVERY.md",
        "BASE1_DRY_RUN_COMMANDS.md",
        "../../base1/HARDWARE_TARGETS.md",
        "../../base1/LIBREBOOT_PROFILE.md",
        "../security/TRUST_MODEL.md",
    ] {
        assert!(
            roadmap.contains(link),
            "missing related doc link {link}: {roadmap}"
        );
    }
}

#[test]
fn x86_64_boot_support_roadmap_preserves_non_claims() {
    let roadmap = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");

    for text in [
        "does not make Base1 bootable on all x86_64 systems",
        "does not claim hardened boot",
        "secure boot support",
        "measured boot support",
        "installer readiness",
        "recovery completion",
        "VM validation",
        "hardware validation",
        "daily-driver readiness",
    ] {
        assert!(
            roadmap.contains(text),
            "missing non-claim text {text}: {roadmap}"
        );
    }
}
