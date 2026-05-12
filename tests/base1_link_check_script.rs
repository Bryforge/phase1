#[test]
fn base1_link_check_script_is_read_only_and_local_only() {
    let script =
        std::fs::read_to_string("scripts/base1-link-check.sh").expect("Base1 link-check script");

    assert!(
        script.contains("Base1 local Markdown link checker"),
        "{script}"
    );
    assert!(script.contains("mode: read-only"), "{script}");
    assert!(script.contains("external-links: skipped"), "{script}");
    assert!(script.contains("anchors: file-only check"), "{script}");
    assert!(script.contains("no host changes were made"), "{script}");
}

#[test]
fn base1_link_check_script_checks_required_surfaces() {
    let script =
        std::fs::read_to_string("scripts/base1-link-check.sh").expect("Base1 link-check script");

    for pattern in [
        "README.md",
        "base1/*.md",
        "docs/base1/*.md",
        "docs/base1/releases/*.md",
        "docs/base1/real-device/*.md",
        "docs/base1/real-device/reports/*.md",
        "docs/os/BASE1_*.md",
        "RELEASE_BASE1_*.md",
        "DEVELOPMENT_CHECKPOINT_BASE1_*.md",
    ] {
        assert!(
            script.contains(pattern),
            "missing checked surface {pattern}: {script}"
        );
    }
}

#[test]
fn base1_link_check_script_reports_missing_targets() {
    let script =
        std::fs::read_to_string("scripts/base1-link-check.sh").expect("Base1 link-check script");

    assert!(script.contains("missing local link target"), "{script}");
    assert!(script.contains("missing-targets"), "{script}");
    assert!(script.contains("exit 1"), "{script}");
}

#[test]
fn base1_link_check_script_skips_external_and_anchor_links() {
    let script =
        std::fs::read_to_string("scripts/base1-link-check.sh").expect("Base1 link-check script");

    for skipped in [
        "http://*",
        "https://*",
        "mailto:*",
        "tel:*",
        "ftp://*",
        "data:*",
        "javascript:*",
        "'#'*",
    ] {
        assert!(
            script.contains(skipped),
            "missing skipped pattern {skipped}: {script}"
        );
    }
}

#[test]
fn quality_gate_runs_base1_link_checker() {
    let quality =
        std::fs::read_to_string("scripts/quality-check.sh").expect("quality-check script");

    assert!(
        quality.contains("run sh scripts/base1-doc-integrity.sh"),
        "{quality}"
    );
    assert!(
        quality.contains("run sh scripts/base1-link-check.sh"),
        "{quality}"
    );
    assert!(quality.contains("base1-docs"), "{quality}");
}
