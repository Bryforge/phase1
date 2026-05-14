use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn staged_runtime_patch_adds_match_arm_and_helpers() {
    let path = "patches/fyr-staged-runtime-stub.patch";

    for row in [
        "Some(\"staged\") => fyr_staged(&args[1..]),",
        "fn fyr_staged(args: &[String]) -> String",
        "fn fyr_staged_visual() -> String",
        "fn fyr_staged_help() -> String",
        "fn fyr_staged_unknown(action: &str) -> String",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_runtime_patch_preserves_black_arts_required_markers() {
    let path = "patches/fyr-staged-runtime-stub.patch";

    for row in [
        "☠ FYR black_arts // STAGED CANDIDATE MODE",
        "[BLACK_ARTS] FYR staged candidate mode",
        "live-system   : untouched",
        "promotion     : blocked-until-validation-and-approval",
        "boundary      : candidate-only | non-live | evidence-bound | claim-boundary",
        "claim-boundary: fixture-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_runtime_patch_keeps_unknown_actions_noop_and_help_guided() {
    let path = "patches/fyr-staged-runtime-stub.patch";

    for row in [
        "status        : unknown staged action",
        "candidate     : none",
        "result        : no-op",
        "help          : fyr staged help",
        "boundaries    : non-live, no-write, evidence-bound, claim-boundary",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn staged_runtime_patch_does_not_introduce_host_or_network_execution() {
    let text = read("patches/fyr-staged-runtime-stub.patch");

    for forbidden in [
        "Command::new",
        "std::process",
        "cargo ",
        "rustc ",
        "bash:",
        "sh:",
        "http://",
        "https://",
        "sys_write",
        "fs::write",
        "promote",
        "discard",
    ] {
        if forbidden == "promote" || forbidden == "discard" {
            continue;
        }
        assert!(
            !text.contains(forbidden),
            "patch should not include forbidden marker: {forbidden}"
        );
    }

    assert!(text.contains("commands      : status, plan, create, apply, validate, promote, discard"));
    assert!(text.contains("promotion     : blocked-until-validation-and-approval"));
}
