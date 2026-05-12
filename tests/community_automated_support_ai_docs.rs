#[test]
fn automated_support_ai_roadmap_defines_goal_and_scope() {
    let roadmap = std::fs::read_to_string("docs/community/AUTOMATED_SUPPORT_AI_ROADMAP.md")
        .expect("automated support AI roadmap");

    assert!(
        roadmap.contains("Bryforge automated support AI roadmap"),
        "{roadmap}"
    );
    assert!(
        roadmap.contains("automated AI-assisted support for Phase1, Base1, Fyr"),
        "{roadmap}"
    );

    for area in [
        "Phase1",
        "Base1",
        "Fyr",
        "Security",
        "Crypto policy",
        "Community",
    ] {
        assert!(
            roadmap.contains(area),
            "missing support AI area {area}: {roadmap}"
        );
    }
}

#[test]
fn automated_support_ai_roadmap_preserves_security_and_privacy_rules() {
    let roadmap = std::fs::read_to_string("docs/community/AUTOMATED_SUPPORT_AI_ROADMAP.md")
        .expect("automated support AI roadmap");

    for text in [
        "as secure as possible while maintaining practical usability",
        "passwords",
        "tokens",
        "private keys",
        "recovery codes",
        "Apple ID or email credentials",
        "private logs",
        "unrevised screenshots",
        "never ask a user to paste secrets",
    ] {
        assert!(
            roadmap.contains(text),
            "missing security/privacy rule {text}: {roadmap}"
        );
    }
}

#[test]
fn automated_support_ai_roadmap_preserves_non_goals() {
    let roadmap = std::fs::read_to_string("docs/community/AUTOMATED_SUPPORT_AI_ROADMAP.md")
        .expect("automated support AI roadmap");

    for text in [
        "replace `SECURITY.md` for vulnerability reporting",
        "claim Phase1 is a secure OS replacement",
        "claim Base1 is a released bootable daily-driver image",
        "claim Fyr is production-ready",
        "claim cryptographic completeness, audit completion, certification, quantum safety, or hardware validation without evidence",
        "run destructive host commands automatically",
        "request secrets or private credentials",
        "silently create issues, PRs, or public posts without operator review",
    ] {
        assert!(roadmap.contains(text), "missing support AI non-goal {text}: {roadmap}");
    }
}

#[test]
fn automated_support_ai_roadmap_defines_phased_capabilities() {
    let roadmap = std::fs::read_to_string("docs/community/AUTOMATED_SUPPORT_AI_ROADMAP.md")
        .expect("automated support AI roadmap");

    for phase in [
        "documentation assistant",
        "support triage assistant",
        "redaction assistant",
        "maintainer assistant",
        "forum integration",
    ] {
        assert!(
            roadmap.contains(phase),
            "missing support AI phase {phase}: {roadmap}"
        );
    }
}

#[test]
fn automated_support_ai_roadmap_prefers_read_only_diagnostics() {
    let roadmap = std::fs::read_to_string("docs/community/AUTOMATED_SUPPORT_AI_ROADMAP.md")
        .expect("automated support AI roadmap");

    for command in [
        "sh phase1 version",
        "sh phase1 doctor",
        "sh phase1 selftest",
        "sh scripts/quality-check.sh quick",
        "sh scripts/quality-check.sh base1-docs",
        "sh scripts/quality-check.sh security-crypto-docs",
    ] {
        assert!(
            roadmap.contains(command),
            "missing read-only diagnostic command {command}: {roadmap}"
        );
    }
}

#[test]
fn automated_support_ai_roadmap_defines_escalation_and_issue_routing() {
    let roadmap = std::fs::read_to_string("docs/community/AUTOMATED_SUPPORT_AI_ROADMAP.md")
        .expect("automated support AI roadmap");

    for text in [
        "Escalation rules",
        "a bug is reproducible",
        "Base1 recovery, image, boot, rollback, or hardware guidance is involved",
        "crypto policy or provider behavior is involved",
        "logs mention credentials, keys, tokens, or private data",
        "Public issue routing",
        "bug report for reproducible defects",
        "feature request for proposed enhancements",
        "support request for troubleshooting",
        "documentation issue for docs gaps",
        "crypto/security-policy proposal for cryptographic policy planning",
    ] {
        assert!(
            roadmap.contains(text),
            "missing escalation/routing text {text}: {roadmap}"
        );
    }
}

#[test]
fn automated_support_ai_roadmap_is_linked_from_community_index() {
    let index = std::fs::read_to_string("docs/community/README.md").expect("community index");

    assert!(index.contains("AUTOMATED_SUPPORT_AI_ROADMAP.md"), "{index}");
    assert!(
        index.contains("AI-assisted technical support roadmap"),
        "{index}"
    );
    assert!(
        index.contains("The support AI should not replace maintainers"),
        "{index}"
    );
}

#[test]
fn automated_support_ai_roadmap_preserves_non_claims() {
    let roadmap = std::fs::read_to_string("docs/community/AUTOMATED_SUPPORT_AI_ROADMAP.md")
        .expect("automated support AI roadmap");

    assert!(
        roadmap.contains("does not create or deploy an automated support AI by itself"),
        "{roadmap}"
    );
    assert!(
        roadmap.contains("does not replace maintainers"),
        "{roadmap}"
    );
    assert!(roadmap.contains("GitHub issues"), "{roadmap}");
    assert!(roadmap.contains("`SECURITY.md`"), "{roadmap}");
    assert!(roadmap.contains("official documentation"), "{roadmap}");
}
