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
fn nest_tree_reports_empty_root() {
    let output = run_phase1("nest tree\nexit\n", "0", "3");

    assert!(output.contains("nest tree"), "{output}");
    assert!(output.contains("root"), "{output}");
    assert!(output.contains("children: none"), "{output}");
}

#[test]
fn nest_tree_reports_spawned_children() {
    let output = run_phase1(
        "nest spawn alpha\nnest spawn beta\nnest tree\nexit\n",
        "0",
        "3",
    );

    assert!(output.contains("nest tree"), "{output}");
    assert!(output.contains("root"), "{output}");
    assert!(output.contains("alpha"), "{output}");
    assert!(output.contains("beta"), "{output}");
    assert!(output.contains("level   : 1/3"), "{output}");
    assert!(output.contains("path    : /nest/alpha"), "{output}");
    assert!(output.contains("path    : /nest/beta"), "{output}");
}

#[test]
fn nest_tree_marks_active_child_context() {
    let output = run_phase1(
        "nest spawn alpha\nnest spawn beta\nnest enter beta\nnest tree\nexit\n",
        "0",
        "3",
    );

    assert!(output.contains("beta *"), "{output}");
    assert!(output.contains("active  : beta"), "{output}");
}

#[test]
fn nest_help_lists_tree_command() {
    let output = run_phase1("nest help\nexit\n", "0", "3");

    assert!(output.contains("nest tree"), "{output}");
}
