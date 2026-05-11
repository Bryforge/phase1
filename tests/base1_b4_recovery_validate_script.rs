use std::fs;
use std::path::Path;
use std::process::Command;

fn read_script() -> String {
    fs::read_to_string("scripts/base1-b4-recovery-validate.sh")
        .expect("read B4 recovery validation script")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn base1_b4_recovery_validate_script_exists_and_has_valid_shell_syntax() {
    let script = read_script();
    assert_contains(&script, "Base1 B4 recovery validation scaffold");
    assert_contains(&script, "without launching kernels");
    assert_contains(&script, "writing bootloaders");
    assert_contains(&script, "formatting disks");
    assert_contains(&script, "modifying host boot settings");
    assert_contains(&script, "validating hardware");

    let status = Command::new("sh")
        .arg("-n")
        .arg("scripts/base1-b4-recovery-validate.sh")
        .status()
        .expect("run sh -n");
    assert!(status.success(), "script should have valid shell syntax");
}

#[test]
fn base1_b4_recovery_validate_prepare_writes_report() {
    let out_dir = "build/base1-b4-recovery-validation-test";
    let report_path = Path::new(out_dir).join("b4-recovery-validation.env");
    let _ = fs::remove_dir_all(out_dir);

    let output = Command::new("sh")
        .arg("scripts/base1-b4-recovery-validate.sh")
        .arg("--prepare")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .arg("--out")
        .arg(out_dir)
        .arg("--write-report")
        .output()
        .expect("run prepare");

    assert!(output.status.success(), "prepare should pass");
    assert!(report_path.exists(), "report should be written");
    let report = fs::read_to_string(report_path).expect("read report");
    assert_contains(&report, "BASE1_B4_RECOVERY_MODE=prepare");
    assert_contains(&report, "BASE1_B4_RECOVERY_PROFILE=x200-supervisor-lite");
    assert_contains(&report, "BASE1_B4_RECOVERY_BOOT_ARTIFACT=planned");
    assert_contains(&report, "BASE1_B4_RECOVERY_ROLLBACK_PATH=planned");
    assert_contains(&report, "BASE1_B4_RECOVERY_EMERGENCY_STOP=planned");
    assert_contains(&report, "BASE1_B4_RECOVERY_CLAIM=not_claimed");
}

#[test]
fn base1_b4_recovery_validate_rejects_unknown_profile_and_non_build_out() {
    let bad_profile = Command::new("sh")
        .arg("scripts/base1-b4-recovery-validate.sh")
        .arg("--profile")
        .arg("unknown")
        .output()
        .expect("run bad profile");
    assert!(!bad_profile.status.success(), "unknown profile should fail");

    let bad_out = Command::new("sh")
        .arg("scripts/base1-b4-recovery-validate.sh")
        .arg("--out")
        .arg("/tmp/base1-b4")
        .output()
        .expect("run bad out");
    assert!(!bad_out.status.success(), "non-build out should fail");
}
