use std::fs;

#[test]
fn real_device_readonly_validation_plan_exists() {
    let doc = fs::read_to_string("docs/base1/real-device/READONLY_VALIDATION_PLAN.md")
        .expect("real-device read-only validation plan exists");

    assert!(doc.contains("Base1 Real-Device Read-Only Validation Plan"));
    assert!(doc.contains("Status: planning only"));
    assert!(doc.contains("read-only real-device validation preparation"));
}

#[test]
fn real_device_readonly_validation_preserves_guardrails() {
    let doc = fs::read_to_string("docs/base1/real-device/READONLY_VALIDATION_PLAN.md")
        .expect("real-device read-only validation plan exists");

    assert!(doc.contains("No disk writes"));
    assert!(doc.contains("No partitioning"));
    assert!(doc.contains("No formatting"));
    assert!(doc.contains("No installer execution"));
    assert!(doc.contains("No firmware flashing"));
    assert!(doc.contains("No bootloader installation"));
    assert!(doc.contains("No automatic target selection"));
}

#[test]
fn real_device_readonly_validation_preserves_non_claims() {
    let doc = fs::read_to_string("docs/base1/real-device/READONLY_VALIDATION_PLAN.md")
        .expect("real-device read-only validation plan exists");

    assert!(doc.contains("Not installer-ready"));
    assert!(doc.contains("Not hardware-validated"));
    assert!(doc.contains("Not daily-driver ready"));
    assert!(doc.contains("No destructive disk writes"));
    assert!(doc.contains("No real-device write path"));
}

#[test]
fn base1_index_links_real_device_readonly_plan() {
    let base_index = fs::read_to_string("docs/base1/README.md").unwrap_or_default();
    let validation_index =
        fs::read_to_string("docs/base1/validation/README.md").unwrap_or_default();

    assert!(
        base_index.contains("READONLY_VALIDATION_PLAN.md")
            || validation_index.contains("READONLY_VALIDATION_PLAN.md")
    );
}
