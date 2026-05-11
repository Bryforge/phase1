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
        "prefers GRUB's unicode.pf2 font",
        "generated monospaced",
        "box/glitch characters",
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
fn base1_b3_uefi_proof_uses_unicode_font_and_keeps_menu_overlay() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 UEFI proof source");

    for text in [
        "MARKER=${BASE1_B3_MARKER:-phase1 6.0.0 ready}",
        "serial --unit=0 --speed=115200",
        "GRUB_FONT=\"$WORK_DIR/boot/grub/fonts/phase1.pf2\"",
        "copy_grub_unicode_font",
        "unicode.pf2",
        "Monaco.ttf",
        "Menlo.ttc",
        "Courier New.ttf",
        "if loadfont /boot/grub/fonts/phase1.pf2; then",
        "terminal_output gfxterm serial",
        "menuentry \"Phase1 / Base1 B3 UEFI proof\"",
        "echo \"base1 b3 uefi proof start\"",
        "echo \"$MARKER\"",
        "echo \"emulator-only evidence; no installer; no hardware-validation claim\"",
        "display: readable GRUB overlay with unicode font",
        "b3-serial.log",
        "b3-summary.env",
        "grep -F \"$MARKER\" \"$SERIAL_LOG\"",
        "BASE1_B3_UEFI_PROOF_RESULT=$result",
        "BASE1_B3_UEFI_PROOF_SERIAL_LOG=reports/b3-serial.log",
    ] {
        assert!(script.contains(text), "missing readable menu proof text {text}: {script}");
    }

    let loadfont_pos = script
        .find("if loadfont /boot/grub/fonts/phase1.pf2; then")
        .expect("loadfont must exist");
    let terminal_output_pos = script
        .find("terminal_output gfxterm serial")
        .expect("terminal_output gfxterm serial must exist");
    assert!(
        loadfont_pos < terminal_output_pos,
        "font must be loaded before enabling gfxterm output to avoid box/glitch glyphs: {script}"
    );

    assert!(
        !script.contains("Do not use menuentry here"),
        "script should keep the boot menu/proof entry and fix font rendering instead: {script}"
    );
    assert!(
        !script.contains("display: direct readable overlay; GRUB menu frame disabled"),
        "script should keep the menu/proof entry and fix font rendering instead: {script}"
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
}
