use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn staged_source_wiring_checklist_names_first_safe_behavior() {
    let path = "docs/fyr/STAGED_SOURCE_WIRING.md";

    for row in [
        "Status: implementation checklist",
        "Codename: `black_arts`",
        "fyr staged",
        "fyr staged help",
        "fyr staged status",
        "fyr staged unknown",
        "fixture-backed",
        "non-live",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_source_wiring_checklist_blocks_later_actions_from_first_pr() {
    let path = "docs/fyr/STAGED_SOURCE_WIRING.md";

    for row in [
        "Do not implement `create`, `apply`, `validate`, `promote`, or `discard` behavior in the first source-wiring PR.",
        "candidate writes",
        "live-system changes",
        "host command execution",
        "network access",
        "promotion behavior",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_source_wiring_checklist_names_required_source_hooks() {
    let path = "docs/fyr/STAGED_SOURCE_WIRING.md";

    for row in [
        "Add `Some(\"staged\") => fyr_staged(&args[1..])` inside `fyr_command`.",
        "Add a `fyr_staged(args: &[String]) -> String` helper.",
        "Add a `fyr_staged_help() -> String` helper.",
        "unknown staged actions return a no-op help-guidance response",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_source_wiring_checklist_links_reference_fixtures() {
    let path = "docs/fyr/STAGED_SOURCE_WIRING.md";

    for fixture in [
        "docs/fyr/fixtures/staged-runtime-stub-ok.txt",
        "docs/fyr/fixtures/staged-help-ok.txt",
        "docs/fyr/fixtures/staged-status-example.txt",
        "docs/fyr/fixtures/staged-unknown-action-ok.txt",
    ] {
        assert_contains(path, fixture);
        assert!(fs::metadata(fixture).is_ok(), "fixture should exist: {fixture}");
    }
}
