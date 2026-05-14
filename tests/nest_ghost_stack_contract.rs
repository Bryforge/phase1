use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn ghost_stack_design_defines_command_surface() {
    let path = "docs/nest/GHOST_STACKS.md";

    for command in [
        "nest stack",
        "nest stack status",
        "nest stack list",
        "nest stack push <label>",
        "nest stack pop",
        "nest stack ghost <label>",
        "nest stack resume <label>",
        "nest stack prune",
        "nest stack exit-all",
    ] {
        assert_contains(path, command);
    }
}

#[test]
fn ghost_stack_status_fixture_has_required_rows() {
    let path = "docs/nest/fixtures/ghost-stack-status-ok.txt";

    for row in [
        "phase1 nest stack",
        "mode          : read-only status",
        "nest-level    : 0/1",
        "root          : active",
        "current       : level-0",
        "ghost-count   : 0",
        "exit-all      : clear",
        "safe-mode     : visible",
        "trust         : visible",
        "guardrail     : no host process spawn | no network | no isolation claim",
        "claim-boundary: control-plane-only",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn ghost_stack_contract_preserves_user_control_cues() {
    let path = "docs/nest/fixtures/ghost-stack-status-ok.txt";

    for row in [
        "first-time   : run nest stack status, then nest help",
        "keyboard     : direct one-line commands",
        "mobile       : compact rows, no wide tables",
        "no-color     : every state has a text label",
        "ascii        : symbols are optional; text is authoritative",
        "low-vision   : safety states are labels, not color-only",
        "recovery     : exit-all state is always visible",
        "unknown      : unknown stack actions are no-op and help-guided",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn ghost_stack_contract_preserves_safety_boundaries() {
    for path in [
        "docs/nest/GHOST_STACKS.md",
        "docs/nest/fixtures/ghost-stack-status-ok.txt",
    ] {
        for row in [
            "host command execution",
            "network access",
            "hidden process spawning",
            "unbounded recursion",
            "autonomous stack mutation",
            "VM isolation",
            "container isolation",
            "physical machine separation",
            "security boundary hardening",
        ] {
            assert_contains(path, row);
        }
    }
}

#[test]
fn ghost_stack_contract_keeps_runtime_claim_evidence_bound() {
    let path = "docs/nest/GHOST_STACKS.md";

    for row in [
        "Non-claim: ghost stacks are not claimed to be separate physical machines, VMs, containers, or security isolation boundaries.",
        "This design contract is not runtime implementation.",
        "Runtime status can advance only when source commands and tests land.",
    ] {
        assert_contains(path, row);
    }
}
