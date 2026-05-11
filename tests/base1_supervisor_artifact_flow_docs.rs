use std::fs;

fn read_doc() -> String {
    fs::read_to_string("docs/os/BASE1_SUPERVISOR_ARTIFACT_FLOW.md")
        .expect("read supervisor artifact flow doc")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn base1_supervisor_artifact_flow_defines_scope_and_purpose() {
    let doc = read_doc();
    assert_contains(&doc, "Base1 supervisor artifact-flow contract");
    assert_contains(&doc, "artifact identity, staging, validation");
    assert_contains(&doc, "direct-first and supervisor-lite compatible");
    assert_contains(&doc, "profile, policy, storage-tier, and evidence vocabulary");
}

#[test]
fn base1_supervisor_artifact_flow_lists_artifact_classes() {
    let doc = read_doc();
    for expected in [
        "kernel",
        "initrd",
        "uefi-proof-log",
        "gnulinux-stage-log",
        "openbsd-stage-image",
        "openbsd-stage-log",
        "storage-scratch-bundle",
        "profile-report",
        "policy-report",
        "validation-report",
        "recovery-metadata",
    ] {
        assert_contains(&doc, expected);
    }
}

#[test]
fn base1_supervisor_artifact_flow_preserves_required_fields() {
    let doc = read_doc();
    for expected in [
        "artifact id",
        "artifact class",
        "source path",
        "staged path",
        "selected Base1 profile",
        "storage-tier policy",
        "policy decision",
        "expected marker",
        "evidence path",
        "validation state",
        "claim state",
        "non-claim boundaries",
        "not_claimed",
    ] {
        assert_contains(&doc, expected);
    }
}

#[test]
fn base1_supervisor_artifact_flow_preserves_x200_vm_and_non_claims() {
    let doc = read_doc();
    assert_contains(&doc, "For x200-supervisor-lite");
    assert_contains(&doc, "one active staged kernel");
    assert_contains(&doc, "no supervisor-concurrent launch path");
    assert_contains(&doc, "For x86_64-vm-validation");
    assert_contains(&doc, "supervisor-concurrent planning");
    assert_contains(&doc, "evidence-required and VM-only until reviewed");
    assert_contains(&doc, "does not make Base1 bootable");
    assert_contains(&doc, "installer-ready");
    assert_contains(&doc, "recovery-complete");
    assert_contains(&doc, "hardened");
    assert_contains(&doc, "hypervisor-ready");
    assert_contains(&doc, "hardware-validated");
    assert_contains(&doc, "release-candidate ready");
    assert_contains(&doc, "daily-driver ready");
}
