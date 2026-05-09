use std::io::Write;
use std::process::{Command, Stdio};

fn run_phase1(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .env("PHASE1_TEST_MODE", "1")
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
        .expect("write stdin");

    let output = child.wait_with_output().expect("phase1 output");
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn fyr_let_bindings_support_addition_expression() {
    let output = run_phase1(
        "fyr init app\n\
echo 'fn main() -> i32 { let base = 40; let answer = base + 2; assert(answer == 42); return answer; }' > app/tests/math_add.fyr\n\
fyr check app\n\
fyr test app\n\
exit\n",
    );

    assert!(
        output.contains("fyr check: ok app/src/main.fyr"),
        "{output}"
    );
    assert!(
        output.contains("test    : app/tests/math_add.fyr ok"),
        "{output}"
    );
    assert!(output.contains("status  : ok"), "{output}");
}

#[test]
fn fyr_let_bindings_support_subtraction_expression() {
    let output = run_phase1(
        "fyr init app\n\
echo 'fn main() -> i32 { let base = 50; let answer = base - 8; assert_eq(answer, 42); return answer; }' > app/tests/math_sub.fyr\n\
fyr check app\n\
fyr test app\n\
exit\n",
    );

    assert!(
        output.contains("fyr check: ok app/src/main.fyr"),
        "{output}"
    );
    assert!(
        output.contains("test    : app/tests/math_sub.fyr ok"),
        "{output}"
    );
    assert!(output.contains("status  : ok"), "{output}");
}
