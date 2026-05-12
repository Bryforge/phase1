#[test]
fn crypto_provider_registry_defines_purpose_and_security_goal() {
    let registry = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_REGISTRY.md")
        .expect("crypto provider registry");

    assert!(
        registry.contains("Phase1 crypto provider registry"),
        "{registry}"
    );
    assert!(
        registry.contains("as secure as possible while maintaining practical usability"),
        "{registry}"
    );
    assert!(
        registry.contains(
            "Listing a provider here does not approve it for production security claims."
        ),
        "{registry}"
    );
}

#[test]
fn crypto_provider_registry_requires_provider_template() {
    let registry = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_REGISTRY.md")
        .expect("crypto provider registry");

    assert!(
        registry.contains("CRYPTO_PROVIDER_TEMPLATE.md"),
        "{registry}"
    );
    assert!(
        registry.contains("Every provider entry must use [`CRYPTO_PROVIDER_TEMPLATE.md`](CRYPTO_PROVIDER_TEMPLATE.md)"),
        "{registry}"
    );
    assert!(
        registry.contains("provider entry uses `CRYPTO_PROVIDER_TEMPLATE.md`"),
        "{registry}"
    );
}

#[test]
fn crypto_provider_registry_lists_required_provider_metadata() {
    let registry = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_REGISTRY.md")
        .expect("crypto provider registry");

    for field in [
        "provider name",
        "library, crate, or system API",
        "upstream source",
        "license",
        "version or source pinning plan",
        "maintenance status",
        "supported platforms",
        "supported algorithm families",
        "supported profiles",
        "supported control points",
        "audit or review status",
        "test-vector source",
        "feature flags",
        "failure behavior",
        "known limitations",
        "migration guidance",
        "non-claims",
    ] {
        assert!(
            registry.contains(field),
            "missing provider metadata field {field}: {registry}"
        );
    }
}

#[test]
fn crypto_provider_registry_defines_status_labels() {
    let registry = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_REGISTRY.md")
        .expect("crypto provider registry");

    for status in [
        "candidate",
        "documented",
        "test-vector-covered",
        "profile-eligible",
        "default-eligible",
        "compatibility-only",
        "lab-only",
        "rejected",
    ] {
        assert!(
            registry.contains(status),
            "missing provider status {status}: {registry}"
        );
    }
}

#[test]
fn crypto_provider_registry_requires_fail_closed_selection() {
    let registry = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_REGISTRY.md")
        .expect("crypto provider registry");

    for rule in [
        "reject unknown providers",
        "reject providers without registry entries",
        "reject providers whose status is incompatible with the requested profile",
        "reject providers whose platform support does not match the host",
        "reject providers whose algorithms are not allowed for the requested control point",
        "report why a provider was rejected",
        "avoid silently falling back to a weaker provider",
        "log provider choice in a redacted audit event",
    ] {
        assert!(
            registry.contains(rule),
            "missing provider selection rule {rule}: {registry}"
        );
    }
}

#[test]
fn crypto_provider_registry_requires_review_before_profile_use() {
    let registry = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_REGISTRY.md")
        .expect("crypto provider registry");

    for item in [
        "provider metadata is complete",
        "provider entry uses `CRYPTO_PROVIDER_TEMPLATE.md`",
        "license is compatible",
        "version/source pinning is defined",
        "maintenance status is acceptable",
        "supported algorithms are documented in `CRYPTO_REGISTRY.md`",
        "algorithm docs use `CRYPTO_ALGORITHM_TEMPLATE.md`",
        "test vectors or equivalent tests exist",
        "failure behavior is fail-closed",
        "provider fallback behavior is explicit",
        "non-claims are preserved",
    ] {
        assert!(
            registry.contains(item),
            "missing provider review item {item}: {registry}"
        );
    }
}

#[test]
fn crypto_provider_registry_is_linked_from_security_docs() {
    let index = std::fs::read_to_string("docs/security/README.md").expect("security docs index");
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");
    let implementation = std::fs::read_to_string("docs/security/CRYPTO_IMPLEMENTATION_PLAN.md")
        .expect("crypto implementation plan");
    let gate = std::fs::read_to_string("scripts/security-crypto-doc-integrity.sh")
        .expect("crypto docs integrity gate");

    for doc in [&index, &roadmap, &implementation, &gate] {
        assert!(doc.contains("CRYPTO_PROVIDER_REGISTRY.md"), "{doc}");
    }
}

#[test]
fn crypto_provider_template_is_linked_from_registry_index_and_integrity_gate() {
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
fn crypto_provider_registry_preserves_non_claims() {
    let registry = std::fs::read_to_string("docs/security/CRYPTO_PROVIDER_REGISTRY.md")
        .expect("crypto provider registry");

    assert!(
        registry.contains("No provider is currently approved by this registry for new production security claims."),
        "{registry}"
    );
    assert!(
        registry.contains("does not make Phase1 or Base1 cryptographically complete"),
        "{registry}"
    );
    assert!(registry.contains("audited"), "{registry}");
    assert!(registry.contains("certified"), "{registry}");
    assert!(registry.contains("quantum-safe"), "{registry}");
    assert!(registry.contains("hardware-validated"), "{registry}");
    assert!(registry.contains("daily-driver ready"), "{registry}");
}
