use std::fs;

#[test]
fn base1_validation_reports_index_exists() {
    assert!(
        fs::metadata("docs/base1/validation/README.md").is_ok(),
        "missing Base1 validation reports index"
    );
}

#[test]
fn base1_index_links_validation_reports_archive() {
    let index = fs::read_to_string("docs/base1/README.md")
        .expect("Base1 docs index should be readable");

    assert!(index.contains("validation/README.md"));
    assert!(index.contains("Store future Base1 reports"));
}

#[test]
fn validation_reports_index_links_template_and_matrix() {
    let index = fs::read_to_string("docs/base1/validation/README.md")
        .expect("Base1 validation reports index should be readable");

    assert!(index.contains("../VALIDATION_REPORT_TEMPLATE.md"));
    assert!(index.contains("../READINESS_MATRIX.md"));
}

#[test]
fn validation_reports_index_preserves_required_fields() {
    let index = fs::read_to_string("docs/base1/validation/README.md")
        .expect("Base1 validation reports index should be readable");

    for expected in [
        "report metadata",
        "evidence level",
        "target summary",
        "commands or checks run",
        "result",
        "observations",
        "evidence links",
        "boundaries and non-claims",
        "promotion recommendation",
        "follow-up work",
    ] {
        assert!(index.contains(expected), "missing report field: {expected}");
    }
}

#[test]
fn validation_reports_index_preserves_non_claims_and_ladder() {
    let index = fs::read_to_string("docs/base1/validation/README.md")
        .expect("Base1 validation reports index should be readable");

    for expected in [
        "does not claim Base1 is bootable",
        "daily-driver ready",
        "installer-ready",
        "recovery-complete",
        "validated on real hardware",
        "Roadmap -> Design -> Dry-run -> Preview -> Validated",
    ] {
        assert!(index.contains(expected), "missing guardrail: {expected}");
    }
}
