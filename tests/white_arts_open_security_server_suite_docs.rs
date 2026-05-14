use std::fs;

#[test]
fn open_security_server_suite_doc_exists_and_preserves_boundaries() {
    let doc = fs::read_to_string("docs/white-arts/OPEN_SECURITY_SERVER_SUITE.md")
        .expect("read open security server suite doc");

    for required in [
        "Status: documentation-first",
        "Runtime claim: not implemented",
        "Security claim: not hardened, not certified, not production-ready",
        "The suite must remain defensive and evidence-bound.",
        "Every service starts read-only.",
        "This plan does not claim a finished SOC, SIEM, EDR, antivirus, malware sandbox, hardened server, compliance platform, or incident-response product.",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn open_security_server_suite_lists_required_service_families() {
    let doc = fs::read_to_string("docs/white-arts/OPEN_SECURITY_SERVER_SUITE.md")
        .expect("read open security server suite doc");

    for required in [
        "posture service",
        "integrity service",
        "audit service",
        "maintenance service",
        "recovery service",
        "analysis service",
        "Fyr service",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn open_security_server_suite_preserves_disallowed_behavior_and_promotion_rule() {
    let doc = fs::read_to_string("docs/white-arts/OPEN_SECURITY_SERVER_SUITE.md")
        .expect("read open security server suite doc");

    for required in [
        "unauthorized access",
        "exploit deployment",
        "malware execution",
        "credential collection",
        "silent mutation",
        "docs",
        "tests",
        "safe failure behavior",
        "no-secret handling",
        "validation report",
        "review decision",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn open_security_server_suite_is_linked_from_index_and_roadmap() {
    let index = fs::read_to_string("docs/white-arts/README.md").expect("read White Arts index");
    let roadmap =
        fs::read_to_string("docs/white-arts/ROADMAP.md").expect("read White Arts roadmap");

    for doc in [&index, &roadmap] {
        assert!(
            doc.contains("OPEN_SECURITY_SERVER_SUITE.md"),
            "missing server suite link"
        );
    }
}
