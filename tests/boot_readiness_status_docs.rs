#[test]
fn boot_readiness_status_defines_current_level_and_target() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    assert!(status.contains("Base1 boot readiness status"), "{status}");
    assert!(status.contains("Current level: **B0 — Documentation ready, in progress**"), "{status}");
    assert!(status.contains("Target next level: **B1 — Read-only detection ready**"), "{status}");
    assert!(
        status.contains("Do not claim Base1 boot readiness, installer readiness, hardware validation, hardened status, recovery completion, or daily-driver readiness from this tracker alone."),
        "{status}"
    );
}

#[test]
fn boot_readiness_status_preserves_ladder() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    for level in [
        "B0",
        "Documentation ready",
        "B1",
        "Read-only detection ready",
        "B2",
        "Dry-run assembly ready",
        "B3",
        "VM boot validated",
        "B4",
        "Recovery validated",
        "B5",
        "Physical target validated",
        "B6",
        "Release candidate",
    ] {
        assert!(status.contains(level), "missing readiness level {level}: {status}");
    }
}

#[test]
fn boot_readiness_status_preserves_finish_before_coding_checklist() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    for item in [
        "Boot readiness race plan.",
        "x86_64 boot support roadmap.",
        "README boot-readiness and x86_64 references.",
        "Contribution guidelines for hardening and x86_64 boot work.",
        "Repository navigation and reorganization indexes.",
        "Asset index and current Fyr asset references.",
        "Boot readiness status tracker.",
        "Boot readiness status tracker tests.",
        "OS roadmap link to this status tracker.",
        "README link to this status tracker.",
        "B1 implementation issue/plan for read-only x86_64 detection.",
    ] {
        assert!(status.contains(item), "missing finish-before-coding item {item}: {status}");
    }
}

#[test]
fn boot_readiness_status_defines_b1_coding_start_gate() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    for text in [
        "B1 coding-start gate",
        "link from [`ROADMAP.md`](ROADMAP.md)",
        "link from [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)",
        "link from [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)",
        "README visibility",
        "tests that preserve the status ladder and non-claims",
    ] {
        assert!(status.contains(text), "missing B1 gate text {text}: {status}");
    }
}

#[test]
fn boot_readiness_status_defines_first_coding_slice() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    for text in [
        "sh scripts/base1-x86_64-detect.sh --dry-run",
        "read-only;",
        "no host mutation;",
        "reports `writes: no`;",
        "reports architecture hints;",
        "reports firmware hints;",
        "reports boot-loader hints;",
        "reports virtualization hints;",
        "reports storage-layout hints;",
        "reports recovery availability hints;",
        "fails closed when required facts are unknown.",
    ] {
        assert!(status.contains(text), "missing B1 first-slice text {text}: {status}");
    }
}

#[test]
fn boot_readiness_status_preserves_evidence_map() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    for text in [
        "ROADMAP.md",
        "BOOT_READINESS_RACE_PLAN.md",
        "X86_64_BOOT_SUPPORT_ROADMAP.md",
        "BOOT_READINESS_STATUS.md",
        "scripts/base1-x86_64-detect.sh",
        "VM validation report",
        "Recovery validation report",
        "Hardware validation report",
    ] {
        assert!(status.contains(text), "missing evidence map text {text}: {status}");
    }
}

#[test]
fn boot_readiness_status_is_linked_from_required_docs() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");
    let race = std::fs::read_to_string("docs/os/BOOT_READINESS_RACE_PLAN.md")
        .expect("boot readiness race plan");
    let x86 = std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md")
        .expect("x86_64 boot support roadmap");
    let readme = std::fs::read_to_string("README.md").expect("README");

    for doc in [&roadmap, &race, &x86, &readme] {
        assert!(doc.contains("BOOT_READINESS_STATUS.md"), "{doc}");
    }
}

#[test]
fn boot_readiness_status_preserves_hardening_and_non_claims() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    assert!(status.contains("Hardening is a roadmap goal and design direction."), "{status}");
    assert!(status.contains("Current status: **planned, evidence-bound**."), "{status}");

    for text in [
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(status.contains(text), "missing non-claim {text}: {status}");
    }
}
