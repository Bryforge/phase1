#[test]
fn compatibility_profile_defines_purpose_and_operator() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/COMPATIBILITY.md")
        .expect("compatibility crypto profile");

    assert!(profile.contains("Compatibility cryptographic profile"), "{profile}");
    assert!(profile.contains("planned interoperability profile"), "{profile}");

    for operator in [
        "Maintainers working with older data or external systems",
        "Advanced operators who understand downgrade risk",
        "Migration operators moving data from old formats into safer profiles",
        "Developers testing interoperability behavior",
    ] {
        assert!(profile.contains(operator), "missing intended operator {operator}: {profile}");
    }
}

#[test]
fn compatibility_profile_lists_allowed_control_points() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/COMPATIBILITY.md")
        .expect("compatibility crypto profile");

    for control_point in [
        "transport",
        "storage",
        "plugins",
        "fyr/packages",
        "logs/evidence",
    ] {
        assert!(
            profile.contains(control_point),
            "missing compatibility control point {control_point}: {profile}"
        );
    }

    assert!(
        profile.contains("Base1 critical paths should avoid this profile"),
        "{profile}"
    );
}

#[test]
fn compatibility_profile_requires_registry_template_and_statuses() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/COMPATIBILITY.md")
        .expect("compatibility crypto profile");

    assert!(profile.contains("../CRYPTO_REGISTRY.md"), "{profile}");
    assert!(profile.contains("../CRYPTO_ALGORITHM_TEMPLATE.md"), "{profile}");

    for status in [
        "documented",
        "compatibility-only",
        "test-vector-covered",
    ] {
        assert!(profile.contains(status), "missing required status {status}: {profile}");
    }
}

#[test]
fn compatibility_profile_requires_explicit_warning_and_consent() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/COMPATIBILITY.md")
        .expect("compatibility crypto profile");

    for text in [
        "operator sees a warning",
        "operator explicitly confirms the compatibility path",
        "audit logging records the exception",
        "safer alternatives are documented",
        "migration guidance is documented",
        "This profile must require explicit selection.",
    ] {
        assert!(profile.contains(text), "missing warning/consent text {text}: {profile}");
    }
}

#[test]
fn compatibility_profile_rejects_unsafe_entries_and_new_weak_data() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/COMPATIBILITY.md")
        .expect("compatibility crypto profile");

    for rejected in [
        "custom security-critical primitives",
        "undocumented registry entries",
        "lab-only entries",
        "rejected entries",
        "silent downgrade behavior",
        "creating new long-term data with weaker choices",
    ] {
        assert!(profile.contains(rejected), "missing rejection {rejected}: {profile}");
    }
}

#[test]
fn compatibility_profile_defines_warning_audit_and_migration_behavior() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/COMPATIBILITY.md")
        .expect("compatibility crypto profile");

    for text in [
        "affected control point",
        "reason for compatibility mode",
        "safer profile recommendation",
        "migration path back to `safe-default` or `high-security`",
        "compatibility reason",
        "data was read, verified, converted, or newly written",
        "migration target profile",
        "failure recovery",
    ] {
        assert!(profile.contains(text), "missing warning/audit/migration text {text}: {profile}");
    }
}

#[test]
fn compatibility_profile_is_linked_from_profiles_index() {
    let index = std::fs::read_to_string("docs/security/crypto-profiles/README.md")
        .expect("crypto profiles index");

    assert!(index.contains("COMPATIBILITY.md"), "{index}");
    assert!(index.contains("compatibility"), "{index}");
}

#[test]
fn compatibility_profile_preserves_non_claims() {
    let profile = std::fs::read_to_string("docs/security/crypto-profiles/COMPATIBILITY.md")
        .expect("compatibility crypto profile");

    assert!(profile.contains("does not make Phase1 or Base1 cryptographically complete"), "{profile}");
    assert!(profile.contains("audited"), "{profile}");
    assert!(profile.contains("certified"), "{profile}");
    assert!(profile.contains("quantum-safe"), "{profile}");
    assert!(profile.contains("hardware-validated"), "{profile}");
    assert!(profile.contains("daily-driver ready"), "{profile}");
}
