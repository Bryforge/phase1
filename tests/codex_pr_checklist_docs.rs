use std::fs;

#[test]
fn codex_pr_checklist_exists() {
    assert!(
        fs::metadata("docs/developers/PR_CHECKLIST.md").is_ok(),
        "missing Codex PR checklist"
    );
}

#[test]
fn codex_pr_checklist_contains_required_sections() {
    let checklist = fs::read_to_string("docs/developers/PR_CHECKLIST.md")
        .expect("PR checklist should be readable");

    for expected in [
        "Required checks",
        "Useful commands",
        "Review handoff",
        "status block",
        "docs tests",
    ] {
        assert!(checklist.contains(expected), "missing checklist content: {expected}");
    }
}

#[test]
fn developer_index_links_pr_checklist() {
    let index = fs::read_to_string("docs/developers/README.md")
        .expect("developer index should be readable");

    assert!(index.contains("PR_CHECKLIST.md"));
}
