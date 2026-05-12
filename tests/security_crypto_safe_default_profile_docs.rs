#[test]
fn safe_default_profile_defines_purpose_and_operator() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/SAFE_DEFAULT.md")
        .expect("safe-default crypto profile");

    assert!(
        profile.contains("Safe-default cryptographic profile"),
        "{profile}"
    );
    assert!(
        profile.contains("planned default cryptographic profile"),
        "{profile}"
    );
    assert!(profile.contains("Normal operators"), "{profile}");
    assert!(profile.contains("First-time users"), "{profile}");
    assert!(profile.contains("Local development users"), "{profile}");
}

#[test]
fn safe_default_profile_lists_control_points() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/SAFE_DEFAULT.md")
        .expect("safe-default crypto profile");

    for control_point in [
        "storage",
        "transport",
        "identity",
        "plugins",
        "logs/evidence",
        "fyr/packages",
    ] {
        assert!(
            profile.contains(control_point),
            "missing control point {control_point}: {profile}"
        );
    }
}

#[test]
fn safe_default_profile_requires_registry_and_template() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/SAFE_DEFAULT.md")
        .expect("safe-default crypto profile");

    assert!(profile.contains("../CRYPTO_REGISTRY.md"), "{profile}");
    assert!(
        profile.contains("../CRYPTO_ALGORITHM_TEMPLATE.md"),
        "{profile}"
    );

    for status in ["documented", "test-vector-covered", "default-eligible"] {
        assert!(
            profile.contains(status),
            "missing required status {status}: {profile}"
        );
    }
}

#[test]
fn safe_default_profile_rejects_unsafe_entries() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/SAFE_DEFAULT.md")
        .expect("safe-default crypto profile");

    for rejected in [
        "custom security-critical primitives",
        "undocumented registry entries",
        "deprecated entries",
        "compatibility-only entries",
        "lab-only entries",
        "algorithms without test-vector coverage",
    ] {
        assert!(
            profile.contains(rejected),
            "missing rejection {rejected}: {profile}"
        );
    }
}

#[test]
fn safe_default_profile_guards_downgrades_and_audit() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/SAFE_DEFAULT.md")
        .expect("safe-default crypto profile");

    for text in [
        "warning",
        "affected control-point display",
        "explicit confirmation",
        "audit logging",
        "rollback guidance",
        "active profile",
        "affected control point",
        "downgrade or compatibility exception",
    ] {
        assert!(
            profile.contains(text),
            "missing downgrade/audit text {text}: {profile}"
        );
    }
}

#[test]
fn safe_default_profile_is_linked_from_profiles_index() {
    let index = std::fs::read_to_string("docs/security/crypto-profiles/README.md")
        .expect("crypto profiles index");

    assert!(index.contains("SAFE_DEFAULT.md"), "{index}");
    assert!(index.contains("safe-default"), "{index}");
}

#[test]
fn safe_default_profile_preserves_non_claims() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/SAFE_DEFAULT.md")
        .expect("safe-default crypto profile");

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
