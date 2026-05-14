use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn apply_helper_targets_checked_patch_and_source() {
    let path = "scripts/apply-fyr-staged-runtime-stub.sh";

    for row in [
        "PATCH_PATH=\"patches/fyr-staged-runtime-stub.patch\"",
        "SOURCE_PATH=\"src/main.rs\"",
        "git apply --check \"${PATCH_PATH}\"",
        "git apply \"${PATCH_PATH}\"",
        "Some(\"staged\") => fyr_staged(&args[1..]),",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn apply_helper_supports_check_only_mode_before_apply() {
    let path = "scripts/apply-fyr-staged-runtime-stub.sh";

    for row in [
        "usage: sh scripts/apply-fyr-staged-runtime-stub.sh [apply|check]",
        "check  : verify the patch can apply without changing files",
        "MODE=\"check\"",
        "status    : check-only-pass",
        "run 'sh scripts/apply-fyr-staged-runtime-stub.sh apply' when ready",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn apply_helper_prints_required_validation_commands() {
    let path = "scripts/apply-fyr-staged-runtime-stub.sh";

    for row in [
        "cargo fmt --all -- --check",
        "cargo test -p phase1 --test fyr_staged_runtime_patch_contract",
        "cargo test -p phase1 --test fyr_staged_runtime_apply_helper",
        "cargo test -p phase1 --test fyr_black_arts_runtime_stub",
        "cargo test -p phase1 --test fyr_black_arts_unknown_action",
        "cargo test --workspace --all-targets",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn local_apply_doc_preserves_completion_rule_and_blocked_scope() {
    let path = "docs/fyr/STAGED_RUNTIME_LOCAL_APPLY.md";

    for row in [
        "Close #317 only after:",
        "`src/main.rs` is patched;",
        "runtime tests land;",
        "manual smoke confirms `fyr staged` is recognized;",
        "blocked behavior remains absent.",
        "candidate creation",
        "candidate apply/change behavior",
        "validation execution",
        "promotion execution",
        "discard execution",
        "host command execution",
        "network access",
        "live-system writes",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn local_apply_doc_lists_runtime_smoke_and_forbidden_markers() {
    let path = "docs/fyr/STAGED_RUNTIME_LOCAL_APPLY.md";

    for row in [
        "fyr staged",
        "fyr staged status",
        "fyr staged help",
        "fyr staged nonsense",
        "cargo ",
        "rustc ",
        "bash:",
        "sh:",
        "http://",
        "https://",
    ] {
        assert_contains(path, row);
    }
}
