use std::fs;

#[test]
fn root_readme_links_white_arts_public_entry_points() {
    let readme = fs::read_to_string("README.md").expect("read root README");

    for required in [
        "White Arts defensive-care track",
        "docs/white-arts/README.md",
        "docs/white-arts/PROTOCOLS_AND_GUARDRAILS.md",
        "docs/white-arts/OPEN_SECURITY_SERVER_SUITE.md",
        "docs/white-arts/TODO.md",
        "read-only first",
        "evidence-bound",
        "non-claiming",
    ] {
        assert!(readme.contains(required), "missing {required:?}");
    }
}

#[test]
fn docs_indexes_link_white_arts_navigation() {
    let docs = fs::read_to_string("docs/README.md").expect("read docs index");
    let nav =
        fs::read_to_string("docs/REPOSITORY_NAVIGATION.md").expect("read repository navigation");

    for text in [&docs, &nav] {
        for required in [
            "White Arts",
            "white-arts/README.md",
            "OPEN_SECURITY_SERVER_SUITE.md",
        ] {
            assert!(text.contains(required), "missing {required:?}");
        }
    }
}

#[test]
fn white_arts_index_and_roadmap_link_server_suite() {
    let index = fs::read_to_string("docs/white-arts/README.md").expect("read White Arts index");
    let roadmap =
        fs::read_to_string("docs/white-arts/ROADMAP.md").expect("read White Arts roadmap");

    for text in [&index, &roadmap] {
        assert!(
            text.contains("OPEN_SECURITY_SERVER_SUITE.md"),
            "missing server suite link"
        );
    }
}
