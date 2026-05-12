#[test]
fn b2_validation_report_defines_scope_and_command() {
    let report = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_VALIDATION.md")
        .expect("B2 dry-run assembly validation report");

    assert!(
        report.contains("Base1 B2 dry-run assembly validation"),
        "{report}"
    );
    assert!(
        report.contains("validation evidence and review checklist for `scripts/base1-b2-assembly-dry-run.sh --dry-run --profile <profile>`"),
        "{report}"
    );
    assert!(
        report.contains(
            "sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation"
        ),
        "{report}"
    );
    assert!(
        report.contains("It does not claim boot readiness."),
        "{report}"
    );
}

#[test]
fn b2_validation_report_lists_test_commands() {
    let report = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_VALIDATION.md")
        .expect("B2 dry-run assembly validation report");

    for command in [
        "cargo test -p phase1 --test base1_b2_assembly_dry_run_script",
        "cargo test -p phase1 --test b2_dry_run_assembly_plan_docs",
        "cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs",
        "cargo test -p phase1 --test boot_readiness_status_docs",
        "cargo test -p phase1 --test boot_readiness_race_plan_docs",
        "cargo test -p phase1 --test x86_64_boot_support_roadmap_docs",
    ] {
        assert!(
            report.contains(command),
            "missing validation command {command}: {report}"
        );
    }
}

#[test]
fn b2_validation_report_preserves_validation_status_table() {
    let report = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_VALIDATION.md")
        .expect("B2 dry-run assembly validation report");

    for text in [
        "Script exists",
        "Script requires `--dry-run`",
        "Script requires `--profile`",
        "Script rejects unsupported profiles",
        "Script accepts planned profiles",
        "Script reports `writes: no`",
        "Script reports B2 preview sections",
        "Script avoids known mutating command patterns",
        "B2 plan exists",
        "B2 limitations exist",
        "B2 status tracker updated",
        "CI/local test result",
        "Pending",
    ] {
        assert!(
            report.contains(text),
            "missing validation status text {text}: {report}"
        );
    }
}

#[test]
fn b2_validation_report_preserves_source_review_checklist() {
    let report = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_VALIDATION.md")
        .expect("B2 dry-run assembly validation report");

    for text in [
        "requires `--dry-run`",
        "requires `--profile`",
        "rejects unsupported profiles",
        "prints `writes: no`",
        "prints `mutation: no`",
        "prints `network: no`",
        "does not write images",
        "does not call mutating boot-loader commands",
        "does not call mutating partition/disk commands",
        "does not install packages",
        "does not require network access",
        "does not write to `/boot`, `/etc`, EFI variables, initramfs files, or partitions",
        "reports unknown or unvalidated facts instead of guessing",
        "keeps bootability, installer readiness, recovery completion, hardening, VM validation, hardware validation, and release-candidate claims explicitly unclaimed",
    ] {
        assert!(report.contains(text), "missing source review text {text}: {report}");
    }
}

#[test]
fn b2_validation_report_preserves_output_review_checklist() {
    let report = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_VALIDATION.md")
        .expect("B2 dry-run assembly validation report");

    for text in [
        "status: B2 dry-run assembly preview",
        "boot_readiness_level: B2",
        "writes: no",
        "mutation: no",
        "network: no",
        "selected profile",
        "B1 detection summary",
        "profile assumptions",
        "image-builder preview",
        "boot handoff preview",
        "installer preview",
        "recovery preview",
        "rollback preview",
        "validation bundle preview",
        "known limitations",
        "next validation step",
    ] {
        assert!(
            report.contains(text),
            "missing output review text {text}: {report}"
        );
    }
}

#[test]
fn b2_validation_report_defines_completion_requirements() {
    let report = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_VALIDATION.md")
        .expect("B2 dry-run assembly validation report");

    for text in [
        "B2 script tests pass in CI or local validation",
        "source review confirms no mutation paths",
        "output review confirms no secret leakage",
        "README, OS roadmap, race plan, x86_64 roadmap, and status tracker all reflect the implemented B2 boundary",
        "non-claims remain intact",
    ] {
        assert!(report.contains(text), "missing completion requirement {text}: {report}");
    }
}

#[test]
fn b2_validation_report_links_related_docs() {
    let report = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_VALIDATION.md")
        .expect("B2 dry-run assembly validation report");

    for link in [
        "BOOT_READINESS_STATUS.md",
        "BOOT_READINESS_RACE_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_PLAN.md",
        "B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md",
        "X86_64_BOOT_SUPPORT_ROADMAP.md",
    ] {
        assert!(
            report.contains(link),
            "missing related doc link {link}: {report}"
        );
    }
}

#[test]
fn b2_validation_report_preserves_non_claims() {
    let report = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_VALIDATION.md")
        .expect("B2 dry-run assembly validation report");

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
        assert!(report.contains(text), "missing non-claim {text}: {report}");
    }
}
