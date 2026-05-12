#[test]
fn b1_validation_report_defines_scope_and_command() {
    let report = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_VALIDATION.md")
        .expect("B1 read-only detection validation report");

    assert!(
        report.contains("Base1 B1 read-only detection validation"),
        "{report}"
    );
    assert!(
        report.contains("validation evidence and review checklist for `scripts/base1-x86_64-detect.sh --dry-run`"),
        "{report}"
    );
    assert!(
        report.contains("sh scripts/base1-x86_64-detect.sh --dry-run"),
        "{report}"
    );
    assert!(
        report.contains("It does not claim boot readiness."),
        "{report}"
    );
}

#[test]
fn b1_validation_report_lists_test_commands() {
    let report = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_VALIDATION.md")
        .expect("B1 read-only detection validation report");

    for command in [
        "cargo test -p phase1 --test base1_x86_64_detect_script",
        "cargo test -p phase1 --test b1_read_only_detection_plan_docs",
        "cargo test -p phase1 --test b1_read_only_detection_limitations_docs",
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
fn b1_validation_report_preserves_validation_status_table() {
    let report = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_VALIDATION.md")
        .expect("B1 read-only detection validation report");

    for text in [
        "Script exists",
        "Script requires `--dry-run`",
        "Script reports `writes: no`",
        "Script reports core sections",
        "Script avoids known mutating command patterns",
        "B1 plan exists",
        "B1 limitations exist",
        "B1 status tracker updated",
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
fn b1_validation_report_preserves_source_review_checklist() {
    let report = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_VALIDATION.md")
        .expect("B1 read-only detection validation report");

    for text in [
        "requires `--dry-run`",
        "prints `writes: no`",
        "does not call mutating boot-loader commands",
        "does not call mutating partition/disk commands",
        "does not install packages",
        "does not require network access",
        "does not write to `/boot`, `/etc`, EFI variables, initramfs files, or partitions",
        "redacts sensitive-looking kernel command-line values",
        "reports unknown values instead of guessing",
        "exits non-zero when `--dry-run` is missing",
    ] {
        assert!(
            report.contains(text),
            "missing source review text {text}: {report}"
        );
    }
}

#[test]
fn b1_validation_report_preserves_output_review_checklist() {
    let report = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_VALIDATION.md")
        .expect("B1 read-only detection validation report");

    for text in [
        "status: B1 read-only detection preview",
        "writes: no",
        "mutation: no",
        "network: no",
        "architecture hints",
        "firmware hints",
        "boot-loader hints",
        "kernel command-line availability with conservative redaction",
        "virtualization hints",
        "storage-layout hints",
        "recovery hints",
        "unknown fields",
        "next read-only check",
    ] {
        assert!(
            report.contains(text),
            "missing output review text {text}: {report}"
        );
    }
}

#[test]
fn b1_validation_report_defines_completion_requirements() {
    let report = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_VALIDATION.md")
        .expect("B1 read-only detection validation report");

    for text in [
        "detector tests pass in CI or local validation",
        "source review confirms no mutation paths",
        "output review confirms no secret leakage",
        "README, OS roadmap, race plan, x86_64 roadmap, and status tracker all reflect the implemented B1 boundary",
        "non-claims remain intact",
    ] {
        assert!(report.contains(text), "missing completion requirement {text}: {report}");
    }
}

#[test]
fn b1_validation_report_links_related_docs() {
    let report = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_VALIDATION.md")
        .expect("B1 read-only detection validation report");

    for link in [
        "BOOT_READINESS_STATUS.md",
        "BOOT_READINESS_RACE_PLAN.md",
        "B1_READ_ONLY_DETECTION_PLAN.md",
        "B1_READ_ONLY_DETECTION_LIMITATIONS.md",
        "X86_64_BOOT_SUPPORT_ROADMAP.md",
    ] {
        assert!(
            report.contains(link),
            "missing related doc link {link}: {report}"
        );
    }
}

#[test]
fn b1_validation_report_preserves_non_claims() {
    let report = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_VALIDATION.md")
        .expect("B1 read-only detection validation report");

    for text in [
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(report.contains(text), "missing non-claim {text}: {report}");
    }
}
