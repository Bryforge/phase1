#[test]
fn readme_links_core_navigation_and_reorganization_docs() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    for link in [
        "docs/REPOSITORY_NAVIGATION.md",
        "docs/REORGANIZATION_PLAN.md",
        "CONTRIBUTING.md",
        ".github/pull_request_template.md",
    ] {
        assert!(readme.contains(link), "README missing link {link}: {readme}");
    }
}

#[test]
fn readme_links_x86_64_and_hardening_roadmap() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md"),
        "README should link x86_64 boot support roadmap: {readme}"
    );
    assert!(
        readme.contains("hardened") || readme.contains("hardening"),
        "README should mention hardening as a roadmap goal without overclaiming: {readme}"
    );
}

#[test]
fn readme_links_organized_destinations() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    for link in [
        "docs/releases/README.md",
        "docs/website/README.md",
        "examples/README.md",
        "tools/README.md",
    ] {
        assert!(readme.contains(link), "README missing organized destination {link}: {readme}");
    }
}

#[test]
fn readme_links_current_support_templates() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    for link in [
        ".github/ISSUE_TEMPLATE/bug_report.yml",
        ".github/ISSUE_TEMPLATE/support_request.yml",
        ".github/ISSUE_TEMPLATE/feature_request.yml",
        ".github/ISSUE_TEMPLATE/documentation_issue.yml",
    ] {
        assert!(readme.contains(link), "README missing issue template link {link}: {readme}");
    }
}
