use std::fs;

const REPORT: &str = "docs/base1/validation/2026-05-10-qemu-phase1-marker.md";
const INDEX: &str = "docs/base1/validation/README.md";

#[test]
fn qemu_phase1_marker_report_exists() {
    assert!(
        fs::metadata(REPORT).is_ok(),
        "missing QEMU Phase1 marker report"
    );
}

#[test]
fn validation_index_links_qemu_phase1_marker_report() {
    let index = fs::read_to_string(INDEX).expect("validation index should be readable");
    assert!(
        index.contains("2026-05-10-qemu-phase1-marker.md"),
        "validation index should link QEMU Phase1 marker report"
    );
}

#[test]
fn qemu_phase1_marker_report_records_evidence() {
    let report = fs::read_to_string(REPORT).expect("report should be readable");

    for expected in [
        "PASS: Base1 QEMU Phase1 marker boot path validated.",
        "phase1 6.0.0 ready",
        "base1 init wrapper reached",
        "base1 handoff: exec alpine init",
        "qemu-exit-code: 124",
        "result: pass",
        "scripts/base1-qemu-boot-check.sh",
    ] {
        assert!(report.contains(expected), "missing evidence: {expected}");
    }
}

#[test]
fn qemu_phase1_marker_report_preserves_non_claims() {
    let report = fs::read_to_string(REPORT).expect("report should be readable");

    for expected in [
        "Full Phase1 OS payload boot",
        "Installer readiness",
        "Hardware validation",
        "Recovery completeness",
        "Daily-driver readiness",
        "released Base1 image",
        "production initramfs",
    ] {
        assert!(report.contains(expected), "missing non-claim: {expected}");
    }
}

#[test]
fn qemu_phase1_marker_report_preserves_promotion_rule() {
    let report = fs::read_to_string(REPORT).expect("report should be readable");

    for expected in [
        "real Phase1 binary",
        "included in the boot payload",
        "launched by the boot path",
        "serial evidence captured by the guarded QEMU checker",
    ] {
        assert!(
            report.contains(expected),
            "missing promotion rule: {expected}"
        );
    }
}
