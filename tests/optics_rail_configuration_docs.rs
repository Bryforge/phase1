use std::fs;

const RAIL_CONFIG_DOC: &str = "docs/ui/OPTICS_RAIL_CONFIGURATION.md";
const OPTICS_DOC: &str = "docs/ui/OPTICS_PRO_UI.md";
const COMMAND_DOC: &str = "docs/ui/OPTICS_COMMAND_SURFACE.md";

#[test]
fn optics_rail_configuration_doc_exists_and_defines_scope() {
    let doc = fs::read_to_string(RAIL_CONFIG_DOC).expect("Optics rail configuration doc should exist");

    for required in [
        "# Optics Rail Configuration",
        "Status: configuration contract",
        "Optics PRO active shell rails",
        "A TOP RAIL",
        "B COMMAND RAIL",
        "C STATUS HUD",
        "D BOTTOM HUD",
        "blank line between B and C",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_rail_configuration_documents_environment_controls() {
    let doc = fs::read_to_string(RAIL_CONFIG_DOC).expect("Optics rail configuration doc should exist");

    for required in [
        "PHASE1_OPTICS_PRO=1",
        "PHASE1_OPTICS_PRO=0",
        "PHASE1_LEGACY_SHELL_UI=1",
        "PHASE1_OPTICS_COMMAND_GAP_LINES=<number>",
        "minimum: 1",
        "maximum: 8",
        "Values below the minimum clamp to 1.",
        "Values above the maximum clamp to 8.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_rail_configuration_preserves_layer_responsibilities() {
    let doc = fs::read_to_string(RAIL_CONFIG_DOC).expect("Optics rail configuration doc should exist");

    for required in [
        "product",
        "channel",
        "profile",
        "root/nest/portal/ghost context",
        "active typing area",
        "result state",
        "mutation state",
        "integrity state",
        "crypto chain state",
        "Base1 state",
        "Fyr state",
        "persistent operator cues",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_rail_configuration_preserves_color_policy() {
    let doc = fs::read_to_string(RAIL_CONFIG_DOC).expect("Optics rail configuration doc should exist");

    for required in [
        "A TOP RAIL: cyan",
        "B COMMAND RAIL label: blue",
        "typed/pasted operator input: bright yellow only",
        "C STATUS HUD: green",
        "D BOTTOM HUD: magenta",
        "denied/failed/unsafe states: red",
        "Bright yellow is reserved for operator input only.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_rail_configuration_preserves_dynamic_spacing_and_non_claims() {
    let doc = fs::read_to_string(RAIL_CONFIG_DOC).expect("Optics rail configuration doc should exist");

    for required in [
        "B-to-C gap",
        "future multi-line editing",
        "Shift+Enter",
        "terminal-supported equivalent",
        "Live interactive gap resizing remains future work.",
        "does not claim",
        "compositor behavior",
        "terminal emulator behavior",
        "sandboxing",
        "security boundary enforcement",
        "crypto enforcement",
        "system integrity guarantees",
        "Base1 boot environment completion",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_rail_configuration_is_linked_from_existing_optics_docs() {
    let optics = fs::read_to_string(OPTICS_DOC).expect("Optics PRO UI doc should exist");
    let command = fs::read_to_string(COMMAND_DOC).expect("Optics command surface doc should exist");

    assert!(
        optics.contains("OPTICS_RAIL_CONFIGURATION.md")
            || command.contains("OPTICS_RAIL_CONFIGURATION.md"),
        "rail configuration doc should be linked from existing Optics docs"
    );
}
