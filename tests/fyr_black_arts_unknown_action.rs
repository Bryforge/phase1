use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_unknown_action_fixture_has_required_rows() {
    let path = "docs/fyr/fixtures/staged-unknown-action-ok.txt";

    for row in [
        "fyr staged unknown",
        "codename      : black_arts",
        "status        : unknown staged action",
        "action        : unknown",
        "live-system   : untouched",
        "candidate     : none",
        "result        : no-op",
        "help          : fyr staged help",
        "boundaries    : non-live, no-write, evidence-bound, claim-boundary",
        "claim-boundary: fixture-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn unknown_action_preserves_noop_boundary() {
    let fixture = read("docs/fyr/fixtures/staged-unknown-action-ok.txt");

    assert!(fixture.contains("result        : no-op"));
    assert!(fixture.contains("candidate     : none"));
    assert!(fixture.contains("live-system   : untouched"));
    assert!(fixture.contains("help          : fyr staged help"));
}

#[test]
fn unknown_action_stays_fixture_only() {
    let fixture = read("docs/fyr/fixtures/staged-unknown-action-ok.txt");

    assert!(fixture.contains("claim-boundary: fixture-only"));
    assert!(fixture.contains("boundaries    : non-live, no-write, evidence-bound, claim-boundary"));
}
