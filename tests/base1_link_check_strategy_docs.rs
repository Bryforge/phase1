#[test]
fn link_check_strategy_defines_required_surfaces() {
    let doc = std::fs::read_to_string("docs/base1/LINK_CHECK_STRATEGY.md")
        .expect("Base1 link-check strategy");

    assert!(doc.contains("Base1 link-check strategy"), "{doc}");

    for surface in [
        "base1/*.md",
        "docs/base1/*.md",
        "docs/base1/releases/*.md",
        "docs/base1/real-device/*.md",
        "docs/base1/real-device/reports/*.md",
        "docs/os/BASE1_*.md",
        "root-level Base1 release/checkpoint notes",
        "README links pointing into Base1 docs",
    ] {
        assert!(doc.contains(surface), "missing link surface {surface}: {doc}");
    }
}

#[test]
fn link_check_strategy_preserves_compatibility_paths() {
    let doc = std::fs::read_to_string("docs/base1/LINK_CHECK_STRATEGY.md")
        .expect("Base1 link-check strategy");

    for text in [
        "Root compatibility path",
        "Organized mirror path",
        "Canonical design path",
        "Manual index path",
        "RELEASE_BASE1_LIBREBOOT_READONLY_V1.md",
        "docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md",
        "base1/RECOVERY_USB_DESIGN.md",
        "docs/base1/README.md",
    ] {
        assert!(doc.contains(text), "missing compatibility text {text}: {doc}");
    }
}

#[test]
fn link_check_strategy_defines_future_checker_behavior() {
    let doc = std::fs::read_to_string("docs/base1/LINK_CHECK_STRATEGY.md")
        .expect("Base1 link-check strategy");

    for text in [
        "Parse Markdown links with relative paths.",
        "Ignore external URLs by default",
        "Resolve relative links from the source file directory.",
        "Fail on missing local targets.",
        "Report the source file and missing target.",
        "Stay read-only.",
        "Run in CI without network access.",
        "sh scripts/base1-link-check.sh",
    ] {
        assert!(doc.contains(text), "missing checker behavior {text}: {doc}");
    }
}

#[test]
fn link_check_strategy_is_linked_from_indexes_and_integrity_gate() {
    let manual = std::fs::read_to_string("docs/base1/README.md").expect("Base1 manual");
    let map = std::fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md")
        .expect("Base1 documentation map");
    let integrity = std::fs::read_to_string("scripts/base1-doc-integrity.sh")
        .expect("Base1 integrity gate");

    for doc in [&manual, &map, &integrity] {
        assert!(doc.contains("LINK_CHECK_STRATEGY.md"), "{doc}");
    }
}

#[test]
fn link_check_strategy_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/base1/LINK_CHECK_STRATEGY.md")
        .expect("Base1 link-check strategy");

    assert!(doc.contains("does not make Base1 installer-ready"), "{doc}");
    assert!(doc.contains("hardware-validated"), "{doc}");
    assert!(doc.contains("daily-driver ready"), "{doc}");
}
