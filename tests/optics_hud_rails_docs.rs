use std::fs;

const RAILS_DOC: &str = "docs/ui/OPTICS_HUD_RAILS.md";
const OPTICS_DOC: &str = "docs/ui/OPTICS_PRO_UI.md";

#[test]
fn optics_hud_rails_doc_exists_and_defines_scope() {
    let doc = fs::read_to_string(RAILS_DOC).expect("Optics HUD rails doc should exist");
    let optics = fs::read_to_string(OPTICS_DOC).expect("Optics PRO UI doc should exist");

    for required in [
        "Optics HUD Rails",
        "Status: design contract",
        "PRO screen real estate model",
        "top HUD rail",
        "bottom HUD rail",
        "command viewport",
        "nests, portals, ghosts, crypto chains, and integrity status",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
    assert!(optics.contains("OPTICS_HUD_RAILS.md"), "{optics}");
}

#[test]
fn optics_hud_rails_doc_defines_top_bottom_and_center_responsibilities() {
    let doc = fs::read_to_string(RAILS_DOC).expect("Optics HUD rails doc should exist");

    for required in [
        "a top HUD rail for global system state",
        "a bottom HUD rail for command input",
        "The center of the screen should remain the command/output viewport.",
        "The top rail should carry stable context",
        "The bottom rail should carry live input and command activity.",
        "The center viewport is reserved for command output",
        "Persistent information belongs in the top or bottom rail",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_hud_rails_doc_preserves_feature_density_groups() {
    let doc = fs::read_to_string(RAILS_DOC).expect("Optics HUD rails doc should exist");

    for required in [
        "Feature density model",
        "root, nest, portal, ghost, analysis, recovery",
        "safe mode, host gate, trust state, denial state",
        "manifest status, file status, evidence status",
        "crypto profile, chain status, provider/service status",
        "hardware, recovery, validation, artifact evidence",
        "package, check, build, test, run, automation",
        "typed command, command family, mutation color",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_hud_rails_doc_covers_contexts_crypto_integrity_and_devices() {
    let doc = fs::read_to_string(RAILS_DOC).expect("Optics HUD rails doc should exist");

    for required in [
        "Nests, portals, and ghosts",
        "TOP  Phase1 edge | PRO | ctx=root > nest:0/1 | portal:none | ghost:none | trust=safe",
        "Crypto chain and integrity indicators should be compact by default.",
        "crypto=chain-planned integrity=not-checked",
        "A rail indicator must not imply runtime enforcement",
        "mobile: use one-line top rail and one-line bottom rail",
        "laptop: use one compact top rail and one or two bottom rows",
        "desktop: allow a two-row top rail and two-row bottom rail",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_hud_rails_doc_preserves_color_accessibility_and_safety_rules() {
    let doc = fs::read_to_string(RAILS_DOC).expect("Optics HUD rails doc should exist");

    for required in [
        "The default rail color is bright blue.",
        "Every color cue must also have a text label.",
        "hide command output",
        "rewrite command input",
        "change parser behavior",
        "change command history",
        "hide dangerous commands",
        "imply security hardening by visual style",
        "imply crypto enforcement from a planned crypto chain",
        "imply total system integrity from a hash check",
        "imply Base1 boot readiness without evidence",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn optics_hud_rails_doc_preserves_runtime_phases_and_non_claims() {
    let doc = fs::read_to_string(RAILS_DOC).expect("Optics HUD rails doc should exist");

    for required in [
        "Phase 1: HUD rail contract",
        "Phase 2: static rail preview",
        "Phase 3: renderer module",
        "Phase 4: shell preview command",
        "Phase 5: live activation gate",
        "not a compositor",
        "terminal emulator",
        "security boundary",
        "cryptographic enforcement layer",
        "system integrity guarantee",
        "Base1 boot environment",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}
