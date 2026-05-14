use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn automation_matrix_covers_current_fyr_command_surface() {
    let path = "docs/fyr/AUTOMATION_VALIDATION_MATRIX.md";

    for command in [
        "`fyr status`",
        "`fyr spec`",
        "`fyr new <name>`",
        "`fyr init <package>`",
        "`fyr cat <file.fyr>`",
        "`fyr check <file.fyr|package>`",
        "`fyr build <file.fyr|package>`",
        "`fyr test <package>`",
        "`fyr self`",
        "`fyr run <file.fyr>`",
        "`fyr color <file.fyr>`",
        "`fyr highlight <file.fyr>`",
        "`fyr staged`",
        "`fyr staged status`",
        "`fyr staged help`",
        "`fyr staged <unknown>`",
    ] {
        assert_contains(path, command);
    }
}

#[test]
fn automation_matrix_covers_language_features_and_diagnostics() {
    let path = "docs/fyr/AUTOMATION_VALIDATION_MATRIX.md";

    for feature in [
        "`fn main() -> i32`",
        "`print(\"literal\")`",
        "`return <integer>`",
        "`let` bindings",
        "integer expressions",
        "`if` handling",
        "`assert_eq`",
        "boolean `assert`",
        "comparison assertions",
        "package manifest",
        "module discovery",
        "syntax coloring",
    ] {
        assert_contains(path, feature);
    }
}

#[test]
fn validation_demo_fixture_contains_user_considerations() {
    let path = "docs/fyr/fixtures/validation-demo-contract-ok.txt";

    for cue in [
        "first-time   : start with fyr status, then fyr help",
        "mobile       : compact rows; avoid wide runtime tables",
        "keyboard     : direct commands and tab-complete guidance",
        "paste-safe   : commands can be copied one line at a time",
        "no-color     : every colored state has a text label",
        "low-vision   : text labels carry meaning; symbols are decorative only",
        "recovery     : unknown commands point to help and remain no-op",
        "trust        : host-tools, network, and live-system writes are visibly blocked",
    ] {
        assert_contains(path, cue);
    }
}

#[test]
fn validation_demo_fixture_contains_control_schemes() {
    let path = "docs/fyr/fixtures/validation-demo-contract-ok.txt";

    for cue in [
        "direct command entry",
        "tab-completion expectations",
        "help-first discovery",
        "copy/paste command flow",
        "mobile terminal constraints",
        "no-color output",
        "compact output",
        "safe-mode boundary",
        "guarded-host boundary",
        "unknown-command recovery",
    ] {
        assert_contains(path, cue);
    }
}

#[test]
fn validation_demo_fixture_blocks_unsafe_or_overclaiming_behavior() {
    let path = "docs/fyr/fixtures/validation-demo-contract-ok.txt";

    for blocked in [
        "host shell execution",
        "network access",
        "Cargo invocation from Fyr commands",
        "Rust compiler invocation from Fyr commands",
        "live-system staged writes",
        "autonomous promotion",
        "autonomous mutation",
        "self-hosting completion",
        "production OS replacement claims",
    ] {
        assert_contains(path, blocked);
    }
}

#[test]
fn automation_matrix_keeps_runtime_claims_evidence_bound() {
    let path = "docs/fyr/AUTOMATION_VALIDATION_MATRIX.md";

    for row in [
        "Non-claim: this matrix does not claim every listed runtime path is implemented yet.",
        "Wire a real runtime command only after the fixture and acceptance tests exist.",
        "Do not raise public Fyr completion percentage until runtime implementation and tests land.",
        "Source | Runtime command or parser behavior exists | Yes, only with tests",
        "Demo path | Built-in operator demo exposes the flow safely | Yes, only with source and tests",
    ] {
        assert_contains(path, row);
    }
}
