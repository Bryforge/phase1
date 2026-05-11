#[test]
fn release_docs_index_defines_preservation_first_release_home() {
    let doc = std::fs::read_to_string("docs/releases/README.md")
        .expect("release docs index");

    assert!(doc.contains("Phase1 release documentation"), "{doc}");
    assert!(doc.contains("organized home for release documentation"), "{doc}");

    for text in [
        "Do not delete root-level release notes unless a future move map explicitly approves it.",
        "Prefer adding organized mirrors or indexes before moving release files.",
        "Keep old path -> new path mappings when files are mirrored or moved.",
        "phase1/",
        "base1/",
        "checkpoints/",
        "sh scripts/quality-check.sh quick",
        "sh scripts/quality-check.sh base1-docs",
    ] {
        assert!(doc.contains(text), "missing release docs index text {text}: {doc}");
    }
}

#[test]
fn website_docs_index_defines_public_site_home_and_claim_safety() {
    let doc = std::fs::read_to_string("docs/website/README.md")
        .expect("website docs index");

    assert!(doc.contains("Phase1 website documentation"), "{doc}");
    assert!(doc.contains("public website planning and documentation"), "{doc}");

    for text in [
        "Keep existing public links working where possible.",
        "Keep branding assets in `assets/` unless a future asset map says otherwise.",
        "Do not claim production readiness, hardened status, hardware validation, audit status, certification, or quantum safety without linked evidence.",
        "CONTENT_MAP.md",
        "BRANDING.md",
        "ACCESSIBILITY.md",
        "RELEASE_CHECKLIST.md",
        "sh scripts/test-website.sh",
    ] {
        assert!(doc.contains(text), "missing website docs index text {text}: {doc}");
    }
}

#[test]
fn examples_index_defines_safe_example_home() {
    let doc = std::fs::read_to_string("examples/README.md")
        .expect("examples index");

    assert!(doc.contains("Phase1 examples"), "{doc}");
    assert!(doc.contains("preferred organized home for examples"), "{doc}");

    for text in [
        "avoid destructive host commands by default",
        "prefer read-only or dry-run behavior",
        "avoid secrets, tokens, private keys, credentials, recovery codes, private logs, or unrevised screenshots",
        "phase1/",
        "fyr/",
        "base1/",
        "security/",
        "community/",
        "sh scripts/quality-check.sh quick",
        "sh scripts/quality-check.sh base1-docs",
        "sh scripts/quality-check.sh security-crypto-docs",
    ] {
        assert!(doc.contains(text), "missing examples index text {text}: {doc}");
    }
}

#[test]
fn tools_index_defines_internal_tooling_boundary() {
    let doc = std::fs::read_to_string("tools/README.md")
        .expect("tools index");

    assert!(doc.contains("Phase1 tools"), "{doc}");
    assert!(doc.contains("internal helper tooling"), "{doc}");

    for text in [
        "User-facing commands should generally stay in `scripts/`",
        "Internal maintainer utilities",
        "compatibility wrapper or clear replacement path",
        "When in doubt, keep user-facing scripts in `scripts/`.",
        "support-ai/",
        "repo-maintenance/",
        "avoid destructive behavior by default",
        "support dry-run or preview mode when mutation is possible",
        "sh scripts/quality-check.sh quick",
        "sh scripts/quality-check.sh scripts",
    ] {
        assert!(doc.contains(text), "missing tools index text {text}: {doc}");
    }
}

#[test]
fn reorganization_plan_mentions_destination_folders() {
    let plan = std::fs::read_to_string("docs/REORGANIZATION_PLAN.md")
        .expect("reorganization plan");

    for text in [
        "docs/releases/",
        "docs/website/",
        "examples/",
        "tools/",
        "Create only folders that are actively needed",
        "Do not create empty folders unless tooling requires placeholders.",
    ] {
        assert!(plan.contains(text), "missing destination folder plan text {text}: {plan}");
    }
}
