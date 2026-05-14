use std::fs;

#[test]
fn white_arts_nominal_state_matrix_lists_required_fields_and_surfaces() {
    let matrix = fs::read_to_string("docs/white-arts/NOMINAL_STATE_MATRIX.md")
        .expect("read White Arts nominal-state matrix");

    for required in [
        "system",
        "owner surface",
        "nominal signal",
        "test command",
        "integrity gate",
        "failure behavior",
        "recovery path",
        "claim boundary",
        "Phase1 boot selector",
        "Phase1 shell",
        "Program Loading + Analysis",
        "Fyr",
        "Base1 docs",
        "Base1 recovery",
        "Security/crypto docs",
        "CI/quality",
    ] {
        assert!(matrix.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_integrity_plan_preserves_read_only_validation_and_promotion_rules() {
    let plan = fs::read_to_string("docs/white-arts/INTEGRITY_VALIDATION_PLAN.md")
        .expect("read White Arts integrity validation plan");

    for required in [
        "read-only first",
        "required docs exist",
        "release metadata agrees across public surfaces",
        "Base1 links and inventories are intact",
        "security and crypto docs preserve guardrails",
        "analysis commands preserve no-execute boundaries",
        "repair-policy    : staged-candidate-only",
        "blocked-claims",
        "Passing integrity checks does not make the project production hardened",
    ] {
        assert!(plan.contains(required), "missing {required:?}");
    }
}

#[test]
fn white_arts_healing_model_blocks_silent_or_unsafe_repair() {
    let model = fs::read_to_string("docs/white-arts/HEALING_MAINTENANCE_MODEL.md")
        .expect("read White Arts healing model");

    for required in [
        "staged defensive repair planning",
        "diagnose before repair",
        "generate candidate patches on a branch",
        "require review before promotion",
        "silently modify tracked files",
        "mutate the host system without explicit gates",
        "collect credentials, tokens, cookies, keys, or recovery codes",
        "auto-merge repair branches",
        "observe -> classify -> explain -> plan -> stage candidate -> validate -> review -> promote or discard",
    ] {
        assert!(model.contains(required), "missing {required:?}");
    }
}
