use std::fs;

fn read_doc() -> String {
    fs::read_to_string("docs/os/B5_LOCAL_BOOT_ARTIFACT_PLAN.md")
        .expect("read B5 local boot artifact plan doc")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn b5_local_boot_artifact_plan_defines_scope_and_purpose() {
    let doc = read_doc();
    assert_contains(&doc, "B5 local boot artifact plan");
    assert_contains(&doc, "single explicit local boot artifact");
    assert_contains(&doc, "fastest safe hardware test path");
    assert_contains(&doc, "B1 read-only detection");
    assert_contains(&doc, "B2 dry-run assembly");
    assert_contains(&doc, "B3 reviewed VM evidence");
    assert_contains(&doc, "B4 reviewed recovery evidence");
}

#[test]
fn b5_local_boot_artifact_plan_preserves_prechecks_and_default_artifact() {
    let doc = read_doc();
    for expected in [
        "build/phase1-uefi.img",
        "B1 read-only detection evidence exists",
        "B2 dry-run assembly evidence exists",
        "B3 reviewed VM evidence exists",
        "B4 reviewed recovery evidence exists",
        "selected artifact path is explicit",
        "selected artifact is local-only",
        "selected artifact is not staged in normal Git history",
        "recovery path remains planned before hardware testing",
    ] {
        assert_contains(&doc, expected);
    }
}

#[test]
fn b5_local_boot_artifact_plan_preserves_non_claims() {
    let doc = read_doc();
    assert_contains(&doc, "does not make Base1 bootable on hardware");
    assert_contains(&doc, "installer-ready");
    assert_contains(&doc, "recovery-complete");
    assert_contains(&doc, "hardened");
    assert_contains(&doc, "hypervisor-ready");
    assert_contains(&doc, "hardware-validated");
    assert_contains(&doc, "release-candidate ready");
    assert_contains(&doc, "daily-driver ready");
}
