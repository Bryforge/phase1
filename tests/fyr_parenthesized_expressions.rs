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
fn fyr_parentheses_group_addition_before_multiplication() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let answer = (40 + 2) * 2; assert(answer == 84); return answer; }' > math.fyr\nfyr check math.fyr\nfyr build math.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok math.fyr"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
}

#[test]
fn fyr_parentheses_group_division_terms() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let total = 84; let answer = total / (1 + 1); assert(answer == 42); return answer; }' > math.fyr\nfyr check math.fyr\nfyr build math.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok math.fyr"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
}

#[test]
fn fyr_unclosed_parenthesis_reports_diagnostic() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let answer = (40 + 2; assert(answer == 42); return answer; }' > bad.fyr\nfyr check bad.fyr\nexit\n",
    );

    assert!(
        output.contains("expected ')' in integer expression"),
        "{output}"
    );
}
