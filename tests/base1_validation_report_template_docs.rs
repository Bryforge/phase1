use std::fs;

#[test]
fn base1_validation_report_template_exists() {
    assert!(
        fs::metadata("docs/base1/VALIDATION_REPORT_TEMPLATE.md").is_ok(),
        "missing Base1 validation report template"
    );
}

#[test]
fn base1_index_links_validation_report_template() {
    let index = fs::read_to_string("docs/base1/README.md")
        .expect("Base1 docs index should be readable");

    assert!(index.contains("VALIDATION_REPORT_TEMPLATE.md"));
    assert!(index.contains("Report rule"));
}

#[test]
fn base1_validation_report_template_records_required_fields() {
    let template = fs::read_to_string("docs/base1/VALIDATION_REPORT_TEMPLATE.md")
        .expect("Base1 validation report template should be readable");

    for expected in [
        "Report metadata",
        "Evidence level",
        "Target summary",
        "Commands or checks run",
        "Result",
        "Observations",
        "Evidence links",
        "Boundaries and non-claims",
        "Promotion recommendation",
        "Follow-up work",
    ] {
        assert!(template.contains(expected), "missing template field: {expected}");
    }
}

#[test]
fn base1_validation_report_template_preserves_result_labels() {
    let template = fs::read_to_string("docs/base1/VALIDATION_REPORT_TEMPLATE.md")
        .expect("Base1 validation report template should be readable");

    for expected in ["pass", "pass-with-notes", "blocked", "failed", "not-run"] {
        assert!(template.contains(expected), "missing result label: {expected}");
    }
}

#[test]
fn base1_validation_report_template_preserves_promotion_ladder() {
    let template = fs::read_to_string("docs/base1/VALIDATION_REPORT_TEMPLATE.md")
        .expect("Base1 validation report template should be readable");

    for expected in [
        "Stay at current level",
        "roadmap to design",
        "design to dry-run",
        "dry-run to preview",
        "preview to validated",
    ] {
        assert!(template.contains(expected), "missing promotion option: {expected}");
    }
}
