use std::fs;

const REAL_SYSTEM_GUIDE: &str = "docs/project/REAL_SYSTEM_DEVELOPMENT_GUIDE.md";
const TRILATERAL_GUIDE: &str = "docs/project/TRILATERAL_PHASE_MOVEMENT.md";
const DOCS_INDEX: &str = "docs/README.md";
const REPOSITORY_NAVIGATION: &str = "docs/REPOSITORY_NAVIGATION.md";

#[test]
fn real_system_development_guide_exists_and_defines_direction() {
    let doc = fs::read_to_string(REAL_SYSTEM_GUIDE).expect("real system guide should exist");

    for required in [
        "Phase1 Real-System Development Guide",
        "not being built as a simulator game",
        "Simulation is a proving layer, not the destination.",
        "Phase1-first operator environment",
        "Base1 evidence-chain boot foundation",
        "Fyr as the native Phase1 language",
        "Optics PRO as the operator lens",
        "A concept becomes real only when it moves through evidence",
        "Do not jump directly to live movement.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn real_system_development_guide_preserves_evidence_ladder_and_roles() {
    let doc = fs::read_to_string(REAL_SYSTEM_GUIDE).expect("real system guide should exist");

    for required in [
        "Level R0: Concept",
        "Level R1: Fixture",
        "Level R2: Tested model",
        "Level R3: Read-only command surface",
        "Level R4: Runtime preview",
        "Level R5: Guarded experiment",
        "Level R6: VM evidence",
        "Level R7: Real-device read-only evidence",
        "Level R8: Real-device execution evidence",
        "Level R9: Promoted real-system capability",
        "Phase1 | Operator environment",
        "Base1 | Real-system boot",
        "Fyr | Native Phase1 language",
        "Optics PRO | Operator lens",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn real_system_development_guide_preserves_concept_notes_and_boundaries() {
    let doc = fs::read_to_string(REAL_SYSTEM_GUIDE).expect("real system guide should exist");

    for required in [
        "Trilateral Phase Movement",
        "Plane A: spatial movement",
        "Plane B: state movement",
        "Plane C: safety movement",
        "safe-portal=planned",
        "rollback=available",
        "dark_phase=off",
        "host-effect=none",
        "external-effect=none",
        "candidate-only",
        "approval-gated",
        "Base1 claims need Base1 evidence.",
        "Does this make the system more real without becoming unsafe or dishonest?",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn trilateral_phase_movement_guide_exists_and_defines_planes() {
    let doc = fs::read_to_string(TRILATERAL_GUIDE).expect("trilateral guide should exist");

    for required in [
        "Trilateral Phase Movement",
        "design contract and fixture-first implementation guide",
        "Spatial plane",
        "State plane",
        "Safety plane",
        "origin=0/0",
        "route=ROOT",
        "axis=ROOT",
        "path=ROOT>0/0",
        "breadcrumb=ROOT",
        "trace=trace-preview",
        "host-effect=none",
        "external-effect=none",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn trilateral_phase_movement_guide_preserves_preview_only_transition_contract() {
    let doc = fs::read_to_string(TRILATERAL_GUIDE).expect("trilateral guide should exist");

    for required in [
        "The first transition engine should be pure.",
        "current_phase_state",
        "movement_direction",
        "next_phase_state",
        "transition_result",
        "phase move preview <direction>",
        "recovery-executed=no",
        "origin-mutated=no",
        "This command should not perform live movement.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn trilateral_phase_movement_guide_preserves_non_claims() {
    let doc = fs::read_to_string(TRILATERAL_GUIDE).expect("trilateral guide should exist");

    for required in [
        "does not currently provide live Phase movement",
        "origin mutation",
        "recovery execution",
        "runtime domain mutation",
        "host mutation",
        "external effects",
        "sandboxing",
        "crypto enforcement",
        "system integrity guarantees",
        "Base1 boot readiness",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn real_system_guides_are_linked_from_indexes() {
    let docs_index = fs::read_to_string(DOCS_INDEX).expect("docs index should exist");
    let repository_navigation =
        fs::read_to_string(REPOSITORY_NAVIGATION).expect("repository navigation should exist");

    for required in [
        "REAL_SYSTEM_DEVELOPMENT_GUIDE.md",
        "TRILATERAL_PHASE_MOVEMENT.md",
    ] {
        assert!(
            docs_index.contains(required),
            "docs index missing {required:?}: {docs_index}"
        );
        assert!(
            repository_navigation.contains(required),
            "repository navigation missing {required:?}: {repository_navigation}"
        );
    }
}
