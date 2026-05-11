use std::process::Command;

const SCRIPT: &str = "scripts/base1-b3-kernel-handoff.sh";

#[test]
fn base1_b3_kernel_handoff_script_exists_and_has_valid_shell_syntax() {
    let metadata = std::fs::metadata(SCRIPT).expect("B3 kernel handoff script exists");
    assert!(metadata.len() > 0, "B3 kernel handoff script should not be empty");

    let output = Command::new("sh")
        .arg("-n")
        .arg(SCRIPT)
        .output()
        .expect("run sh -n on B3 kernel handoff script");

    assert!(
        output.status.success(),
        "script syntax should pass sh -n\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn base1_b3_kernel_handoff_help_documents_scope_inputs_and_outputs() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--help")
        .output()
        .expect("run B3 kernel handoff help");

    assert!(
        output.status.success(),
        "--help should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    for text in [
        "base1 B3 kernel/initrd handoff",
        "--kernel <path>",
        "--initrd <path>",
        "--prepare",
        "--dry-run",
        "--check",
        "--timeout <seconds>",
        "--expect <text>",
        "A kernel and initrd that are already safe to run in QEMU",
        "does not build or download them",
        "<out>/staging/boot/vmlinuz",
        "<out>/staging/boot/initrd.img",
        "<out>/reports/qemu-boot.log when --check is used",
    ] {
        assert!(stdout.contains(text), "missing help text {text}: {stdout}");
    }
}

#[test]
fn base1_b3_kernel_handoff_requires_inputs_and_rejects_unknown_arguments() {
    let no_args = Command::new("sh")
        .arg(SCRIPT)
        .output()
        .expect("run B3 kernel handoff without args");
    assert!(!no_args.status.success(), "script should require kernel/initrd");
    let no_args_stderr = String::from_utf8_lossy(&no_args.stderr);
    assert!(
        no_args_stderr.contains("--kernel is required for B3 kernel handoff"),
        "stderr was: {no_args_stderr}"
    );

    let unknown = Command::new("sh")
        .arg(SCRIPT)
        .arg("--unknown")
        .output()
        .expect("run B3 kernel handoff with unknown option");
    assert!(!unknown.status.success(), "script should reject unknown args");
    let unknown_stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(unknown_stderr.contains("unknown option: --unknown"), "stderr was: {unknown_stderr}");
}

#[test]
fn base1_b3_kernel_handoff_uses_existing_emulator_stack_and_guarded_check() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 kernel handoff source");

    for text in [
        "scripts/base1-emulator-preview.sh",
        "scripts/base1-qemu-boot-check.sh",
        "--out \"$OUT_DIR\"",
        "--kernel \"$KERNEL\"",
        "--initrd \"$INITRD\"",
        "--bundle \"$OUT_DIR\"",
        "--dry-run",
        "--execute",
        "--confirm launch-qemu-base1-preview",
        "--expect \"$EXPECT\"",
        "build/base1-b3-kernel-handoff",
        "staging/boot/vmlinuz + staging/boot/initrd.img",
    ] {
        assert!(script.contains(text), "missing handoff orchestration text {text}: {script}");
    }
}

#[test]
fn base1_b3_kernel_handoff_preserves_build_directory_and_non_claims() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 kernel handoff source");

    for text in [
        "require_build_out_dir",
        "output directory must be under build/",
        "emulator-only B3 handoff evidence",
        "does not install Base1",
        "validate physical hardware",
        "validate recovery",
        "validate an installer",
        "prove hardening",
        "prove daily-driver readiness",
        "non_claims: emulator-only; no installer; no hardware validation; no daily-driver claim",
    ] {
        assert!(script.contains(text), "missing boundary text {text}: {script}");
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
            "B3 kernel handoff should not contain forbidden host mutation/network pattern {forbidden}: {script}"
        );
    }
}
