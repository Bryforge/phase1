use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_staged_stub_contract_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-stub-contract-ok.txt";

    for field in [
        "name: black-arts-staged-stub-contract",
        "kind: staged-stub-contract-fixture",
        "command-family: fyr staged",
        "current-state: contract-before-implementation",
        "expected-entrypoint: fyr_command",
        "required-actions:",
        "status",
        "plan",
        "create",
        "apply",
        "validate",
        "promote",
        "discard",
        "required-response-boundaries:",
        "fixture-backed",
        "non-live",
        "candidate-only",
        "evidence-bound",
        "claim-boundary",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn staged_candidate_doc_links_stub_contract_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-stub-contract-ok.txt",
    );
}

#[test]
fn source_contains_fyr_command_entrypoint_for_future_stub() {
    let source = read("src/main.rs");

    assert!(source.contains("fn fyr_command"));
    assert!(source.contains("Some(\"help\") | Some(\"-h\") | Some(\"--help\") => fyr_help()"));
}
