use std::fs;

fn read_doc() -> String {
    fs::read_to_string("docs/os/B6_HARDWARE_BOOT_EVIDENCE.md")
        .expect("read B6 hardware boot evidence doc")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn b6_hardware_boot_evidence_defines_scope_and_purpose() {
    let doc = read_doc();
    assert_contains(&doc, "B6 named hardware boot evidence");
    assert_contains(&doc, "named local hardware boot observation");
    assert_contains(&doc, "artifact identity");
    assert_contains(&doc, "machine identity");
    assert_contains(&doc, "operator-observed boot result");
}

#[test]
fn b6_hardware_boot_evidence_lists_inputs_and_result_states() {
    let doc = read_doc();
    for expected in [
        "B1 read-only detection evidence",
        "B2 dry-run assembly evidence",
        "B3 reviewed VM evidence",
        "B4 reviewed recovery evidence",
        "B5 local boot artifact plan",
        "not_attempted",
        "boot_menu_seen",
        "boot_started",
        "phase1_marker_seen",
        "blocked",
        "failed",
        "phase1 6.0.0 ready",
    ] {
        assert_contains(&doc, expected);
    }
}

#[test]
fn b6_hardware_boot_evidence_preserves_non_claims() {
    let doc = read_doc();
    assert_contains(&doc, "does not make Base1 installer-ready");
    assert_contains(&doc, "recovery-complete");
    assert_contains(&doc, "hardened");
    assert_contains(&doc, "hypervisor-ready");
    assert_contains(&doc, "release-candidate ready");
    assert_contains(&doc, "daily-driver ready");
}

#[test]
fn b6_hardware_boot_evidence_records_x200_phase1_marker() {
    let doc = read_doc();
    for expected in [
        "Status: X200 phase1 marker observed",
        "X200 phase1 marker evidence",
        "build/base1-b3-uefi-proof.img",
        "688518c1437003c7b8325b1d5d479bc97f77c3404c8fd27dace6d823d406b79b",
        "PHASE1B42",
        "phase1 6.0.0 ready",
        "phase1_marker_seen",
        "claim state: `not_claimed`",
    ] {
        assert_contains(&doc, expected);
    }
}
