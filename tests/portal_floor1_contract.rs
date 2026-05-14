use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn portal_contract_defines_command_surface() {
    let path = "docs/portal/FLOOR1_PORTALS.md";

    for command in [
        "portal status",
        "portal list",
        "portal open <name>",
        "portal enter <name>",
        "portal leave",
        "portal close <name>",
        "portal inspect <name>",
        "portal help",
    ] {
        assert_contains(path, command);
    }
}

#[test]
fn portal_fixture_preserves_floor1_network_default_and_claim_boundary() {
    let path = "docs/portal/fixtures/floor1-portal-status-ok.txt";

    for row in [
        "phase1 portals",
        "mode              : read-only status",
        "floor             : floor1",
        "active-portal     : root",
        "open-portals      : root",
        "portal-layer      : workspace/session",
        "split-mode        : local-view",
        "network-owner     : floor1",
        "network-mode      : denied",
        "network-default   : denied",
        "network           : blocked",
        "claim-boundary    : workspace-context-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn portal_contract_keeps_future_network_modes_staged() {
    let path = "docs/portal/FLOOR1_PORTALS.md";

    for row in [
        "denied",
        "local-only",
        "brokered-egress",
        "local-link",
        "Default mode. The portal has no network access.",
        "future network access must be brokered through `floor1`",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn portal_fixture_preserves_user_control_cues() {
    let path = "docs/portal/fixtures/floor1-portal-status-ok.txt";

    for row in [
        "first-time   : portals are doors into separate work areas",
        "keyboard     : direct one-line commands",
        "mobile       : compact rows, no wide tables",
        "no-color     : every state has a text label",
        "ascii        : symbols are optional; text is authoritative",
        "recovery     : floor1 owns recovery and policy",
        "unknown      : unknown portal actions are no-op and help-guided",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn portal_contract_lists_runtime_staging_and_completion_rule() {
    let path = "docs/portal/FLOOR1_PORTALS.md";

    for row in [
        "Add read-only runtime status/list/help.",
        "Add runtime tests for status/list/help and unknown-action no-op output.",
        "Add `portal open`, `portal enter`, and `portal leave` as explicit local state changes.",
        "Do not close #336 until the contract, fixture, read-only runtime source, runtime tests, and manual smoke have landed.",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn portal_docs_match_current_floor1_runtime_surface() {
    let docs = std::fs::read_to_string("docs/portal/FLOOR1_PORTALS.md")
        .expect("floor1 portal docs should be readable");

    for row in [
        "portal network <name> <denied|local-only|brokered-egress>",
        "portal split <left> <right>",
        "portal clone <source> <name>",
        "portal link <left> <right>",
        "portal snapshot <name>",
        "portal restore <name>",
        "network-default",
        "denied",
        "local-only",
        "brokered-egress",
        "claim-boundary",
        "workspace-context-only",
    ] {
        assert!(docs.contains(row), "missing {row} in floor1 portal docs");
    }
}
