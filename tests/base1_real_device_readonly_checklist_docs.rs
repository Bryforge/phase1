use std::fs;

#[test]
fn real_device_readonly_checklist_exists() {
    let doc = fs::read_to_string("docs/base1/real-device/CHECKLIST.md").unwrap();
    assert!(doc.contains("Base1 Real-Device Read-Only Validation Checklist"));
    assert!(doc.contains("read-only operator checklist"));
}

#[test]
fn real_device_readonly_checklist_lists_required_command() {
    let doc = fs::read_to_string("docs/base1/real-device/CHECKLIST.md").unwrap();
    assert!(doc.contains(
        "base1-real-device-readonly-validation-bundle.sh --dry-run --target /dev/YOUR_TARGET"
    ));
}

#[test]
fn real_device_readonly_checklist_preserves_steps() {
    let doc = fs::read_to_string("docs/base1/real-device/CHECKLIST.md").unwrap();
    assert!(doc.contains("Before Running"));
    assert!(doc.contains("During Running"));
    assert!(doc.contains("After Running"));
    assert!(doc.contains("READONLY_REPORT_TEMPLATE.md"));
}

#[test]
fn real_device_readonly_checklist_preserves_guardrails_and_non_claims() {
    let doc = fs::read_to_string("docs/base1/real-device/CHECKLIST.md").unwrap();
    assert!(doc.contains("Dry-run required"));
    assert!(doc.contains("No disk writes"));
    assert!(doc.contains("No real-device write path"));
    assert!(doc.contains("Not installer-ready"));
    assert!(doc.contains("Not hardware-validated"));
    assert!(doc.contains("Not daily-driver ready"));
}

#[test]
fn real_device_index_links_checklist() {
    let index = fs::read_to_string("docs/base1/real-device/README.md").unwrap_or_default();
    assert!(index.contains("CHECKLIST.md"));
}
