#[test]
fn crypto_policy_roadmap_defines_operator_selectable_profiles() {
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    assert!(roadmap.contains("Phase1 cryptographic policy roadmap"), "{roadmap}");
    assert!(
        roadmap.contains("as secure as possible while maintaining practical usability"),
        "{roadmap}"
    );

    for profile in [
        "safe-default",
        "high-security",
        "compatibility",
        "post-quantum-preview",
        "lab-only",
    ] {
        assert!(roadmap.contains(profile), "missing crypto profile {profile}: {roadmap}");
    }
}

#[test]
fn crypto_policy_roadmap_defines_points_of_control() {
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    for control in [
        "Storage",
        "Transport",
        "Identity",
        "Boot/Base1",
        "Plugins",
        "Logs/evidence",
        "Fyr/packages",
    ] {
        assert!(roadmap.contains(control), "missing control point {control}: {roadmap}");
    }
}

#[test]
fn crypto_policy_roadmap_rejects_custom_security_primitives() {
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    assert!(
        roadmap.contains("Phase1 should not invent new cryptographic primitives for security-critical use."),
        "{roadmap}"
    );
    assert!(
        roadmap.contains("reviewed open-source libraries and established algorithms"),
        "{roadmap}"
    );
    assert!(
        roadmap.contains("Educational examples may exist only when clearly labeled as lab-only"),
        "{roadmap}"
    );
}

#[test]
fn crypto_policy_roadmap_documents_registry_and_algorithm_requirements() {
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    for text in [
        "Cryptographic capability registry",
        "algorithm or design name",
        "implementation provider",
        "allowed use cases",
        "disallowed use cases",
        "test vectors",
        "migration guidance",
        "Documentation requirement",
        "Parameters and limits",
        "Security notes",
        "Usability notes",
    ] {
        assert!(roadmap.contains(text), "missing registry/doc requirement {text}: {roadmap}");
    }
}

#[test]
fn crypto_policy_roadmap_preserves_non_claims() {
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    assert!(roadmap.contains("does not make Phase1 or Base1 cryptographically complete"), "{roadmap}");
    assert!(roadmap.contains("audited"), "{roadmap}");
    assert!(roadmap.contains("quantum-safe"), "{roadmap}");
    assert!(roadmap.contains("hardware-validated"), "{roadmap}");
    assert!(roadmap.contains("daily-driver ready"), "{roadmap}");
}

#[test]
fn security_index_links_crypto_policy_roadmap() {
    let index = std::fs::read_to_string("docs/security/README.md")
        .expect("security docs index");

    assert!(index.contains("CRYPTO_POLICY_ROADMAP.md"), "{index}");
    assert!(
        index.contains("cryptographic policy docs"),
        "{index}"
    );
    assert!(
        index.contains("cryptographic completeness"),
        "{index}"
    );
}
