use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn dev_dock_exposes_self_hosted_workflow_commands() {
    let dev = fs::read_to_string("plugins/dev.py").expect("plugins/dev.py exists");
    assert!(dev.contains("dev docs"));
    assert!(dev.contains("dev checkpoint"));
    assert!(dev.contains("no changes"));
    assert!(dev.contains("scripts/update-docs.py"));
}

#[test]
fn dev_dock_docs_explain_inside_phase1_workflow() {
    let docs = fs::read_to_string("DEV_DOCK.md").expect("DEV_DOCK.md exists");
    assert!(docs.contains("dev docs"));
    assert!(docs.contains("dev checkpoint"));
    assert!(docs.contains("inside Phase1"));
}

#[test]
fn dev_plugin_reads_phase1_context_args_from_stdin() {
    let mut child = Command::new("python3")
        .arg("plugins/dev.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn dev plugin");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"COMMAND=dev\nARGS=definitely-missing\nUSER=root\nCWD=/\nVERSION=test\n")
        .expect("write plugin context");

    let output = child.wait_with_output().expect("plugin output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("dev: unknown action: definitely-missing"),
        "{}",
        stdout
    );
}
