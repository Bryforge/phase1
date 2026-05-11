#[test]
fn base1_test_inventory_script_is_read_only() {
    let script = std::fs::read_to_string("scripts/base1-test-inventory.sh")
        .expect("Base1 test inventory script");

    assert!(script.contains("Base1 test inventory reporter"), "{script}");
    assert!(script.contains("mode: read-only"), "{script}");
    assert!(
        script.contains("inventory complete; no files were changed"),
        "{script}"
    );
}

#[test]
fn base1_test_inventory_script_lists_expected_patterns() {
    let script = std::fs::read_to_string("scripts/base1-test-inventory.sh")
        .expect("Base1 test inventory script");

    for pattern in [
        "tests/base1_*.rs",
        "tests/quality_base1_*.rs",
        "tests/*base1*.rs",
    ] {
        assert!(script.contains(pattern), "missing pattern {pattern}: {script}");
    }
}

#[test]
fn base1_test_inventory_script_reports_counts() {
    let script = std::fs::read_to_string("scripts/base1-test-inventory.sh")
        .expect("Base1 test inventory script");

    assert!(script.contains("count:"), "{script}");
    assert!(script.contains("Base1 integration tests"), "{script}");
    assert!(script.contains("Base1 quality tests"), "{script}");
    assert!(script.contains("Other Base1-named tests"), "{script}");
}

#[test]
fn base1_test_inventory_is_documented_and_integrity_checked() {
    let inventory = std::fs::read_to_string("docs/base1/TEST_INVENTORY.md")
        .expect("Base1 test inventory docs");
    let integrity = std::fs::read_to_string("scripts/base1-doc-integrity.sh")
        .expect("Base1 integrity gate");

    assert!(
        inventory.contains("sh scripts/base1-test-inventory.sh"),
        "{inventory}"
    );
    assert!(
        integrity.contains("scripts/base1-test-inventory.sh"),
        "{integrity}"
    );
    assert!(
        integrity.contains("tests/quality_base1_*.rs"),
        "{integrity}"
    );
}
