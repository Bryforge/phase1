use std::process::Command;

const SCRIPT: &str = "scripts/base1-b3-gnulinux-stage.sh";

#[test]
fn base1_b3_gnulinux_stage_script_exists_and_has_valid_shell_syntax() {
    let metadata = std::fs::metadata(SCRIPT).expect("B3 GNU/Linux stage script exists");
    assert!(metadata.len() > 0, "B3 GNU/Linux stage script should not be empty");

    let output = Command::new("sh")
        .arg("-n")
        .arg(SCRIPT)
        .output()
        .expect("run sh -n on B3 GNU/Linux stage script");

    assert!(
        output.status.success(),
        "script syntax should pass sh -n\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn base1_b3_gnulinux_stage_help_documents_stage_model_and_hardened_profile() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--help")
        .output()
        .expect("run B3 GNU/Linux stage help");

    assert!(output.status.success(), "--help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);

    for text in [
        "base1 B3 GNU/Linux stage",
        "--root <dir>",
        "--boot <dir>",
        "--kernel <path>",
        "--initrd <path>",
        "--prepare",
        "--dry-run",
        "--check",
        "--boot-profile <p>",
        "default: hardened",
        "standard, hardened",
        "--append <text>",
        "uses GNU/Linux as a known boot payload staging point",
        "It does not build,",
        "download, install, or trust a GNU/Linux distribution by itself",
        "local kernel/initrd files",
        "The GNU/Linux stage defaults to the hardened QEMU boot profile",
        "module signature",
        "lockdown",
        "allocator initialization",
        "debugfs disablement",
        "requested boot profile, not proof",
        "emulator-only staging evidence",
        "does not make Base1 a GNU/Linux",
        "validate physical hardware",
        "prove hardening",
        "prove daily-driver readiness",
    ] {
        assert!(stdout.contains(text), "missing help text {text}: {stdout}");
    }
}

#[test]
fn base1_b3_gnulinux_stage_requires_inputs_and_rejects_unknown_arguments() {
    let no_args = Command::new("sh")
        .arg(SCRIPT)
        .output()
        .expect("run B3 GNU/Linux stage without inputs");
    assert!(!no_args.status.success(), "script should require input paths");
    let stderr = String::from_utf8_lossy(&no_args.stderr);
    assert!(
        stderr.contains("provide --kernel and --initrd, or provide --root/--boot for detection"),
        "stderr was: {stderr}"
    );

    let unknown = Command::new("sh")
        .arg(SCRIPT)
        .arg("--unknown")
        .output()
        .expect("run B3 GNU/Linux stage with unknown option");
    assert!(!unknown.status.success(), "script should reject unknown args");
    let unknown_stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(unknown_stderr.contains("unknown option: --unknown"), "stderr was: {unknown_stderr}");
}

#[test]
fn base1_b3_gnulinux_stage_rejects_unknown_boot_profile() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--boot-profile")
        .arg("unsafe")
        .output()
        .expect("run B3 GNU/Linux stage with invalid boot profile");

    assert!(!output.status.success(), "script should reject invalid boot profile");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported boot profile: unsafe"), "stderr was: {stderr}");
}

#[test]
fn base1_b3_gnulinux_stage_detects_common_kernel_and_initrd_names() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 GNU/Linux stage source");

    for text in [
        "detect_kernel",
        "detect_initrd",
        "\"$dir\"/vmlinuz",
        "\"$dir\"/vmlinuz-*",
        "\"$dir\"/bzImage",
        "\"$dir\"/kernel",
        "\"$dir\"/Image",
        "\"$dir\"/initrd.img",
        "\"$dir\"/initrd.img-*",
        "\"$dir\"/initramfs.img",
        "\"$dir\"/initramfs-*",
        "\"$dir\"/initrd",
        "\"$dir\"/initramfs",
        "BOOT_DIR=$ROOT_DIR/boot",
    ] {
        assert!(script.contains(text), "missing detection text {text}: {script}");
    }
}

#[test]
fn base1_b3_gnulinux_stage_delegates_to_kernel_handoff() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 GNU/Linux stage source");

    for text in [
        "scripts/base1-b3-kernel-handoff.sh",
        "--out \"$OUT_DIR\"",
        "--kernel \"$KERNEL\"",
        "--initrd \"$INITRD\"",
        "--prepare",
        "--dry-run",
        "--check",
        "--timeout \"$TIMEOUT_SECONDS\"",
        "--expect \"$EXPECT\"",
        "--boot-profile \"$BOOT_PROFILE\"",
        "--append \"$EXTRA_APPEND\"",
        "BOOT_PROFILE=${BASE1_QEMU_BOOT_PROFILE:-hardened}",
        "valid_boot_profile",
        "build/base1-b3-gnulinux-stage",
        "GNU/Linux local kernel/initrd handoff",
    ] {
        assert!(script.contains(text), "missing handoff delegation text {text}: {script}");
    }
}

#[test]
fn base1_b3_gnulinux_stage_preserves_local_only_non_claims() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 GNU/Linux stage source");

    for text in [
        "require_build_out_dir",
        "output directory must be under build/",
        "does not download a distribution",
        "install Base1",
        "modify host boot settings",
        "partition disks",
        "claim hardware readiness",
        "emulator-only GNU/Linux stage",
        "hardened profile is request-only",
        "no installer",
        "no hardware validation",
        "no hardening proof",
        "no daily-driver claim",
    ] {
        assert!(script.contains(text), "missing non-claim text {text}: {script}");
    }
}
