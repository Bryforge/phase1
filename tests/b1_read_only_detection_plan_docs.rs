#[test]
fn b1_read_only_detection_plan_defines_scope_and_command() {
    let plan = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_PLAN.md")
        .expect("B1 read-only detection plan");

    assert!(plan.contains("Base1 B1 read-only detection plan"), "{plan}");
    assert!(
        plan.contains("first B1 boot-readiness implementation slice"),
        "{plan}"
    );
    assert!(
        plan.contains("sh scripts/base1-x86_64-detect.sh --dry-run"),
        "{plan}"
    );
    assert!(
        plan.contains("The command must require `--dry-run` for the initial implementation."),
        "{plan}"
    );
}

#[test]
fn b1_read_only_detection_plan_lists_required_output() {
    let plan = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_PLAN.md")
        .expect("B1 read-only detection plan");

    for text in [
        "`writes: no`",
        "architecture hints",
        "firmware hints",
        "boot-loader hints",
        "virtualization hints",
        "storage-layout hints",
        "recovery availability hints",
        "unknown or unsupported state warnings",
        "next recommended read-only check",
    ] {
        assert!(plan.contains(text), "missing required output text {text}: {plan}");
    }
}

#[test]
fn b1_read_only_detection_plan_defines_detection_categories() {
    let plan = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_PLAN.md")
        .expect("B1 read-only detection plan");

    for text in [
        "Architecture",
        "Firmware",
        "Boot loader",
        "Kernel command line",
        "Storage layout",
        "Virtualization",
        "Recovery",
        "Detect `x86_64` where available; report unknown otherwise.",
        "Read only; redact sensitive tokens if any appear.",
        "Report hints only; do not modify boot entries.",
    ] {
        assert!(plan.contains(text), "missing detection category text {text}: {plan}");
    }
}

#[test]
fn b1_read_only_detection_plan_preserves_fail_closed_behavior() {
    let plan = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_PLAN.md")
        .expect("B1 read-only detection plan");

    for text in [
        "Fail-closed behavior",
        "`--dry-run` is missing",
        "architecture cannot be determined",
        "required read-only tools are unavailable and no fallback exists",
        "output would require privileged mutation",
        "a command would write to disk, firmware, boot loader configuration, or partitions",
        "which read-only check can be run next",
    ] {
        assert!(plan.contains(text), "missing fail-closed text {text}: {plan}");
    }
}

#[test]
fn b1_read_only_detection_plan_preserves_redaction_and_non_mutation_rules() {
    let plan = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_PLAN.md")
        .expect("B1 read-only detection plan");

    for text in [
        "The detector must not print secrets.",
        "tokens;",
        "private keys;",
        "recovery codes;",
        "credentials;",
        "private kernel command-line secrets",
        "Non-mutation rules",
        "call `grub-install`",
        "call `efibootmgr` with write flags",
        "write boot entries",
        "edit `/boot`, `/etc`, EFI variables, partitions, or initramfs files",
        "install packages",
        "require network access",
    ] {
        assert!(plan.contains(text), "missing redaction/non-mutation text {text}: {plan}");
    }
}

#[test]
fn b1_read_only_detection_plan_defines_test_expectations() {
    let plan = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_PLAN.md")
        .expect("B1 read-only detection plan");

    for text in [
        "script exists",
        "script syntax passes `sh -n`",
        "script requires `--dry-run`",
        "script prints `writes: no` with `--dry-run`",
        "script does not contain known mutating boot commands",
        "docs link this plan",
        "non-claims are preserved",
    ] {
        assert!(plan.contains(text), "missing test expectation {text}: {plan}");
    }
}

#[test]
fn b1_read_only_detection_plan_is_linked_from_required_docs() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status");
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");
    let race = std::fs::read_to_string("docs/os/BOOT_READINESS_RACE_PLAN.md")
        .expect("boot readiness race plan");
    let x86 = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");

    for doc in [&status, &roadmap, &race, &x86] {
        assert!(doc.contains("B1_READ_ONLY_DETECTION_PLAN.md"), "{doc}");
    }
}

#[test]
fn b1_read_only_detection_plan_preserves_non_claims() {
    let plan = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_PLAN.md")
        .expect("B1 read-only detection plan");

    for text in [
        "does not implement the detector by itself",
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(plan.contains(text), "missing non-claim {text}: {plan}");
    }
}
