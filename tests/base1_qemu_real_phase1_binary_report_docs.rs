use std::fs;
use std::path::Path;

const REPORT: &str = "docs/base1/validation/2026-05-10-qemu-real-phase1-binary.md";
const INDEX: &str = "docs/base1/validation/README.md";

#[test]
fn qemu_real_phase1_binary_report_exists() {
    assert!(
        Path::new(REPORT).exists(),
        "real Phase1 binary QEMU report should exist"
    );
}

#[test]
fn validation_index_links_qemu_real_phase1_binary_report() {
    let index = fs::read_to_string(INDEX).expect("validation index should be readable");

    assert!(
        index.contains("2026-05-10-qemu-real-phase1-binary.md"),
        "validation index should link real Phase1 binary QEMU report"
    );
}

#[test]
fn qemu_real_phase1_binary_report_records_evidence() {
    let report = fs::read_to_string(REPORT).expect("report should be readable");

    for expected in [
        "real `phase1` x86_64 Linux musl binary",
        "base1 launching real Phase1 binary",
        "phase1.log",
        "phase1.workspace",
        "phase1.conf",
        "static-pie linked",
        "--expect \"phase1.workspace\"",
    ] {
        assert!(report.contains(expected), "missing evidence: {expected}");
    }
}

#[test]
fn qemu_real_phase1_binary_report_preserves_non_claims() {
    let report = fs::read_to_string(REPORT).expect("report should be readable");

    for expected in [
        "Installer readiness",
        "Hardware validation",
        "Recovery completeness",
        "Daily-driver readiness",
        "released Base1 image",
        "production initramfs",
        "cleanly exited",
        "full userspace handoff",
    ] {
        assert!(report.contains(expected), "missing non-claim: {expected}");
    }
}

#[test]
fn qemu_real_phase1_binary_report_preserves_promotion_rule() {
    let report = fs::read_to_string(REPORT).expect("report should be readable");

    for expected in [
        "normal Base1 boot payload",
        "serial evidence",
        "provenance",
        "repeatable build instructions",
        "non-preview release boundary",
    ] {
        assert!(
            report.contains(expected),
            "missing promotion rule: {expected}"
        );
    }
}
