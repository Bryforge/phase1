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
fn fyr_supports_grouped_boolean_assertions() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let answer = 42; assert((answer >= 40 && answer <= 50) || answer == 0); return answer; }' > group.fyr\nfyr check group.fyr\nfyr build group.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok group.fyr"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
}

#[test]
fn fyr_supports_nested_grouped_boolean_assertions() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let answer = 42; assert((answer == 42) && !(answer < 0 || answer > 100)); return answer; }' > group.fyr\nfyr check group.fyr\nfyr build group.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok group.fyr"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
}

#[test]
fn fyr_reports_failed_grouped_boolean_assertion() {
    let output = run_phase1(
        "fyr init app\necho 'fn main() -> i32 { let answer = 42; assert((answer < 0 || answer > 100) && answer == 42); return answer; }' > app/tests/group_fail.fyr\nfyr test app\nexit\n",
    );

    assert!(
        output.contains("test    : app/tests/group_fail.fyr failed: assertion failed: 42 < 0 || 42 > 100 && 42 == 42"),
        "{output}"
    );
    assert!(output.contains("status  : failed"), "{output}");
}
