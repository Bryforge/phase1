#[test]
fn crypto_registry_defines_profile_classes_and_control_points() {
    let registry =
        std::fs::read_to_string("docs/security/CRYPTO_REGISTRY.md").expect("crypto registry");

    assert!(
        registry.contains("Phase1 cryptographic capability registry"),
        "{registry}"
    );

    for profile in [
        "safe-default",
        "high-security",
        "compatibility",
        "post-quantum-preview",
        "lab-only",
    ] {
        assert!(
            registry.contains(profile),
            "missing profile {profile}: {registry}"
        );
    }

    for control_point in [
        "storage",
        "transport",
        "identity",
        "base1",
        "plugins",
        "logs/evidence",
        "fyr/packages",
    ] {
        assert!(
            registry.contains(control_point),
            "missing control point {control_point}: {registry}"
        );
    }
}

#[test]
fn crypto_registry_requires_algorithm_template_and_review_fields() {
    let registry =
        std::fs::read_to_string("docs/security/CRYPTO_REGISTRY.md").expect("crypto registry");

    assert!(
        registry.contains("CRYPTO_ALGORITHM_TEMPLATE.md"),
        "{registry}"
    );

    for field in [
        "allowed use cases",
        "disallowed use cases",
        "implementation provider",
        "parameters and limits",
        "test vectors",
        "migration guidance",
        "usability impact",
        "non-claims",
    ] {
        assert!(
            registry.contains(field),
            "missing required field {field}: {registry}"
        );
    }
}

#[test]
fn crypto_registry_lists_algorithm_families() {
    let registry =
        std::fs::read_to_string("docs/security/CRYPTO_REGISTRY.md").expect("crypto registry");

    for family in [
        "Entropy and random generation",
        "Cryptographic hashes",
        "Message authentication",
        "Key derivation and password hashing",
        "Symmetric encryption and AEAD",
        "Public-key signatures",
        "Public-key encryption and KEMs",
        "Key agreement",
        "Post-quantum cryptography",
        "Threshold and multisignature designs",
        "Authenticated data structures",
        "Non-cryptographic checksums",
    ] {
        assert!(
            registry.contains(family),
            "missing algorithm family {family}: {registry}"
        );
    }
}

#[test]
fn crypto_registry_defines_entry_status_labels() {
    let registry =
        std::fs::read_to_string("docs/security/CRYPTO_REGISTRY.md").expect("crypto registry");

    for status in [
        "candidate",
        "documented",
        "test-vector-covered",
        "profile-eligible",
        "default-eligible",
        "compatibility-only",
        "deprecated",
        "rejected",
        "lab-only",
    ] {
        assert!(
            registry.contains(status),
            "missing status {status}: {registry}"
        );
    }
}

#[test]
fn crypto_registry_preserves_initial_non_claims() {
    let registry =
        std::fs::read_to_string("docs/security/CRYPTO_REGISTRY.md").expect("crypto registry");

    assert!(
        registry.contains("No algorithm is currently approved by this registry for new production security claims."),
        "{registry}"
    );
    assert!(
        registry.contains("does not make Phase1 or Base1 cryptographically complete"),
        "{registry}"
    );
    assert!(registry.contains("audited"), "{registry}");
    assert!(registry.contains("quantum-safe"), "{registry}");
    assert!(registry.contains("hardware-validated"), "{registry}");
    assert!(registry.contains("daily-driver ready"), "{registry}");
}

#[test]
fn crypto_registry_is_linked_from_security_index_and_roadmap() {
    let index = std::fs::read_to_string("docs/security/README.md").expect("security docs index");
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    assert!(index.contains("CRYPTO_REGISTRY.md"), "{index}");
    assert!(roadmap.contains("CRYPTO_REGISTRY.md"), "{roadmap}");
}
