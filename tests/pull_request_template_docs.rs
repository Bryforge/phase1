#[test]
fn pull_request_template_defines_summary_project_area_and_validation() {
    let template =
        std::fs::read_to_string(".github/pull_request_template.md").expect("pull request template");

    assert!(template.contains("# Pull request"), "{template}");
    assert!(template.contains("## Summary"), "{template}");
    assert!(template.contains("## Project area"), "{template}");
    assert!(template.contains("## Validation run"), "{template}");

    for command in [
        "cargo fmt --all -- --check",
        "cargo check --all-targets",
        "cargo test --all-targets",
        "sh scripts/quality-check.sh quick",
        "sh scripts/quality-check.sh base1-docs",
        "sh scripts/quality-check.sh base1-reorg",
        "sh scripts/quality-check.sh security-crypto-docs",
    ] {
        assert!(
            template.contains(command),
            "missing validation command {command}: {template}"
        );
    }
}

#[test]
fn pull_request_template_lists_project_areas() {
    let template =
        std::fs::read_to_string(".github/pull_request_template.md").expect("pull request template");

    for area in [
        "Phase1",
        "Base1",
        "Fyr",
        "Security",
        "Crypto policy",
        "Website / branding",
        "Community / support",
        "Documentation only",
        "Tests / quality gates",
    ] {
        assert!(
            template.contains(area),
            "missing project area {area}: {template}"
        );
    }
}

#[test]
fn pull_request_template_preserves_safety_checklist() {
    let template =
        std::fs::read_to_string(".github/pull_request_template.md").expect("pull request template");

    for item in [
        "Safe defaults are preserved.",
        "Host-backed behavior remains explicit and gated.",
        "No credentials, tokens, private keys, recovery codes, private logs, or unrevised screenshots are included.",
        "Documentation does not overclaim security, OS readiness, hardware validation, cryptographic completeness, audit status, certification, or quantum safety.",
        "Tests or docs were updated for changed behavior.",
    ] {
        assert!(template.contains(item), "missing safety checklist item {item}: {template}");
    }
}

#[test]
fn pull_request_template_preserves_compatibility_checklist() {
    let template =
        std::fs::read_to_string(".github/pull_request_template.md").expect("pull request template");

    for item in [
        "No compatibility paths were removed.",
        "Base1 release archive paths are preserved when relevant.",
        "Script/operator command paths remain stable or wrappers are documented.",
        "Migration or rollback guidance is documented when relevant.",
    ] {
        assert!(
            template.contains(item),
            "missing compatibility checklist item {item}: {template}"
        );
    }
}

#[test]
fn pull_request_template_preserves_crypto_checklist() {
    let template =
        std::fs::read_to_string(".github/pull_request_template.md").expect("pull request template");

    for item in [
        "No custom security-critical primitive is added for real protection.",
        "Algorithm entries use `docs/security/CRYPTO_ALGORITHM_TEMPLATE.md`.",
        "Provider entries use `docs/security/CRYPTO_PROVIDER_TEMPLATE.md`.",
        "Unknown profiles, scopes, algorithms, and providers fail closed.",
        "Lab-only behavior is isolated from production control points.",
        "Non-claims are preserved.",
    ] {
        assert!(
            template.contains(item),
            "missing crypto checklist item {item}: {template}"
        );
    }
}

#[test]
fn pull_request_template_links_contribution_quality_and_security_docs() {
    let template =
        std::fs::read_to_string(".github/pull_request_template.md").expect("pull request template");

    for doc in ["CONTRIBUTING.md", "docs/quality/QUALITY.md", "SECURITY.md"] {
        assert!(
            template.contains(doc),
            "missing related doc {doc}: {template}"
        );
    }
}
