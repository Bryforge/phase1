use std::fs;

#[test]
fn base1_validation_runbook_exists() {
    assert!(
        fs::metadata("docs/base1/VALIDATION_RUNBOOK.md").is_ok(),
        "missing Base1 validation runbook"
    );
}

#[test]
fn base1_index_links_validation_runbook() {
    let index = fs::read_to_string("docs/base1/README.md")
        .expect("Base1 docs index should be readable");

    assert!(index.contains("VALIDATION_RUNBOOK.md"));
    assert!(index.contains("Runbook rule"));
}

#[test]
fn base1_validation_runbook_lists_docs_only_checks() {
    let runbook = fs::read_to_string("docs/base1/VALIDATION_RUNBOOK.md")
        .expect("Base1 validation runbook should be readable");

    for expected in [
        "base1_readiness_matrix_docs",
        "base1_validation_report_template_docs",
        "base1_validation_reports_index_docs",
        "base1_docs_evidence_chain_report_docs",
    ] {
        assert!(runbook.contains(expected), "missing docs check: {expected}");
    }
}

#[test]
fn base1_validation_runbook_preserves_documentation_only_scope() {
    let runbook = fs::read_to_string("docs/base1/VALIDATION_RUNBOOK.md")
        .expect("Base1 validation runbook should be readable");

    for expected in [
        "Documentation-only runbook",
        "documentation checks only",
        "What this does not verify",
        "does not verify",
        "bootable Base1 image",
        "hardware validation",
        "recovery completion",
        "installer readiness",
        "rollback execution",
        "image creation",
        "persistence behavior",
        "daily-driver readiness",
    ] {
        assert!(runbook.contains(expected), "missing runbook boundary: {expected}");
    }
}
