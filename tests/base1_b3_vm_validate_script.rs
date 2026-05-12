use std::process::Command;

const SCRIPT: &str = "scripts/base1-b3-vm-validate.sh";

#[test]
fn base1_b3_vm_validate_script_exists_and_has_valid_shell_syntax() {
    let metadata = std::fs::metadata(SCRIPT).expect("B3 VM validate script exists");
    assert!(
        metadata.len() > 0,
        "B3 VM validate script should not be empty"
    );

    let script = std::fs::read_to_string(SCRIPT).expect("B3 VM validate source");
    for text in [
        "profiles/base1/*.env",
        "PROFILE_FILE=\"$PROFILE_DIR/$PROFILE.env\"",
        ". \"$PROFILE_FILE\"",
        "BASE1_PROFILE_ALLOWED_DELIVERY_MODES",
        "BASE1_PROFILE_NON_CLAIM_HYPERVISOR",
    ] {
        assert!(
            script.contains(text),
            "missing profile loading text {text}: {script}"
        );
    }

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
fn base1_b3_vm_validate_help_documents_dry_run_scope_profiles_and_non_claims() {
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
        "--profile-dir <dir>",
        "--write-report",
        "--uefi-dir <dir>",
        "--handoff-dir <dir>",
        "--gnulinux-dir <dir>",
        "--openbsd-dir <dir>",
        "profile source:",
        "profiles/base1/x86_64-vm-validation.env",
        "profiles/base1/x200-supervisor-lite.env",
        "profiles/base1/workstation-supervisor.env",
        "scaffold-only",
        "evidence-present",
        "evidence-incomplete",
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hypervisor-ready",
        "hardware-validated",
        "daily-driver ready",
    ] {
        assert!(stdout.contains(text), "missing help text {text}: {stdout}");
    }
}

#[test]
fn base1_b3_vm_validate_rejects_unknown_profile_unknown_args_and_non_build_paths() {
    let unknown = Command::new("sh")
        .arg(SCRIPT)
        .arg("--unknown")
        .output()
        .expect("run B3 VM validate with unknown arg");
    assert!(
        !unknown.status.success(),
        "script should reject unknown args"
    );
    let unknown_stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        unknown_stderr.contains("unknown option: --unknown"),
        "stderr was: {unknown_stderr}"
    );

    let bad_profile = Command::new("sh")
        .arg(SCRIPT)
        .arg("--dry-run")
        .arg("--profile")
        .arg("physical-machine")
        .output()
        .expect("run B3 VM validate with unsupported profile");
    assert!(
        !bad_profile.status.success(),
        "script should reject missing profile file"
    );
    let bad_profile_stderr = String::from_utf8_lossy(&bad_profile.stderr);
    assert!(
        bad_profile_stderr.contains("profile file not found: profiles/base1/physical-machine.env"),
        "stderr was: {bad_profile_stderr}"
    );

    let bad_report = Command::new("sh")
        .arg(SCRIPT)
        .arg("--dry-run")
        .arg("--report")
        .arg("/tmp/b3-validation-scaffold.env")
        .output()
        .expect("run B3 VM validate with bad report path");
    assert!(
        !bad_report.status.success(),
        "non-build report path should fail"
    );
}

#[test]
fn base1_b3_vm_validate_uses_expected_evidence_paths_profile_fields_and_report_fields() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 VM validate source");

    for text in [
        "build/base1-b3-vm-validation/b3-validation-scaffold.env",
        "build/base1-b3-uefi-proof",
        "build/base1-b3-kernel-handoff",
        "build/base1-b3-gnulinux-stage",
        "build/base1-b3-openbsd-stage",
        "reports/b3-summary.env",
        "reports/b3-serial.log",
        "reports/qemu-boot-summary.env",
        "reports/qemu-boot.log",
        "reports/openbsd-qemu-summary.env",
        "reports/openbsd-qemu-boot.log",
        "BASE1_B3_VM_VALIDATION_MODE=scaffold-only",
        "BASE1_B3_VM_VALIDATION_PROFILE=$PROFILE",
        "BASE1_B3_VM_VALIDATION_PROFILE_FILE=$PROFILE_FILE",
        "BASE1_B3_VM_VALIDATION_PROFILE_CLASS=${BASE1_PROFILE_CLASS:-}",
        "BASE1_B3_VM_VALIDATION_PROFILE_TARGET_RAM_MB=${BASE1_PROFILE_TARGET_RAM_MB:-}",
        "BASE1_B3_VM_VALIDATION_PROFILE_ALLOWED_MODES=${BASE1_PROFILE_ALLOWED_DELIVERY_MODES:-}",
        "BASE1_B3_VM_VALIDATION_PROFILE_VM_MEMORY_MB=${BASE1_PROFILE_VM_MEMORY_MB:-}",
        "BASE1_B3_VM_VALIDATION_PROFILE_STORAGE_TIER_POLICY=${BASE1_PROFILE_STORAGE_TIER_POLICY:-}",
        "BASE1_B3_EVIDENCE_STATE=$evidence_state",
        "BASE1_B3_EVIDENCE_SUMMARY_COUNT=$present_count",
        "BASE1_B3_UEFI_SUMMARY_PRESENT=",
        "BASE1_B3_HANDOFF_SUMMARY_PRESENT=",
        "BASE1_B3_GNULINUX_SUMMARY_PRESENT=",
        "BASE1_B3_OPENBSD_SUMMARY_PRESENT=",
        "BASE1_B3_VALIDATION_CLAIM=not_claimed",
    ] {
        assert!(
            script.contains(text),
            "missing evidence/report text {text}: {script}"
        );
    }
}

