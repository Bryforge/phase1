use std::fs;

fn read_doc() -> String {
    fs::read_to_string("docs/os/B3_REVIEWED_VM_EVIDENCE.md")
        .expect("read B3 reviewed VM evidence doc")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn b3_reviewed_vm_evidence_defines_scope_and_purpose() {
    let doc = read_doc();
    assert_contains(&doc, "B3 reviewed VM evidence");
    assert_contains(&doc, "reviewed local VM evidence");
    assert_contains(&doc, "B2 focused test-suite pass record");
    assert_contains(&doc, "B3 UEFI proof");
    assert_contains(&doc, "kernel/initrd handoff");
    assert_contains(&doc, "GNU/Linux stage");
    assert_contains(&doc, "OpenBSD launch stage");
    assert_contains(&doc, "X200 emulator evidence report");
}

#[test]
fn b3_reviewed_vm_evidence_lists_required_inputs() {
    let doc = read_doc();
    for expected in [
        "build/base1-b2-test-suite/b2-test-suite-summary.env",
        "build/base1-b3-uefi-proof/reports/b3-summary.env",
        "build/base1-b3-uefi-proof/reports/b3-serial.log",
        "build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env",
        "build/base1-b3-kernel-handoff/reports/qemu-boot.log",
        "build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env",
        "build/base1-b3-gnulinux-stage/reports/qemu-boot.log",
        "build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env",
        "build/base1-b3-openbsd-stage/reports/openbsd-qemu-boot.log",
        "docs/os/B3_OPENBSD_SERIAL_LIMITATION.md",
        "build/base1-b3-vm-validation/b3-validation-scaffold.env",
        "build/base1-b3-vm-validation/b3-log-bundle-review.env",
        "docs/os/B3_X200_EMULATOR_EVIDENCE_REPORT.md",
    ] {
        assert_contains(&doc, expected);
    }
}

#[test]
fn b3_reviewed_vm_evidence_preserves_pass_markers() {
    let doc = read_doc();
    for expected in [
        "BASE1_B2_TEST_SUITE_RESULT=pass",
        "BASE1_B2_TEST_SUITE_FAILED_COUNT=0",
        "BASE1_B3_EVIDENCE_STATE=evidence-present",
        "BASE1_B3_EVIDENCE_SUMMARY_COUNT=4",
        "BASE1_B3_UEFI_SUMMARY_PRESENT=yes",
        "BASE1_B3_HANDOFF_SUMMARY_PRESENT=yes",
        "BASE1_B3_GNULINUX_SUMMARY_PRESENT=yes",
        "BASE1_B3_LOG_REVIEW_RESULT=pass",
        "BASE1_B3_LOG_REVIEW_CLAIM=not_claimed",
        "BASE1_B3_LOG_REVIEW_OPENBSD_SUMMARY_PRESENT=yes",
        "BASE1_B3_LOG_REVIEW_OPENBSD_LOG_PRESENT=yes",
        "reviewed_vm_evidence",
        "not_claimed",
    ] {
        assert_contains(&doc, expected);
    }
}

#[test]
fn b3_reviewed_vm_evidence_preserves_non_claims() {
    let doc = read_doc();
    assert_contains(&doc, "does not mean Base1 is bootable on physical hardware");
    assert_contains(&doc, "does not mean Base1 has an installer");
    assert_contains(&doc, "does not mean Base1 has recovery validation");
    assert_contains(&doc, "does not prove hardening");
    assert_contains(&doc, "does not validate hardware");
    assert_contains(&doc, "does not make Base1 release-candidate ready");
    assert_contains(&doc, "does not make Base1 daily-driver ready");
}
