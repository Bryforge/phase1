use std::process::Command;

const SCRIPT: &str = "scripts/base1-b3-vm-validate.sh";

#[test]
fn base1_b3_vm_validate_script_exists_and_has_valid_shell_syntax() {
    let metadata = std::fs::metadata(SCRIPT).expect("B3 VM validate script exists");
    assert!(metadata.len() > 0, "B3 VM validate script should not be empty");

    let output = Command::new("sh")
        .arg("-n")
        .arg(SCRIPT)
        .output()
        .expect("run sh -n on B3 VM validate script");

    assert!(
        output.status.success(),
        "script syntax should pass sh -n\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn base1_b3_vm_validate_help_documents_dry_run_scope_and_non_claims() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--help")
        .output()
        .expect("run B3 VM validate help");

    assert!(output.status.success(), "--help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);

    for text in [
        "base1 B3 VM validation scaffold",
        "--dry-run",
        "--profile <profile>",
        "--write-report",
        "--uefi-dir <dir>",
        "--handoff-dir <dir>",
        "--gnulinux-dir <dir>",
        "scaffold-only",
        "evidence-present",
        "evidence-incomplete",
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardware-validated",
        "daily-driver ready",
    ] {
        assert!(stdout.contains(text), "missing help text {text}: {stdout}");
    }
}

#[test]
fn base1_b3_vm_validate_rejects_unknown_profile_and_unknown_args() {
    let unknown = Command::new("sh")
        .arg(SCRIPT)
        .arg("--unknown")
        .output()
        .expect("run B3 VM validate with unknown arg");
    assert!(!unknown.status.success(), "script should reject unknown args");
    let unknown_stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(unknown_stderr.contains("unknown option: --unknown"), "stderr was: {unknown_stderr}");

    let bad_profile = Command::new("sh")
        .arg(SCRIPT)
        .arg("--dry-run")
        .arg("--profile")
        .arg("physical-machine")
        .output()
        .expect("run B3 VM validate with unsupported profile");
    assert!(!bad_profile.status.success(), "script should reject unsupported profiles");
    let bad_profile_stderr = String::from_utf8_lossy(&bad_profile.stderr);
    assert!(
        bad_profile_stderr.contains("unsupported B3 profile: physical-machine"),
        "stderr was: {bad_profile_stderr}"
    );
}

#[test]
fn base1_b3_vm_validate_uses_expected_evidence_paths_and_report_fields() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 VM validate source");

    for text in [
        "build/base1-b3-vm-validation/b3-validation-scaffold.env",
        "build/base1-b3-uefi-proof",
        "build/base1-b3-kernel-handoff",
        "build/base1-b3-gnulinux-stage",
        "reports/b3-summary.env",
        "reports/b3-serial.log",
        "reports/qemu-boot-summary.env",
        "reports/qemu-boot.log",
        "BASE1_B3_VM_VALIDATION_MODE=scaffold-only",
        "BASE1_B3_EVIDENCE_STATE=$evidence_state",
        "BASE1_B3_UEFI_SUMMARY_PRESENT=",
        "BASE1_B3_HANDOFF_SUMMARY_PRESENT=",
        "BASE1_B3_GNULINUX_SUMMARY_PRESENT=",
        "BASE1_B3_VALIDATION_CLAIM=not_claimed",
    ] {
        assert!(script.contains(text), "missing evidence/report text {text}: {script}");
    }
}

#[test]
fn base1_b3_vm_validate_preserves_build_paths_and_non_claims() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 VM validate source");

    for text in [
        "require_build_path",
        "report path must be under build/",
        "UEFI evidence dir must be under build/",
        "handoff evidence dir must be under build/",
        "GNU/Linux evidence dir must be under build/",
        "only --dry-run mode is currently supported",
        "does not launch QEMU",
        "install Base1",
        "mutate disks",
        "fetch kernels",
        "validate hardware",
        "validate recovery",
        "BASE1_B3_NON_CLAIM_BOOTABLE_PHYSICAL=1",
        "BASE1_B3_NON_CLAIM_INSTALLER=1",
        "BASE1_B3_NON_CLAIM_RECOVERY=1",
        "BASE1_B3_NON_CLAIM_HARDENED=1",
        "BASE1_B3_NON_CLAIM_HARDWARE=1",
        "BASE1_B3_NON_CLAIM_RELEASE_CANDIDATE=1",
        "BASE1_B3_NON_CLAIM_DAILY_DRIVER=1",
    ] {
        assert!(script.contains(text), "missing boundary/non-claim text {text}: {script}");
    }
}
