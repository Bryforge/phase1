use std::fs;

#[test]
fn real_device_readonly_runbook_exists() {
    let doc = fs::read_to_string("docs/base1/real-device/RUNBOOK.md").unwrap();
    assert!(doc.contains("Base1 Real-Device Read-Only Validation Runbook"));
    assert!(doc.contains("read-only workflow runbook"));
}

#[test]
fn real_device_readonly_runbook_lists_required_command() {
    let doc = fs::read_to_string("docs/base1/real-device/RUNBOOK.md").unwrap();
    assert!(doc.contains(
        "base1-real-device-readonly-validation-bundle.sh --dry-run --target /dev/YOUR_TARGET"
    ));
}

#[test]
fn real_device_readonly_runbook_preserves_result_labels() {
    let doc = fs::read_to_string("docs/base1/real-device/RUNBOOK.md").unwrap();
    assert!(doc.contains("read-only-observed"));
    assert!(doc.contains("blocked-before-device-access"));
    assert!(doc.contains("operator-aborted"));
    assert!(doc.contains("needs-follow-up"));
}

#[test]
fn real_device_readonly_runbook_preserves_guardrails_and_non_claims() {
    let doc = fs::read_to_string("docs/base1/real-device/RUNBOOK.md").unwrap();
    assert!(doc.contains("Dry-run required"));
    assert!(doc.contains("No disk writes"));
    assert!(doc.contains("No real-device write path"));
    assert!(doc.contains("Not installer-ready"));
    assert!(doc.contains("Not hardware-validated"));
    assert!(doc.contains("Not daily-driver ready"));
}

#[test]
fn real_device_index_links_runbook() {
    let index = fs::read_to_string("docs/base1/real-device/README.md").unwrap_or_default();
    assert!(index.contains("RUNBOOK.md"));
}
