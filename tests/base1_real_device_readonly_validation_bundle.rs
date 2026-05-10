use std::fs;
use std::process::Command;

const SCRIPT: &str = "scripts/base1-real-device-readonly-validation-bundle.sh";

#[test]
fn real_device_readonly_validation_bundle_exists() {
    let script = fs::read_to_string(SCRIPT).expect("validation bundle script exists");

    assert!(script.contains("--dry-run"));
    assert!(script.contains("--target"));
    assert!(script.contains("base1-real-device-readonly-preview.sh"));
    assert!(script.contains("base1-real-device-readonly-report.sh"));
    assert!(script.contains("READONLY_VALIDATION_PLAN.md"));
    assert!(script.contains("READONLY_REPORT_TEMPLATE.md"));
}

#[test]
fn real_device_readonly_validation_bundle_requires_dry_run() {
    let output = Command::new("bash")
        .arg(SCRIPT)
        .arg("--target")
        .arg("/dev/disk-test")
        .output()
        .expect("script runs");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--dry-run is required"));
}

#[test]
fn real_device_readonly_validation_bundle_requires_dev_target() {
    let output = Command::new("bash")
        .arg(SCRIPT)
        .arg("--dry-run")
        .arg("--target")
        .arg("disk-test")
        .output()
        .expect("script runs");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("target must be under /dev/"));
}

#[test]
fn real_device_readonly_validation_bundle_runs_non_mutating_previews() {
    let output = Command::new("bash")
        .arg(SCRIPT)
        .arg("--dry-run")
        .arg("--target")
        .arg("/dev/disk-test")
        .output()
        .expect("script runs");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Base1 real-device read-only validation bundle"));
    assert!(stdout.contains("status: dry-run only"));
    assert!(stdout.contains("target: /dev/disk-test"));
    assert!(stdout.contains("writes: disabled"));
    assert!(stdout.contains("mutation: disabled"));
    assert!(stdout.contains("installer: disabled"));
    assert!(stdout.contains("hardware-validation-claim: false"));
    assert!(stdout.contains("daily-driver-claim: false"));
    assert!(stdout.contains("READONLY_VALIDATION_PLAN.md: present"));
    assert!(stdout.contains("READONLY_REPORT_TEMPLATE.md: present"));
    assert!(stdout.contains("running read-only doctor:"));
    assert!(stdout.contains("Base1 real-device read-only doctor"));
    assert!(stdout.contains("scope: documentation, scripts, and local tool availability only"));
    assert!(stdout.contains("Base1 real-device read-only preview"));
    assert!(stdout.contains("Base1 Real-Device Read-Only Validation Report"));
    assert!(stdout.contains("not installer-ready"));
    assert!(stdout.contains("not hardware-validated"));
    assert!(stdout.contains("not daily-driver ready"));
    assert!(stdout.contains("no destructive disk writes"));
    assert!(stdout.contains("no real-device write path"));
}
