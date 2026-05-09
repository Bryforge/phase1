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
fn nest_inspect_reports_child_metadata() {
    let output = run_phase1("nest spawn demo\nnest inspect demo\nexit\n", "0", "3");

    assert!(output.contains("nest inspect"), "{output}");
    assert!(output.contains("name    : demo"), "{output}");
    assert!(output.contains("level   : 1/3"), "{output}");
    assert!(output.contains("active  : no"), "{output}");
    assert!(output.contains("path    : /nest/demo"), "{output}");
    assert!(output.contains("mode    : isolated"), "{output}");
}

#[test]
fn nest_inspect_reports_active_child() {
    let output = run_phase1(
        "nest spawn demo\nnest enter demo\nnest inspect demo\nexit\n",
        "0",
        "3",
    );

    assert!(output.contains("name    : demo"), "{output}");
    assert!(output.contains("active  : yes"), "{output}");
}

#[test]
fn nest_inspect_reports_missing_child() {
    let output = run_phase1("nest inspect missing\nexit\n", "0", "3");

    assert!(
        output.contains("nest inspect: missing not found"),
        "{output}"
    );
}

#[test]
fn nest_inspect_rejects_invalid_names() {
    let output = run_phase1("nest inspect ../escape\nexit\n", "0", "3");

    assert!(
        output.contains("nest inspect: invalid nest name"),
        "{output}"
    );
}
