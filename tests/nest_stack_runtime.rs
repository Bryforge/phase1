use std::io::Write;
use std::process::{Command, Stdio};

fn run_phase1(input: &str, level: &str, max: &str) -> String {
    let exe = env!("CARGO_BIN_EXE_phase1");
    let mut child = Command::new(exe)
        .env("PHASE1_TEST_MODE", "1")
        .env("PHASE1_NO_COLOR", "1")
        .env("PHASE1_ASCII", "1")
        .env("PHASE1_COOKED_INPUT", "1")
        .env("PHASE1_MOBILE_MODE", "1")
        .env("PHASE1_NESTED_LEVEL", level)
        .env("PHASE1_NESTED_MAX", max)
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
fn stack_status_reports_read_only_rows() {
    let output = run_phase1("nest stack\nnest stack status\nnest stack list\nexit\n", "1", "3");

    for row in [
        "phase1 nest stack",
        "mode          : read-only status",
        "nest-level    : 1/3",
        "current       : level-1",
        "claim-boundary: control-plane-only",
    ] {
        assert!(output.contains(row), "missing {row}:\n{output}");
    }
}

#[test]
fn stack_pending_action_is_noop_and_help_guided() {
    let output = run_phase1("nest stack push demo\nexit\n", "0", "1");

    for row in [
        "nest stack push",
        "status        : not-yet-implemented",
        "result        : no-op",
        "help          : nest stack status",
        "claim-boundary: control-plane-only",
    ] {
        assert!(output.contains(row), "missing {row}:\n{output}");
    }
}
