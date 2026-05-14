use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_runtime_stub_fixture_has_required_rows() {
    let path = "docs/fyr/fixtures/staged-runtime-stub-ok.txt";

    for row in [
        "fyr staged",
        "codename      : black_arts",
        "status        : fixture-backed design stub",
        "live-system   : untouched",
        "workspace     : .phase1/staged-candidates",
        "commands      : status, plan, create, apply, validate, promote, discard",
        "boundaries    : candidate-only, non-live, evidence-bound, claim-boundary",
        "implementation: pending",
        "next          : wire fyr staged in fyr_command",
        "claim-boundary: fixture-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_candidate_doc_links_runtime_stub_fixture() {
    assert_contains(
        "docs/fyr/STAGED_CANDIDATES.md",
        "docs/fyr/fixtures/staged-runtime-stub-ok.txt",
    );
}

#[test]
fn source_entrypoint_is_ready_for_future_fyr_staged_wiring() {
    let source = read("src/main.rs");

    assert!(source.contains("fn fyr_command"));
    assert!(source.contains("Some(\"status\") => fyr_status()"));
    assert!(source.contains("Some(\"help\") | Some(\"-h\") | Some(\"--help\") => fyr_help()"));
}
