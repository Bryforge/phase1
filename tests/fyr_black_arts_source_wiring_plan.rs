use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_source_wiring_plan_has_required_shape() {
    let path = "docs/fyr/fixtures/staged-source-wiring-plan-ok.txt";

    for row in [
        "name: black-arts-source-wiring-plan",
        "kind: staged-source-wiring-plan-fixture",
        "entrypoint: fyr_command",
        "first-safe-action: fyr staged",
        "expected-output-source: docs/fyr/fixtures/staged-runtime-stub-ok.txt",
        "match staged in fyr_command",
        "add fyr_staged helper",
        "add fyr_staged_help helper",
        "add fixture-backed status response",
        "fyr staged prints runtime stub rows",
        "fyr staged help prints help rows",
        "unknown staged action stays non-live",
        "no candidate writes",
        "no live-system changes",
        "no host command execution",
        "no promotion",
        "claim: fixture-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_candidate_doc_links_source_wiring_plan() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-source-wiring-plan-ok.txt",
    );
}

#[test]
fn source_has_fyr_command_entrypoint_for_staged_wiring() {
    let source = read("src/main.rs");

    assert!(source.contains("fn fyr_command"));
    assert!(source.contains("Some(\"run\") => fyr_run(shell, &args[1..])"));
    assert!(source.contains("Some(\"help\") | Some(\"-h\") | Some(\"--help\") => fyr_help()"));
}
