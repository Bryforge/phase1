use std::io::Write;
use std::process::{Command, Stdio};

fn run_phase1(input: &str) -> String {
    let exe = env!("CARGO_BIN_EXE_phase1");
    let mut child = Command::new(exe)
        .env("PHASE1_TEST_MODE", "1")
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
fn fyr_supports_if_return_statement() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let answer = 42; if answer == 42 { return 1; } return 0; }' > if.fyr\nfyr check if.fyr\nfyr build if.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok if.fyr"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
}

#[test]
fn fyr_supports_if_with_grouped_boolean_condition() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let answer = 42; if (answer >= 40 && answer <= 50) { return 1; } return 0; }' > if.fyr\nfyr check if.fyr\nfyr build if.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok if.fyr"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
}

#[test]
fn fyr_if_requires_boolean_condition() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { if 42 { return 1; } return 0; }' > bad.fyr\nfyr check bad.fyr\nexit\n",
    );

    assert!(
        output.contains("bad.fyr: expected boolean if condition"),
        "{output}"
    );
}
