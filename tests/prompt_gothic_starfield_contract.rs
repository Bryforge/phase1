use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn prompt_gothic_starfield_contract_exists() {
    let path = "docs/ui/PROMPT_GOTHIC_STARFIELD.md";

    for row in [
        "# Prompt gothic and starfield visual mode",
        "Status: design contract",
        "Scope: prompt-area visual treatment",
        "Non-claim: this is not a terminal-wide font switch",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn prompt_gothic_starfield_is_prompt_only_and_opt_in() {
    let path = "docs/ui/PROMPT_GOTHIC_STARFIELD.md";

    for row in [
        "PHASE1_PROMPT_GOTHIC=1",
        "PHASE1_STARFIELD=1",
        "prompt label only",
        "Everything outside the prompt remains the normal readable Phase1 output.",
        "The feature must not change parser behavior.",
        "The feature must not affect command history contents.",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn prompt_gothic_starfield_preserves_required_fallbacks() {
    let path = "docs/ui/PROMPT_GOTHIC_STARFIELD.md";

    for row in [
        "PHASE1_NO_COLOR=1",
        "PHASE1_ASCII=1",
        "PHASE1_TEST_MODE=1",
        "PHASE1_COOKED_INPUT=1",
        "phase1://black_arts ~ >",
        "phase1://portal/alpha ~ >",
        "deterministic or disabled in tests",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn prompt_gothic_starfield_keeps_starfield_subtle() {
    let path = "docs/ui/PROMPT_GOTHIC_STARFIELD.md";

    for row in [
        "ambient prompt decoration only",
        "not a full scrolling background",
        "full-screen animated background",
        "wide visual noise that breaks mobile terminals",
        "stars inside diagnostics, logs, or copied command text",
    ] {
        assert_contains(path, row);
    }
}
