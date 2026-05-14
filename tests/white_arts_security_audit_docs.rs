use std::fs;

#[test]
fn white_arts_security_audit_movement_lists_required_audit_scopes() {
    let audit = fs::read_to_string("docs/white-arts/SECURITY_AUDIT_MOVEMENT.md")
        .expect("read White Arts security audit movement");

    for required in [
        "WA-SEC-001",
        "Threat model refresh",
        "WA-SEC-002",
        "Command capability metadata",
        "WA-SEC-003",
        "Safe-mode and host-trust gates",
        "WA-SEC-004",
        "Secret redaction",
        "WA-SEC-008",
        "Program Loading + Analysis",
        "WA-SEC-009",
        "Fyr package/runtime safety",
        "WA-SEC-010",
        "Base1 recovery and dry-runs",
        "WA-SEC-011",
        "Crypto policy readiness",
        "WA-SEC-012",
        "CI and release metadata",
    ] {
        assert!(audit.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_security_audit_preserves_non_claims_and_report_fields() {
    let audit = fs::read_to_string("docs/white-arts/SECURITY_AUDIT_MOVEMENT.md")
        .expect("read White Arts security audit movement");

    for required in [
        "audit id",
        "scope",
        "risk level",
        "required evidence",
        "commands run",
        "findings",
        "blocked claims",
        "required fixes",
        "reviewer notes",
        "not production hardened",
        "not malware-safe",
        "not forensic-admissible",
        "not a certified sandbox",
        "not installer-ready",
        "not daily-driver ready",
        "not cryptographically complete",
    ] {
        assert!(audit.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_report_template_preserves_reviewable_report_shape() {
    let template = fs::read_to_string("docs/white-arts/REPORT_TEMPLATE.md")
        .expect("read White Arts report template");

    for required in [
        "white-arts report",
        "report id",
        "systems checked",
        "commands run",
        "nominal findings",
        "integrity findings",
        "security findings",
        "repair candidates",
        "blocked claims",
        "mutation",
        "host-execution   : disabled by default",
        "repair-policy    : staged-candidate-only",
        "rollback path",
        "promotion decision",
        "claim-boundary   : evidence-bound-maintenance",
        "not a certification",
    ] {
        assert!(template.contains(required), "missing {required:?}");
    }
}
