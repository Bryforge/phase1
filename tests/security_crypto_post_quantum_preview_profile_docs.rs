#[test]
fn post_quantum_preview_profile_defines_purpose_and_operator() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md")
        .expect("post-quantum-preview crypto profile");

    assert!(
        profile.contains("Post-quantum-preview cryptographic profile"),
        "{profile}"
    );
    assert!(profile.contains("planned preview profile"), "{profile}");

    for operator in [
        "Advanced operators evaluating PQC readiness",
        "Security reviewers tracking future cryptographic migration",
        "Maintainers preparing long-term compatibility plans",
        "Researchers and developers testing PQC-capable workflows",
    ] {
        assert!(
            profile.contains(operator),
            "missing intended operator {operator}: {profile}"
        );
    }
}

#[test]
fn post_quantum_preview_profile_lists_preview_control_points() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md")
        .expect("post-quantum-preview crypto profile");

    for control_point in [
        "transport",
        "identity",
        "base1",
        "logs/evidence",
        "plugins",
        "fyr/packages",
        "Storage use should require special caution",
    ] {
        assert!(
            profile.contains(control_point),
            "missing post-quantum-preview control point {control_point}: {profile}"
        );
    }
}

#[test]
fn post_quantum_preview_profile_requires_registry_template_and_statuses() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md")
        .expect("post-quantum-preview crypto profile");

    assert!(profile.contains("../CRYPTO_REGISTRY.md"), "{profile}");
    assert!(
        profile.contains("../CRYPTO_ALGORITHM_TEMPLATE.md"),
        "{profile}"
    );

    for status in ["documented", "test-vector-covered", "profile-eligible"] {
        assert!(
            profile.contains(status),
            "missing required status {status}: {profile}"
        );
    }
}

#[test]
fn post_quantum_preview_profile_requires_preview_warnings() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md")
        .expect("post-quantum-preview crypto profile");

    for text in [
        "preview maturity warning",
        "ecosystem compatibility warning",
        "recovery and rollback warning",
        "whether a hybrid mode is being used",
        "safer default recommendation",
        "new data, keys, signatures, or evidence",
    ] {
        assert!(
            profile.contains(text),
            "missing preview warning {text}: {profile}"
        );
    }
}

#[test]
fn post_quantum_preview_profile_rejects_unsafe_claims_and_silent_migration() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md")
        .expect("post-quantum-preview crypto profile");

    for rejected in [
        "custom security-critical primitives",
        "undocumented registry entries",
        "lab-only entries presented as protection",
        "unsupported claims of quantum safety",
        "algorithms without test-vector coverage",
        "silent migration of existing keys, signatures, or encrypted data",
        "critical Base1 paths without recovery documentation",
    ] {
        assert!(
            profile.contains(rejected),
            "missing rejection {rejected}: {profile}"
        );
    }
}

#[test]
fn post_quantum_preview_profile_defines_base1_preview_requirements() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md")
        .expect("post-quantum-preview crypto profile");

    for text in [
        "Base1 requirements",
        "preview-only until stronger evidence exists",
        "image provenance compatibility",
        "recovery media verification compatibility",
        "rollback metadata verification compatibility",
        "signed validation report compatibility",
        "operator recovery if a verifier cannot validate the selected scheme",
        "quantum safety, boot security, and hardware validation",
    ] {
        assert!(
            profile.contains(text),
            "missing Base1 PQC requirement {text}: {profile}"
        );
    }
}

#[test]
fn post_quantum_preview_profile_is_linked_from_profiles_index() {
    let index = std::fs::read_to_string("docs/security/crypto-profiles/README.md")
        .expect("crypto profiles index");

    assert!(index.contains("POST_QUANTUM_PREVIEW.md"), "{index}");
    assert!(index.contains("post-quantum-preview"), "{index}");
}

#[test]
fn post_quantum_preview_profile_preserves_non_claims() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/POST_QUANTUM_PREVIEW.md")
        .expect("post-quantum-preview crypto profile");

    assert!(
        profile.contains("does not make Phase1 or Base1 cryptographically complete"),
        "{profile}"
    );
    assert!(profile.contains("audited"), "{profile}");
    assert!(profile.contains("certified"), "{profile}");
    assert!(profile.contains("quantum-safe"), "{profile}");
    assert!(profile.contains("hardware-validated"), "{profile}");
    assert!(profile.contains("daily-driver ready"), "{profile}");
}
