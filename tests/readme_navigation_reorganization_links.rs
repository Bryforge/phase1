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
fn readme_links_current_public_asset_components() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    for asset in [
        "assets/phase1_base_fyr_banner1.png",
        "assets/phase1-splash.png",
        "assets/fyr_symbol.png",
        "assets/fyr_word.png",
    ] {
        assert!(readme.contains(asset), "README missing current asset component {asset}: {readme}");
    }

    assert!(
        !readme.contains("assets/phase1-splash.svg"),
        "README should not reference the outdated Phase1 splash SVG: {readme}"
    );
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
fn readme_links_boot_readiness_b1_docs_and_detector() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    for link in [
        "docs/os/BOOT_READINESS_STATUS.md",
        "docs/os/BOOT_READINESS_RACE_PLAN.md",
        "docs/os/B1_READ_ONLY_DETECTION_PLAN.md",
        "docs/os/B1_READ_ONLY_DETECTION_LIMITATIONS.md",
        "scripts/base1-x86_64-detect.sh --dry-run",
    ] {
        assert!(readme.contains(link), "README missing B1 boot-readiness link/text {link}: {readme}");
    }

    for text in [
        "B1 initial script present",
        "B1 detection preview",
        "next target is completing B1 validation before B2 dry-run assembly",
        "cargo test -p phase1 --test base1_x86_64_detect_script",
        "cargo test -p phase1 --test b1_read_only_detection_limitations_docs",
    ] {
        assert!(readme.contains(text), "README missing B1 status text {text}: {readme}");
    }
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
