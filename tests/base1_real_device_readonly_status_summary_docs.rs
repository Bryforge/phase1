use std::fs;

#[test]
fn real_device_readonly_status_summary_exists() {
    let doc = fs::read_to_string("docs/base1/real-device/STATUS_SUMMARY.md").unwrap();
    assert!(doc.contains("Base1 Real-Device Read-Only Validation Status Summary"));
    assert!(doc.contains("read-only validation workflow assembled"));
}

#[test]
fn real_device_readonly_status_summary_records_completed_chain() {
    let doc = fs::read_to_string("docs/base1/real-device/STATUS_SUMMARY.md").unwrap();
    assert!(doc.contains("Real Phase1 initrd builder evidence"));
    assert!(doc.contains("QEMU boot evidence"));
    assert!(doc.contains("Real-device read-only validation plan"));
    assert!(doc.contains("Real-device read-only validation bundle"));
    assert!(doc.contains("Real-device read-only evidence capture report instance"));
}

#[test]
fn real_device_readonly_status_summary_lists_primary_command() {
    let doc = fs::read_to_string("docs/base1/real-device/STATUS_SUMMARY.md").unwrap();
    assert!(doc.contains(
        "base1-real-device-readonly-validation-bundle.sh --dry-run --target /dev/YOUR_TARGET"
    ));
}

#[test]
fn real_device_readonly_status_summary_preserves_current_claim_and_non_claims() {
    let doc = fs::read_to_string("docs/base1/real-device/STATUS_SUMMARY.md").unwrap();
    assert!(doc.contains("safe read-only real-device validation workflow prepared"));
    assert!(doc.contains("Not installer-ready"));
    assert!(doc.contains("Not hardware-validated"));
    assert!(doc.contains("Not daily-driver ready"));
    assert!(doc.contains("No destructive disk writes"));
    assert!(doc.contains("No real-device write path"));
}

#[test]
fn real_device_index_links_status_summary() {
    let index = fs::read_to_string("docs/base1/real-device/README.md").unwrap_or_default();
    assert!(index.contains("STATUS_SUMMARY.md"));
}
