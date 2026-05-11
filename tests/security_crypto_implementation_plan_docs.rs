#[test]
fn crypto_implementation_plan_defines_safe_sequence() {
    let plan = std::fs::read_to_string("docs/security/CRYPTO_IMPLEMENTATION_PLAN.md")
        .expect("crypto implementation plan");

    assert!(plan.contains("Phase1 crypto implementation plan"), "{plan}");
    assert!(
        plan.contains("policy first, registry second, provider integration third"),
        "{plan}"
    );

    for phase in [
        "Documentation and registry surface",
        "Read-only command surface",
        "Config parser and validator",
        "Provider abstraction",
        "Test-vector harness",
        "Profile policy engine",
        "Scoped integration points",
        "Migration and rollback tooling",
        "External review and audit preparation",
    ] {
        assert!(plan.contains(phase), "missing implementation phase {phase}: {plan}");
    }
}

#[test]
fn crypto_implementation_plan_preserves_security_rules() {
    let plan = std::fs::read_to_string("docs/security/CRYPTO_IMPLEMENTATION_PLAN.md")
        .expect("crypto implementation plan");

    for rule in [
        "do not invent custom security-critical primitives",
        "prefer reviewed open-source libraries and established algorithms",
        "keep `safe-default` as the normal profile",
        "advanced profile choices explicit and auditable",
        "fail closed on unknown profiles, scopes, algorithms, and providers",
        "lab-only behavior isolated from real protection decisions",
        "preserve rollback and recovery paths before protecting real data",
    ] {
        assert!(plan.contains(rule), "missing implementation security rule {rule}: {plan}");
    }
}

#[test]
fn crypto_implementation_plan_lists_required_docs_including_provider_registry() {
    let plan = std::fs::read_to_string("docs/security/CRYPTO_IMPLEMENTATION_PLAN.md")
        .expect("crypto implementation plan");

    for doc in [
        "CRYPTO_POLICY_ROADMAP.md",
        "CRYPTO_REGISTRY.md",
        "CRYPTO_PROVIDER_REGISTRY.md",
        "CRYPTO_ALGORITHM_TEMPLATE.md",
        "CRYPTO_OPERATOR_COMMANDS.md",
        "CRYPTO_CONFIG_SCHEMA.md",
        "crypto-profiles/README.md",
    ] {
        assert!(plan.contains(doc), "missing required crypto doc {doc}: {plan}");
    }

    assert!(
        plan.contains("no provider is approved for production claims by documentation alone"),
        "{plan}"
    );
}

#[test]
fn crypto_implementation_plan_defines_provider_abstraction_requirements() {
    let plan = std::fs::read_to_string("docs/security/CRYPTO_IMPLEMENTATION_PLAN.md")
        .expect("crypto implementation plan");

    assert!(
        plan.contains("Provider metadata and review requirements are tracked in [`CRYPTO_PROVIDER_REGISTRY.md`](CRYPTO_PROVIDER_REGISTRY.md)."),
        "{plan}"
    );

    for item in [
        "provider name",
        "library or crate name",
        "version or source pinning plan",
        "license",
        "supported algorithms",
        "supported platforms",
        "audit/review status",
        "test-vector source",
        "failure behavior",
        "unsupported providers fail closed",
        "providers are not silently selected",
        "no provider is used for production claims until registry status allows it",
    ] {
        assert!(plan.contains(item), "missing provider requirement {item}: {plan}");
    }
}

#[test]
fn crypto_implementation_plan_blocks_runtime_use_until_prerequisites() {
    let plan = std::fs::read_to_string("docs/security/CRYPTO_IMPLEMENTATION_PLAN.md")
        .expect("crypto implementation plan");

    assert!(
        plan.contains("No runtime control point should use cryptographic policy until the earlier phases are complete for that scope."),
        "{plan}"
    );

    for boundary in [
        "real encryption of persistent user data",
        "production signing of packages",
        "Base1 boot trust claims",
        "identity key management",
        "automatic migration of protected data",
        "post-quantum default behavior",
        "compatibility downgrade defaults",
    ] {
        assert!(plan.contains(boundary), "missing implementation boundary {boundary}: {plan}");
    }
}

#[test]
fn crypto_implementation_plan_defines_scoped_integration_order() {
    let plan = std::fs::read_to_string("docs/security/CRYPTO_IMPLEMENTATION_PLAN.md")
        .expect("crypto implementation plan");

    for scope in [
        "`logs/evidence`",
        "`plugins`",
        "`fyr/packages`",
        "`base1`",
        "`storage`",
        "`transport`",
        "`identity`",
    ] {
        assert!(plan.contains(scope), "missing scoped integration {scope}: {plan}");
    }

    for criterion in [
        "scope-specific docs exist",
        "tests exist",
        "migration/rollback guidance exists",
        "non-claims are preserved",
        "no unsupported security claim is introduced",
    ] {
        assert!(plan.contains(criterion), "missing scoped exit criterion {criterion}: {plan}");
    }
}

#[test]
fn crypto_implementation_plan_is_linked_from_index_roadmap_and_integrity_gate() {
    let index = std::fs::read_to_string("docs/security/README.md")
        .expect("security docs index");
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");
    let gate = std::fs::read_to_string("scripts/security-crypto-doc-integrity.sh")
        .expect("crypto docs integrity gate");

    for doc in [&index, &roadmap, &gate] {
        assert!(doc.contains("CRYPTO_IMPLEMENTATION_PLAN.md"), "{doc}");
    }
}

#[test]
fn crypto_implementation_plan_preserves_non_claims() {
    let plan = std::fs::read_to_string("docs/security/CRYPTO_IMPLEMENTATION_PLAN.md")
        .expect("crypto implementation plan");

    assert!(plan.contains("does not make Phase1 or Base1 cryptographically complete"), "{plan}");
    assert!(plan.contains("audited"), "{plan}");
    assert!(plan.contains("certified"), "{plan}");
    assert!(plan.contains("quantum-safe"), "{plan}");
    assert!(plan.contains("hardware-validated"), "{plan}");
    assert!(plan.contains("daily-driver ready"), "{plan}");
}
