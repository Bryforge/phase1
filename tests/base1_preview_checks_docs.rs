use std::fs;

const CHECKS: &str = "docs/base1/PREVIEW_CHECKS.md";
const INDEX: &str = "docs/base1/README.md";

#[test]
fn base1_preview_checks_doc_exists() {
    assert!(fs::metadata(CHECKS).is_ok(), "missing Base1 preview checks doc");
}

#[test]
fn base1_index_links_preview_checks_doc() {
    let index = fs::read_to_string(INDEX).expect("Base1 index should be readable");

    assert!(
        index.contains("PREVIEW_CHECKS.md"),
        "Base1 index should link preview checks doc"
    );
}

#[test]
fn base1_preview_checks_lists_current_test_set() {
    let checks = fs::read_to_string(CHECKS).expect("Base1 preview checks doc should be readable");

    for expected in [
        "base1_preview_inputs_script",
        "base1_emulator_preview_script",
        "base1_emulator_doctor_script",
        "base1_preview_gate_script",
        "base1_preview_provenance_script",
        "base1_preview_verify_script",
        "base1_preview_stack_script",
        "base1_preview_stack_runbook_docs",
        "base1_preview_stack_validation_report_docs",
    ] {
        assert!(checks.contains(expected), "missing test target: {expected}");
    }
}

#[test]
fn base1_preview_checks_records_safe_manual_smoke() {
    let checks = fs::read_to_string(CHECKS).expect("Base1 preview checks doc should be readable");

    for expected in [
        "scripts/base1-preview-stack.sh",
        "--bundle build/base1-preview-stack-demo",
        "--kernel build/base1-test-vmlinuz",
        "--initrd build/base1-test-initrd.img",
        "--no-qemu-check",
        "reports/provenance.env",
        "reports/SHA256SUMS",
    ] {
        assert!(checks.contains(expected), "missing smoke detail: {expected}");
    }
}

#[test]
fn base1_preview_checks_preserves_non_claims() {
    let checks = fs::read_to_string(CHECKS).expect("Base1 preview checks doc should be readable");

    for expected in [
        "does not claim that Base1 is bootable",
        "installer-ready",
        "hardware-validated",
        "recovery-complete",
        "daily-driver ready",
        "preview stack launches QEMU",
        "validated Base1 kernel/initrd",
        "preview-only path",
    ] {
        assert!(checks.contains(expected), "missing non-claim: {expected}");
    }
}
