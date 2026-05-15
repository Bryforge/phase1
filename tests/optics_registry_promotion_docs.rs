use std::fs;

const PROMOTION_DOC: &str = "docs/ui/OPTICS_REGISTRY_PROMOTION.md";
const COMMAND_DOC: &str = "docs/ui/OPTICS_COMMAND_SURFACE.md";

#[test]
fn optics_registry_promotion_doc_exists_and_is_linked() {
    let doc =
        fs::read_to_string(PROMOTION_DOC).expect("Optics registry promotion doc should exist");
    let command = fs::read_to_string(COMMAND_DOC).expect("Optics command surface doc should exist");

    for required in [
        "Optics Registry Promotion",
        "Status: initial registry promotion",
        "command registry, help, completion, and manual visibility",
        "Optics currently works through the read-only WASI-lite plugin route.",
        "Optics is now initially promoted into the command registry for discovery.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
    assert!(
        command.contains("OPTICS_REGISTRY_PROMOTION.md"),
        "{command}"
    );
}

#[test]
fn optics_registry_promotion_doc_preserves_target_command_shape() {
    let doc =
        fs::read_to_string(PROMOTION_DOC).expect("Optics registry promotion doc should exist");

    for required in [
        "optics preview",
        "optics rails",
        "optics status",
        "optics [preview|rails|status|help]",
        "aliases: `pro`, `hudrails`",
        "category: `user`",
        "capability: `none`",
        "read-only Optics PRO preview and HUD rail preview surface",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_registry_promotion_doc_preserves_discovery_targets() {
    let doc =
        fs::read_to_string(PROMOTION_DOC).expect("Optics registry promotion doc should exist");

    for required in [
        "help optics",
        "man optics",
        "complete opt",
        "complete pro",
        "complete hud",
        "capabilities",
        "`lookup(\"optics\")` resolves to `optics`",
        "`canonical_name(\"pro\")` resolves to `optics`",
        "`canonical_name(\"hudrails\")` resolves to `optics`",
        "`completions(\"opt\")` contains `optics`",
        "`completions(\"pro\")` contains `pro`",
        "`man_page(\"optics\")` contains `optics [preview|rails|status|help]`",
        "`capabilities_report()` lists `optics` with capability `none`",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_registry_promotion_doc_preserves_status_surface() {
    let doc =
        fs::read_to_string(PROMOTION_DOC).expect("Optics registry promotion doc should exist");

    for required in [
        "Status surface",
        "`optics status` reports the current preview state",
        "preview-only mode",
        "Rust static renderer source",
        "top rail preview readiness",
        "bottom rail preview readiness",
        "live HUD disabled state",
        "explicit activation gate requirement",
        "input, history, and parser non-mutation labels",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_registry_promotion_doc_preserves_safety_rules_and_non_claims() {
    let doc =
        fs::read_to_string(PROMOTION_DOC).expect("Optics registry promotion doc should exist");

    for required in [
        "Registry promotion does not activate live HUD rails.",
        "enable live HUD rails",
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
