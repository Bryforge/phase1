use std::fs;

#[test]
fn real_device_readonly_index_exists() {
    let doc = fs::read_to_string("docs/base1/real-device/README.md").unwrap();
    assert!(doc.contains("Base1 Real-Device Read-Only Validation Index"));
    assert!(doc.contains("read-only planning and evidence workflow index"));
}

#[test]
fn real_device_readonly_index_links_documents() {
    let doc = fs::read_to_string("docs/base1/real-device/README.md").unwrap();
    assert!(doc.contains("READONLY_VALIDATION_PLAN.md"));
    assert!(doc.contains("READONLY_REPORT_TEMPLATE.md"));
    assert!(doc.contains("READONLY_VALIDATION_BUNDLE_REPORT.md"));
}

#[test]
fn real_device_readonly_index_lists_scripts() {
    let doc = fs::read_to_string("docs/base1/real-device/README.md").unwrap();
    assert!(doc.contains("base1-real-device-readonly-preview.sh"));
    assert!(doc.contains("base1-real-device-readonly-report.sh"));
    assert!(doc.contains("base1-real-device-readonly-validation-bundle.sh"));
}

#[test]
fn real_device_readonly_index_preserves_guardrails_and_non_claims() {
    let doc = fs::read_to_string("docs/base1/real-device/README.md").unwrap();
    assert!(doc.contains("Dry-run required"));
    assert!(doc.contains("No disk writes"));
    assert!(doc.contains("No real-device write path"));
    assert!(doc.contains("Not installer-ready"));
    assert!(doc.contains("Not hardware-validated"));
    assert!(doc.contains("Not daily-driver ready"));
}

#[test]
fn base1_index_links_real_device_readonly_index() {
    let index = fs::read_to_string("docs/base1/README.md").unwrap_or_default();
    assert!(index.contains("real-device/README.md"));
}
