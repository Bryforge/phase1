#[test]
fn contributing_guidelines_define_project_scope_and_ground_rules() {
    let doc = std::fs::read_to_string("CONTRIBUTING.md")
        .expect("CONTRIBUTING.md");

    assert!(doc.contains("Contributing to Phase1"), "{doc}");

    for text in [
        "Phase1",
        "Base1",
        "Fyr",
        "Security",
        "Website/community",
        "Keep safety and usability together.",
        "Do not overclaim security, OS readiness, hardware validation, cryptographic completeness, or audit status.",
        "Prefer tests over claims.",
        "Never commit secrets, tokens, private keys, credentials, recovery codes, private logs, or unrevised screenshots.",
    ] {
        assert!(doc.contains(text), "missing contribution scope or rule {text}: {doc}");
    }
}

#[test]
fn contributing_guidelines_define_branch_pr_and_validation_workflow() {
    let doc = std::fs::read_to_string("CONTRIBUTING.md")
        .expect("CONTRIBUTING.md");

    for text in [
        "branch from `edge/stable`",
        "open PRs back into `edge/stable`",
        "cargo fmt --all -- --check",
        "cargo check --all-targets",
        "cargo test --all-targets",
        "sh scripts/quality-check.sh quick",
        "sh scripts/quality-check.sh base1-docs",
        "sh scripts/quality-check.sh security-crypto-docs",
        "sh scripts/quality-check.sh full",
    ] {
        assert!(doc.contains(text), "missing workflow or validation text {text}: {doc}");
    }
}

#[test]
fn contributing_guidelines_define_pr_checklist() {
    let doc = std::fs::read_to_string("CONTRIBUTING.md")
        .expect("CONTRIBUTING.md");

    for text in [
        "what changed;",
        "why it changed;",
        "which project area is affected;",
        "what validation was run;",
        "what risks remain;",
        "whether docs were updated;",
        "whether tests were updated;",
        "whether safe defaults are preserved;",
        "whether any compatibility path is affected.",
    ] {
        assert!(doc.contains(text), "missing PR checklist item {text}: {doc}");
    }
}

#[test]
fn contributing_guidelines_preserve_docs_claim_rules() {
    let doc = std::fs::read_to_string("CONTRIBUTING.md")
        .expect("CONTRIBUTING.md");

    for text in [
        "> **Status:** Implemented | Experimental | Design | Dry-run | Preview | Roadmap | Not claimed",
        "> **Validation:** tests, scripts, release notes, or manual verification path",
        "> **Non-claims:** what this page does not guarantee",
        "blocked by default",
        "requires explicit confirmation",
        "read-only validation",
        "dry-run",
        "not claimed",
        "cryptographically complete",
        "quantum-safe",
        "hardware-validated",
    ] {
        assert!(doc.contains(text), "missing docs claim rule {text}: {doc}");
    }
}

#[test]
fn contributing_guidelines_define_security_crypto_base1_and_fyr_rules() {
    let doc = std::fs::read_to_string("CONTRIBUTING.md")
        .expect("CONTRIBUTING.md");

    for text in [
        "Security contribution rules",
        "Crypto policy contribution rules",
        "Base1 contribution rules",
        "Fyr contribution rules",
        "do not invent custom security-critical primitives",
        "docs/security/CRYPTO_ALGORITHM_TEMPLATE.md",
        "docs/security/CRYPTO_PROVIDER_TEMPLATE.md",
        "keep unknown profiles, scopes, algorithms, and providers fail-closed",
        "keep root compatibility paths recoverable",
        "do not claim installer readiness, boot readiness, hardware validation, or daily-driver readiness without evidence",
        "Do not describe Fyr as production-ready",
    ] {
        assert!(doc.contains(text), "missing project-specific rule {text}: {doc}");
    }
}

#[test]
fn contributing_guidelines_define_testing_expectations_and_rejection_criteria() {
    let doc = std::fs::read_to_string("CONTRIBUTING.md")
        .expect("CONTRIBUTING.md");

    for text in [
        "Add or update tests when you:",
        "add a feature;",
        "change command behavior;",
        "change security, crypto, Base1, or release claims;",
        "fix a bug that could recur.",
        "Maintainers may reject or ask for changes when a contribution:",
        "weakens safe defaults;",
        "removes compatibility paths without approval;",
        "introduces secret leakage risk;",
        "adds custom cryptography for real protection;",
    ] {
        assert!(doc.contains(text), "missing testing/rejection criterion {text}: {doc}");
    }
}

#[test]
fn contributing_guidelines_preserve_non_claims() {
    let doc = std::fs::read_to_string("CONTRIBUTING.md")
        .expect("CONTRIBUTING.md");

    assert!(doc.contains("These guidelines do not guarantee acceptance of any contribution"), "{doc}");
    assert!(doc.contains("provide legal advice"), "{doc}");
    assert!(doc.contains("launch a support program"), "{doc}");
    assert!(doc.contains("make Phase1, Base1, or Fyr production-ready"), "{doc}");
}
