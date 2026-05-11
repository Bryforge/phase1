#[test]
fn repository_navigation_guide_defines_purpose_and_scope() {
    let nav = std::fs::read_to_string("docs/REPOSITORY_NAVIGATION.md")
        .expect("repository navigation guide");

    assert!(nav.contains("Phase1 repository navigation guide"), "{nav}");
    assert!(
        nav.contains("repository organization, reader paths, contribution paths, support paths, and validation paths"),
        "{nav}"
    );

    for workstream in [
        "Phase1 terminal-first virtual OS console",
        "Base1 OS foundation and recovery planning",
        "Fyr native language track",
        "security and crypto policy planning",
        "community support and future support AI planning",
    ] {
        assert!(nav.contains(workstream), "missing workstream {workstream}: {nav}");
    }
}

#[test]
fn repository_navigation_guide_lists_fast_paths() {
    let nav = std::fs::read_to_string("docs/REPOSITORY_NAVIGATION.md")
        .expect("repository navigation guide");

    for link in [
        "../README.md",
        "../FEATURE_STATUS.md",
        "REORGANIZATION_PLAN.md",
        "../CONTRIBUTING.md",
        "../.github/pull_request_template.md",
        "../.github/ISSUE_TEMPLATE/bug_report.yml",
        "../.github/ISSUE_TEMPLATE/support_request.yml",
        "../.github/ISSUE_TEMPLATE/feature_request.yml",
        "../.github/ISSUE_TEMPLATE/documentation_issue.yml",
        "security/README.md",
        "security/CRYPTO_POLICY_ROADMAP.md",
        "../QUALITY.md",
        "base1/README.md",
        "fyr/README.md",
        "community/README.md",
        "releases/README.md",
        "website/README.md",
        "../examples/README.md",
        "../tools/README.md",
    ] {
        assert!(nav.contains(link), "missing fast-path link {link}: {nav}");
    }
}

#[test]
fn repository_navigation_guide_maps_repository_paths() {
    let nav = std::fs::read_to_string("docs/REPOSITORY_NAVIGATION.md")
        .expect("repository navigation guide");

    for path in [
        "README.md",
        "CONTRIBUTING.md",
        "SECURITY.md",
        "QUALITY.md",
        "FEATURE_STATUS.md",
        "PHASE1_NATIVE_LANGUAGE.md",
        ".github/",
        "src/",
        "src/bin/",
        "phase1-core/",
        "base1/",
        "docs/",
        "docs/REPOSITORY_NAVIGATION.md",
        "docs/REORGANIZATION_PLAN.md",
        "docs/releases/",
        "docs/website/",
        "docs/security/",
        "docs/community/",
        "docs/base1/",
        "docs/fyr/",
        "examples/",
        "tools/",
        "scripts/",
        "tests/",
        "assets/",
    ] {
        assert!(nav.contains(path), "missing repository map path {path}: {nav}");
    }
}

#[test]
fn repository_navigation_guide_defines_reader_paths() {
    let nav = std::fs::read_to_string("docs/REPOSITORY_NAVIGATION.md")
        .expect("repository navigation guide");

    for reader in [
        "First-time user",
        "Contributor",
        "Repository organizer",
        "Release/documentation organizer",
        "Website/asset organizer",
        "Examples/tools organizer",
        "Security reviewer",
        "Crypto contributor",
        "Base1 contributor",
        "Fyr contributor",
        "Community/support contributor",
    ] {
        assert!(nav.contains(reader), "missing reader path {reader}: {nav}");
    }
}

#[test]
fn repository_navigation_guide_defines_destination_organizer_rules() {
    let nav = std::fs::read_to_string("docs/REPOSITORY_NAVIGATION.md")
        .expect("repository navigation guide");

    for text in [
        "Keep root-level release notes and checkpoint files as compatibility paths unless a move map approves a change.",
        "Prefer organized mirrors or indexes before moving release files.",
        "Keep website claims aligned with repository evidence.",
        "Keep branding assets under `assets/` unless a future asset map says otherwise.",
        "Keep user-facing scripts in `scripts/` unless a move map and compatibility wrapper exist.",
        "Prefer read-only or dry-run examples.",
    ] {
        assert!(nav.contains(text), "missing destination organizer rule {text}: {nav}");
    }
}

#[test]
fn repository_navigation_guide_defines_quality_gate_chooser() {
    let nav = std::fs::read_to_string("docs/REPOSITORY_NAVIGATION.md")
        .expect("repository navigation guide");

    for command in [
        "sh scripts/quality-check.sh quick",
        "sh scripts/quality-check.sh full",
        "sh scripts/quality-check.sh base1-docs",
        "sh scripts/quality-check.sh base1-reorg",
        "sh scripts/quality-check.sh security-crypto-docs",
        "sh scripts/quality-check.sh scripts",
        "sh scripts/quality-check.sh files",
        "sh scripts/quality-check.sh score",
    ] {
        assert!(nav.contains(command), "missing quality gate {command}: {nav}");
    }
}

#[test]
fn repository_navigation_guide_defines_issue_template_chooser() {
    let nav = std::fs::read_to_string("docs/REPOSITORY_NAVIGATION.md")
        .expect("repository navigation guide");

    for text in [
        "Reproducible defect",
        "Bug report",
        "Need help using the project",
        "Support request",
        "Proposed improvement",
        "Feature request",
        "Missing, confusing, outdated, or unsafe docs",
        "Documentation issue",
        "Private vulnerability or sensitive security report",
        "Do not open a public issue; follow `SECURITY.md`",
    ] {
        assert!(nav.contains(text), "missing issue chooser text {text}: {nav}");
    }
}

#[test]
fn repository_navigation_guide_preserves_reorganization_rules_and_non_claims() {
    let nav = std::fs::read_to_string("docs/REPOSITORY_NAVIGATION.md")
        .expect("repository navigation guide");

    for rule in [
        "Repository reorganization should be preservation-first.",
        "Keep existing public/root compatibility paths unless a future plan explicitly replaces them.",
        "Use [`REORGANIZATION_PLAN.md`](REORGANIZATION_PLAN.md) before broad restructuring.",
        "Prefer adding indexes and mirrors before moving files.",
        "Add navigation docs before broad restructuring.",
        "Add tests for new navigation and required links.",
        "Keep safety, non-claims, and validation paths visible.",
    ] {
        assert!(nav.contains(rule), "missing reorganization rule {rule}: {nav}");
    }

    assert!(nav.contains("does not move files"), "{nav}");
    assert!(nav.contains("does not"), "{nav}");
    assert!(nav.contains("make Phase1, Base1, or Fyr production-ready"), "{nav}");
}

#[test]
fn docs_index_links_repository_navigation_guide() {
    let index = std::fs::read_to_string("docs/README.md").expect("docs index");

    assert!(index.contains("REPOSITORY_NAVIGATION.md"), "{index}");
    assert!(index.contains("Fast navigation"), "{index}");
    assert!(index.contains("Repository navigator"), "{index}");
    assert!(index.contains("Support and issue entry points"), "{index}");
}
