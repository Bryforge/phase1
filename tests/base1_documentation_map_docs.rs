use std::fs;

#[test]
fn base1_documentation_map_exists() {
    let doc = fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md").unwrap();
    assert!(doc.contains("Base1 Documentation Map"));
    assert!(doc.contains("Status: active documentation index"));
    assert!(doc.contains("without moving files"));
}

#[test]
fn base1_documentation_map_links_core_entry_points() {
    let doc = fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md").unwrap();
    assert!(doc.contains("README.md"));
    assert!(doc.contains("VALIDATION_RUNBOOK.md"));
    assert!(doc.contains("VALIDATION_REPORT_TEMPLATE.md"));
    assert!(doc.contains("VALIDATION_REPORTS.md"));
}

#[test]
fn base1_documentation_map_links_real_device_track() {
    let doc = fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md").unwrap();
    assert!(doc.contains("real-device/README.md"));
    assert!(doc.contains("real-device/READONLY_VALIDATION_PLAN.md"));
    assert!(doc.contains("real-device/READONLY_REPORT_TEMPLATE.md"));
    assert!(doc.contains("real-device/RUNBOOK.md"));
    assert!(doc.contains("real-device/CHECKLIST.md"));
    assert!(doc.contains("real-device/STATUS_SUMMARY.md"));
}

#[test]
fn base1_documentation_map_lists_readonly_scripts() {
    let doc = fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md").unwrap();
    assert!(doc.contains("base1-real-device-readonly-preview.sh"));
    assert!(doc.contains("base1-real-device-readonly-report.sh"));
    assert!(doc.contains("base1-real-device-readonly-validation-bundle.sh"));
    assert!(doc.contains("base1-real-device-readonly-doctor.sh"));
}

#[test]
fn base1_documentation_map_preserves_non_claims() {
    let doc = fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md").unwrap();
    assert!(doc.contains("Not installer-ready"));
    assert!(doc.contains("Not hardware-validated"));
    assert!(doc.contains("Not daily-driver ready"));
    assert!(doc.contains("No destructive disk writes"));
    assert!(doc.contains("No real-device write path"));
}

#[test]
fn base1_index_links_documentation_map() {
    let index = fs::read_to_string("docs/base1/README.md").unwrap_or_default();
    assert!(index.contains("DOCUMENTATION_MAP.md"));
}
