use std::fs;

const COMMAND_DOC: &str = "docs/ui/OPTICS_COMMAND_SURFACE.md";
const OPTICS_DOC: &str = "docs/ui/OPTICS_PRO_UI.md";

#[test]
fn optics_command_surface_doc_exists_and_is_linked() {
    let doc = fs::read_to_string(COMMAND_DOC).expect("Optics command surface doc should exist");
    let optics = fs::read_to_string(OPTICS_DOC).expect("Optics PRO UI doc should exist");

    for required in [
        "Optics Command Surface",
        "Status: implemented read-only command surface checkpoint",
        "Optics PRO preview, rails, status, device previews",
        "Optics PRO",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
    assert!(optics.contains("OPTICS_COMMAND_SURFACE.md"), "{optics}");
}

#[test]
fn optics_command_surface_preserves_current_preview_routes() {
    let doc = fs::read_to_string(COMMAND_DOC).expect("Optics command surface doc should exist");

    for required in [
        "optics help",
        "optics preview",
        "optics rails",
        "optics status",
        "optics device mobile",
        "optics device laptop",
        "optics device desktop",
        "optics device terminal",
        "read-only preview work",
        "current rail renderer",
        "visible status labels",
        "before live movement exists",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_command_surface_preserves_device_preview_profiles() {
    let doc = fs::read_to_string(COMMAND_DOC).expect("Optics command surface doc should exist");

    for required in [
        "displays the rail preview for a selected device profile",
        "Supported read-only device previews",
        "mobile",
        "laptop",
        "desktop",
        "terminal",
        "optics device mobile",
        "optics device laptop",
        "optics device desktop",
        "optics device terminal",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_command_surface_preserves_phase_visibility_labels() {
    let doc = fs::read_to_string(COMMAND_DOC).expect("Optics command surface doc should exist");

    for required in [
        "origin=0/0",
        "route=ROOT",
        "axis=ROOT",
        "path=ROOT>0/0",
        "breadcrumb=ROOT",
        "trace=trace-preview",
        "safe-portal=planned",
        "rollback=available",
        "health=nominal",
        "risk=low",
        "lock=open",
        "dark_phase=off",
        "host-effect=none",
        "external-effect=none",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_command_surface_preserves_discovery_and_execution_boundaries() {
    let doc = fs::read_to_string(COMMAND_DOC).expect("Optics command surface doc should exist");

    for required in [
        "Discovery versus execution",
        "complete opt",
        "complete pro",
        "complete hud",
        "Those are discovery checks. They do not execute the preview surface.",
        "optics rails",
        "optics preview",
        "optics status",
        "optics device terminal",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_command_surface_preserves_completion_checkpoint() {
    let doc = fs::read_to_string(COMMAND_DOC).expect("Optics command surface doc should exist");

    for required in [
        "Completion checkpoint",
        "optics help",
        "optics preview",
        "optics status",
        "optics rails",
        "optics device mobile",
        "optics device laptop",
        "optics device desktop",
        "optics device terminal",
        "cargo test -p phase1 --test optics_command_surface_complete",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_command_surface_preserves_safety_rules_and_non_claims() {
    let doc = fs::read_to_string(COMMAND_DOC).expect("Optics command surface doc should exist");

    for required in [
        "activate live HUD rails",
        "mutate shell state",
        "change parser behavior",
        "change command history",
        "hide command output",
        "claim a compositor",
        "claim a terminal emulator",
        "claim a sandbox",
        "claim a security boundary",
        "claim crypto enforcement",
        "claim a system integrity guarantee",
        "claim a Base1 boot environment",
        "claim live Phase movement",
        "claim origin mutation",
        "claim safe-portal recovery execution",
        "claim runtime domain mutation",
        "claim host mutation",
        "claim external effects",
        "Live Optics PRO HUD activation remains future work.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}
