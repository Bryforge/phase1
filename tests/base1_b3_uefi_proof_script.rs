use std::process::Command;

const SCRIPT: &str = "scripts/base1-b3-uefi-proof.sh";

#[test]
fn base1_b3_uefi_proof_script_exists_and_has_valid_shell_syntax() {
    let metadata = std::fs::metadata(SCRIPT).expect("B3 UEFI proof script exists");
    assert!(metadata.len() > 0, "B3 UEFI proof script should not be empty");

    let output = Command::new("sh")
        .arg("-n")
        .arg(SCRIPT)
        .output()
        .expect("run sh -n on B3 UEFI proof script");

    assert!(
        output.status.success(),
        "script syntax should pass sh -n\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn base1_b3_uefi_proof_help_documents_usage_marker_display_and_boundaries() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--help")
        .output()
        .expect("run B3 UEFI proof help");

    assert!(
        output.status.success(),
        "--help should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    for text in [
        "usage: sh scripts/base1-b3-uefi-proof.sh --build [--run|--check] [--fullscreen]",
        "Builds a local B3 UEFI proof image under build/",
        "displays the fitted Phase1 word-mark splash",
        "emits a serial proof marker",
        "Visible QEMU runs keep the screen splash-only",
        "Proof text is routed to serial",
        "--build",
        "--run",
        "--check",
        "--fullscreen",
        "--timeout N",
        "phase1 6.0.0 ready",
        "QEMU/OVMF proof-of-life only",
        "does not make Base1 installer-ready",
        "hardware-validated",
        "recovery-complete",
        "hardened",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(stdout.contains(text), "missing help text {text}: {stdout}");
    }
}

#[test]
fn base1_b3_uefi_proof_requires_action_and_rejects_unknown_arguments() {
    let no_action = Command::new("sh")
        .arg(SCRIPT)
        .output()
        .expect("run B3 UEFI proof without action");
    assert!(!no_action.status.success(), "script should require an action");
    let no_action_stderr = String::from_utf8_lossy(&no_action.stderr);
    assert!(
        no_action_stderr.contains("choose --build, --run, --check, or a combination"),
        "stderr was: {no_action_stderr}"
    );

    let unknown = Command::new("sh")
        .arg(SCRIPT)
        .arg("--unknown")
        .output()
        .expect("run B3 UEFI proof with unknown argument");
    assert!(!unknown.status.success(), "script should reject unknown arguments");
    let unknown_stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        unknown_stderr.contains("unknown argument: --unknown"),
        "stderr was: {unknown_stderr}"
    );
}

#[test]
fn base1_b3_uefi_proof_uses_expected_artifacts_and_splash_fitting() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 UEFI proof source");

    for text in [
        "build/base1-b3-uefi-proof",
        "build/base1-b3-uefi-proof.img",
        "assets/phase1_word.png",
        "phase1-qemu-splash-fit.png",
        "phase1-qemu-splash.png",
        "SPLASH_WIDTH=1024",
        "SPLASH_HEIGHT=768",
        "SPLASH_MAX_EDGE=560",
        "sips -Z \"$SPLASH_MAX_EDGE\"",
        "--padToHeightWidth \"$SPLASH_HEIGHT\" \"$SPLASH_WIDTH\"",
        "--padColor 000000",
        ">/dev/null 2>&1",
        "EFI/BOOT/BOOTX64.EFI",
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
fn base1_b3_uefi_proof_routes_marker_to_serial_without_dirtying_gfxterm() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 UEFI proof source");

    for text in [
        "MARKER=${BASE1_B3_MARKER:-phase1 6.0.0 ready}",
        "serial --unit=0 --speed=115200",
        "terminal_output gfxterm",
        "terminal_output serial",
        "echo \"$MARKER\"",
        "terminal_output gfxterm",
        "display: splash-only; proof text routed to serial",
        "b3-serial.log",
        "b3-summary.env",
        "grep -F \"$MARKER\" \"$SERIAL_LOG\"",
        "BASE1_B3_UEFI_PROOF_RESULT=$result",
        "BASE1_B3_UEFI_PROOF_SERIAL_LOG=reports/b3-serial.log",
    ] {
        assert!(script.contains(text), "missing serial proof text {text}: {script}");
    }

    assert!(
        !script.contains("terminal_output gfxterm serial"),
        "visible output must not mirror proof text onto gfxterm and serial: {script}"
    );
}

#[test]
fn base1_b3_uefi_proof_preserves_non_claims_and_safe_launch_shape() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 UEFI proof source");

    for required in [
        "emulator-only evidence",
        "no installer",
        "no hardware validation",
        "no daily-driver claim",
        "BASE1_B3_NON_CLAIM_INSTALLER=1",
        "BASE1_B3_NON_CLAIM_HARDWARE=1",
        "BASE1_B3_NON_CLAIM_DAILY_DRIVER=1",
        "-machine q35,accel=tcg",
        "-drive if=pflash,format=raw,unit=0,readonly=on,file=\"$ovmf\"",
        "-drive if=none,id=phase1usb,format=raw,file=\"$IMG\"",
        "-device usb-storage,drive=phase1usb,bootindex=1",
        "-net none",
    ] {
        assert!(script.contains(required), "missing required safety text {required}: {script}");
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
            "B3 UEFI proof script should not contain forbidden host mutation/network pattern {forbidden}: {script}"
        );
    }
}
