use std::fs;

const CONTRACT: &str = "docs/phase/ROUTE_ORIGIN_CONTRACT.md";

fn contract() -> String {
    fs::read_to_string(CONTRACT).expect("read Phase route origin contract")
}

#[test]
fn route_origin_contract_exists_and_defines_route_model() {
    let doc = contract();

    assert!(doc.contains("# Phase Route and Origin Contract"));
    assert!(doc.contains("`ROOT` is the center anchor"));
    assert!(doc.contains("`0/0` is the distinguished origin marker"));

    for route in ["u/NUM", "d/NUM", "L/NUM", "R/NUM", "0/0", "ROOT"] {
        assert!(doc.contains(route), "missing route label: {route}");
    }
}

#[test]
fn route_origin_contract_preserves_canonical_paths() {
    let doc = contract();

    for path in [
        "ROOT",
        "0/0",
        "ROOT>0/0",
        "ROOT>u/1",
        "ROOT>d/1",
        "ROOT>L/2",
        "ROOT>R/3",
        "ROOT>R/3>u/1",
        "ROOT>L/2>d/1",
    ] {
        assert!(doc.contains(path), "missing canonical path: {path}");
    }
}

#[test]
fn route_origin_contract_preserves_command_vocabulary() {
    let doc = contract();

    for command in [
        "phase origin",
        "phase back-origin",
        "phase return 0/0",
        "phase move 0/0",
        "phase origin plant",
        "phase origin plant <name>",
        "phase plant-origin",
        "phase set-origin 0/0",
        "phase route status",
        "phase origin status",
        "phase whereami",
        "phase compass",
        "phase path",
    ] {
        assert!(doc.contains(command), "missing command: {command}");
    }
}

#[test]
fn route_origin_contract_preserves_status_fields() {
    let doc = contract();

    for field in [
        "root-anchor      : ROOT",
        "origin           : 0/0",
        "origin-name      : <name|none>",
        "current-route    : <route|0/0>",
        "current-axis     : u|d|L|R|ROOT",
        "path             : ROOT>R/3>u/1",
        "previous-origin  : <origin-id|none>",
        "rollback-target  : <route|0/0|ROOT>",
        "trace-id         : <trace-id>",
        "operator-intent  : explicit",
        "safe-mode        : enforced",
        "claim-boundary   : internal-phase-universe-only",
    ] {
        assert!(doc.contains(field), "missing status field: {field}");
    }
}

#[test]
fn route_origin_contract_preserves_safeguards_and_boundaries() {
    let doc = contract();

    for required in [
        "previous-origin preservation",
        "rollback-target recording",
        "safe portal readiness check where available",
        "action           : plant-origin",
        "mutation         : planned-only",
        "action           : return-origin",
        "transition       : reversible",
        "origin=0/0",
        "rule=root-remains-anchor",
        "origin-return=ready root-anchor=preserved",
    ] {
        assert!(doc.contains(required), "missing safeguard: {required}");
    }

    for boundary in [
        "real network movement",
        "host compromise",
        "privilege escalation",
        "credential collection",
        "uncontrolled execution",
        "destructive runtime state",
        "bypass of safe-mode or audit",
        "external side effects",
    ] {
        assert!(doc.contains(boundary), "missing hard boundary: {boundary}");
    }
}
