#[test]
fn docs_index_links_contribution_entry_points() {
    let doc = std::fs::read_to_string("docs/README.md").expect("docs index");

    for link in [
        "../CONTRIBUTING.md",
        "developers/README.md",
        "developers/DOCS_CONTRIBUTING.md",
        "developers/PR_CHECKLIST.md",
        "../.github/pull_request_template.md",
    ] {
        assert!(
            doc.contains(link),
            "missing contribution link {link}: {doc}"
        );
    }
}

#[test]
fn docs_index_lists_contributor_and_community_paths() {
    let doc = std::fs::read_to_string("docs/README.md").expect("docs index");

    for text in [
        "Contributor",
        "Prepare repository contributions while preserving quality, safety, compatibility, and non-claims.",
        "Community/support contributor",
        "community/README.md",
        "Community support and forum planning.",
    ] {
        assert!(doc.contains(text), "missing contributor/community path {text}: {doc}");
    }
}

#[test]
fn docs_index_preserves_status_and_safety_language() {
    let doc = std::fs::read_to_string("docs/README.md").expect("docs index");

    for text in [
        "Status boundary",
        "Phase1 is not currently a finished secure OS replacement",
        "Base1 is not currently a released bootable daily-driver image",
        "Fyr is not currently claimed as a production language",
        "Required page status block",
        "Canonical safety language",
        "Use narrow, testable statements.",
    ] {
        assert!(
            doc.contains(text),
            "missing status/safety text {text}: {doc}"
        );
    }
}
