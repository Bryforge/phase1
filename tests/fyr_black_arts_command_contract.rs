use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_command_contract_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-command-contract-ok.txt";

    for field in [
        "name: black-arts-command-contract-fixture",
        "kind: staged-command-contract-fixture",
        "codename: black_arts",
        "canonical-surface: fyr staged",
        "commands:",
        "status",
        "plan",
        "create",
        "apply",
        "validate",
        "promote",
        "discard",
        "required-guards:",
        "candidate-only",
        "declared-workspace",
        "validation-before-promote",
        "operator-approval",
        "evidence-recorded",
        "claim-boundary",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn staged_candidate_doc_links_command_contract_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-command-contract-ok.txt",
    );
}

#[test]
fn command_contract_matches_documented_surface() {
    let doc = read("docs/fyr/STAGED_CANDIDATES.md");

    for command in [
        "fyr staged status",
        "fyr staged plan",
        "fyr staged create <candidate>",
        "fyr staged apply <candidate> <plan.fyr>",
        "fyr staged validate <candidate>",
        "fyr staged promote <candidate>",
        "fyr staged discard <candidate>",
    ] {
        assert!(doc.contains(command), "staged candidate doc should include {command}");
    }
}
