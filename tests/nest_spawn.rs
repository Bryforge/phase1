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
fn nest_spawn_creates_named_child_context() {
    let output = run_phase1("nest spawn demo\nnest list\nexit\n", "0", "2");

    assert!(output.contains("nest spawn: created demo"), "{output}");
    assert!(output.contains("nest list"), "{output}");
    assert!(output.contains("demo"), "{output}");
    assert!(output.contains("level   : 1/2"), "{output}");
    assert!(output.contains("mode    : isolated"), "{output}");
}

#[test]
fn nest_spawn_rejects_duplicate_child_context() {
    let output = run_phase1("nest spawn demo\nnest spawn demo\nexit\n", "0", "2");

    assert!(output.contains("nest spawn: created demo"), "{output}");
    assert!(
        output.contains("nest spawn: demo already exists"),
        "{output}"
    );
}

#[test]
fn nest_spawn_rejects_invalid_names() {
    let output = run_phase1("nest spawn ../escape\nexit\n", "0", "2");

    assert!(output.contains("nest spawn: invalid nest name"), "{output}");
}

#[test]
fn nest_spawn_respects_depth_cap() {
    let output = run_phase1("nest spawn too-deep\nexit\n", "2", "2");

    assert!(
        output.contains("nest spawn: max depth reached 2/2"),
        "{output}"
    );
}
