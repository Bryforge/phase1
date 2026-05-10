use std::fs;
use std::process::Command;

const SCRIPT: &str = "scripts/base1-real-device-readonly-report.sh";

#[test]
fn real_device_readonly_report_script_exists() {
    let script = fs::read_to_string(SCRIPT).expect("report script exists");

    assert!(script.contains("--dry-run"));
    assert!(script.contains("--target"));
    assert!(script.contains("No disk writes"));
    assert!(script.contains("No partitioning"));
    assert!(script.contains("No formatting"));
    assert!(script.contains("No installer execution"));
    assert!(script.contains("No firmware flashing"));
}

#[test]
fn real_device_readonly_report_requires_dry_run() {
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
fn real_device_readonly_report_requires_dev_target() {
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
fn real_device_readonly_report_prints_non_mutating_report() {
    let output = Command::new("bash")
        .arg(SCRIPT)
        .arg("--dry-run")
        .arg("--target")
        .arg("/dev/disk-test")
        .output()
        .expect("script runs");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Base1 Real-Device Read-Only Validation Report"));
    assert!(stdout.contains("Device path: /dev/disk-test"));
    assert!(stdout.contains("No disk writes"));
    assert!(stdout.contains("No partitioning"));
    assert!(stdout.contains("No formatting"));
    assert!(stdout.contains("No installer execution"));
    assert!(stdout.contains("No firmware flashing"));
    assert!(stdout.contains("Not installer-ready"));
    assert!(stdout.contains("Not hardware-validated"));
    assert!(stdout.contains("Not daily-driver ready"));
    assert!(stdout.contains("No destructive disk writes"));
    assert!(stdout.contains("No real-device write path"));
}
