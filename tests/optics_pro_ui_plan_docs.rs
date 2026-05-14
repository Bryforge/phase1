use std::fs;

const OPTICS_DOC: &str = "docs/ui/OPTICS_PRO_UI.md";

#[test]
fn optics_pro_doc_exists_and_defines_codename_scope() {
    let doc = fs::read_to_string(OPTICS_DOC).expect("Optics PRO UI doc should exist");

    for required in [
        "Optics PRO UI overhaul",
        "Status: design contract",
        "Optics is the codename",
        "PRO is the operator-facing interface profile",
        "Phase1 PRO operator interface",
        "top and bottom HUD rail model",
        "Phase1, Base1, and Fyr",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_pro_doc_preserves_minimal_ui_and_start_state() {
    let doc = fs::read_to_string(OPTICS_DOC).expect("Optics PRO UI doc should exist");

    for required in [
        "start clean with Phase1 edge enabled",
        "only compact top and bottom HUD rails remain persistent",
        "no large boot card by default",
        "no wide decorative banners by default",
        "no constant full-screen animation by default",
        "no command-output distortion",
        "Phase1 edge active",
        "safe operator mode visible",
        "top HUD rail online",
        "bottom HUD rail online",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_pro_doc_defines_hud_rails_and_mutation_colors() {
    let doc = fs::read_to_string(OPTICS_DOC).expect("Optics PRO UI doc should exist");

    for required in [
        "OPTICS_HUD_RAILS.md",
        "a top HUD rail for global system state",
        "a center viewport for command output and detailed reports",
        "a bottom HUD rail for command input",
        "bright blue by default",
        "active product: Phase1, Base1, or Fyr",
        "input mutation state",
        "integrity status",
        "crypto chain status",
        "bright blue",
        "cyan pulse",
        "violet",
        "amber",
        "red",
        "green",
        "All color states must have text labels",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_pro_doc_preserves_typed_command_safety_rules() {
    let doc = fs::read_to_string(OPTICS_DOC).expect("Optics PRO UI doc should exist");

    for required in [
        "changing the command text before execution",
        "hiding dangerous command text",
        "adding hidden characters",
        "writing different history than what the operator typed",
        "changing parser behavior",
        "affecting copied commands",
        "guarded-operation warning",
        "safe-mode or host-tool cue",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_pro_doc_defines_phase1_base1_fyr_integration() {
    let doc = fs::read_to_string(OPTICS_DOC).expect("Optics PRO UI doc should exist");

    for required in [
        "Phase1 integration",
        "Base1 integration",
        "Fyr integration",
        "portal, nest, ghost, analysis, and security contexts remain visible",
        "Base1 state appears as readiness, recovery, validation, artifact, hardware, or evidence context",
        "Base1 non-claims remain visible",
        "Fyr package, check, build, test, and run phases can light the HUD",
        "Fyr diagnostics remain text-first",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_pro_doc_preserves_accessibility_phases_and_non_claims() {
    let doc = fs::read_to_string(OPTICS_DOC).expect("Optics PRO UI doc should exist");

    for required in [
        "PHASE1_NO_COLOR=1",
        "PHASE1_ASCII=1",
        "PHASE1_TEST_MODE=1",
        "PHASE1_COOKED_INPUT=1",
        "Add this Optics PRO design contract.",
        "Do not change runtime UI yet.",
        "static PRO preview",
        "typed-command state",
        "not a new kernel",
        "not a new kernel, compositor, graphical desktop, terminal emulator, sandbox, security boundary",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}
