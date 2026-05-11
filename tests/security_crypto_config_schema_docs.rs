#[test]
fn crypto_config_schema_defines_default_safe_posture() {
    let schema = std::fs::read_to_string("docs/security/CRYPTO_CONFIG_SCHEMA.md")
        .expect("crypto config schema");

    assert!(
        schema.contains("Phase1 crypto policy configuration schema"),
        "{schema}"
    );
    assert!(schema.contains("default_profile = \"safe-default\""), "{schema}");
    assert!(
        schema.contains("If no crypto configuration exists, Phase1 should behave as though `safe-default` is active"),
        "{schema}"
    );
    assert!(
        schema.contains("No configuration should silently enable compatibility, post-quantum-preview, or lab-only behavior."),
        "{schema}"
    );
}

#[test]
fn crypto_config_schema_lists_allowed_profiles_and_scopes() {
    let schema = std::fs::read_to_string("docs/security/CRYPTO_CONFIG_SCHEMA.md")
        .expect("crypto config schema");

    for profile in [
        "safe-default",
        "high-security",
        "compatibility",
        "post-quantum-preview",
        "lab-only",
    ] {
        assert!(schema.contains(profile), "missing profile {profile}: {schema}");
    }

    for scope in [
        "storage",
        "transport",
        "identity",
        "base1",
        "plugins",
        "logs-evidence",
        "fyr-packages",
        "lab",
        "docs",
        "tests",
    ] {
        assert!(schema.contains(scope), "missing scope {scope}: {schema}");
    }
}

#[test]
fn crypto_config_schema_defines_fail_closed_validation() {
    let schema = std::fs::read_to_string("docs/security/CRYPTO_CONFIG_SCHEMA.md")
        .expect("crypto config schema");

    for rule in [
        "Unknown profiles must fail closed.",
        "Unknown scopes must fail closed.",
        "unknown profiles;",
        "unknown scopes;",
        "`lab-only` outside `lab`, `docs`, or `tests`;",
        "`compatibility` without a reason;",
        "`post-quantum-preview` without a reason;",
        "production scopes using deprecated, rejected, or lab-only registry entries;",
        "algorithm docs that do not use [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md).",
    ] {
        assert!(schema.contains(rule), "missing fail-closed rule {rule}: {schema}");
    }
}

#[test]
fn crypto_config_schema_defines_warning_and_audit_rules() {
    let schema = std::fs::read_to_string("docs/security/CRYPTO_CONFIG_SCHEMA.md")
        .expect("crypto config schema");

    for text in [
        "a scope is configured away from `safe-default`;",
        "`compatibility` is selected;",
        "`post-quantum-preview` is selected;",
        "a Base1 scope is configured below `high-security`;",
        "old profile;",
        "new profile;",
        "scope;",
        "reason;",
        "confirmation status;",
        "verification result.",
        "Audit output must not include secrets, private keys, seed material, tokens, or raw credentials.",
    ] {
        assert!(schema.contains(text), "missing warning/audit rule {text}: {schema}");
    }
}

#[test]
fn crypto_config_schema_includes_safe_and_rejected_examples() {
    let schema = std::fs::read_to_string("docs/security/CRYPTO_CONFIG_SCHEMA.md")
        .expect("crypto config schema");

    assert!(schema.contains("Example safe configuration"), "{schema}");
    assert!(schema.contains("Example rejected configuration"), "{schema}");
    assert!(
        schema.contains("This should be rejected because `lab-only` must not protect production storage."),
        "{schema}"
    );
}

#[test]
fn crypto_config_schema_is_linked_from_security_index_roadmap_and_integrity_gate() {
    let index = std::fs::read_to_string("docs/security/README.md")
        .expect("security docs index");
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");
    let gate = std::fs::read_to_string("scripts/security-crypto-doc-integrity.sh")
        .expect("crypto docs integrity gate");

    for doc in [&index, &roadmap, &gate] {
        assert!(doc.contains("CRYPTO_CONFIG_SCHEMA.md"), "{doc}");
    }
}

#[test]
fn crypto_config_schema_preserves_non_claims() {
    let schema = std::fs::read_to_string("docs/security/CRYPTO_CONFIG_SCHEMA.md")
        .expect("crypto config schema");

    assert!(schema.contains("does not make Phase1 or Base1 cryptographically complete"), "{schema}");
    assert!(schema.contains("audited"), "{schema}");
    assert!(schema.contains("certified"), "{schema}");
    assert!(schema.contains("quantum-safe"), "{schema}");
    assert!(schema.contains("hardware-validated"), "{schema}");
    assert!(schema.contains("daily-driver ready"), "{schema}");
}
