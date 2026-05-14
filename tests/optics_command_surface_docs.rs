use std::fs;

const COMMAND_DOC: &str = "docs/ui/OPTICS_COMMAND_SURFACE.md";
const OPTICS_DOC: &str = "docs/ui/OPTICS_PRO_UI.md";

#[test]
fn optics_command_surface_doc_exists_and_is_linked() {
    let doc = fs::read_to_string(COMMAND_DOC).expect("Optics command surface doc should exist");
    let optics = fs::read_to_string(OPTICS_DOC).expect("Optics PRO UI doc should exist");

    for required in [
        "Optics Command Surface",
        "Status: command surface contract",
        "read-only Optics commands",
        "WASI-lite preview routing",
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
        "optics preview",
        "optics rails",
        "optics status",
        "WASI-lite plugin path",
        "capability `none`",
        "minimal main screen",
        "Phase1 edge enabled",
        "bottom HUD preview",
        "top HUD rail",
        "center viewport",
        "bottom HUD rail",
        "preview-only mode",
        "Rust static renderer source",
        "live HUD disabled state",
        "explicit activation gate requirement",
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
        "hudrails",
        "optics preview",
        "pro",
        "optics status",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_command_surface_preserves_registry_expectation() {
    let doc = fs::read_to_string(COMMAND_DOC).expect("Optics command surface doc should exist");

    for required in [
        "Future work should promote Optics into the regular command registry",
        "help, completions, and manuals",
        "optics [preview|rails|status|help]",
        "Until that registry row exists, the WASI-lite route remains the safe preview path.",
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
        "Live Optics PRO HUD activation remains future work.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}
