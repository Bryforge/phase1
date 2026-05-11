#[test]
fn os_roadmap_defines_os_track_and_boundaries() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");

    assert!(roadmap.contains("Phase1 operating-system track"), "{roadmap}");
    assert!(
        roadmap.contains("does not mean the current Phase1 Rust console is already a kernel"),
        "{roadmap}"
    );
    assert!(
        roadmap.contains("The current project remains a terminal-first virtual OS console."),
        "{roadmap}"
    );
}

#[test]
fn os_roadmap_tracks_current_b1_and_b2_status() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");

    for text in [
        "Current status: B1 read-only detection has an initial script and tests.",
        "B2 dry-run assembly has an initial script, tests, limitations, validation report, output review, and focused test-suite command bundle",
        "remains dry-run preview only until the focused B2 test suite passes locally or in CI",
        "BOOT_READINESS_STATUS.md",
    ] {
        assert!(roadmap.contains(text), "missing status text {text}: {roadmap}");
    }
}

#[test]
fn os_roadmap_links_b1_documents_and_commands() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");

    for text in [
        "B1_READ_ONLY_DETECTION_PLAN.md",
        "sh scripts/base1-x86_64-detect.sh --dry-run",
        "cargo test -p phase1 --test base1_x86_64_detect_script",
    ] {
        assert!(roadmap.contains(text), "missing B1 roadmap text {text}: {roadmap}");
    }
}

#[test]
fn os_roadmap_links_b2_review_set_and_commands() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");

    for text in [
        "B2_DRY_RUN_ASSEMBLY_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md",
        "B2_DRY_RUN_ASSEMBLY_VALIDATION.md",
        "B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md",
        "B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md",
        "sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation",
    ] {
        assert!(roadmap.contains(text), "missing B2 review/command text {text}: {roadmap}");
    }
}

#[test]
fn os_roadmap_lists_focused_b2_tests() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");

    for command in [
        "cargo test -p phase1 --test b2_dry_run_assembly_plan_docs",
        "cargo test -p phase1 --test base1_b2_assembly_dry_run_script",
        "cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs",
        "cargo test -p phase1 --test b2_dry_run_assembly_validation_docs",
        "cargo test -p phase1 --test b2_dry_run_assembly_output_review_docs",
        "cargo test -p phase1 --test b2_dry_run_assembly_test_suite_docs",
    ] {
        assert!(roadmap.contains(command), "missing focused B2 test {command}: {roadmap}");
    }
}

#[test]
fn os_roadmap_preserves_readiness_ladder_and_guardrails() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");

    for text in [
        "B0",
        "B1",
        "B2",
        "B3",
        "B4",
        "B5",
        "B6",
        "Do not skip levels when strengthening boot, hardware, recovery, installer, daily-driver, or hardened-status claims.",
        "Keep safe mode default-on.",
        "Keep host tool execution guarded.",
        "Keep destructive/admin actions explicit.",
        "Keep recovery available.",
        "Keep security claims conservative until validated.",
        "Keep hardening claims evidence-bound.",
    ] {
        assert!(roadmap.contains(text), "missing ladder/guardrail text {text}: {roadmap}");
    }
}

#[test]
fn os_roadmap_updates_first_engineering_slices_for_b2_review_work() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");

    for text in [
        "Add the [`Base1 B2 dry-run assembly plan`](B2_DRY_RUN_ASSEMBLY_PLAN.md).",
        "Add the B2 dry-run assembly script: `scripts/base1-b2-assembly-dry-run.sh`.",
        "Add B2 dry-run assembly tests: `tests/base1_b2_assembly_dry_run_script.rs`.",
        "Add B2 limitations, validation, output review, and test-suite docs.",
        "Add the [`Base1 x86_64 boot support roadmap`](X86_64_BOOT_SUPPORT_ROADMAP.md).",
    ] {
        assert!(roadmap.contains(text), "missing first engineering slice {text}: {roadmap}");
    }
}

#[test]
fn os_roadmap_preserves_non_claims() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");

    for text in [
        "does not make Phase1 or Base1 a finished OS",
        "hardened system",
        "installer-ready system",
        "hardware-validated system",
        "recovery-complete system",
        "daily-driver replacement",
    ] {
        assert!(roadmap.contains(text), "missing non-claim text {text}: {roadmap}");
    }
}
