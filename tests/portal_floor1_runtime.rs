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
fn portal_help_reports_first_slice_and_floor1_policy() {
    let output = run_phase1("portal help\nexit\n");

    for row in [
        "portal help",
        "usage             : portal <status|list|help>",
        "first-slice       : read-only status/list/help",
        "floor             : floor1",
        "network-default   : denied",
        "claim-boundary    : workspace-context-only",
    ] {
        assert!(output.contains(row), "missing {row}:\n{output}");
    }
}

#[test]
fn portal_pending_actions_are_noop_and_help_guided() {
    let output = run_phase1("portal open alpha\nexit\n");

    for row in [
        "portal open",
        "status            : not-yet-implemented",
        "result            : no-op",
        "help              : portal status",
        "claim-boundary    : workspace-context-only",
    ] {
        assert!(output.contains(row), "missing {row}:\n{output}");
    }
}
