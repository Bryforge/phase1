#[test]
fn crypto_profiles_index_defines_profile_classes() {
    let profiles = std::fs::read_to_string("docs/security/crypto-profiles/README.md")
        .expect("crypto profiles index");

    assert!(profiles.contains("Phase1 cryptographic profiles"), "{profiles}");
    assert!(
        profiles.contains("as secure as possible while maintaining practical usability"),
        "{profiles}"
    );

    for profile in [
        "safe-default",
        "high-security",
        "compatibility",
        "post-quantum-preview",
        "lab-only",
    ] {
        assert!(profiles.contains(profile), "missing crypto profile {profile}: {profiles}");
    }
}

#[test]
fn crypto_profiles_index_requires_profile_documentation_fields() {
    let profiles = std::fs::read_to_string("docs/security/crypto-profiles/README.md")
        .expect("crypto profiles index");

    for field in [
        "intended operator",
        "allowed control points",
        "default algorithms or registry entries",
        "unavailable or rejected algorithms",
        "downgrade and compatibility warnings",
        "confirmation requirements",
        "audit/logging expectations",
        "migration and rollback guidance",
        "test and review requirements",
        "non-claims",
    ] {
        assert!(profiles.contains(field), "missing profile field {field}: {profiles}");
    }
}

#[test]
fn crypto_profiles_index_preserves_profile_safety_rules() {
    let profiles = std::fs::read_to_string("docs/security/crypto-profiles/README.md")
        .expect("crypto profiles index");

    for rule in [
        "Safe defaults should require the least operator decision-making.",
        "Advanced profiles should require explicit selection.",
        "Compatibility profiles should warn before weaker choices are used.",
        "Lab-only profiles must not protect real data or production workflows.",
        "No profile may claim audit completion, certification, quantum safety, or production hardening without evidence.",
        "No profile may use registry entries that lack documentation through `docs/security/CRYPTO_ALGORITHM_TEMPLATE.md`.",
    ] {
        assert!(profiles.contains(rule), "missing profile safety rule {rule}: {profiles}");
    }
}

#[test]
fn crypto_profiles_index_links_related_crypto_docs() {
    let profiles = std::fs::read_to_string("docs/security/crypto-profiles/README.md")
        .expect("crypto profiles index");

    for link in [
        "../CRYPTO_POLICY_ROADMAP.md",
        "../CRYPTO_REGISTRY.md",
        "../CRYPTO_ALGORITHM_TEMPLATE.md",
        "../TRUST_MODEL.md",
    ] {
        assert!(profiles.contains(link), "missing related doc {link}: {profiles}");
    }
}

#[test]
fn crypto_profiles_index_is_linked_from_security_index_and_roadmap() {
    let index = std::fs::read_to_string("docs/security/README.md")
        .expect("security docs index");
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    assert!(index.contains("crypto-profiles/README.md"), "{index}");
    assert!(roadmap.contains("crypto-profiles/README.md"), "{roadmap}");
}

#[test]
fn crypto_profiles_index_preserves_non_claims() {
    let profiles = std::fs::read_to_string("docs/security/crypto-profiles/README.md")
        .expect("crypto profiles index");

    assert!(profiles.contains("does not make Phase1 or Base1 cryptographically complete"), "{profiles}");
    assert!(profiles.contains("audited"), "{profiles}");
    assert!(profiles.contains("certified"), "{profiles}");
    assert!(profiles.contains("quantum-safe"), "{profiles}");
    assert!(profiles.contains("hardware-validated"), "{profiles}");
    assert!(profiles.contains("daily-driver ready"), "{profiles}");
}
