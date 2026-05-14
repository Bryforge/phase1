use std::fs;

#[test]
fn white_arts_docs_exist_and_define_defensive_scope() {
    for path in [
        "docs/white-arts/README.md",
        "docs/white-arts/ROADMAP.md",
        "docs/white-arts/NOMINAL_STATE_MATRIX.md",
        "docs/white-arts/INTEGRITY_VALIDATION_PLAN.md",
        "docs/white-arts/HEALING_MAINTENANCE_MODEL.md",
        "docs/white-arts/SECURITY_AUDIT_MOVEMENT.md",
        "docs/white-arts/REPORT_TEMPLATE.md",
    ] {
        assert!(fs::metadata(path).is_ok(), "missing {path}");
    }

    let readme = fs::read_to_string("docs/white-arts/README.md").expect("read White Arts README");
    for required in [
        "defensive-care track",
        "integrity verification",
        "healing proposals",
        "security audit support",
        "autonomous repair power",
        "malware-safety claims",
        "production forensic admissibility claims",
        "evidence-bound-maintenance",
    ] {
        assert!(readme.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_roadmap_preserves_promotion_ladder_and_non_claims() {
    let roadmap =
        fs::read_to_string("docs/white-arts/ROADMAP.md").expect("read White Arts roadmap");
    for required in [
        "planned -> documented -> read-only checked -> locally validated -> CI validated -> reviewed -> release eligible",
        "W1 — Doctrine and vocabulary",
        "W2 — Nominal-state matrix",
        "W3 — Integrity validation layer",
        "W4 — Healing and maintenance model",
        "W5 — Security audit movement",
        "W6 — Read-only command surface",
        "W7 — Evidence reports and promotion gates",
        "W8 — Base1 and recovery alignment",
        "W9 — Fyr integration",
        "W10 — First implementation milestones",
        "production hardened",
        "malware-safe",
        "forensic-admissible",
        "certified sandbox",
        "installer-ready",
        "daily-driver ready",
        "cryptographically complete",
    ] {
        assert!(roadmap.contains(required), "missing {required:?}");
    }
}
