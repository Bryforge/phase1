use std::fs;

fn read_doc() -> String {
    fs::read_to_string("docs/os/BASE1_SUPERVISOR_CONTROL_PLANE.md")
        .expect("read Base1 supervisor control-plane contract")
}

fn assert_contains(doc: &str, needle: &str) {
    assert!(doc.contains(needle), "missing {needle}: {doc}");
}

#[test]
fn base1_supervisor_control_plane_defines_scope_and_purpose() {
    let doc = read_doc();
    assert_contains(&doc, "# Base1 supervisor control-plane contract");
    assert_contains(&doc, "Status: planning scaffold");
    assert_contains(&doc, "Scope: command surface, policy gates, artifact flow, evidence flow, recovery hooks, and non-claim boundaries");
    assert_contains(
        &doc,
        "coordinate staged kernels, evidence capture, storage tiers, and recovery hooks",
    );
    assert_contains(&doc, "without slowing the direct-first path");
}

#[test]
fn base1_supervisor_control_plane_lists_command_surface() {
    let doc = read_doc();
    for command in [
        "status",
        "plan",
        "stage-artifact",
        "validate-artifact",
        "launch-preview",
        "capture-evidence",
        "request-recovery",
        "stop",
    ] {
        assert_contains(&doc, command);
    }
}

#[test]
fn base1_supervisor_control_plane_preserves_policy_gates() {
    let doc = read_doc();
    assert_contains(
        &doc,
        "The control plane must load the selected Base1 profile before doing any work.",
    );
    assert_contains(&doc, "allowed delivery modes");
    assert_contains(&doc, "maximum staged-kernel concurrency");
    assert_contains(&doc, "target RAM budget");
    assert_contains(&doc, "storage-tier policy");
    assert_contains(&doc, "evidence requirements");
    assert_contains(&doc, "recovery behavior");
    assert_contains(&doc, "non-claim boundaries");
}

#[test]
fn base1_supervisor_control_plane_preserves_non_claims() {
    let doc = read_doc();
    assert_contains(&doc, "does not make Base1 bootable");
    assert_contains(&doc, "installer-ready");
    assert_contains(&doc, "recovery-complete");
    assert_contains(&doc, "hardened");
    assert_contains(&doc, "hypervisor-ready");
    assert_contains(&doc, "hardware-validated");
    assert_contains(&doc, "release-candidate ready");
    assert_contains(&doc, "daily-driver ready");
}
