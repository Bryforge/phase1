use std::fs;

#[test]
fn real_device_readonly_evidence_capture_report_exists() {
    let doc = fs::read_to_string(
        "docs/base1/real-device/reports/2026-05-10-readonly-evidence-capture.md",
    )
    .unwrap();
    assert!(doc.contains("Base1 Real-Device Read-Only Evidence Capture Report"));
    assert!(doc.contains("draft-read-only evidence capture"));
}

#[test]
fn real_device_readonly_evidence_capture_report_records_required_command() {
    let doc = fs::read_to_string(
        "docs/base1/real-device/reports/2026-05-10-readonly-evidence-capture.md",
    )
    .unwrap();
    assert!(doc.contains(
        "base1-real-device-readonly-validation-bundle.sh --dry-run --target /dev/YOUR_TARGET"
    ));
}

#[test]
fn real_device_readonly_evidence_capture_report_records_sources() {
    let doc = fs::read_to_string(
        "docs/base1/real-device/reports/2026-05-10-readonly-evidence-capture.md",
    )
    .unwrap();
    assert!(doc.contains("Base1 real-device read-only validation plan"));
    assert!(doc.contains("Base1 real-device read-only validation runbook"));
    assert!(doc.contains("Base1 real-device read-only checklist"));
    assert!(doc.contains("Base1 real-device read-only validation bundle"));
    assert!(doc.contains("QEMU evidence chain"));
}

#[test]
fn real_device_readonly_evidence_capture_report_preserves_guardrails_and_non_claims() {
    let doc = fs::read_to_string(
        "docs/base1/real-device/reports/2026-05-10-readonly-evidence-capture.md",
    )
    .unwrap();
    assert!(doc.contains("Dry-run required"));
    assert!(doc.contains("No disk writes"));
    assert!(doc.contains("No real-device write path"));
    assert!(doc.contains("Not installer-ready"));
    assert!(doc.contains("Not hardware-validated"));
    assert!(doc.contains("Not daily-driver ready"));
}

#[test]
fn real_device_index_links_evidence_capture_report() {
    let index = fs::read_to_string("docs/base1/real-device/README.md").unwrap_or_default();
    assert!(index.contains("2026-05-10-readonly-evidence-capture.md"));
}
