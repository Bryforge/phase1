use std::io::Write;
use std::process::{Command, Stdio};

fn run_phase1(input: &str, level: &str, max: &str) -> String {
    let exe = env!("CARGO_BIN_EXE_phase1");
    let mut child = Command::new(exe)
        .env("PHASE1_TEST_MODE", "1")
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
fn nest_destroy_removes_child_context() {
    let output = run_phase1(
        "nest spawn demo\nnest destroy demo\nnest list\nexit\n",
        "0",
        "2",
    );

    assert!(output.contains("nest spawn: created demo"), "{output}");
    assert!(output.contains("nest destroy: removed demo"), "{output}");
    assert!(output.contains("children: none"), "{output}");
}

#[test]
fn nest_destroy_reports_missing_child_context() {
    let output = run_phase1("nest destroy missing\nexit\n", "0", "2");

    assert!(
        output.contains("nest destroy: missing not found"),
        "{output}"
    );
}

#[test]
fn nest_destroy_rejects_invalid_names() {
    let output = run_phase1("nest destroy ../escape\nexit\n", "0", "2");

    assert!(
        output.contains("nest destroy: invalid nest name"),
        "{output}"
    );
}

#[test]
fn nest_destroy_active_context_returns_to_root() {
    let output = run_phase1(
        "nest spawn demo\nnest enter demo\nnest destroy demo\nnest status\nexit\n",
        "0",
        "2",
    );

    assert!(output.contains("nest enter: demo"), "{output}");
    assert!(output.contains("nest destroy: removed demo"), "{output}");
    assert!(output.contains("active  : root"), "{output}");
    assert!(output.contains("level   : 0/2"), "{output}");
    assert!(output.contains("path    : /"), "{output}");
}
