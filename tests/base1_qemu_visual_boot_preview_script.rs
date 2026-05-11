use std::process::Command;

const SCRIPT: &str = "scripts/base1-qemu-visual-boot-preview.sh";

#[test]
fn qemu_visual_boot_preview_script_exists_and_has_valid_shell_syntax() {
    let metadata = std::fs::metadata(SCRIPT).expect("QEMU visual boot preview script exists");
    assert!(metadata.len() > 0, "QEMU visual boot preview script should not be empty");

    let output = Command::new("sh")
        .arg("-n")
        .arg(SCRIPT)
        .output()
        .expect("run sh -n on QEMU visual boot preview script");

    assert!(
        output.status.success(),
        "script syntax should pass sh -n\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn qemu_visual_boot_preview_script_help_documents_usage_and_boundaries() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--help")
        .output()
        .expect("run QEMU visual boot preview help");

    assert!(
        output.status.success(),
        "--help should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    for text in [
        "usage: sh scripts/base1-qemu-visual-boot-preview.sh --build [--run] [--fullscreen]",
        "Builds a local UEFI FAT image",
        "assets/phase1-splash.png",
        "visual boot preview only",
        "--build",
        "--run",
        "--fullscreen",
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(stdout.contains(text), "missing help text {text}: {stdout}");
    }
}

#[test]
fn qemu_visual_boot_preview_script_requires_action() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .output()
        .expect("run QEMU visual boot preview without action");

    assert!(!output.status.success(), "script should require an action");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("choose --build, --run, or both"), "stderr was: {stderr}");
}

#[test]
fn qemu_visual_boot_preview_script_rejects_unknown_argument() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--unknown")
        .output()
        .expect("run QEMU visual boot preview with unknown argument");

    assert!(!output.status.success(), "script should reject unknown arguments");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown argument: --unknown"), "stderr was: {stderr}");
}

#[test]
fn qemu_visual_boot_preview_script_uses_expected_local_artifacts() {
    let script = std::fs::read_to_string(SCRIPT).expect("QEMU visual boot preview source");

    for text in [
        "build/base1-qemu-visual-boot-preview",
        "build/base1-qemu-visual-boot-preview.img",
        "assets/phase1-splash.png",
        "EFI/BOOT/BOOTX64.EFI",
        "boot/grub/phase1-splash.png",
        "boot/grub/fonts/phase1.pf2",
        "x86_64-elf-grub-mkstandalone",
        "mformat",
        "mcopy",
        "edk2-x86_64-code.fd",
        "qemu-system-x86_64",
    ] {
        assert!(script.contains(text), "missing expected artifact/tool text {text}: {script}");
    }
}

#[test]
fn qemu_visual_boot_preview_script_uses_safe_qemu_launch_shape() {
    let script = std::fs::read_to_string(SCRIPT).expect("QEMU visual boot preview source");

    for text in [
        "-machine q35,accel=tcg",
        "-m 4096",
        "-smp 4",
        "if=pflash,format=raw,unit=0,readonly=on",
        "-drive if=none,id=phase1usb,format=raw,file=\"$IMG\"",
        "-device qemu-xhci",
        "-device usb-storage,drive=phase1usb,bootindex=1",
        "-boot menu=off",
        "-vga std",
        "-display cocoa,zoom-to-fit=on",
        "-net none",
    ] {
        assert!(script.contains(text), "missing expected QEMU launch text {text}: {script}");
    }
}

#[test]
fn qemu_visual_boot_preview_script_preserves_non_install_boundary() {
    let script = std::fs::read_to_string(SCRIPT).expect("QEMU visual boot preview source");

    for required in [
        "showcase-only boot splash path",
        "does not install Base1",
        "modify host boot settings",
        "partition disks",
        "boot_readiness_claim: no",
        "writes: build-directory-only",
        "no boot-readiness claim",
    ] {
        assert!(script.contains(required), "missing boundary text {required}: {script}");
    }

    for forbidden in [
        "sudo ",
        "grub-install",
        "efibootmgr",
        "diskutil eraseDisk",
        "diskutil partitionDisk",
        "parted ",
        "sfdisk",
        "fdisk ",
        "mkfs",
        "mount -o remount,rw",
        "curl ",
        "wget ",
    ] {
        assert!(
            !script.contains(forbidden),
            "preview script should not contain forbidden host mutation/network pattern {forbidden}: {script}"
        );
    }
}
