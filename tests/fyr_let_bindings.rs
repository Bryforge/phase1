use std::io::Write;
use std::process::{Command, Stdio};

fn run_phase1(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_phase1"))
        .env("PHASE1_BLEEDING_EDGE", "1")
        .env("PHASE1_DEVICE_MODE", "desktop")
        .env("PHASE1_MOBILE_MODE", "0")
        .env("PHASE1_COOKED_INPUT", "1")
        .env_remove("PHASE1_THEME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn phase1");

    let booted_input = format!("\n{input}");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(booted_input.as_bytes())
        .expect("write input");

    let output = child.wait_with_output().expect("phase1 output");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn fyr_let_bindings_work_in_single_file_check_and_build() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let answer = 42; assert(answer == 42); return answer; }' > vars.fyr\nfyr check vars.fyr\nfyr build vars.fyr\nexit\n",
    );

    assert!(output.contains("fyr check: ok vars.fyr"), "{output}");
    assert!(output.contains("source  : vars.fyr"), "{output}");
    assert!(
        output.contains("status  : dry-run artifact ready"),
        "{output}"
    );
}

#[test]
fn fyr_let_bindings_work_in_package_tests() {
    let output = run_phase1(
        "fyr init app\necho 'fn main() -> i32 { let x = 7; assert_eq(x, 7); assert(x > 3); return x; }' > app/tests/vars.fyr\nfyr test app\nexit\n",
    );

    assert!(
        output.contains("test    : app/tests/vars.fyr ok"),
        "{output}"
    );
    assert!(output.contains("passed  : 2"), "{output}");
    assert!(output.contains("failed  : 0"), "{output}");
    assert!(output.contains("status  : ok"), "{output}");
}

#[test]
fn fyr_let_binding_rejects_non_integer_values() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { let x = nope; assert(x == 1); return 0; }' > bad.fyr\nfyr check bad.fyr\nexit\n",
    );

    assert!(
        output.contains("expected integer let binding value"),
        "{output}"
    );
}
