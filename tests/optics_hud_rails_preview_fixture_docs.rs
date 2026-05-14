use std::fs;

const RAILS_DOC: &str = "docs/ui/OPTICS_HUD_RAILS.md";
const RAILS_FIXTURE: &str = "docs/ui/fixtures/optics-hud-rails-preview.txt";

#[test]
fn optics_hud_rails_preview_fixture_exists_and_is_linked() {
    let doc = fs::read_to_string(RAILS_DOC).expect("Optics HUD rails doc should exist");
    let fixture =
        fs::read_to_string(RAILS_FIXTURE).expect("Optics HUD rails preview fixture should exist");

    assert!(
        doc.contains("fixtures/optics-hud-rails-preview.txt"),
        "{doc}"
    );
    assert!(doc.contains("read-only design evidence"), "{doc}");
    assert!(fixture.contains("OPTICS HUD RAILS PREVIEW"), "{fixture}");
    assert!(fixture.contains("layout      top-rail center-viewport bottom-rail"), "{fixture}");
    assert!(fixture.contains("runtime     not-wired"), "{fixture}");
}

#[test]
fn optics_hud_rails_preview_fixture_preserves_top_center_bottom_layout() {
    let fixture =
        fs::read_to_string(RAILS_FIXTURE).expect("Optics HUD rails preview fixture should exist");

    for required in [
        "TOP HUD",
        "product     Phase1",
        "channel     edge",
        "profile     PRO",
        "context     root > nest:0/1 > portal:none > ghost:none",
        "trust       safe/armed",
        "security    safe-mode host-gated",
        "integrity   not-checked",
        "crypto      chain-planned",
        "CENTER VIEWPORT",
        "role        command-output",
        "chrome      none-permanent",
        "rule        center remains primary workspace",
        "BOTTOM HUD",
        "hud-color   bright-blue",
        "input       active",
        "mutation    none",
        "active-task idle",
    ] {
        assert!(
            fixture.contains(required),
            "missing {required:?}: {fixture}"
        );
    }
}

#[test]
fn optics_hud_rails_preview_fixture_preserves_feature_groups() {
    let fixture =
        fs::read_to_string(RAILS_FIXTURE).expect("Optics HUD rails preview fixture should exist");

    for required in [
        "FEATURE GROUPS",
        "context     root nest portal ghost analysis recovery",
        "security    safe-mode host-gate trust denial",
        "integrity   manifest file evidence",
        "crypto      profile chain provider-service",
        "base1       hardware recovery validation artifact evidence",
        "fyr         package check build test run automation",
        "command     typed-input family mutation",
        "result      ok changed denied failed warning",
    ] {
        assert!(
            fixture.contains(required),
            "missing {required:?}: {fixture}"
        );
    }
}

#[test]
fn optics_hud_rails_preview_fixture_preserves_device_rules_and_non_claims() {
    let fixture =
        fs::read_to_string(RAILS_FIXTURE).expect("Optics HUD rails preview fixture should exist");

    for required in [
        "DEVICE RULES",
        "mobile      one-line-top one-line-bottom",
        "laptop      compact-top one-or-two-bottom",
        "desktop     two-row-top two-row-bottom",
        "ascii       labels-visible",
        "no-color    labels-visible",
        "NON-CLAIMS",
        "not-runtime-wired",
        "not-compositor",
        "not-terminal-emulator",
        "not-sandbox",
        "not-security-boundary",
        "not-crypto-enforcement",
        "not-system-integrity-guarantee",
        "not-base1-boot-environment",
    ] {
        assert!(
            fixture.contains(required),
            "missing {required:?}: {fixture}"
        );
    }
}
