use std::fs;

#[test]
fn white_arts_todo_doc_exists_and_preserves_posture_items() {
    let doc = fs::read_to_string("docs/white-arts/TODO.md").expect("read White Arts TODO");

    for required in [
        "living maintenance checklist",
        "Keep White Arts docs defensive-only",
        "Keep all healing language staged and review-required",
        "Keep all security claims evidence-bound",
        "Keep all first-stage commands read-only/report-only",
        "Keep unknown states reported as unknown",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_todo_lists_docs_tests_commands_and_security_work() {
    let doc = fs::read_to_string("docs/white-arts/TODO.md").expect("read White Arts TODO");

    for required in [
        "Add docs tests for protocols and guardrails",
        "Add docs tests for the TODO maintenance checklist",
        "Add `white-arts status` as a read-only command",
        "Add `white-arts audit security` as documentation-backed output",
        "Define security finding severity labels",
        "Define redaction tests for White Arts reports",
        "Define network-scope authorization rules before any server scan feature",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_todo_lists_server_base1_fyr_and_analysis_alignment() {
    let doc = fs::read_to_string("docs/white-arts/TODO.md").expect("read White Arts TODO");

    for required in [
        "Define server inventory model",
        "Define service inventory model",
        "Define network policy state model",
        "Define identity and access review model",
        "Map Base1 recovery docs into White Arts report fields",
        "Define safe Fyr scripts for White Arts checks",
        "Keep `analyze load` metadata-only",
        "Preserve `execution-state : not-executed` in every linked output",
        "Preserve `sandbox-claim : not-claimed` in every linked output",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_todo_preserves_review_cadence_and_done_definition() {
    let doc = fs::read_to_string("docs/white-arts/TODO.md").expect("read White Arts TODO");

    for required in [
        "weekly: docs links, TODO status, failed tests, stale status metadata",
        "per PR: claim language, test coverage, failure behavior, redaction, rollback notes",
        "per release candidate: full White Arts report, Base1 evidence review, security audit summary, promotion decision",
        "documentation",
        "safe-default behavior",
        "non-claim language",
        "reviewable evidence",
        "owner-visible next step",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}
