#[test]
fn crypto_operator_commands_define_planned_command_surface() {
    let doc = std::fs::read_to_string("docs/security/CRYPTO_OPERATOR_COMMANDS.md")
        .expect("crypto operator command plan");

    assert!(doc.contains("Phase1 crypto operator command plan"), "{doc}");
    assert!(doc.contains("documentation-first"), "{doc}");

    for command in [
        "crypto status",
        "crypto profiles",
        "crypto explain <profile-or-algorithm>",
        "crypto select <profile> --scope <control-point> --confirm",
        "crypto policy export",
        "crypto policy verify",
        "crypto policy verify --profile <profile>",
        "crypto policy verify --scope <control-point>",
    ] {
        assert!(
            doc.contains(command),
            "missing planned command {command}: {doc}"
        );
    }
}

#[test]
fn crypto_operator_commands_define_allowed_scopes_and_profiles() {
    let doc = std::fs::read_to_string("docs/security/CRYPTO_OPERATOR_COMMANDS.md")
        .expect("crypto operator command plan");

    for scope in [
        "storage",
        "transport",
        "identity",
        "base1",
        "plugins",
        "logs/evidence",
        "fyr/packages",
        "lab",
        "docs",
        "tests",
    ] {
        assert!(doc.contains(scope), "missing scope {scope}: {doc}");
    }

    for profile in [
        "safe-default",
        "high-security",
        "compatibility",
        "post-quantum-preview",
        "lab-only",
    ] {
        assert!(doc.contains(profile), "missing profile {profile}: {doc}");
    }

    assert!(doc.contains("Unknown scopes should fail closed."), "{doc}");
    assert!(
        doc.contains("Unknown profiles should fail closed."),
        "{doc}"
    );
}

#[test]
fn crypto_operator_commands_define_selection_safety_gates() {
    let doc = std::fs::read_to_string("docs/security/CRYPTO_OPERATOR_COMMANDS.md")
        .expect("crypto operator command plan");

    for text in [
        "current profile",
        "requested profile",
        "affected control point",
        "upgrade, downgrade, compatibility exception, preview selection, or lab-only selection",
        "warning text when appropriate",
        "rollback command or guidance",
        "audit/logging notice",
        "Downgrades and compatibility exceptions must require `--confirm`.",
        "Lab-only selections must fail outside `lab`, `docs`, or `tests` scopes.",
    ] {
        assert!(
            doc.contains(text),
            "missing selection safety gate {text}: {doc}"
        );
    }
}

#[test]
fn crypto_operator_commands_define_verification_rules() {
    let doc = std::fs::read_to_string("docs/security/CRYPTO_OPERATOR_COMMANDS.md")
        .expect("crypto operator command plan");

    for rule in [
        "every configured profile exists",
        "every configured scope exists",
        "profiles are allowed for the selected scope",
        "profile docs exist",
        "registry entries exist",
        "algorithm docs use [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md)",
        "deprecated, rejected, or lab-only entries are not used in production scopes",
        "non-claims are preserved",
    ] {
        assert!(
            doc.contains(rule),
            "missing verification rule {rule}: {doc}"
        );
    }
}

#[test]
fn crypto_operator_commands_define_audit_expectations() {
    let doc = std::fs::read_to_string("docs/security/CRYPTO_OPERATOR_COMMANDS.md")
        .expect("crypto operator command plan");

    for text in [
        "old profile",
        "new profile",
        "scope",
        "reason, when provided",
        "whether confirmation was supplied",
        "verification result",
        "rollback guidance reference",
        "must not print secrets, private keys, seed material, tokens, or raw credentials",
    ] {
        assert!(
            doc.contains(text),
            "missing audit expectation {text}: {doc}"
        );
    }
}

#[test]
fn crypto_operator_commands_are_linked_from_security_index_and_roadmap() {
    let index = std::fs::read_to_string("docs/security/README.md").expect("security docs index");
    let roadmap = std::fs::read_to_string("docs/security/CRYPTO_POLICY_ROADMAP.md")
        .expect("crypto policy roadmap");

    assert!(index.contains("CRYPTO_OPERATOR_COMMANDS.md"), "{index}");
    assert!(roadmap.contains("CRYPTO_OPERATOR_COMMANDS.md"), "{roadmap}");
}

#[test]
fn crypto_operator_commands_preserve_non_claims() {
    let doc = std::fs::read_to_string("docs/security/CRYPTO_OPERATOR_COMMANDS.md")
        .expect("crypto operator command plan");

    assert!(
        doc.contains("does not make Phase1 or Base1 cryptographically complete"),
        "{doc}"
    );
    assert!(doc.contains("audited"), "{doc}");
    assert!(doc.contains("certified"), "{doc}");
    assert!(doc.contains("quantum-safe"), "{doc}");
    assert!(doc.contains("hardware-validated"), "{doc}");
    assert!(doc.contains("daily-driver ready"), "{doc}");
}
