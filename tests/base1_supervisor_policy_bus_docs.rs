use std::fs;

fn read_doc() -> String {
    fs::read_to_string("docs/os/BASE1_SUPERVISOR_POLICY_BUS.md")
        .expect("read supervisor policy bus doc")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn base1_supervisor_policy_bus_defines_scope_and_purpose() {
    let doc = read_doc();
    assert_contains(&doc, "Base1 supervisor policy bus");
    assert_contains(&doc, "profile-gated command authorization");
    assert_contains(&doc, "decides whether a requested supervisor control-plane action is allowed");
    assert_contains(&doc, "keeps the X200 path lightweight");
}

#[test]
fn base1_supervisor_policy_bus_lists_inputs_and_decisions() {
    let doc = read_doc();
    for expected in [
        "selected Base1 profile",
        "requested supervisor command",
        "allowed delivery modes",
        "maximum staged-kernel concurrency",
        "target RAM budget",
        "storage-tier policy",
        "evidence requirements",
        "recovery behavior",
        "non-claim boundaries",
        "allow",
        "deny",
        "plan-only",
        "evidence-required",
        "profile-upgrade-required",
    ] {
        assert_contains(&doc, expected);
    }
}

#[test]
fn base1_supervisor_policy_bus_preserves_x200_and_vm_rules() {
    let doc = read_doc();
    assert_contains(&doc, "For x200-supervisor-lite");
    assert_contains(&doc, "one active staged kernel");
    assert_contains(&doc, "supervisor-lite only");
    assert_contains(&doc, "zram plus SSD scratch");
    assert_contains(&doc, "no supervisor-concurrent launch path");
    assert_contains(&doc, "For x86_64-vm-validation");
    assert_contains(&doc, "supervisor-concurrent planning");
    assert_contains(&doc, "profile concurrency limit");
}

#[test]
fn base1_supervisor_policy_bus_preserves_non_claims() {
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
