use std::fs;

#[test]
fn real_device_readonly_bundle_report_exists() {
    let doc =
        fs::read_to_string("docs/base1/real-device/READONLY_VALIDATION_BUNDLE_REPORT.md").unwrap();

    assert!(doc.contains("Base1 Real-Device Read-Only Validation Bundle Report"));
    assert!(doc.contains("read-only bundle evidence recorded"));
    assert!(doc.contains("Base1 real-device read-only validation bundle"));
}

#[test]
fn real_device_readonly_bundle_report_records_evidence_chain() {
    let doc =
        fs::read_to_string("docs/base1/real-device/READONLY_VALIDATION_BUNDLE_REPORT.md").unwrap();

    assert!(doc.contains("Base1 real-device read-only validation plan"));
    assert!(doc.contains("Base1 real-device read-only preview script"));
    assert!(doc.contains("Base1 real-device read-only report template"));
    assert!(doc.contains("Base1 real-device read-only report generator"));
    assert!(doc.contains("Base1 real-device read-only validation bundle"));
}

#[test]
fn real_device_readonly_bundle_report_preserves_guardrails() {
    let doc =
        fs::read_to_string("docs/base1/real-device/READONLY_VALIDATION_BUNDLE_REPORT.md").unwrap();

    assert!(doc.contains("Requires `--dry-run`"));
    assert!(doc.contains("Requires `--target /dev/<device>`"));
    assert!(doc.contains("Rejects non-`/dev/` targets"));
    assert!(doc.contains("No disk writes"));
    assert!(doc.contains("No destructive repair commands"));
}

#[test]
fn real_device_readonly_bundle_report_preserves_non_claims() {
    let doc =
        fs::read_to_string("docs/base1/real-device/READONLY_VALIDATION_BUNDLE_REPORT.md").unwrap();

    assert!(doc.contains("Not installer-ready"));
    assert!(doc.contains("Not hardware-validated"));
    assert!(doc.contains("Not daily-driver ready"));
    assert!(doc.contains("No destructive disk writes"));
    assert!(doc.contains("No real-device write path"));
}

#[test]
fn base1_index_links_real_device_readonly_bundle_report() {
    let index = fs::read_to_string("docs/base1/README.md").unwrap_or_default();

    assert!(index.contains("READONLY_VALIDATION_BUNDLE_REPORT.md"));
}
