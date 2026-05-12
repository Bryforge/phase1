#[test]
fn developer_docs_link_repository_contribution_guidelines() {
    let doc = std::fs::read_to_string("docs/developers/README.md").expect("developer reader path");

    assert!(doc.contains("../../CONTRIBUTING.md"), "{doc}");
    assert!(
        doc.contains("repository-wide contribution guidelines"),
        "{doc}"
    );
}

#[test]
fn developer_docs_define_development_reading_order() {
    let doc = std::fs::read_to_string("docs/developers/README.md").expect("developer reader path");

    for link in [
        "../../CONTRIBUTING.md",
        "../README.md",
        "../MANUAL_ROADMAP.md",
        "DOCS_CONTRIBUTING.md",
        "PR_CHECKLIST.md",
        "../security/DOCS_CLAIMS.md",
        "../security/TRUST_MODEL.md",
    ] {
        assert!(
            doc.contains(link),
            "missing developer reading link {link}: {doc}"
        );
    }
}

#[test]
fn developer_docs_include_contribution_gates() {
    let doc = std::fs::read_to_string("docs/developers/README.md").expect("developer reader path");

    for command in [
        "cargo fmt --all -- --check",
        "cargo check --all-targets",
        "cargo test --all-targets",
        "sh scripts/quality-check.sh quick",
        "sh scripts/quality-check.sh base1-docs",
        "sh scripts/quality-check.sh security-crypto-docs",
    ] {
        assert!(
            doc.contains(command),
            "missing developer gate {command}: {doc}"
        );
    }
}

#[test]
fn developer_docs_mark_crypto_as_safety_sensitive() {
    let doc = std::fs::read_to_string("docs/developers/README.md").expect("developer reader path");

    for text in [
        "cryptographic policy, providers, profiles, config, or runtime integration",
        "cryptographically complete",
        "audited",
        "certified",
        "quantum-safe",
        "hardware-validated",
    ] {
        assert!(
            doc.contains(text),
            "missing crypto safety-sensitive text {text}: {doc}"
        );
    }
}

#[test]
fn developer_docs_preserve_developer_rule() {
    let doc = std::fs::read_to_string("docs/developers/README.md").expect("developer reader path");

    assert!(
        doc.contains("Every implementation claim should have a runnable path, a test path, or a linked validation artifact."),
        "{doc}"
    );
    assert!(
        doc.contains("document the item as design, dry-run, preview, roadmap, or not claimed"),
        "{doc}"
    );
}
