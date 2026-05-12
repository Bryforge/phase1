use std::process::Command;

#[test]
fn base1_b2_test_suite_check_script_exists_and_has_valid_shell_syntax() {
    let script = "scripts/base1-b2-test-suite-check.sh";
    let contents = std::fs::read_to_string(script).expect("B2 test suite checker script");

    assert!(
        contents.contains("Base1 B2 focused test-suite checker"),
        "{contents}"
    );
    assert!(contents.contains("set -eu"), "{contents}");
    assert!(contents.contains("require_build_out_dir"), "{contents}");
    assert!(contents.contains("build/*"), "{contents}");

    let status = Command::new("sh")
        .arg("-n")
        .arg(script)
        .status()
        .expect("run sh -n on B2 test suite checker");

    assert!(
        status.success(),
        "B2 test suite checker should have valid shell syntax"
    );
}

#[test]
fn base1_b2_test_suite_check_help_documents_scope_commands_outputs_and_non_claims() {
    let output = Command::new("sh")
        .arg("scripts/base1-b2-test-suite-check.sh")
        .arg("--help")
        .output()
        .expect("run B2 test suite checker help");

    assert!(output.status.success(), "help should exit successfully");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "base1 B2 focused test-suite checker",
        "--dry-run",
        "--check",
        "--write-report",
        "cargo test -p phase1 --test b2_dry_run_assembly_plan_docs",
        "cargo test -p phase1 --test base1_b2_assembly_dry_run_script",
        "cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs",
        "cargo test -p phase1 --test b2_dry_run_assembly_validation_docs",
        "cargo test -p phase1 --test b2_dry_run_assembly_output_review_docs",
        "cargo test -p phase1 --test boot_readiness_status_docs",
        "cargo test -p phase1 --test boot_readiness_race_plan_docs",
        "cargo test -p phase1 --test x86_64_boot_support_roadmap_docs",
        "cargo test -p phase1 --test readme_navigation_reorganization_links",
        "<out>/b2-test-suite-summary.env",
        "<out>/b2-test-suite.log when --check is used",
        "does not make",
        "VM-validated",
        "hardware-validated",
        "daily-driver ready",
    ] {
        assert!(stdout.contains(text), "missing help text {text}: {stdout}");
    }
}

#[test]
fn base1_b2_test_suite_check_dry_run_prints_plan_and_can_write_report() {
    let out_dir = "build/test-base1-b2-test-suite-check";
    let _ = std::fs::remove_dir_all(out_dir);

    let output = Command::new("sh")
        .arg("scripts/base1-b2-test-suite-check.sh")
        .arg("--dry-run")
        .arg("--write-report")
        .arg("--out")
        .arg(out_dir)
        .output()
        .expect("run B2 test suite checker dry-run");

    assert!(output.status.success(), "dry-run should pass");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "BASE1 B2 FOCUSED TEST SUITE",
        "mode  : dry-run",
        "plan: cargo test -p phase1 --test b2_dry_run_assembly_plan_docs",
        "plan: cargo test -p phase1 --test readme_navigation_reorganization_links",
        "written_report:",
        "result: dry-run",
        "no tests executed",
        "no VM/hardware validation claim",
    ] {
        assert!(
            stdout.contains(text),
            "missing dry-run stdout text {text}: {stdout}"
        );
    }

    let summary = std::fs::read_to_string(format!("{out_dir}/b2-test-suite-summary.env"))
        .expect("dry-run summary");

    for text in [
        "BASE1_B2_TEST_SUITE_MODE=dry-run",
        "BASE1_B2_TEST_SUITE_RESULT=not_run",
        "BASE1_B2_TEST_SUITE_CLAIM=not_claimed",
        "BASE1_B2_NON_CLAIM_BOOTABLE=1",
        "BASE1_B2_NON_CLAIM_INSTALLER=1",
        "BASE1_B2_NON_CLAIM_VM_VALIDATED=1",
        "BASE1_B2_NON_CLAIM_HARDWARE=1",
        "BASE1_B2_NON_CLAIM_DAILY_DRIVER=1",
    ] {
        assert!(
            summary.contains(text),
            "missing summary text {text}: {summary}"
        );
    }
}

#[test]
fn base1_b2_test_suite_check_rejects_unknown_args_and_non_build_output() {
    let bad_arg = Command::new("sh")
        .arg("scripts/base1-b2-test-suite-check.sh")
        .arg("--wat")
        .output()
        .expect("run B2 test suite checker bad arg");
    assert!(!bad_arg.status.success(), "unknown args should fail");

    let bad_out = Command::new("sh")
        .arg("scripts/base1-b2-test-suite-check.sh")
        .arg("--dry-run")
        .arg("--out")
        .arg("/tmp/base1-b2-test-suite")
        .output()
        .expect("run B2 test suite checker bad out");
    assert!(!bad_out.status.success(), "non-build out should fail");
}

#[test]
fn base1_b2_test_suite_check_preserves_non_claims_and_local_build_boundary() {
    let contents = std::fs::read_to_string("scripts/base1-b2-test-suite-check.sh")
        .expect("B2 test suite checker script");

    for text in [
        "build an installer",
        "mutate disks",
        "alter host boot settings",
        "claim VM/hardware readiness",
        "output directory must be under build/",
        "BASE1_B2_TEST_SUITE_CLAIM=not_claimed",
        "BASE1_B2_NON_CLAIM_BOOTABLE=1",
        "BASE1_B2_NON_CLAIM_INSTALLER=1",
        "BASE1_B2_NON_CLAIM_VM_VALIDATED=1",
        "BASE1_B2_NON_CLAIM_HARDWARE=1",
        "BASE1_B2_NON_CLAIM_DAILY_DRIVER=1",
    ] {
        assert!(
            contents.contains(text),
            "missing script boundary text {text}: {contents}"
        );
    }
}
