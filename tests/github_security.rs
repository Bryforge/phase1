use std::fs;

#[test]
fn dependabot_config_targets_required_ecosystems() {
    let content = fs::read_to_string(".github/dependabot.yml").expect("read dependabot config");
    assert!(content.contains("version: 2"));
    assert!(content.contains("package-ecosystem: \"cargo\""));
    assert!(content.contains("package-ecosystem: \"github-actions\""));
    assert!(content.contains("open-pull-requests-limit"));
    assert!(content.contains("rust-patch-and-minor"));
    assert!(content.contains("github-actions-security"));
}

#[test]
fn workflows_have_explicit_permissions_blocks() {
    for workflow in [
        ".github/workflows/rust.yml",
        ".github/workflows/rust-ci.yml",
        ".github/workflows/security.yml",
        ".github/workflows/quality.yml",
    ] {
        let content = fs::read_to_string(workflow).expect("read workflow file");
        assert!(
            content.contains("permissions:"),
            "workflow is missing explicit permissions block: {workflow}"
        );
        assert!(
            content.contains("contents: read"),
            "workflow is missing read-only contents permission: {workflow}"
        );
    }
}

#[test]
fn codeql_workflow_keeps_required_security_permissions() {
    let content = fs::read_to_string(".github/workflows/codeql.yml").expect("read CodeQL workflow");
    assert!(content.contains("permissions:"));
    assert!(content.contains("security-events: write"));
    assert!(content.contains("contents: read"));
    assert!(content.contains("actions: read"));
}
