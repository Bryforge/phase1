#[test]
fn boot_readiness_status_defines_current_level_and_target() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    assert!(status.contains("Base1 boot readiness status"), "{status}");
    assert!(status.contains("Current level: **B2 — Dry-run assembly ready, initial script present**"), "{status}");
    assert!(status.contains("Target next level: **B3 — VM boot validated**"), "{status}");
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
        "Initial script present",
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
        "B1 plan tests.",
        "B1 plan link from OS roadmap, x86_64 roadmap, and race plan.",
    ] {
        assert!(status.contains(item), "missing finish-before-coding item {item}: {status}");
    }

    assert!(
        status.contains("Finish-first status: **complete for B1 implementation start**."),
        "{status}"
    );
}

#[test]
fn boot_readiness_status_defines_b1_implementation_status() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    for text in [
        "B1 implementation status",
        "B1 initial implementation is now present:",
        "sh scripts/base1-x86_64-detect.sh --dry-run",
        "cargo test -p phase1 --test base1_x86_64_detect_script",
        "B1 limitations are documented in [`B1_READ_ONLY_DETECTION_LIMITATIONS.md`](B1_READ_ONLY_DETECTION_LIMITATIONS.md).",
        "cargo test -p phase1 --test b1_read_only_detection_limitations_docs",
        "B1 validation expectations are documented in [`B1_READ_ONLY_DETECTION_VALIDATION.md`](B1_READ_ONLY_DETECTION_VALIDATION.md).",
        "cargo test -p phase1 --test b1_read_only_detection_validation_docs",
        "B1 detector must stay inside the read-only, dry-run, non-mutating scope",
    ] {
        assert!(status.contains(text), "missing B1 implementation status text {text}: {status}");
    }
}

#[test]
fn boot_readiness_status_preserves_b1_completion_checklist() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    for text in [
        "B1 completion checklist",
        "B1 implementation plan exists.",
        "B1 read-only detector script exists.",
        "B1 detector script tests exist.",
        "B1 known limitations are documented.",
        "B1 limitations tests exist.",
        "B1 validation report exists.",
        "B1 validation report tests exist.",
        "B1 detector test suite passes in CI or local validation.",
        "B1 status is linked from README, OS roadmap, race plan, and x86_64 roadmap after implementation.",
        "B1 output is reviewed for secret redaction.",
        "B1 does not contain mutating boot, disk, package, or network commands.",
    ] {
        assert!(status.contains(text), "missing B1 completion text {text}: {status}");
    }
}

#[test]
fn boot_readiness_status_defines_b2_implementation_status() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    for text in [
        "B2 implementation status",
        "B2 dry-run assembly initial implementation is now present:",
        "sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation",
        "cargo test -p phase1 --test b2_dry_run_assembly_plan_docs",
        "cargo test -p phase1 --test base1_b2_assembly_dry_run_script",
        "B2 limitations are documented in [`B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md`](B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md).",
        "cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs",
        "B2 validation expectations are documented in [`B2_DRY_RUN_ASSEMBLY_VALIDATION.md`](B2_DRY_RUN_ASSEMBLY_VALIDATION.md).",
        "cargo test -p phase1 --test b2_dry_run_assembly_validation_docs",
        "B2 output review is documented in [`B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md`](B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md).",
        "cargo test -p phase1 --test b2_dry_run_assembly_output_review_docs",
        "B2 focused test-suite command bundle is documented in [`B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md`](B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md).",
        "cargo test -p phase1 --test b2_dry_run_assembly_test_suite_docs",
        "B2 status and boundaries are linked from README, OS roadmap, race plan, and x86_64 roadmap.",
        "B2 remains dry-run-only until validation has passed locally or in CI.",
    ] {
        assert!(status.contains(text), "missing B2 implementation status text {text}: {status}");
    }
}

#[test]
fn boot_readiness_status_preserves_b2_completion_checklist() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    for text in [
        "B2 completion checklist",
        "B2 dry-run assembly plan exists.",
        "B2 plan tests exist.",
        "B2 dry-run assembly script exists.",
        "B2 script tests exist.",
        "B2 known limitations are documented.",
        "B2 limitations tests exist.",
        "B2 validation report exists.",
        "B2 validation report tests exist.",
        "B2 status is linked from README, OS roadmap, race plan, and x86_64 roadmap.",
        "B2 does not contain mutating boot, disk, package, or network commands.",
        "B2 output is reviewed for secret redaction.",
        "B2 output review tests exist.",
        "B2 focused test-suite command bundle exists.",
        "B2 test-suite bundle tests exist.",
        "B2 test suite passes in CI or local validation.",
    ] {
        assert!(status.contains(text), "missing B2 completion text {text}: {status}");
    }
}

#[test]
fn boot_readiness_status_defines_first_coding_slice() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    for text in [
        "sh scripts/base1-x86_64-detect.sh --dry-run",
        "B1_READ_ONLY_DETECTION_PLAN.md",
        "B1_READ_ONLY_DETECTION_LIMITATIONS.md",
        "B1_READ_ONLY_DETECTION_VALIDATION.md",
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
        "B1_READ_ONLY_DETECTION_PLAN.md",
        "B1_READ_ONLY_DETECTION_LIMITATIONS.md",
        "B1_READ_ONLY_DETECTION_VALIDATION.md",
        "tests/b1_read_only_detection_plan_docs.rs",
        "tests/b1_read_only_detection_limitations_docs.rs",
        "tests/b1_read_only_detection_validation_docs.rs",
        "scripts/base1-x86_64-detect.sh",
        "tests/base1_x86_64_detect_script.rs",
        "B2_DRY_RUN_ASSEMBLY_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md",
        "B2_DRY_RUN_ASSEMBLY_VALIDATION.md",
        "B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md",
        "B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md",
        "tests/b2_dry_run_assembly_plan_docs.rs",
        "tests/b2_dry_run_assembly_limitations_docs.rs",
        "tests/b2_dry_run_assembly_validation_docs.rs",
        "tests/b2_dry_run_assembly_output_review_docs.rs",
        "tests/b2_dry_run_assembly_test_suite_docs.rs",
        "scripts/base1-b2-assembly-dry-run.sh",
        "tests/base1_b2_assembly_dry_run_script.rs",
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
        "first B1 read-only detection script exists",
        "detection-preview behavior only",
        "B2 has an initial dry-run assembly script but remains dry-run preview only",
    ] {
        assert!(status.contains(text), "missing non-claim {text}: {status}");
    }
}
