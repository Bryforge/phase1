use std::fs;

#[test]
fn codex_template_docs_exist() {
    for path in [
        "docs/templates/README.md",
        "docs/templates/STATUS_BLOCKS.md",
        "docs/templates/PAGE_SKELETONS.md",
        "docs/templates/CLAIM_REVIEW_EXAMPLES.md",
    ] {
        assert!(fs::metadata(path).is_ok(), "missing Codex template doc: {path}");
    }
}

#[test]
fn docs_index_links_codex_templates() {
    let index = fs::read_to_string("docs/README.md")
        .expect("docs index should be readable");

    assert!(index.contains("templates/README.md"));
    assert!(index.contains("templates/"));
}

#[test]
fn template_index_links_core_templates() {
    let index = fs::read_to_string("docs/templates/README.md")
        .expect("templates index should be readable");

    for expected in [
        "STATUS_BLOCKS.md",
        "PAGE_SKELETONS.md",
        "CLAIM_REVIEW_EXAMPLES.md",
    ] {
        assert!(index.contains(expected), "templates index missing {expected}");
    }
}

#[test]
fn status_blocks_include_required_statuses() {
    let status_blocks = fs::read_to_string("docs/templates/STATUS_BLOCKS.md")
        .expect("status block templates should be readable");

    for expected in [
        "Implemented",
        "Experimental",
        "Design",
        "Dry-run",
        "Preview",
        "Roadmap",
        "Not claimed",
    ] {
        assert!(status_blocks.contains(expected), "missing status template: {expected}");
    }
}
