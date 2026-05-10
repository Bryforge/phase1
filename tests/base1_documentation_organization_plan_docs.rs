use std::fs;

#[test]
fn base1_documentation_organization_plan_exists() {
    let doc = fs::read_to_string("docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md").unwrap();
    assert!(doc.contains("Base1 Documentation Organization Plan"));
    assert!(doc.contains("Status: proposed organization plan"));
    assert!(doc.contains("without moving files in this PR"));
}

#[test]
fn base1_documentation_organization_plan_preserves_move_rules() {
    let doc = fs::read_to_string("docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md").unwrap();
    assert!(doc.contains("Do not move existing Base1 markdown files"));
    assert!(doc.contains("Move one document group per PR"));
    assert!(doc.contains("Keep or update every inbound link"));
    assert!(doc.contains("Add or update tests for every moved document"));
}

#[test]
fn base1_documentation_organization_plan_lists_groups() {
    let doc = fs::read_to_string("docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md").unwrap();
    assert!(doc.contains("### Core"));
    assert!(doc.contains("### Real-Device Read-Only"));
    assert!(doc.contains("### Future Candidate Folders"));
    assert!(doc.contains("real-device/reports/*.md"));
}

#[test]
fn base1_documentation_organization_plan_preserves_non_claims() {
    let doc = fs::read_to_string("docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md").unwrap();
    assert!(doc.contains("No file moves in this plan"));
    assert!(doc.contains("No runtime behavior change"));
    assert!(doc.contains("Not installer-ready"));
    assert!(doc.contains("Not hardware-validated"));
    assert!(doc.contains("No destructive disk writes"));
    assert!(doc.contains("No real-device write path"));
}

#[test]
fn base1_index_links_documentation_organization_plan() {
    let index = fs::read_to_string("docs/base1/README.md").unwrap_or_default();
    assert!(index.contains("DOCUMENTATION_ORGANIZATION_PLAN.md"));
}
