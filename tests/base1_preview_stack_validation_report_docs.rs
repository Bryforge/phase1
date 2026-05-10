use std::fs;

const REPORT: &str = "docs/base1/validation/2026-05-10-preview-stack.md";
const INDEX: &str = "docs/base1/validation/README.md";

#[test]
fn base1_preview_stack_validation_report_exists() {
    assert!(
        fs::metadata(REPORT).is_ok(),
        "missing Base1 preview stack validation report"
    );
}

#[test]
fn validation_index_links_preview_stack_report() {
    let index = fs::read_to_string(INDEX).expect("Base1 validation index should be readable");

    assert!(
        index.contains("2026-05-10-preview-stack.md"),
        "validation index should link the preview stack report"
    );
    assert!(
        index.contains("safe Base1 preview-stack mechanics evidence report"),
        "validation index should explain the preview stack report scope"
    );
}

#[test]
fn preview_stack_report_records_scope_and_evidence() {
    let report = fs::read_to_string(REPORT).expect("Base1 preview stack report should be readable");

    for expected in [
        "Base1 Validation Report: Safe Preview Stack",
        "Date: 2026-05-10",
        "Status: preview evidence record",
        "Result: PASS for preview-stack mechanics",
        "scripts/base1-preview-inputs.sh",
        "scripts/base1-emulator-preview.sh",
        "scripts/base1-emulator-doctor.sh",
        "scripts/base1-preview-gate.sh --dry-run",
        "scripts/base1-preview-provenance.sh",
        "scripts/base1-preview-verify.sh",
        "scripts/base1-preview-stack.sh",
        "docs/base1/PREVIEW_STACK_RUNBOOK.md",
        "reports/provenance.env",
        "reports/SHA256SUMS",
    ] {
        assert!(
            report.contains(expected),
            "report missing expected evidence: {expected}"
        );
    }
}

#[test]
fn preview_stack_report_preserves_non_claims() {
    let report = fs::read_to_string(REPORT).expect("Base1 preview stack report should be readable");

    for expected in [
        "does not claim that Base1 is bootable",
        "released Base1 image",
        "secure OS replacement",
        "daily-driver-ready system",
        "hardware installation path",
        "destructive installer path",
        "Recovery USB completion",
        "Rollback completion",
        "Real hardware behavior",
        "Emulator launch success",
        "Kernel correctness",
        "Initrd correctness",
        "GRUB runtime behavior",
        "Do not use this report to mark Base1 as bootable",
    ] {
        assert!(
            report.contains(expected),
            "report missing non-claim: {expected}"
        );
    }
}

#[test]
fn preview_stack_report_preserves_promotion_boundary() {
    let report = fs::read_to_string(REPORT).expect("Base1 preview stack report should be readable");

    for expected in [
        "preview evidence level",
        "safe preview stack mechanics only",
        "real target identity",
        "exact kernel/initrd provenance",
        "emulator launch evidence",
        "boot logs",
        "failure logs",
        "rollback/recovery notes",
        "explicit non-claims",
    ] {
        assert!(
            report.contains(expected),
            "report missing promotion boundary: {expected}"
        );
    }
}
