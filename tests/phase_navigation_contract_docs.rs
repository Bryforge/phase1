use std::fs;

const CONTRACT: &str = "docs/phase/PHASE_NAVIGATION_CONTRACT.md";

fn contract() -> String {
    fs::read_to_string(CONTRACT).expect("read Phase navigation contract")
}

#[test]
fn phase_navigation_contract_exists_and_defines_scope() {
    let doc = contract();

    assert!(doc.contains("# Phase Navigation Contract"));
    assert!(doc.contains("internal navigation contract for Phase1 spaces"));
    assert!(doc.contains("floors represent named or numbered operator spaces"));
    assert!(doc.contains("domains represent bounded Phase1-controlled context groups"));
    assert!(doc.contains("portals represent explicit links between approved Phase1 contexts"));
    assert!(doc.contains("root remains the reversible base context"));
}

#[test]
fn phase_navigation_contract_lists_command_surface() {
    let doc = contract();

    for command in [
        "phase status",
        "phase floor create <num|name>",
        "phase floor enter <num|name>",
        "phase floor main <num|name>",
        "phase portal link <from> <to>",
        "phase domain establish <name>",
        "phase back",
        "phase root",
    ] {
        assert!(doc.contains(command), "missing command contract: {command}");
    }
}

#[test]
fn phase_navigation_contract_preserves_status_fields() {
    let doc = contract();

    for field in [
        "phase-navigation : planned",
        "execution-state  : not-executed",
        "active-floor     : root",
        "active-domain    : none",
        "active-portal    : none",
        "path             : root",
        "transition       : none",
        "phase-safe       : on",
        "host-tools       : gated",
        "claim-boundary   : internal-context-navigation-only",
    ] {
        assert!(doc.contains(field), "missing status field: {field}");
    }
}

#[test]
fn phase_navigation_contract_preserves_optics_rails() {
    let doc = contract();

    assert!(doc.contains("Optics A/B/C/D rail contract"));
    assert!(doc.contains("A TOP RAIL     floor=<num|root> domain=<name|none> portal=<active|none> nest=<level/max>"));
    assert!(doc.contains("B COMMAND      phase floor enter 2"));
    assert!(doc.contains("C STATUS HUD   active-main=floor/2 transition=ok path=root>floor/2"));
    assert!(doc.contains("D BOTTOM HUD   back=root phase-safe=on host-tools=gated"));
}

#[test]
fn phase_navigation_contract_preserves_boundaries_and_promotion_rule() {
    let doc = contract();

    for forbidden in [
        "network lateral movement",
        "host compromise",
        "privilege escalation",
        "hidden persistence",
        "uncontrolled execution",
        "sandbox hardening",
        "malware safety",
        "production isolation",
    ] {
        assert!(doc.contains(forbidden), "missing forbidden claim: {forbidden}");
    }

    for requirement in [
        "explicit",
        "visible",
        "logged",
        "reversible",
        "bounded by current safe-mode and host-tool policy",
        "compatible with existing nest and portal concepts",
        "represented through text labels, not color or icons alone",
        "planned -> documented -> command-contract tested -> runtime stub -> local-state runtime -> Optics-integrated runtime -> reviewed -> release eligible",
        "The first PR for this track is documentation-only",
        "must not create live `phase` runtime behavior yet",
    ] {
        assert!(doc.contains(requirement), "missing safety or promotion rule: {requirement}");
    }
}
