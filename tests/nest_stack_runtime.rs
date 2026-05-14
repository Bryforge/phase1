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
fn nest_stack_status_reports_read_only_stack_rows() {
    let output = run_phase1(
        "nest stack\nnest stack status\nnest stack list\nexit\n",
        "1",
        "3",
    );

    assert!(output.contains("phase1 nest stack"), "{output}");
    assert!(
        output.contains("mode          : read-only status"),
        "{output}"
    );
    assert!(output.contains("nest-level    : 1/3"), "{output}");
    assert!(output.contains("current       : level-1"), "{output}");
    assert!(output.contains("ghost-count   : 0"), "{output}");
    assert!(
        output.contains("claim-boundary: control-plane-only"),
        "{output}"
    );
}

#[test]
fn nest_stack_pending_actions_are_noop_and_help_guided() {
    let output = run_phase1(
        "nest stack push demo\nnest stack ghost demo\nnest stack resume demo\nexit\n",
        "0",
        "1",
    );

    assert!(output.contains("nest stack push"), "{output}");
    assert!(output.contains("nest stack ghost"), "{output}");
    assert!(output.contains("nest stack resume"), "{output}");
    assert!(
        output.contains("status        : not-yet-implemented"),
        "{output}"
    );
    assert!(output.contains("result        : no-op"), "{output}");
    assert!(
        output.contains("help          : nest stack status"),
        "{output}"
    );
    assert!(
        output.contains("claim-boundary: control-plane-only"),
        "{output}"
    );
}

#[test]
fn nest_stack_unknown_action_is_noop() {
    let output = run_phase1("nest stack warp\nexit\n", "0", "1");

    assert!(output.contains("nest stack warp"), "{output}");
    assert!(
        output.contains("status        : unknown stack action"),
        "{output}"
    );
    assert!(output.contains("result        : no-op"), "{output}");
    assert!(
        output.contains("help          : nest stack status"),
        "{output}"
    );
    assert!(
        output.contains("claim-boundary: control-plane-only"),
        "{output}"
    );
}
