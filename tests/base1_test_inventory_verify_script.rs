#[test]
fn base1_test_inventory_verifier_is_read_only() {
    let script = std::fs::read_to_string("scripts/base1-test-inventory-verify.sh")
        .expect("Base1 test inventory verifier");

    assert!(script.contains("Base1 test inventory verifier"), "{script}");
    assert!(script.contains("mode: read-only"), "{script}");
    assert!(
        script.contains("verification complete; no files were changed"),
        "{script}"
    );
}

#[test]
fn base1_test_inventory_verifier_compares_reporter_to_docs() {
    let script = std::fs::read_to_string("scripts/base1-test-inventory-verify.sh")
        .expect("Base1 test inventory verifier");

    assert!(script.contains("docs/base1/TEST_INVENTORY.md"), "{script}");
    assert!(script.contains("scripts/base1-test-inventory.sh"), "{script}");
    assert!(script.contains("reported test missing from docs inventory"), "{script}");
    assert!(script.contains("missing-from-doc"), "{script}");
    assert!(script.contains("tests-reported"), "{script}");
}

#[test]
fn base1_test_inventory_verifier_is_wired_into_quality_gate() {
    let quality = std::fs::read_to_string("scripts/quality-check.sh")
        .expect("quality-check script");
    let integrity = std::fs::read_to_string("scripts/base1-doc-integrity.sh")
        .expect("Base1 integrity gate");

    assert!(
        quality.contains("run sh scripts/base1-test-inventory-verify.sh"),
        "{quality}"
    );
    assert!(
        integrity.contains("scripts/base1-test-inventory-verify.sh"),
        "{integrity}"
    );
}

#[test]
fn base1_test_inventory_verifier_preserves_failure_behavior() {
    let script = std::fs::read_to_string("scripts/base1-test-inventory-verify.sh")
        .expect("Base1 test inventory verifier");

    assert!(script.contains("exit 1"), "{script}");
    assert!(script.contains("missing inventory doc"), "{script}");
    assert!(script.contains("missing reporter"), "{script}");
}
