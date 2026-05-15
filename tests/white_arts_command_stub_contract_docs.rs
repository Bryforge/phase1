use std::fs;

const CONTRACT: &str = "docs/white-arts/COMMAND_STUB_CONTRACT.md";
const INDEX: &str = "docs/white-arts/README.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn white_arts_command_stub_contract_exists_and_is_linked() {
    let contract = read(CONTRACT);
    let index = read(INDEX);

    assert!(contract.contains("# White Arts Command Stub Contract"));
    assert!(contract.contains("read-only, report-only, deterministic"));
    assert!(index.contains("[Command stub contract](COMMAND_STUB_CONTRACT.md)"));
}

#[test]
fn white_arts_command_stub_contract_lists_initial_surface() {
    let contract = read(CONTRACT);

    for command in [
        "white-arts",
        "white-arts status",
        "white-arts help",
        "white-arts inventory",
        "white-arts check",
        "white-arts report",
        "white-arts audit security",
        "white-arts audit integrity",
        "white-arts audit base1",
        "white-arts audit fyr",
    ] {
        assert!(contract.contains(command), "missing command: {command}");
    }
}

#[test]
fn white_arts_command_stub_contract_preserves_required_status_fields() {
    let contract = read(CONTRACT);

    for field in [
        "phase1 white arts",
        "status           : planned",
        "mode             : defensive-read-only",
        "mutation         : disabled",
        "repair-policy    : staged-candidate-only",
        "execution-state  : not-executed",
        "host-execution   : disabled",
        "network-access   : disabled",
        "credential-access: disabled",
        "sandbox-claim    : not-claimed",
        "audit-scope      : report-only",
        "integrity-scope  : docs-scripts-release-metadata",
        "base1-scope      : read-only-recovery-and-evidence",
        "fyr-scope        : metadata-and-reporting-only",
        "claim-boundary   : evidence-bound-maintenance",
    ] {
        assert!(contract.contains(field), "missing status field: {field}");
    }
}

#[test]
fn white_arts_command_stub_contract_blocks_unsafe_behavior() {
    let contract = read(CONTRACT);

    for forbidden in [
        "execute host tools",
        "access the network",
        "collect credentials",
        "silently modify files",
        "write boot artifacts",
        "delete recovery material",
        "promote candidate repairs",
        "imply malware safety",
        "imply forensic admissibility",
        "imply production hardening",
    ] {
        assert!(
            contract.contains(forbidden),
            "missing forbidden behavior: {forbidden}"
        );
    }
}

#[test]
fn white_arts_command_stub_contract_preserves_audit_optics_and_promotion_rules() {
    let contract = read(CONTRACT);

    for requirement in [
        "white-arts.status mode=read-only mutation=disabled",
        "C STATUS HUD   white-arts=planned integrity=report-only repair=staged-candidate-only",
        "D BOTTOM HUD   mutation=disabled host-tools=gated claim=evidence-bound-maintenance",
        "contract-tested -> runtime-status-stub -> inventory-reporter -> check-reporter -> candidate-repair-planner -> reviewed-repair-flow",
        "Runtime implementation should not begin with repair behavior",
        "Every promotion requires focused tests, docs, non-claim language, and a failure behavior note",
    ] {
        assert!(
            contract.contains(requirement),
            "missing audit, optics, or promotion rule: {requirement}"
        );
    }
}
