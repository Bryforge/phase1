#[test]
fn crypto_provider_template_defines_required_sections() {
    let template = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_TEMPLATE.md")
        .expect("crypto provider template");

    assert!(
        template.contains("Crypto provider documentation template"),
        "{template}"
    );

    for section in [
        "Provider summary",
        "Source and license",
        "Supported capabilities",
        "Supported platforms",
        "Allowed use cases",
        "Disallowed use cases",
        "Profile compatibility",
        "Control-point compatibility",
        "Test-vector coverage",
        "Failure behavior",
        "Fallback behavior",
        "Audit and logging expectations",
        "Review checklist",
        "Non-claims",
    ] {
        assert!(
            template.contains(section),
            "missing provider template section {section}: {template}"
        );
    }
}

#[test]
fn crypto_provider_template_requires_source_license_and_pinning() {
    let template = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_TEMPLATE.md")
        .expect("crypto provider template");

    for field in [
        "Upstream project:",
        "Package/crate name:",
        "Source repository:",
        "Documentation URL:",
        "License:",
        "License compatibility review:",
        "Version or source pinning plan:",
        "Maintenance status:",
    ] {
        assert!(
            template.contains(field),
            "missing source/license field {field}: {template}"
        );
    }
}

#[test]
fn crypto_provider_template_lists_profile_and_control_point_compatibility() {
    let template = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_TEMPLATE.md")
        .expect("crypto provider template");

    for profile in [
        "safe-default",
        "high-security",
        "compatibility",
        "post-quantum-preview",
        "lab-only",
    ] {
        assert!(
            template.contains(profile),
            "missing profile {profile}: {template}"
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
        "lab/docs/tests",
    ] {
        assert!(
            template.contains(control_point),
            "missing control point {control_point}: {template}"
        );
    }
}

#[test]
fn crypto_provider_template_requires_fail_closed_and_explicit_fallback() {
    let template = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_TEMPLATE.md")
        .expect("crypto provider template");

    for text in [
        "Expected behavior should be fail-closed with clear operator-visible errors.",
        "Fallback must never silently change the intended security posture.",
        "Is fallback allowed: yes/no",
        "Required warning:",
        "Required audit event:",
        "Conditions where fallback is rejected:",
    ] {
        assert!(
            template.contains(text),
            "missing fail-closed/fallback text {text}: {template}"
        );
    }
}

#[test]
fn crypto_provider_template_preserves_review_checklist() {
    let template = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_TEMPLATE.md")
        .expect("crypto provider template");

    for item in [
        "Provider metadata is complete.",
        "License compatibility is reviewed.",
        "Version/source pinning is defined.",
        "Maintenance status is documented.",
        "Supported algorithms are documented in `CRYPTO_REGISTRY.md`.",
        "Algorithm docs use `CRYPTO_ALGORITHM_TEMPLATE.md`.",
        "Test vectors or equivalent tests exist.",
        "Failure behavior is fail-closed.",
        "Fallback behavior is explicit.",
        "Non-claims are preserved.",
    ] {
        assert!(
            template.contains(item),
            "missing provider review item {item}: {template}"
        );
    }
}

#[test]
fn crypto_provider_template_is_linked_from_provider_registry_index_and_integrity_gate() {
    let registry = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_REGISTRY.md")
        .expect("crypto provider registry");
    let index = std::fs::read_to_string("docs/security/README.md").expect("security docs index");
    let gate = std::fs::read_to_string("scripts/security-crypto-doc-integrity.sh")
        .expect("crypto docs integrity gate");

    for doc in [&registry, &index, &gate] {
        assert!(doc.contains("CRYPTO_PROVIDER_TEMPLATE.md"), "{doc}");
    }
}

#[test]
fn crypto_provider_template_preserves_non_claims() {
    let template = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_TEMPLATE.md")
        .expect("crypto provider template");

    assert!(
        template.contains("does not make Phase1 or Base1 cryptographically complete"),
        "{template}"
    );
    assert!(template.contains("audited"), "{template}");
    assert!(template.contains("certified"), "{template}");
    assert!(template.contains("quantum-safe"), "{template}");
    assert!(template.contains("hardware-validated"), "{template}");
    assert!(template.contains("daily-driver ready"), "{template}");
}
