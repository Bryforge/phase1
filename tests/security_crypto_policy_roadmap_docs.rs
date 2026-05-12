#[test]
fn crypto_policy_roadmap_defines_operator_selectable_profiles() {
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    assert!(
        roadmap.contains("Phase1 cryptographic policy roadmap"),
        "{roadmap}"
    );
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
        assert!(
            roadmap.contains(profile),
            "missing crypto profile {profile}: {roadmap}"
        );
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
        assert!(
            roadmap.contains(control),
            "missing control point {control}: {roadmap}"
        );
    }
}

#[test]
fn crypto_policy_roadmap_rejects_custom_security_primitives() {
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    assert!(
        roadmap.contains(
            "Phase1 should not invent new cryptographic primitives for security-critical use."
        ),
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
        "CRYPTO_REGISTRY.md",
        "CRYPTO_ALGORITHM_TEMPLATE.md",
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
        assert!(
            roadmap.contains(text),
            "missing registry/doc requirement {text}: {roadmap}"
        );
    }
}

#[test]
fn crypto_policy_roadmap_documents_provider_registry() {
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    for text in [
        "Provider registry",
        "CRYPTO_PROVIDER_REGISTRY.md",
        "provider source, license, version/source pinning",
        "supported algorithms",
        "supported platforms",
        "feature flags",
        "failure behavior",
        "No provider should be connected to runtime behavior until it is documented, reviewed, and compatible with the selected profile and control point.",
        "Every provider entry should follow [`CRYPTO_PROVIDER_REGISTRY.md`](CRYPTO_PROVIDER_REGISTRY.md)",
        "Create provider registry: [`CRYPTO_PROVIDER_REGISTRY.md`](CRYPTO_PROVIDER_REGISTRY.md).",
        "Provider inventory reports.",
    ] {
        assert!(roadmap.contains(text), "missing provider roadmap text {text}: {roadmap}");
    }
}

#[test]
fn crypto_policy_roadmap_links_operator_commands_and_config_schema() {
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    for text in [
        "CRYPTO_OPERATOR_COMMANDS.md",
        "CRYPTO_CONFIG_SCHEMA.md",
        "Operator command surface",
        "Configuration model",
        "Unknown profiles and scopes should fail closed.",
        "Every config schema change should follow [`CRYPTO_CONFIG_SCHEMA.md`](CRYPTO_CONFIG_SCHEMA.md)",
        "Create operator command plan: [`CRYPTO_OPERATOR_COMMANDS.md`](CRYPTO_OPERATOR_COMMANDS.md).",
        "Create config schema plan: [`CRYPTO_CONFIG_SCHEMA.md`](CRYPTO_CONFIG_SCHEMA.md).",
    ] {
        assert!(roadmap.contains(text), "missing roadmap config/operator text {text}: {roadmap}");
    }
}

#[test]
fn crypto_policy_roadmap_links_all_profile_planning_docs() {
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    assert!(roadmap.contains("crypto-profiles/README.md"), "{roadmap}");
    assert!(
        roadmap.contains("Draft safe-default, high-security, compatibility, post-quantum-preview, and lab-only profiles."),
        "{roadmap}"
    );
}

#[test]
fn crypto_algorithm_template_defines_required_sections() {
    let template = std::fs::read_to_string("docs/security/CRYPTO_ALGORITHM_TEMPLATE.md")
        .expect("crypto algorithm template");

    for text in [
        "Cryptographic algorithm documentation template",
        "Algorithm summary",
        "Implementation source",
        "Allowed use cases",
        "Disallowed use cases",
        "Parameters and limits",
        "Security notes",
        "Usability notes",
        "Test vectors",
        "Profile behavior",
        "Migration and rotation",
        "Review checklist",
        "Non-claims",
    ] {
        assert!(
            template.contains(text),
            "missing template section {text}: {template}"
        );
    }
}

#[test]
fn crypto_algorithm_template_preserves_safety_requirements() {
    let template = std::fs::read_to_string("docs/security/CRYPTO_ALGORITHM_TEMPLATE.md")
        .expect("crypto algorithm template");

    for text in [
        "Uses a reviewed open-source implementation where possible.",
        "Does not invent a custom security-critical primitive.",
        "Status is clearly labeled.",
        "Test vectors are identified.",
        "Operator usability impact is documented.",
        "Migration/rotation guidance is documented.",
        "Non-claims are preserved.",
    ] {
        assert!(
            template.contains(text),
            "missing template safety requirement {text}: {template}"
        );
    }
}

#[test]
fn crypto_policy_roadmap_preserves_non_claims() {
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    assert!(
        roadmap.contains("does not make Phase1 or Base1 cryptographically complete"),
        "{roadmap}"
    );
    assert!(roadmap.contains("audited"), "{roadmap}");
    assert!(roadmap.contains("quantum-safe"), "{roadmap}");
    assert!(roadmap.contains("hardware-validated"), "{roadmap}");
    assert!(roadmap.contains("daily-driver ready"), "{roadmap}");
}

#[test]
fn security_index_links_crypto_policy_surface() {
    let index = std::fs::read_to_string("docs/security/README.md").expect("security docs index");

    for text in [
        "CRYPTO_POLICY_ROADMAP.md",
        "CRYPTO_REGISTRY.md",
        "CRYPTO_PROVIDER_REGISTRY.md",
        "CRYPTO_OPERATOR_COMMANDS.md",
        "CRYPTO_CONFIG_SCHEMA.md",
        "CRYPTO_ALGORITHM_TEMPLATE.md",
        "crypto-profiles/README.md",
        "cryptographic completeness",
        "algorithm pages use [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md)",
        "provider entries are listed or planned through [`CRYPTO_PROVIDER_REGISTRY.md`](CRYPTO_PROVIDER_REGISTRY.md)",
        "crypto configuration follows [`CRYPTO_CONFIG_SCHEMA.md`](CRYPTO_CONFIG_SCHEMA.md)",
    ] {
        assert!(index.contains(text), "missing security index text {text}: {index}");
    }
}
