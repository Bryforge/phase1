use std::fs;

#[test]
fn phase1_codex_docs_exist() {
    for path in [
        "docs/README.md",
        "docs/MANUAL_ROADMAP.md",
        "docs/phase1/README.md",
        "docs/phase1/OPERATOR_MANUAL.md",
        "docs/base1/README.md",
        "docs/base1/FOUNDATION_MANUAL.md",
        "docs/fyr/README.md",
        "docs/security/README.md",
        "docs/security/TRUST_MODEL.md",
        "docs/security/DOCS_CLAIMS.md",
    ] {
        assert!(fs::metadata(path).is_ok(), "missing documentation file: {path}");
    }
}

#[test]
fn phase1_codex_title_is_present() {
    let roadmap = fs::read_to_string("docs/MANUAL_ROADMAP.md")
        .expect("manual roadmap should be readable");

    assert!(roadmap.contains("The Phase1 Codex"));
    assert!(roadmap.contains("Building a Terminal-First Operating World"));
}
