#[test]
fn high_security_profile_defines_purpose_and_operator() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/HIGH_SECURITY.md")
        .expect("high-security crypto profile");

    assert!(profile.contains("High-security cryptographic profile"), "{profile}");
    assert!(profile.contains("planned stricter cryptographic profile"), "{profile}");

    for operator in [
        "Advanced operators",
        "Security reviewers",
        "Base1 recovery and image-provenance operators",
        "Maintainers preparing signed releases",
    ] {
        assert!(profile.contains(operator), "missing intended operator {operator}: {profile}");
    }
}

#[test]
fn high_security_profile_lists_stricter_control_points() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/HIGH_SECURITY.md")
        .expect("high-security crypto profile");

    for control_point in [
        "storage",
        "identity",
        "base1",
        "logs/evidence",
        "plugins",
        "fyr/packages",
        "transport",
    ] {
        assert!(
            profile.contains(control_point),
            "missing high-security control point {control_point}: {profile}"
        );
    }
}

#[test]
fn high_security_profile_requires_registry_template_and_eligible_statuses() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/HIGH_SECURITY.md")
        .expect("high-security crypto profile");

    assert!(profile.contains("../CRYPTO_REGISTRY.md"), "{profile}");
    assert!(profile.contains("../CRYPTO_ALGORITHM_TEMPLATE.md"), "{profile}");

    for status in [
        "test-vector-covered",
        "profile-eligible",
        "default-eligible",
    ] {
        assert!(profile.contains(status), "missing required status {status}: {profile}");
    }
}

#[test]
fn high_security_profile_rejects_unsafe_or_ambiguous_entries() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/HIGH_SECURITY.md")
        .expect("high-security crypto profile");

    for rejected in [
        "custom security-critical primitives",
        "undocumented registry entries",
        "deprecated entries",
        "compatibility-only entries",
        "lab-only entries",
        "algorithms without test-vector coverage",
        "silent downgrade behavior",
        "operational recovery path is undocumented",
    ] {
        assert!(profile.contains(rejected), "missing rejection {rejected}: {profile}");
    }
}

#[test]
fn high_security_profile_guards_downgrades_and_audit() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/HIGH_SECURITY.md")
        .expect("high-security crypto profile");

    for text in [
        "warning",
        "affected control-point display",
        "explicit confirmation",
        "audit logging",
        "rollback guidance",
        "reason field",
        "active profile",
        "verification status",
    ] {
        assert!(profile.contains(text), "missing downgrade/audit text {text}: {profile}");
    }
}

#[test]
fn high_security_profile_defines_base1_requirements() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/HIGH_SECURITY.md")
        .expect("high-security crypto profile");

    for text in [
        "Base1 requirements",
        "image provenance requirements",
        "recovery media verification requirements",
        "rollback metadata integrity requirements",
        "signed validation report requirements",
        "operator recovery if keys or signatures fail",
        "boot security and hardware validation",
    ] {
        assert!(profile.contains(text), "missing Base1 requirement {text}: {profile}");
    }
}

#[test]
fn high_security_profile_is_linked_from_profiles_index() {
    let index = std::fs::read_to_string("docs/security/crypto-profiles/README.md")
        .expect("crypto profiles index");

    assert!(index.contains("HIGH_SECURITY.md"), "{index}");
    assert!(index.contains("high-security"), "{index}");
}

#[test]
fn high_security_profile_preserves_non_claims() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/HIGH_SECURITY.md")
        .expect("high-security crypto profile");

    assert!(profile.contains("does not make Phase1 or Base1 cryptographically complete"), "{profile}");
    assert!(profile.contains("audited"), "{profile}");
    assert!(profile.contains("certified"), "{profile}");
    assert!(profile.contains("quantum-safe"), "{profile}");
    assert!(profile.contains("hardware-validated"), "{profile}");
    assert!(profile.contains("daily-driver ready"), "{profile}");
}
