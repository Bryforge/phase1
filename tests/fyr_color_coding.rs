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
fn fyr_color_highlights_core_language_tokens() {
    let output = run_phase1(
        "echo 'fn main() -> i32 { print(\"hi\"); assert(1 == 1); return 0; }' > color.fyr\nfyr color color.fyr\nexit\n",
    );

    assert!(
        output.contains("\u{1b}["),
        "expected ANSI color output:\n{output}"
    );
    assert!(output.contains("fn"), "{output}");
    assert!(output.contains("main"), "{output}");
    assert!(output.contains("print"), "{output}");
    assert!(output.contains("\"hi\""), "{output}");
    assert!(output.contains("return"), "{output}");
}

#[test]
fn fyr_color_reports_missing_file() {
    let output = run_phase1("fyr color missing.fyr\nexit\n");

    assert!(
        output.contains("fyr color: no such file: missing.fyr"),
        "{output}"
    );
}
