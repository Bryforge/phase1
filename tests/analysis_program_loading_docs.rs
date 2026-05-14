const PROGRAM_LOADING_ANALYSIS: &str = include_str!("../docs/analysis/PROGRAM_LOADING_ANALYSIS.md");
const ANALYSIS_INDEX: &str = include_str!("../docs/analysis/README.md");

#[test]
fn program_loading_analysis_doc_keeps_no_execute_boundary() {
    for required in [
        "metadata-only",
        "must not execute the target",
        "execution_state = not-executed",
        "No hostile binary is executed by default",
        "host-backed execution remains explicit and restricted",
    ] {
        assert!(
            PROGRAM_LOADING_ANALYSIS.contains(required),
            "missing no-execute boundary phrase: {required}"
        );
    }
}

#[test]
fn program_loading_analysis_doc_keeps_ecosystem_roles_accurate() {
    for required in [
        "**Phase1** is the console and analysis environment.",
        "**Base1** is the system foundation and evidence-bound host/recovery track.",
        "**Fyr** is the Phase1-native programming language for programmable analysis workflows.",
    ] {
        assert!(
            PROGRAM_LOADING_ANALYSIS.contains(required),
            "missing ecosystem role phrase: {required}"
        );
    }
}

#[test]
fn program_loading_analysis_doc_preserves_non_claims() {
    for required in [
        "hardened malware sandboxing",
        "VM or container isolation",
        "safe execution of hostile binaries",
        "production forensic admissibility",
        "finished OS replacement behavior",
    ] {
        assert!(
            PROGRAM_LOADING_ANALYSIS.contains(required),
            "missing non-claim phrase: {required}"
        );
    }
}

#[test]
fn program_loading_analysis_doc_defines_initial_command_vocabulary() {
    for required in [
        "analyze status",
        "analyze load <path>",
        "analyze inspect <id>",
        "analyze report <id>",
        "analyze forget <id>",
    ] {
        assert!(
            PROGRAM_LOADING_ANALYSIS.contains(required),
            "missing command vocabulary phrase: {required}"
        );
    }
}

#[test]
fn analysis_index_points_to_program_loading_contract() {
    assert!(ANALYSIS_INDEX.contains("PROGRAM_LOADING_ANALYSIS.md"));
    assert!(ANALYSIS_INDEX.contains("no-execute by default"));
    assert!(ANALYSIS_INDEX.contains("without executing the sample"));
}
