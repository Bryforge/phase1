use std::fs;
use std::path::Path;

const REPORT: &str = "docs/base1/validation/2026-05-10-real-phase1-initrd-builder.md";
const INDEX: &str = "docs/base1/validation/README.md";

#[test]
fn real_phase1_initrd_builder_report_exists() {
    assert!(Path::new(REPORT).exists(), "report should exist");
}

#[test]
fn validation_index_links_real_phase1_initrd_builder_report() {
    let index = fs::read_to_string(INDEX).expect("validation index should be readable");

    assert!(
        index.contains("2026-05-10-real-phase1-initrd-builder.md"),
        "validation index should link real Phase1 initrd builder report"
    );
}

#[test]
fn real_phase1_initrd_builder_report_records_repeatable_flow() {
    let report = fs::read_to_string(REPORT).expect("report should be readable");

    for expected in [
        "RUSTFLAGS=\"-C linker=rust-lld\"",
        "scripts/base1-real-phase1-initrd-preview.sh",
        "scripts/base1-preview-stack.sh",
        "scripts/base1-qemu-boot-check.sh",
        "phase1.workspace",
    ] {
        assert!(
            report.contains(expected),
            "missing repeatable-flow evidence: {expected}"
        );
    }
}

#[test]
fn real_phase1_initrd_builder_report_records_serial_evidence() {
    let report = fs::read_to_string(REPORT).expect("report should be readable");

    for expected in [
        "Linux version",
        "base1 init wrapper reached",
        "base1 launching real Phase1 binary",
        "phase1.log",
        "phase1.workspace",
        "phase1.conf",
    ] {
        assert!(
            report.contains(expected),
            "missing serial evidence: {expected}"
        );
    }
}

#[test]
fn real_phase1_initrd_builder_report_preserves_non_claims() {
    let report = fs::read_to_string(REPORT).expect("report should be readable");

    for expected in [
        "Installer readiness",
        "Hardware validation",
        "Recovery completeness",
        "Daily-driver readiness",
        "released Base1 image",
        "Full Base1 userspace handoff",
        "Phase1 cleanly exited",
        "production-ready",
    ] {
        assert!(report.contains(expected), "missing non-claim: {expected}");
    }
}
