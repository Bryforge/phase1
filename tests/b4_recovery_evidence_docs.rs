use std::fs;

fn read_doc() -> String {
    fs::read_to_string("docs/os/B4_RECOVERY_EVIDENCE.md")
        .expect("read B4 recovery evidence doc")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn b4_recovery_evidence_defines_scope_and_purpose() {
    let doc = read_doc();
    assert_contains(&doc, "B4 recovery evidence");
    assert_contains(&doc, "reviewed local recovery-validation evidence");
    assert_contains(&doc, "recovery path");
    assert_contains(&doc, "rollback path");
    assert_contains(&doc, "emergency stop path");
    assert_contains(&doc, "operator-visible failure reason");
}

#[test]
fn b4_recovery_evidence_lists_required_inputs_and_markers() {
    let doc = read_doc();
    for expected in [
        "build/base1-b4-recovery-validation/b4-recovery-validation.env",
        "scripts/base1-b4-recovery-validate.sh",
        "docs/os/B4_RECOVERY_VALIDATION.md",
        "BASE1_B4_RECOVERY_MODE=prepare",
        "BASE1_B4_RECOVERY_PROFILE=x200-supervisor-lite",
        "BASE1_B4_RECOVERY_BOOT_ARTIFACT=planned",
        "BASE1_B4_RECOVERY_ARTIFACT=planned",
        "BASE1_B4_RECOVERY_ROLLBACK_PATH=planned",
        "BASE1_B4_RECOVERY_EMERGENCY_STOP=planned",
        "BASE1_B4_RECOVERY_FAILURE_REASON=operator-visible",
        "BASE1_B4_RECOVERY_DRY_RUN_ONLY=1",
        "BASE1_B4_RECOVERY_CLAIM=not_claimed",
    ] {
        assert_contains(&doc, expected);
    }
}

#[test]
fn b4_recovery_evidence_preserves_result_and_non_claims() {
    let doc = read_doc();
    assert_contains(&doc, "reviewed_recovery_evidence");
    assert_contains(&doc, "not_claimed");
    assert_contains(&doc, "does not make Base1 bootable");
    assert_contains(&doc, "installer-ready");
    assert_contains(&doc, "recovery-complete");
    assert_contains(&doc, "hardened");
    assert_contains(&doc, "hypervisor-ready");
    assert_contains(&doc, "hardware-validated");
    assert_contains(&doc, "release-candidate ready");
    assert_contains(&doc, "daily-driver ready");
}
