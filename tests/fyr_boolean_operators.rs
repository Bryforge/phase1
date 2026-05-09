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
fn fyr_supports_boolean_and_assertions() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let answer = 42; assert(answer >= 40 && answer <= 50); return answer; }' > bool.fyr\nfyr check bool.fyr\nfyr build bool.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok bool.fyr"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
}

#[test]
fn fyr_supports_boolean_or_assertions() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let answer = 42; assert(answer < 0 || answer == 42); return answer; }' > bool.fyr\nfyr check bool.fyr\nfyr build bool.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok bool.fyr"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
}

#[test]
fn fyr_reports_failed_boolean_and_assertion() {
    let output = run_phase1(
        "fyr init app\necho 'fn main() -> i32 { let answer = 42; assert(answer >= 40 && answer < 40); return answer; }' > app/tests/bool_fail.fyr\nfyr test app\nexit\n",
    );

    assert!(
        output.contains(
            "test    : app/tests/bool_fail.fyr failed: assertion failed: 42 >= 40 && 42 < 40"
        ),
        "{output}"
    );
    assert!(output.contains("status  : failed"), "{output}");
}
