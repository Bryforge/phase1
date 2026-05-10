use std::fs;
use std::process::Command;

#[test]
fn real_device_readonly_doctor_script_exists() {
    let script = fs::read_to_string("scripts/base1-real-device-readonly-doctor.sh").unwrap();
    assert!(script.contains("Base1 real-device read-only doctor"));
    assert!(script.contains("--dry-run is required"));
    assert!(script.contains("writes: disabled"));
    assert!(script.contains("mutation: disabled"));
    assert!(script.contains("installer: disabled"));
}

#[test]
fn real_device_readonly_doctor_requires_dry_run() {
    let output = Command::new("bash")
        .arg("scripts/base1-real-device-readonly-doctor.sh")
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--dry-run is required"));
}

#[test]
fn real_device_readonly_doctor_reports_safe_surface() {
    let output = Command::new("bash")
        .args(["scripts/base1-real-device-readonly-doctor.sh", "--dry-run"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Base1 real-device read-only doctor"));
    assert!(stdout.contains("status: dry-run only"));
    assert!(stdout.contains("scope: documentation, scripts, and local tool availability only"));
    assert!(stdout.contains("READONLY_VALIDATION_PLAN.md: present"));
    assert!(stdout.contains("READONLY_REPORT_TEMPLATE.md: present"));
    assert!(stdout.contains("base1-real-device-readonly-preview.sh"));
    assert!(stdout.contains("base1-real-device-readonly-report.sh"));
    assert!(stdout.contains("base1-real-device-readonly-validation-bundle.sh"));
    assert!(stdout.contains("not installer-ready"));
    assert!(stdout.contains("not hardware-validated"));
    assert!(stdout.contains("not daily-driver ready"));
    assert!(stdout.contains("no destructive disk writes"));
    assert!(stdout.contains("no real-device write path"));
}

#[test]
fn real_device_readonly_doctor_avoids_mutating_tools() {
    let script = fs::read_to_string("scripts/base1-real-device-readonly-doctor.sh").unwrap();
    for forbidden in [
        "mkfs",
        "dd if=",
        "diskutil eraseDisk",
        "diskutil partitionDisk",
        "parted",
        "fdisk",
        "sgdisk",
    ] {
        assert!(
            !script.contains(forbidden),
            "found forbidden mutating token: {forbidden}"
        );
    }
}
