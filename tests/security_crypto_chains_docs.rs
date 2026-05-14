use std::fs;

const CHAINS_DOC: &str = "docs/security/CRYPTO_CHAINS.md";

#[test]
fn crypto_chains_doc_exists_and_defines_scope() {
    let doc = fs::read_to_string(CHAINS_DOC).expect("CRYPTO_CHAINS.md should exist");

    assert!(doc.contains("# Crypto Chains"));
    assert!(doc.contains("Status: planning"));
    assert!(doc.contains("floors"));
    assert!(doc.contains("nests"));
    assert!(doc.contains("portals"));
    assert!(doc.contains("future service contexts"));
}

#[test]
fn crypto_chains_doc_defines_chain_relationships() {
    let doc = fs::read_to_string(CHAINS_DOC).expect("CRYPTO_CHAINS.md should exist");

    assert!(doc.contains("parent context"));
    assert!(doc.contains("child context"));
    assert!(doc.contains("selected crypto profile"));
    assert!(doc.contains("selected crypto provider"));
    assert!(doc.contains("selected crypto service family"));
    assert!(doc.contains("allowed data scope"));
    assert!(doc.contains("denied data scope"));
}

#[test]
fn crypto_chains_doc_preserves_guardrails_and_non_claims() {
    let doc = fs::read_to_string(CHAINS_DOC).expect("CRYPTO_CHAINS.md should exist");

    assert!(doc.contains("planning records only"));
    assert!(doc.contains("completed cryptographic isolation"));
    assert!(doc.contains("hardware-backed secrecy"));
    assert!(doc.contains("formal sandboxing"));
    assert!(doc.contains("production-grade key management"));
    assert!(doc.contains("certified cryptographic compliance"));
    assert!(doc.contains("post-quantum security"));
}

#[test]
fn crypto_chains_doc_defines_fail_closed_behavior() {
    let doc = fs::read_to_string(CHAINS_DOC).expect("CRYPTO_CHAINS.md should exist");

    assert!(doc.contains("fail-closed"));
    assert!(doc.contains("deny the operation"));
    assert!(doc.contains("silently falling back"));
}

#[test]
fn crypto_chains_doc_links_related_crypto_docs() {
    let doc = fs::read_to_string(CHAINS_DOC).expect("CRYPTO_CHAINS.md should exist");

    assert!(doc.contains("CRYPTO_POLICY_ROADMAP.md"));
    assert!(doc.contains("CRYPTO_PROVIDER_REGISTRY.md"));
    assert!(doc.contains("CRYPTO_PROVIDER_SERVICE_MATRIX.md"));
}
