use std::fs;

const OPTICS_DOC: &str = "docs/ui/OPTICS_PRO_UI.md";
const PREVIEW_FIXTURE: &str = "docs/ui/fixtures/optics-pro-preview.txt";

#[test]
fn optics_preview_fixture_exists_and_is_linked() {
    let doc = fs::read_to_string(OPTICS_DOC).expect("Optics PRO UI doc should exist");
    let fixture =
        fs::read_to_string(PREVIEW_FIXTURE).expect("Optics PRO preview fixture should exist");

    assert!(doc.contains("fixtures/optics-pro-preview.txt"), "{doc}");
    assert!(doc.contains("read-only design evidence"), "{doc}");
    assert!(fixture.contains("OPTICS PRO PREVIEW"), "{fixture}");
    assert!(fixture.contains("runtime     not-wired"), "{fixture}");
}

#[test]
fn optics_preview_fixture_preserves_minimal_start_state() {
    let fixture =
        fs::read_to_string(PREVIEW_FIXTURE).expect("Optics PRO preview fixture should exist");

    for required in [
        "status      static-fixture",
        "mode        read-only-preview",
        "profile     PRO",
        "codename    Optics",
        "channel     phase1 edge enabled",
        "surface     minimal-main-screen",
        "persistent  bottom-hud",
        "phase1://edge/root >",
    ] {
        assert!(
            fixture.contains(required),
            "missing {required:?}: {fixture}"
        );
    }
}

#[test]
fn optics_preview_fixture_preserves_bottom_hud_rows() {
    let fixture =
        fs::read_to_string(PREVIEW_FIXTURE).expect("Optics PRO preview fixture should exist");

    for required in [
        "BOTTOM HUD",
        "product     Phase1",
        "context     root",
        "channel     edge",
        "hud-color   bright-blue",
        "state       ready",
        "input       active",
        "security    safe-mode visible",
        "host-tools  gated",
        "integrity   planned-read-only",
        "crypto      chain-status-planned",
        "base1       readiness/evidence context planned",
        "fyr         package/check/build/test/run context planned",
    ] {
        assert!(
            fixture.contains(required),
            "missing {required:?}: {fixture}"
        );
    }
}

#[test]
fn optics_preview_fixture_preserves_mutation_and_typing_labels() {
    let fixture =
        fs::read_to_string(PREVIEW_FIXTURE).expect("Optics PRO preview fixture should exist");

    for required in [
        "MUTATION STATES",
        "bright-blue ready",
        "cyan-pulse  command-being-typed",
        "violet      command-family-detected",
        "amber       guarded-operation-or-confirmation",
        "red         denied-failed-unsafe-invalid",
        "green       completed-successfully",
        "gray        inactive-disabled-unavailable",
        "raw-input   preserved",
        "history     preserves-operator-text",
        "parser      unchanged",
        "copy        unchanged",
    ] {
        assert!(
            fixture.contains(required),
            "missing {required:?}: {fixture}"
        );
    }
}

#[test]
fn optics_preview_fixture_preserves_fallbacks_and_non_claims() {
    let fixture =
        fs::read_to_string(PREVIEW_FIXTURE).expect("Optics PRO preview fixture should exist");

    for required in [
        "no-color    labels remain visible",
        "ascii       labels remain visible",
        "test-mode   deterministic output",
        "cooked      deterministic output",
        "not-kernel",
        "not-compositor",
        "not-desktop",
        "not-terminal-emulator",
        "not-sandbox",
        "not-security-boundary",
        "not-crypto-enforcement",
        "not-base1-boot-environment",
    ] {
        assert!(
            fixture.contains(required),
            "missing {required:?}: {fixture}"
        );
    }
}
