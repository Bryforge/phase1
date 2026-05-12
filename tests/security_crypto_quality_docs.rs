#[test]
fn quality_docs_describe_security_crypto_docs_gate() {
    let quality = std::fs::read_to_string("QUALITY.md").expect("QUALITY.md");

    assert!(quality.contains("Security crypto docs"), "{quality}");
    assert!(
        quality.contains("sh scripts/quality-check.sh security-crypto-docs"),
        "{quality}"
    );
    assert!(
        quality.contains("sh scripts/quality-check.sh crypto-docs"),
        "{quality}"
    );
    assert!(
        quality.contains("sh scripts/security-crypto-doc-integrity.sh"),
        "{quality}"
    );
}

#[test]
fn quality_docs_list_security_crypto_required_script() {
    let quality = std::fs::read_to_string("QUALITY.md").expect("QUALITY.md");

    assert!(
        quality.contains("scripts/security-crypto-doc-integrity.sh"),
        "{quality}"
    );
    assert!(quality.contains("docs/security/CRYPTO_*"), "{quality}");
    assert!(
        quality.contains("docs/security/crypto-profiles/"),
        "{quality}"
    );
}

#[test]
fn quality_docs_define_security_crypto_gate_scope() {
    let quality = std::fs::read_to_string("QUALITY.md").expect("QUALITY.md");

    for text in [
        "Cryptographic policy work is documentation-first until implementation, tests, review, and validation exist.",
        "docs/security/CRYPTO_POLICY_ROADMAP.md",
        "docs/security/CRYPTO_REGISTRY.md",
        "docs/security/CRYPTO_ALGORITHM_TEMPLATE.md",
        "docs/security/CRYPTO_OPERATOR_COMMANDS.md",
        "docs/security/CRYPTO_CONFIG_SCHEMA.md",
        "docs/security/crypto-profiles/README.md",
        "all current profile drafts are present",
        "profile drafts link the registry and algorithm template",
        "operator command plans fail closed for unknown scopes and profiles",
        "config schema fails closed for unknown scopes and profiles",
        "config schema blocks `lab-only` outside lab/docs/tests scopes",
        "config schema requires reasons for `compatibility` and `post-quantum-preview`",
        "lab-only profile use is blocked outside lab/docs/tests scopes",
        "deprecated, rejected, or lab-only entries are blocked from production scopes",
        "custom security-critical primitives are rejected",
        "lab-only behavior is not presented as real security protection",
        "unsupported quantum-safety claims are blocked",
        "crypto non-claims are preserved",
    ] {
        assert!(quality.contains(text), "missing crypto quality scope {text}: {quality}");
    }
}

#[test]
fn quality_docs_preserve_crypto_safety_baseline() {
    let quality = std::fs::read_to_string("QUALITY.md").expect("QUALITY.md");

    assert!(
        quality.contains("cryptographic policy work remains documentation-first until implementation, tests, review, and validation exist"),
        "{quality}"
    );
    assert!(
        quality.contains(
            "custom security-critical primitives are not accepted as production protection"
        ),
        "{quality}"
    );
    assert!(
        quality.contains(
            "cryptographic operator commands fail closed for unknown scopes and profiles"
        ),
        "{quality}"
    );
    assert!(
        quality.contains("cryptographic config fails closed for unknown scopes and profiles"),
        "{quality}"
    );
}
