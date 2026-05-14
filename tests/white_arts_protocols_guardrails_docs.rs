use std::fs;

#[test]
fn white_arts_protocols_doc_exists_and_defines_standing_scope() {
    let doc = fs::read_to_string("docs/white-arts/PROTOCOLS_AND_GUARDRAILS.md")
        .expect("read White Arts protocols and guardrails doc");

    for required in [
        "standing White Arts protocols and security guardrails",
        "Phase1 / Base1 / Fyr defensive maintenance",
        "not hardened, not certified, not production-ready",
        "Evidence before claim",
        "Read-only first",
        "No-execute analysis boundary",
        "Explicit operator consent for mutation",
        "Secret hygiene and privacy",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_protocols_preserve_required_evidence_and_promotion_rules() {
    let doc = fs::read_to_string("docs/white-arts/PROTOCOLS_AND_GUARDRAILS.md")
        .expect("read White Arts protocols and guardrails doc");

    for required in [
        "implemented behavior",
        "test coverage",
        "negative tests",
        "safe failure behavior",
        "validation report",
        "review notes",
        "rollback or recovery notes",
        "planned -> documented -> read-only checked -> locally validated -> CI validated -> reviewed -> release eligible",
        "Promotion blockers",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_protocols_block_unsafe_mutation_secrets_and_overclaims() {
    let doc = fs::read_to_string("docs/white-arts/PROTOCOLS_AND_GUARDRAILS.md")
        .expect("read White Arts protocols and guardrails doc");

    for required in [
        "silent writes",
        "host mutation",
        "service restart",
        "firewall mutation",
        "privilege escalation",
        "credential access",
        "automatic promotion",
        "passwords",
        "personal access tokens",
        "SSH private keys",
        "API keys",
        "ransomware-proof",
        "production-ready",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_protocols_preserve_analysis_and_server_suite_boundaries() {
    let doc = fs::read_to_string("docs/white-arts/PROTOCOLS_AND_GUARDRAILS.md")
        .expect("read White Arts protocols and guardrails doc");

    for required in [
        "execution-state  : not-executed",
        "host-execution   : disabled",
        "sandbox-claim    : not-claimed",
        "claim-boundary   : metadata-only",
        "open-source-first component preference",
        "read-only inventory before any agent behavior",
        "no unauthorized network scanning",
        "report-only default mode",
        "data flow",
        "retention policy",
        "failure behavior",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_protocols_are_linked_from_index_and_roadmap() {
    let index = fs::read_to_string("docs/white-arts/README.md").expect("read White Arts index");
    let roadmap =
        fs::read_to_string("docs/white-arts/ROADMAP.md").expect("read White Arts roadmap");

    for doc in [&index, &roadmap] {
        assert!(
            doc.contains("PROTOCOLS_AND_GUARDRAILS.md"),
            "protocols doc is not linked"
        );
        assert!(doc.contains("TODO.md"), "TODO doc is not linked");
    }
}