#[test]
fn base1_b3_vm_validate_writes_report_with_profile_contract_fields() {
    let out_dir = "build/test-base1-b3-vm-validation";
    let report_path = format!("{out_dir}/b3-validation-scaffold.env");
    let _ = std::fs::remove_dir_all(out_dir);

    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--dry-run")
        .arg("--profile")
        .arg("x86_64-vm-validation")
        .arg("--report")
        .arg(&report_path)
        .arg("--write-report")
        .output()
        .expect("run B3 VM validate write report");

    assert!(output.status.success(), "write report should pass");
    let stdout = String::from_utf8_lossy(&output.stdout);
    for text in [
        "profile_file: profiles/base1/x86_64-vm-validation.env",
        "profile_cls : vm-validation",
        "written_report:",
        "non_claims: no installer; no recovery validation; no hardening; no hypervisor claim; no hardware validation; no daily-driver claim",
    ] {
        assert!(stdout.contains(text), "missing stdout text {text}: {stdout}");
    }

    let report = std::fs::read_to_string(&report_path).expect("B3 VM validation report");
    for text in [
        "BASE1_B3_VM_VALIDATION_MODE=scaffold-only",
        "BASE1_B3_VM_VALIDATION_PROFILE=x86_64-vm-validation",
        "BASE1_B3_VM_VALIDATION_PROFILE_FILE=profiles/base1/x86_64-vm-validation.env",
        "BASE1_B3_VM_VALIDATION_PROFILE_CLASS=vm-validation",
        "BASE1_B3_VM_VALIDATION_PROFILE_TARGET_RAM_MB=4096",
        "BASE1_B3_VM_VALIDATION_PROFILE_DEFAULT_MODE=supervisor-lite",
        "BASE1_B3_VM_VALIDATION_PROFILE_ALLOWED_MODES=direct-first,supervisor-lite,supervisor-concurrent",
        "BASE1_B3_VM_VALIDATION_PROFILE_MAX_CONCURRENCY=3",
        "BASE1_B3_VM_VALIDATION_PROFILE_VM_MEMORY_MB=1024",
        "BASE1_B3_VM_VALIDATION_PROFILE_OPENBSD_MEMORY_MB=1024",
        "BASE1_B3_VM_VALIDATION_PROFILE_STORAGE_TIER_POLICY=build-directory-scratch",
        "BASE1_B3_VALIDATION_CLAIM=not_claimed",
        "BASE1_B3_NON_CLAIM_BOOTABLE_PHYSICAL=1",
        "BASE1_B3_NON_CLAIM_INSTALLER=1",
        "BASE1_B3_NON_CLAIM_RECOVERY=1",
        "BASE1_B3_NON_CLAIM_HARDENED=1",
        "BASE1_B3_NON_CLAIM_HYPERVISOR=1",
        "BASE1_B3_NON_CLAIM_HARDWARE=1",
        "BASE1_B3_NON_CLAIM_RELEASE_CANDIDATE=1",
        "BASE1_B3_NON_CLAIM_DAILY_DRIVER=1",
    ] {
        assert!(report.contains(text), "missing report text {text}: {report}");
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
        "OpenBSD evidence dir must be under build/",
        "only --dry-run mode is currently supported",
        "does not launch QEMU",
        "install Base1",
        "mutate disks",
        "fetch kernels",
        "validate hardware",
        "validate recovery",
        "prove hardening",
        "BASE1_B3_NON_CLAIM_BOOTABLE_PHYSICAL=1",
        "BASE1_B3_NON_CLAIM_INSTALLER=1",
        "BASE1_B3_NON_CLAIM_RECOVERY=1",
        "BASE1_B3_NON_CLAIM_HARDENED=1",
        "BASE1_B3_NON_CLAIM_HYPERVISOR=1",
        "BASE1_B3_NON_CLAIM_HARDWARE=1",
        "BASE1_B3_NON_CLAIM_RELEASE_CANDIDATE=1",
        "BASE1_B3_NON_CLAIM_DAILY_DRIVER=1",
    ] {
        assert!(
            script.contains(text),
            "missing boundary/non-claim text {text}: {script}"
        );
    }
}
