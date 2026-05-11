use std::fs;

fn read_doc() -> String {
    fs::read_to_string("docs/os/BASE1_SUPERVISOR_STORAGE_TIER.md")
        .expect("read supervisor storage tier doc")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn base1_supervisor_storage_tier_defines_scope_and_purpose() {
    let doc = read_doc();
    assert_contains(&doc, "Base1 supervisor storage-tier contract");
    assert_contains(&doc, "RAM, zram, tmpfs, SSD scratch, swap backstop");
    assert_contains(&doc, "without pretending disk is equivalent to RAM");
    assert_contains(&doc, "artifact staging");
    assert_contains(&doc, "temporary execution workspace");
}

#[test]
fn base1_supervisor_storage_tier_preserves_tier_order() {
    let doc = read_doc();
    for expected in [
        "real RAM for hot state",
        "small tmpfs",
        "zram",
        "SSD scratch",
        "swap backstop",
        "persistent evidence logs",
        "must not be described as RAM-equivalent",
    ] {
        assert_contains(&doc, expected);
    }
}

#[test]
fn base1_supervisor_storage_tier_preserves_policy_fields() {
    let doc = read_doc();
    for expected in [
        "target RAM budget",
        "tmpfs budget",
        "zram budget",
        "SSD scratch budget",
        "swap backstop budget",
        "maximum staged-kernel concurrency",
        "storage-tier policy",
        "evidence path policy",
        "non-claim boundaries",
        "reject storage requests that exceed the selected profile",
    ] {
        assert_contains(&doc, expected);
    }
}

#[test]
fn base1_supervisor_storage_tier_preserves_x200_and_vm_rules() {
    let doc = read_doc();
    assert_contains(&doc, "For x200-supervisor-lite");
    assert_contains(&doc, "low-memory behavior");
    assert_contains(&doc, "zram-first pressure relief");
    assert_contains(&doc, "SSD scratch as a workflow backstop");
    assert_contains(&doc, "one active staged kernel");
    assert_contains(&doc, "must not enable supervisor-concurrent");
    assert_contains(&doc, "For x86_64-vm-validation");
    assert_contains(&doc, "build-directory scratch");
    assert_contains(&doc, "VM evidence only until reviewed");
}

#[test]
fn base1_supervisor_storage_tier_preserves_non_claims() {
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
