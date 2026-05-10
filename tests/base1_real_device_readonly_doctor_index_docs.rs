use std::fs;

#[test]
fn real_device_index_links_readonly_doctor() {
    let index = fs::read_to_string("docs/base1/real-device/README.md").unwrap_or_default();
    assert!(index.contains("base1-real-device-readonly-doctor.sh"));
}

#[test]
fn real_device_status_summary_lists_readonly_doctor() {
    let summary =
        fs::read_to_string("docs/base1/real-device/STATUS_SUMMARY.md").unwrap_or_default();
    assert!(summary.contains("Real-device read-only doctor"));
}

#[test]
fn readonly_doctor_script_still_exists_and_is_guarded() {
    let script = fs::read_to_string("scripts/base1-real-device-readonly-doctor.sh").unwrap();
    assert!(script.contains("Base1 real-device read-only doctor"));
    assert!(script.contains("--dry-run is required"));
    assert!(script.contains("writes: disabled"));
    assert!(script.contains("installer: disabled"));
}
