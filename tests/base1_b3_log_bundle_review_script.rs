use std::process::Command;

#[test]
fn base1_b3_log_bundle_review_script_exists_and_has_valid_shell_syntax() {
    let script = "scripts/base1-b3-log-bundle-review.sh";
    let contents = std::fs::read_to_string(script).expect("B3 log bundle review script");

    assert!(contents.contains("Base1 B3 log bundle review scaffold"), "{contents}");
    assert!(contents.contains("set -eu"), "{contents}");
    assert!(contents.contains("require_build_out_dir"), "{contents}");
    assert!(contents.contains("build/*"), "{contents}");

    let status = Command::new("sh")
        .arg("-n")
        .arg(script)
        .status()
        .expect("run sh -n on B3 log bundle review script");

    assert!(status.success(), "B3 log bundle review script should have valid shell syntax");
}

#[test]
fn base1_b3_log_bundle_review_help_documents_scope_paths_outputs_and_non_claims() {
    let output = Command::new("sh")
        .arg("scripts/base1-b3-log-bundle-review.sh")
        .arg("--help")
        .output()
        .expect("run B3 log bundle review help");

    assert!(output.status.success(), "help should exit successfully");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "base1 B3 log bundle review",
        "--dry-run",
        "--review",
        "--write-report",
        "build/base1-b2-test-suite/b2-test-suite-summary.env",
        "build/base1-b3-uefi-proof/reports/b3-summary.env",
        "build/base1-b3-uefi-proof/reports/b3-serial.log",
        "build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env",
        "build/base1-b3-kernel-handoff/reports/qemu-boot.log",
        "build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env",
        "build/base1-b3-gnulinux-stage/reports/qemu-boot.log",
        "build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env",
        "build/base1-b3-openbsd-stage/reports/openbsd-qemu-boot.log",
        "docs/os/B3_OPENBSD_SERIAL_LIMITATION.md",
        "<out>/b3-log-bundle-review.env",
        "does not make Base1",
        "bootable",
        "installer-ready",
        "hardware-validated",
        "daily-driver ready",
    ] {
        assert!(stdout.contains(text), "missing help text {text}: {stdout}");
    }
}

#[test]
fn base1_b3_log_bundle_review_dry_run_prints_paths_and_can_write_report() {
    let out_dir = "build/test-base1-b3-log-bundle-review";
    let _ = std::fs::remove_dir_all(out_dir);

    let output = Command::new("sh")
        .arg("scripts/base1-b3-log-bundle-review.sh")
        .arg("--dry-run")
        .arg("--write-report")
        .arg("--out")
        .arg(out_dir)
        .output()
        .expect("run B3 log bundle review dry-run");

    assert!(output.status.success(), "dry-run should pass");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "BASE1 B3 LOG BUNDLE REVIEW",
        "mode  : dry-run",
        "b2_summary:",
        "uefi_summary:",
        "handoff_summary:",
        "gnulinux_summary:",
        "openbsd_summary:",
        "openbsd_limitation:",
        "written_report:",
        "result: dry-run",
        "no B3 claim",
    ] {
        assert!(stdout.contains(text), "missing dry-run stdout text {text}: {stdout}");
    }

    let report = std::fs::read_to_string(format!("{out_dir}/b3-log-bundle-review.env"))
        .expect("dry-run review report");

    for text in [
        "BASE1_B3_LOG_REVIEW_MODE=dry-run",
        "BASE1_B3_LOG_REVIEW_RESULT=not_run",
        "BASE1_B3_LOG_REVIEW_CLAIM=not_claimed",
        "BASE1_B3_LOG_REVIEW_B2_SUMMARY=build/base1-b2-test-suite/b2-test-suite-summary.env",
        "BASE1_B3_LOG_REVIEW_OPENBSD_LIMITATION=docs/os/B3_OPENBSD_SERIAL_LIMITATION.md",
        "BASE1_B3_NON_CLAIM_BOOTABLE_PHYSICAL=1",
        "BASE1_B3_NON_CLAIM_INSTALLER=1",
        "BASE1_B3_NON_CLAIM_RECOVERY=1",
        "BASE1_B3_NON_CLAIM_HARDENED=1",
        "BASE1_B3_NON_CLAIM_HARDWARE=1",
        "BASE1_B3_NON_CLAIM_RELEASE_CANDIDATE=1",
        "BASE1_B3_NON_CLAIM_DAILY_DRIVER=1",
    ] {
        assert!(report.contains(text), "missing report text {text}: {report}");
    }
}

#[test]
fn base1_b3_log_bundle_review_rejects_unknown_args_and_non_build_output() {
    let bad_arg = Command::new("sh")
        .arg("scripts/base1-b3-log-bundle-review.sh")
        .arg("--wat")
        .output()
        .expect("run B3 log bundle review bad arg");
    assert!(!bad_arg.status.success(), "unknown args should fail");

    let bad_out = Command::new("sh")
        .arg("scripts/base1-b3-log-bundle-review.sh")
        .arg("--dry-run")
        .arg("--out")
        .arg("/tmp/base1-b3-log-review")
        .output()
        .expect("run B3 log bundle review bad out");
    assert!(!bad_out.status.success(), "non-build out should fail");
}

#[test]
fn base1_b3_log_bundle_review_preserves_boundaries_and_expected_markers() {
    let contents = std::fs::read_to_string("scripts/base1-b3-log-bundle-review.sh")
        .expect("B3 log bundle review script");

    for text in [
        "does not launch",
        "install Base1",
        "mutate disks",
        "modify host boot settings",
        "hardening",
        "validate hardware",
        "claim daily-driver readiness",
        "BASE1_B3_LOG_REVIEW_CLAIM=not_claimed",
        "BASE1_B3_NON_CLAIM_BOOTABLE_PHYSICAL=1",
        "BASE1_B3_NON_CLAIM_INSTALLER=1",
        "BASE1_B3_NON_CLAIM_RECOVERY=1",
        "BASE1_B3_NON_CLAIM_HARDENED=1",
        "BASE1_B3_NON_CLAIM_HARDWARE=1",
        "BASE1_B3_NON_CLAIM_RELEASE_CANDIDATE=1",
        "BASE1_B3_NON_CLAIM_DAILY_DRIVER=1",
        "result: pass",
        "BASE1_B3_LOG_REVIEW_RESULT=$result",
    ] {
        assert!(contents.contains(text), "missing boundary/marker text {text}: {contents}");
    }
}
