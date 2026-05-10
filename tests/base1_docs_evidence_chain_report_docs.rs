use std::fs;

const REPORT: &str = "docs/base1/validation/2026-05-10-docs-evidence-chain.md";

#[test]
fn base1_docs_evidence_chain_report_exists() {
    assert!(
        fs::metadata(REPORT).is_ok(),
        "missing Base1 docs evidence-chain report"
    );
}

#[test]
fn validation_index_links_docs_evidence_chain_report() {
    let index = fs::read_to_string("docs/base1/validation/README.md")
        .expect("Base1 validation reports index should be readable");

    assert!(index.contains("2026-05-10-docs-evidence-chain.md"));
    assert!(index.contains("documentation-only Base1 evidence-chain report"));
}

#[test]
fn base1_docs_evidence_chain_report_preserves_documentation_only_scope() {
    let report =
        fs::read_to_string(REPORT).expect("Base1 docs evidence-chain report should be readable");

    for expected in [
        "Documentation-only validation report",
        "documentation files and docs tests only",
        "Base1 documentation governance",
        "This report records documentation structure only",
        "Stay at current level",
    ] {
        assert!(
            report.contains(expected),
            "missing documentation-only marker: {expected}"
        );
    }
}

#[test]
fn base1_docs_evidence_chain_report_preserves_non_claims() {
    let report =
        fs::read_to_string(REPORT).expect("Base1 docs evidence-chain report should be readable");

    for expected in [
        "does not claim a bootable Base1 image",
        "hardware validation",
        "recovery completion",
        "installer readiness",
        "daily-driver readiness",
        "does not validate",
        "boot behavior",
        "image creation",
        "installer behavior",
        "hardware behavior",
        "rollback execution",
        "recovery execution",
        "persistence behavior",
    ] {
        assert!(report.contains(expected), "missing non-claim: {expected}");
    }
}

#[test]
fn base1_docs_evidence_chain_report_links_expected_surfaces() {
    let report =
        fs::read_to_string(REPORT).expect("Base1 docs evidence-chain report should be readable");

    for expected in [
        "docs/base1/READINESS_MATRIX.md",
        "docs/base1/VALIDATION_REPORT_TEMPLATE.md",
        "docs/base1/validation/README.md",
        "tests/base1_readiness_matrix_docs.rs",
        "tests/base1_validation_report_template_docs.rs",
        "tests/base1_validation_reports_index_docs.rs",
    ] {
        assert!(
            report.contains(expected),
            "missing evidence link: {expected}"
        );
    }
}
