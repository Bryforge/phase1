#[test]
fn security_crypto_doc_integrity_gate_is_read_only() {
    let script = std::fs::read_to_string("scripts/security-crypto-doc-integrity.sh")
        .expect("security crypto docs integrity script");

    assert!(
        script.contains("Phase1 security crypto documentation integrity gate"),
        "{script}"
    );
    assert!(script.contains("This check is read-only"), "{script}");
    assert!(script.contains("writes: no"), "{script}");
}

#[test]
fn security_crypto_doc_integrity_gate_checks_required_docs() {
    let script = std::fs::read_to_string("scripts/security-crypto-doc-integrity.sh")
        .expect("security crypto docs integrity script");

    for path in [
        "SECURITY.md",
        "SECURITY_REVIEW.md",
        "README.md",
        "docs/security/README.md",
        "docs/security/TRUST_MODEL.md",
        "docs/security/CRYPTO_POLICY_ROADMAP.md",
        "docs/security/CRYPTO_REGISTRY.md",
        "docs/security/CRYPTO_ALGORITHM_TEMPLATE.md",
        "docs/security/CRYPTO_OPERATOR_COMMANDS.md",
        "docs/security/crypto-profiles/README.md",
        "docs/security/crypto-profiles/SAFE_DEFAULT.md",
        "docs/security/crypto-profiles/HIGH_SECURITY.md",
        "docs/security/crypto-profiles/COMPATIBILITY.md",
        "docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md",
        "docs/security/crypto-profiles/LAB_ONLY.md",
    ] {
        assert!(script.contains(path), "missing required path {path}: {script}");
    }
}

#[test]
fn security_crypto_doc_integrity_gate_checks_links_and_guardrails() {
    let script = std::fs::read_to_string("scripts/security-crypto-doc-integrity.sh")
        .expect("security crypto docs integrity script");

    for text in [
        "Cryptographic policy goal",
        "Security and usability principle",
        "CRYPTO_POLICY_ROADMAP.md",
        "CRYPTO_REGISTRY.md",
        "CRYPTO_OPERATOR_COMMANDS.md",
        "crypto-profiles/README.md",
        "CRYPTO_ALGORITHM_TEMPLATE.md",
        "Phase1 should not invent new cryptographic primitives for security-critical use.",
        "No algorithm is currently approved by this registry for new production security claims.",
        "Does not invent a custom security-critical primitive.",
        "unsupported claims of quantum safety",
        "must not be used as a real security profile",
    ] {
        assert!(script.contains(text), "missing guardrail text {text}: {script}");
    }
}

#[test]
fn security_crypto_doc_integrity_gate_checks_operator_command_plan() {
    let script = std::fs::read_to_string("scripts/security-crypto-doc-integrity.sh")
        .expect("security crypto docs integrity script");

    for text in [
        "check_operator_commands()",
        "crypto status",
        "crypto profiles",
        "crypto explain <profile-or-algorithm>",
        "crypto select <profile> --scope <control-point> --confirm",
        "crypto policy export",
        "crypto policy verify",
        "Unknown scopes should fail closed.",
        "Unknown profiles should fail closed.",
        "Lab-only selections must fail outside `lab`, `docs`, or `tests` scopes.",
        "deprecated, rejected, or lab-only entries are not used in production scopes",
    ] {
        assert!(script.contains(text), "missing operator command check {text}: {script}");
    }
}

#[test]
fn security_crypto_doc_integrity_gate_checks_profile_non_claims() {
    let script = std::fs::read_to_string("scripts/security-crypto-doc-integrity.sh")
        .expect("security crypto docs integrity script");

    for text in [
        "does not make Phase1 or Base1 cryptographically complete",
        "audited",
        "quantum-safe",
        "hardware-validated",
        "daily-driver ready",
    ] {
        assert!(script.contains(text), "missing non-claim text {text}: {script}");
    }
}

#[test]
fn quality_check_runs_security_crypto_docs_gate() {
    let quality = std::fs::read_to_string("scripts/quality-check.sh")
        .expect("quality-check script");

    assert!(
        quality.contains("check_security_crypto_docs()"),
        "{quality}"
    );
    assert!(
        quality.contains("run sh scripts/security-crypto-doc-integrity.sh"),
        "{quality}"
    );
    assert!(quality.contains("security-crypto-docs"), "{quality}");
    assert!(quality.contains("crypto-docs"), "{quality}");
}
