#[test]
fn lab_only_profile_defines_purpose_and_operator() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/LAB_ONLY.md")
        .expect("lab-only crypto profile");

    assert!(profile.contains("Lab-only cryptographic profile"), "{profile}");
    assert!(profile.contains("planned profile"), "{profile}");

    for operator in [
        "Researchers",
        "Educators",
        "Developers testing ideas",
        "Advanced operators reviewing non-production behavior",
        "Contributors writing documentation or examples",
    ] {
        assert!(profile.contains(operator), "missing intended operator {operator}: {profile}");
    }
}

#[test]
fn lab_only_profile_limits_allowed_control_points() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/LAB_ONLY.md")
        .expect("lab-only crypto profile");

    for allowed in [
        "lab",
        "docs",
        "tests",
    ] {
        assert!(profile.contains(allowed), "missing allowed control point {allowed}: {profile}");
    }

    assert!(
        profile.contains("This profile should not be enabled for production control points."),
        "{profile}"
    );
}

#[test]
fn lab_only_profile_blocks_production_control_points() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/LAB_ONLY.md")
        .expect("lab-only crypto profile");

    for blocked in [
        "storage",
        "transport",
        "identity",
        "base1",
        "plugins",
        "logs/evidence",
        "fyr/packages",
    ] {
        assert!(profile.contains(blocked), "missing blocked control point {blocked}: {profile}");
    }

    assert!(
        profile.contains("Any future command that attempts to bind `lab-only` to a production control point should fail"),
        "{profile}"
    );
}

#[test]
fn lab_only_profile_requires_registry_and_template() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/LAB_ONLY.md")
        .expect("lab-only crypto profile");

    assert!(profile.contains("../CRYPTO_REGISTRY.md"), "{profile}");
    assert!(profile.contains("../CRYPTO_ALGORITHM_TEMPLATE.md"), "{profile}");

    for status in [
        "lab-only",
        "rejected",
        "documented",
    ] {
        assert!(profile.contains(status), "missing required status {status}: {profile}");
    }
}

#[test]
fn lab_only_profile_rejects_real_protection_and_unsafe_claims() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/LAB_ONLY.md")
        .expect("lab-only crypto profile");

    for rejected in [
        "protecting real data",
        "protecting Base1 material",
        "protecting operator credentials or identity",
        "production package signing",
        "production transport or storage policy",
        "silent promotion into another profile",
        "any claim that lab behavior is secure, audited, certified, quantum-safe, or hardened",
    ] {
        assert!(profile.contains(rejected), "missing rejection {rejected}: {profile}");
    }
}

#[test]
fn lab_only_profile_defines_warnings_audit_and_cleanup() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/LAB_ONLY.md")
        .expect("lab-only crypto profile");

    for text in [
        "non-production warning",
        "real data must not be protected",
        "Base1 material must not be protected",
        "rollback path to `safe-default`",
        "explicit lab-only warning acknowledgement",
        "docs-only, test-only, or demo-only",
        "artifact location",
        "cleanup command",
        "proof that no production control point was changed",
    ] {
        assert!(profile.contains(text), "missing warning/audit/cleanup text {text}: {profile}");
    }
}

#[test]
fn lab_only_profile_is_linked_from_profiles_index() {
    let index = std::fs::read_to_string("docs/security/crypto-profiles/README.md")
        .expect("crypto profiles index");

    assert!(index.contains("LAB_ONLY.md"), "{index}");
    assert!(index.contains("lab-only"), "{index}");
}

#[test]
fn lab_only_profile_preserves_non_claims() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/LAB_ONLY.md")
        .expect("lab-only crypto profile");

    assert!(profile.contains("does not make Phase1 or Base1 cryptographically complete"), "{profile}");
    assert!(profile.contains("audited"), "{profile}");
    assert!(profile.contains("certified"), "{profile}");
    assert!(profile.contains("quantum-safe"), "{profile}");
    assert!(profile.contains("hardware-validated"), "{profile}");
    assert!(profile.contains("daily-driver ready"), "{profile}");
}
