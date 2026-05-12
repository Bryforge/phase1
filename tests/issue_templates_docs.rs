#[test]
fn issue_template_config_disables_blank_issues_and_links_support_paths() {
    let config = std::fs::read_to_string(".github/ISSUE_TEMPLATE/config.yml")
        .expect("issue template config");

    assert!(config.contains("blank_issues_enabled: false"), "{config}");

    for text in [
        "Security vulnerability report",
        "https://github.com/Bryforge/phase1/security/policy",
        "Phase1 documentation",
        "Contribution guidelines",
        "Quality gates",
        "Community and support roadmap",
    ] {
        assert!(
            config.contains(text),
            "missing config contact link {text}: {config}"
        );
    }
}

#[test]
fn bug_report_template_collects_reproduction_context() {
    let template = std::fs::read_to_string(".github/ISSUE_TEMPLATE/bug_report.yml")
        .expect("bug report template");

    for text in [
        "name: Bug report",
        "Project area",
        "Version, branch, or commit",
        "Environment",
        "Command or workflow",
        "Expected behavior",
        "Actual behavior",
        "Minimal reproduction steps",
        "Redacted logs or screenshots",
        "Safety check",
    ] {
        assert!(
            template.contains(text),
            "missing bug template text {text}: {template}"
        );
    }
}

#[test]
fn bug_report_template_preserves_secret_and_security_routing() {
    let template = std::fs::read_to_string(".github/ISSUE_TEMPLATE/bug_report.yml")
        .expect("bug report template");

    for text in [
        "Do not include secrets, tokens, private keys, recovery codes, private logs, or unrevised screenshots.",
        "Security vulnerabilities should not be reported through a public issue.",
        "Follow `SECURITY.md` instead.",
        "I removed secrets, tokens, private keys, recovery codes, private logs, and unrevised screenshots.",
        "This is not a private vulnerability report.",
    ] {
        assert!(template.contains(text), "missing bug safety text {text}: {template}");
    }
}

#[test]
fn feature_request_template_captures_scope_risk_and_validation() {
    let template = std::fs::read_to_string(".github/ISSUE_TEMPLATE/feature_request.yml")
        .expect("feature request template");

    for text in [
        "name: Feature request",
        "Project area",
        "Problem or opportunity",
        "Proposed solution",
        "Alternatives considered",
        "Risk level",
        "Safety and claim checks",
        "Suggested validation",
        "Related docs, issues, or context",
    ] {
        assert!(
            template.contains(text),
            "missing feature template text {text}: {template}"
        );
    }
}

#[test]
fn feature_request_template_preserves_claim_safety() {
    let template = std::fs::read_to_string(".github/ISSUE_TEMPLATE/feature_request.yml")
        .expect("feature request template");

    for text in [
        "This request preserves safe defaults or explains why a change is needed.",
        "This request does not claim production readiness, hardware validation, audit status, certification, or quantum safety without evidence.",
        "This request does not include secrets, private logs, tokens, private keys, recovery codes, or unrevised screenshots.",
    ] {
        assert!(template.contains(text), "missing feature safety text {text}: {template}");
    }
}

#[test]
fn support_request_template_collects_help_context() {
    let template = std::fs::read_to_string(".github/ISSUE_TEMPLATE/support_request.yml")
        .expect("support request template");

    for text in [
        "name: Support request",
        "Project area",
        "What are you trying to do?",
        "Version, branch, or commit",
        "Environment",
        "Command, page, or workflow involved",
        "What happened?",
        "What have you already tried?",
        "Redacted context",
        "Support urgency",
        "Safety check",
    ] {
        assert!(
            template.contains(text),
            "missing support template text {text}: {template}"
        );
    }
}

#[test]
fn support_request_template_preserves_security_routing() {
    let template = std::fs::read_to_string(".github/ISSUE_TEMPLATE/support_request.yml")
        .expect("support request template");

    for text in [
        "Please do not include secrets, tokens, private keys, recovery codes, private logs, or unrevised screenshots.",
        "If this involves a private vulnerability, account compromise, safety bypass, or sensitive data exposure",
        "follow `SECURITY.md` instead",
        "I removed secrets, tokens, private keys, recovery codes, private logs, and unrevised screenshots.",
        "public support is not the place for private vulnerability details",
    ] {
        assert!(template.contains(text), "missing support safety text {text}: {template}");
    }
}

#[test]
fn documentation_issue_template_collects_docs_context() {
    let template = std::fs::read_to_string(".github/ISSUE_TEMPLATE/documentation_issue.yml")
        .expect("documentation issue template");

    for text in [
        "name: Documentation issue",
        "Project area",
        "Documentation path or page",
        "Documentation issue type",
        "What is wrong or missing?",
        "Suggested change",
        "User impact",
        "Claim and safety check",
    ] {
        assert!(
            template.contains(text),
            "missing documentation template text {text}: {template}"
        );
    }
}

#[test]
fn documentation_issue_template_preserves_claim_and_secret_safety() {
    let template = std::fs::read_to_string(".github/ISSUE_TEMPLATE/documentation_issue.yml")
        .expect("documentation issue template");

    for text in [
        "Unsupported claim or overclaim",
        "Missing status / validation / non-claims block",
        "Unsafe instruction or secret-sharing risk",
        "do not include secrets, tokens, private keys, recovery codes, private logs, or unrevised screenshots",
        "Follow `SECURITY.md` instead.",
        "If this reports an unsupported claim, I have identified the claim or page where it appears.",
    ] {
        assert!(template.contains(text), "missing documentation safety text {text}: {template}");
    }
}

#[test]
fn issue_templates_cover_project_areas() {
    for path in [
        ".github/ISSUE_TEMPLATE/bug_report.yml",
        ".github/ISSUE_TEMPLATE/feature_request.yml",
        ".github/ISSUE_TEMPLATE/support_request.yml",
        ".github/ISSUE_TEMPLATE/documentation_issue.yml",
    ] {
        let template = std::fs::read_to_string(path).expect(path);
        for area in [
            "Phase1",
            "Base1",
            "Fyr",
            "Security",
            "Crypto policy",
            "Website",
            "Community",
            "Documentation",
        ] {
            assert!(
                template.contains(area),
                "missing project area {area} in {path}: {template}"
            );
        }
    }
}
