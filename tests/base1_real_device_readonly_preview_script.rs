use std::process::Command;

const SCRIPT: &str = "scripts/base1-real-device-readonly-preview.sh";

#[test]
fn real_device_readonly_preview_script_exists() {
    let script = std::fs::read_to_string(SCRIPT).expect("read-only preview script exists");

    assert!(script.contains("Base1 real-device read-only preview"));
    assert!(script.contains("--dry-run is required"));
    assert!(script.contains("no disk writes"));
    assert!(script.contains("no partitioning"));
    assert!(script.contains("no formatting"));
    assert!(script.contains("no firmware writes"));
    assert!(script.contains("no installer execution"));
}

#[test]
fn real_device_readonly_preview_requires_dry_run() {
    let output = Command::new("bash")
        .args([SCRIPT, "--target", "/dev/disk-test"])
        .output()
        .expect("run preview script");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--dry-run is required"));
}

#[test]
fn real_device_readonly_preview_requires_dev_target() {
    let output = Command::new("bash")
        .args([SCRIPT, "--target", "disk-test", "--dry-run"])
        .output()
        .expect("run preview script");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("target must be a /dev/ path"));
}

#[test]
fn real_device_readonly_preview_reports_non_mutating_contract() {
    let output = Command::new("bash")
        .args([SCRIPT, "--target", "/dev/disk-test", "--dry-run"])
        .output()
        .expect("run preview script");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("status: dry-run only"));
    assert!(stdout.contains("target: /dev/disk-test"));
    assert!(stdout.contains("writes: disabled"));
    assert!(stdout.contains("mutation: disabled"));
    assert!(stdout.contains("installer: disabled"));
    assert!(stdout.contains("partitioning: disabled"));
    assert!(stdout.contains("formatting: disabled"));
    assert!(stdout.contains("firmware-writes: disabled"));
    assert!(stdout.contains("hardware-validation-claim: false"));
    assert!(stdout.contains("daily-driver-claim: false"));
    assert!(stdout.contains("no destructive disk writes"));
    assert!(stdout.contains("no real-device write path"));
}
