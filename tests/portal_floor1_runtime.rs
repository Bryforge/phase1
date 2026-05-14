use std::io::Write;
use std::process::{Command, Stdio};

fn run_phase1(input: &str) -> String {
    let exe = env!("CARGO_BIN_EXE_phase1");
    let mut child = Command::new(exe)
        .env("PHASE1_TEST_MODE", "1")
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
        .env("PHASE1_COOKED_INPUT", "1")
        .env("PHASE1_MOBILE_MODE", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn phase1");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(format!("\n{input}").as_bytes())
        .expect("write input");

    let output = child.wait_with_output().expect("wait");
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn portal_status_reports_floor1_network_denied_rows() {
    let output = run_phase1("portal\nportal status\nportal list\nexit\n");

    for row in [
        "phase1 portals",
        "mode              : read-only status",
        "floor             : floor1",
        "active-portal     : root",
        "open-portals      : root",
        "network-owner     : floor1",
        "network-mode      : denied",
        "network-default   : denied",
        "brokered-egress   : planned-disabled",
        "network           : blocked",
        "claim-boundary    : workspace-context-only",
    ] {
        assert!(output.contains(row), "missing {row}:\n{output}");
    }
}

#[test]
fn portal_help_reports_local_state_and_floor1_policy() {
    let output = run_phase1("portal help\nexit\n");

    for row in [
        "portal help",
        "usage             : portal <status|list|open|enter|leave|close|inspect|network|split|help>",
        "local-state       : open, enter, leave, close, inspect, network, split",
        "floor             : floor1",
        "network-default   : denied",
        "claim-boundary    : workspace-context-only",
    ] {
        assert!(output.contains(row), "missing {row}:\n{output}");
    }
}

#[test]
fn portal_open_enter_inspect_leave_updates_local_state_only() {
    let output = run_phase1(
        "portal open alpha\nportal status\nportal enter alpha\nportal inspect alpha\nportal leave\nportal status\nexit\n",
    );

    for row in [
        "portal open alpha",
        "status            : opened",
        "open-portals      : root,alpha",
        "portal enter alpha",
        "status            : entered",
        "active-portal     : alpha",
        "portal inspect alpha",
        "state             : active",
        "portal leave",
        "status            : left",
        "active-portal     : root",
        "network-mode      : denied",
        "network-owner     : floor1",
        "claim-boundary    : workspace-context-only",
    ] {
        assert!(output.contains(row), "missing {row}:\n{output}");
    }
}

#[test]
fn portal_missing_or_invalid_actions_are_noop_and_help_guided() {
    let output = run_phase1("portal enter missing\nportal open root\nportal nonsense\nexit\n");

    for row in [
        "portal enter missing",
        "status            : missing-portal",
        "result            : no-op",
        "portal open root",
        "status            : invalid-name",
        "portal nonsense",
        "status            : not-yet-implemented",
        "claim-boundary    : workspace-context-only",
    ] {
        assert!(output.contains(row), "missing {row}:\n{output}");
    }
}

#[test]
fn portal_close_removes_local_portal_and_returns_active_to_root() {
    let output = run_phase1(
        "portal open alpha\nportal enter alpha\nportal close alpha\nportal status\nexit\n",
    );

    for row in [
        "portal close alpha",
        "status            : closed",
        "active-portal     : root",
        "open-portals      : root",
        "network-mode      : denied",
        "network-owner     : floor1",
        "claim-boundary    : workspace-context-only",
    ] {
        assert!(output.contains(row), "missing {row}:\n{output}");
    }
}

#[test]
fn portal_network_modes_are_policy_state_only() {
    let output = run_phase1(
        "portal open alpha\nportal network alpha local-only\nportal enter alpha\nportal inspect alpha\nportal network alpha brokered-egress\nportal status\nportal network alpha denied\nportal status\nexit\n",
    );

    for row in [
        "portal network alpha",
        "status            : updated",
        "portal            : alpha",
        "network-owner     : floor1",
        "network-mode      : local-only",
        "network-mode      : brokered-egress",
        "network-mode      : denied",
        "network-default   : denied",
        "local-link        : planned-disabled",
        "brokered-egress   : planned-disabled",
        "network           : blocked",
        "result            : policy-state-only",
        "claim-boundary    : workspace-context-only",
    ] {
        assert!(output.contains(row), "missing {row}:\n{output}");
    }
}

#[test]
fn portal_split_opens_two_local_portals_with_denied_network() {
    let output = run_phase1(
        "portal split alpha beta\nportal status\nportal inspect alpha\nportal inspect beta\nexit\n",
    );

    for row in [
        "portal split alpha beta",
        "status            : split-opened",
        "left              : alpha",
        "right             : beta",
        "active-portal     : alpha",
        "open-portals      : root,alpha,beta",
        "split-mode        : two-pane-local",
        "local-link        : planned-disabled",
        "network-owner     : floor1",
        "network-mode      : denied",
        "network-default   : denied",
        "network           : blocked",
        "portal inspect alpha",
        "portal inspect beta",
        "claim-boundary    : workspace-context-only",
    ] {
        assert!(output.contains(row), "missing {row}:\n{output}");
    }
}
