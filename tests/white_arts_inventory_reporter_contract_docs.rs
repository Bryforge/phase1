use std::fs;

const CONTRACT: &str = "docs/white-arts/INVENTORY_REPORTER_CONTRACT.md";
const INDEX: &str = "docs/white-arts/README.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn white_arts_inventory_reporter_contract_exists_and_is_linked() {
    let contract = read(CONTRACT);
    let index = read(INDEX);

    assert!(contract.contains("# White Arts Inventory Reporter Contract"));
    assert!(contract.contains("white-arts inventory"));
    assert!(index.contains("[Inventory reporter contract](INVENTORY_REPORTER_CONTRACT.md)"));
}

#[test]
fn white_arts_inventory_reporter_contract_preserves_header_fields() {
    let contract = read(CONTRACT);

    for field in [
        "phase1 white arts inventory",
        "status           : planned",
        "mode             : defensive-read-only",
        "mutation         : disabled",
        "host-execution   : disabled",
        "network-access   : disabled",
        "repair-policy    : staged-candidate-only",
        "claim-boundary   : evidence-bound-maintenance",
    ] {
        assert!(
            contract.contains(field),
            "missing inventory header field: {field}"
        );
    }
}

#[test]
fn white_arts_inventory_reporter_contract_preserves_table_shape() {
    let contract = read(CONTRACT);

    for column in [
        "system",
        "owner-surface",
        "nominal-signal",
        "test-command",
        "integrity-gate",
        "failure-behavior",
        "recovery-path",
        "claim-boundary",
    ] {
        assert!(
            contract.contains(column),
            "missing inventory column: {column}"
        );
    }
}

#[test]
fn white_arts_inventory_reporter_contract_lists_required_system_rows() {
    let contract = read(CONTRACT);

    for system in [
        "Phase1 boot and shell",
        "VFS and kernel model",
        "Policy and host gates",
        "Command registry",
        "Storage and host workspace",
        "WASI-lite plugins",
        "Program analysis",
        "Nest and portal surfaces",
        "Fyr language",
        "Base1 evidence track",
        "Security and crypto docs",
        "Website and release metadata",
        "CI and quality gates",
    ] {
        assert!(
            contract.contains(system),
            "missing inventory system row: {system}"
        );
    }
}

#[test]
fn white_arts_inventory_reporter_contract_preserves_safety_and_promotion_rules() {
    let contract = read(CONTRACT);

    for rule in [
        "prefer deterministic text output",
        "use text labels, not color or icons alone",
        "avoid host execution",
        "avoid network access",
        "avoid collecting credentials",
        "avoid writing files",
        "avoid deleting or repairing material",
        "avoid claiming malware safety, forensic admissibility, production hardening, or OS completion",
        "white-arts.inventory mode=read-only mutation=disabled rows=<count>",
        "must not include raw secrets, credential-bearing paths, unredacted command bodies, or host output",
        "contract-tested -> runtime-inventory-stub -> deterministic-output-tested -> quality-gate-linked -> reviewed",
        "Repair planning remains a later staged-candidate feature",
    ] {
        assert!(contract.contains(rule), "missing safety or promotion rule: {rule}");
    }
}
