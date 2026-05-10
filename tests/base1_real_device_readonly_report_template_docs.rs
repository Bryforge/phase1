use std::fs;

#[test]
fn real_device_readonly_report_template_exists() {
    let doc = fs::read_to_string("docs/base1/real-device/READONLY_REPORT_TEMPLATE.md")
        .expect("real-device read-only report template exists");

    assert!(doc.contains("Base1 Real-Device Read-Only Validation Report Template"));
    assert!(doc.contains("Status: template only"));
    assert!(doc.contains("Scope: read-only real-device observation"));
}

#[test]
fn real_device_readonly_report_template_requires_identity() {
    let doc = fs::read_to_string("docs/base1/real-device/READONLY_REPORT_TEMPLATE.md").unwrap();

    assert!(doc.contains("Required Target Identity"));
    assert!(doc.contains("Device path:"));
    assert!(doc.contains("Model:"));
    assert!(doc.contains("Serial or redacted identifier:"));
    assert!(doc.contains("Transport:"));
}

#[test]
fn real_device_readonly_report_template_preserves_guardrails() {
    let doc = fs::read_to_string("docs/base1/real-device/READONLY_REPORT_TEMPLATE.md").unwrap();

    assert!(doc.contains("No disk writes"));
    assert!(doc.contains("No partitioning"));
    assert!(doc.contains("No formatting"));
    assert!(doc.contains("No installer execution"));
    assert!(doc.contains("No firmware flashing"));
    assert!(doc.contains("No automatic target selection"));
}

#[test]
fn real_device_readonly_report_template_preserves_non_claims() {
    let doc = fs::read_to_string("docs/base1/real-device/READONLY_REPORT_TEMPLATE.md").unwrap();

    assert!(doc.contains("Not installer-ready"));
    assert!(doc.contains("Not hardware-validated"));
    assert!(doc.contains("Not daily-driver ready"));
    assert!(doc.contains("No destructive disk writes"));
    assert!(doc.contains("No real-device write path"));
}

#[test]
fn base1_index_links_real_device_readonly_report_template() {
    let index = fs::read_to_string("docs/base1/README.md").unwrap_or_default();
    assert!(index.contains("READONLY_REPORT_TEMPLATE.md"));
}
