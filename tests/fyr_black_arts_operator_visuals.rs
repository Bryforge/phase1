use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn black_arts_operator_visual_fixture_has_required_rows() {
    let path = "docs/fyr/fixtures/staged-operator-visual-ok.txt";

    for row in [
        "☠ FYR black_arts // STAGED CANDIDATE MODE",
        "candidate     : phase1-base1-candidate",
        "workspace     : .phase1/staged-candidates/phase1-base1-candidate",
        "state         : fixture-backed",
        "live-system   : untouched",
        "promotion     : blocked-until-validation-and-approval",
        "evidence      : docs/fyr/fixtures/staged-lifecycle-example.txt",
        "boundary      : candidate-only | non-live | evidence-bound | claim-boundary",
        "ascii-fallback: [BLACK_ARTS] FYR staged candidate mode",
        "prompt        : fyr:black_arts(candidate=phase1-base1-candidate,state=staged)>",
        "claim-boundary: fixture-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn black_arts_operator_visual_spec_preserves_text_safety_cues() {
    let path = "docs/fyr/BLACK_ARTS_OPERATOR_VISUALS.md";

    for row in [
        "Do not hide the safety state behind icons.",
        "live-system   : untouched",
        "promotion     : blocked-until-validation-and-approval",
        "boundary      : candidate-only | non-live | evidence-bound | claim-boundary",
        "[BLACK_ARTS] FYR staged candidate mode",
        "fyr:black_arts(candidate=phase1-base1-candidate,state=staged)>",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn black_arts_operator_visual_spec_links_fixture() {
    assert_contains(
        "docs/fyr/BLACK_ARTS_OPERATOR_VISUALS.md",
        "docs/fyr/fixtures/staged-operator-visual-ok.txt",
    );
}
