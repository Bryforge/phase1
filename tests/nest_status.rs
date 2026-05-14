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
fn nest_status_reports_level_and_max_depth() {
    let output = run_phase1("nest status\nexit\n", "1", "3");

    assert!(output.contains("nest status"), "{output}");
    assert!(output.contains("level   : 1/3"), "{output}");
    assert!(output.contains("root    : no"), "{output}");
    assert!(output.contains("mode    : isolated"), "{output}");
}

#[test]
fn nest_status_reports_root_context() {
    let output = run_phase1("nest status\nexit\n", "0", "2");

    assert!(output.contains("nest status"), "{output}");
    assert!(output.contains("level   : 0/2"), "{output}");
    assert!(output.contains("root    : yes"), "{output}");
}

#[test]
fn nest_help_lists_status_command() {
    let output = run_phase1("nest help\nexit\n", "0", "1");

    assert!(output.contains("nest status"), "{output}");
    assert!(output.contains("nest stack"), "{output}");
    assert!(output.contains("nest exit-all"), "{output}");
}
