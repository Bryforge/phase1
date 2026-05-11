#[test]
fn b2_test_suite_defines_scope_and_primary_command() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md")
        .expect("B2 dry-run assembly test suite bundle");

    assert!(doc.contains("Base1 B2 dry-run assembly test suite"), "{doc}");
    assert!(
        doc.contains("focused B2 test commands for dry-run assembly planning"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation"),
        "{doc}"
    );
    assert!(
        doc.contains("It does not claim that the tests have passed."),
        "{doc}"
    );
}

#[test]
fn b2_test_suite_lists_expected_command_boundary() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md")
        .expect("B2 dry-run assembly test suite bundle");

    for text in [
        "dry-run only",
        "requires `--dry-run`",
        "requires `--profile`",
        "accepts only documented profiles",
        "reports `writes: no`",
        "reports `mutation: no`",
        "reports `network: no`",
        "does not claim bootability, installer readiness, recovery completion, hardening, VM validation, hardware validation, release-candidate readiness, or daily-driver readiness",
    ] {
        assert!(doc.contains(text), "missing command boundary text {text}: {doc}");
    }
}

#[test]
fn b2_test_suite_lists_focused_b2_tests() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md")
        .expect("B2 dry-run assembly test suite bundle");

    for command in [
        "cargo test -p phase1 --test b2_dry_run_assembly_plan_docs",
        "cargo test -p phase1 --test base1_b2_assembly_dry_run_script",
        "cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs",
        "cargo test -p phase1 --test b2_dry_run_assembly_validation_docs",
        "cargo test -p phase1 --test b2_dry_run_assembly_output_review_docs",
        "cargo test -p phase1 --test boot_readiness_status_docs",
        "cargo test -p phase1 --test boot_readiness_race_plan_docs",
        "cargo test -p phase1 --test x86_64_boot_support_roadmap_docs",
        "cargo test -p phase1 --test readme_navigation_reorganization_links",
    ] {
        assert!(doc.contains(command), "missing focused B2 test command {command}: {doc}");
    }
}

#[test]
fn b2_test_suite_lists_optional_broader_checks() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md")
        .expect("B2 dry-run assembly test suite bundle");

    for command in [
        "sh scripts/quality-check.sh quick",
        "sh scripts/quality-check.sh base1-docs",
    ] {
        assert!(doc.contains(command), "missing optional broader check {command}: {doc}");
    }
}

#[test]
fn b2_test_suite_preserves_pass_criteria() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md")
        .expect("B2 dry-run assembly test suite bundle");

    for text in [
        "every focused B2 test command exits successfully",
        "the B2 dry-run assembly script syntax passes",
        "the B2 dry-run assembly script requires `--dry-run`",
        "the B2 dry-run assembly script requires a supported profile",
        "the B2 dry-run assembly script reports `writes: no`",
        "B2 docs preserve limitations and non-claims",
        "README, OS roadmap, race plan, x86_64 roadmap, and status tracker reflect the B2 boundary",
        "no mutating boot, disk, package, or network command pattern appears in the B2 script",
    ] {
        assert!(doc.contains(text), "missing pass criterion {text}: {doc}");
    }
}

#[test]
fn b2_test_suite_preserves_current_result_status() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md")
        .expect("B2 dry-run assembly test suite bundle");

    assert!(
        doc.contains("Current result: **not recorded in this document**."),
        "{doc}"
    );
    assert!(
        doc.contains("Do not treat missing status checks as a pass."),
        "{doc}"
    );
}

#[test]
fn b2_test_suite_links_related_docs() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md")
        .expect("B2 dry-run assembly test suite bundle");

    for link in [
        "BOOT_READINESS_STATUS.md",
        "BOOT_READINESS_RACE_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md",
        "B2_DRY_RUN_ASSEMBLY_VALIDATION.md",
        "B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md",
        "X86_64_BOOT_SUPPORT_ROADMAP.md",
    ] {
        assert!(doc.contains(link), "missing related doc link {link}: {doc}");
    }
}

#[test]
fn b2_test_suite_is_linked_from_status_tracker() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    assert!(
        status.contains("B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md"),
        "{status}"
    );
    assert!(
        status.contains("B2 focused test-suite command bundle exists."),
        "{status}"
    );
}

#[test]
fn b2_test_suite_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md")
        .expect("B2 dry-run assembly test suite bundle");

    for text in [
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "VM-validated",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(doc.contains(text), "missing non-claim {text}: {doc}");
    }
}
