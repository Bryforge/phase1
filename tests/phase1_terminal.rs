use std::fs;
use std::process::Command;

#[test]
fn terminal_wrapper_files_exist_and_have_valid_shell_syntax() {
    for path in [
        "terminal/bin/phase1-terminal",
        "scripts/test-phase1-terminal.sh",
    ] {
        assert!(fs::metadata(path).is_ok(), "missing terminal file: {path}");
        let status = Command::new("sh")
            .arg("-n")
            .arg(path)
            .status()
            .expect("run sh -n");
        assert!(status.success(), "script has invalid shell syntax: {path}");
    }
}

#[test]
fn terminal_help_describes_safe_wrapper_contract() {
    let output = Command::new("sh")
        .arg("terminal/bin/phase1-terminal")
        .arg("help")
        .output()
        .expect("run terminal help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Phase1 Terminal"));
    assert!(stdout.contains("thin, safe wrapper"));
    assert!(stdout.contains("phase1-terminal gina"));
}

#[test]
fn terminal_doctor_delegates_to_start_phase1() {
    let output = Command::new("sh")
        .arg("terminal/bin/phase1-terminal")
        .arg("doctor")
        .output()
        .expect("run terminal doctor");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Phase1 launch doctor"));
    assert!(stdout.contains("gina"));
    assert!(stdout.contains("base1"));
}

#[test]
fn terminal_selftest_and_validation_script_pass() {
    let selftest = Command::new("sh")
        .arg("terminal/bin/phase1-terminal")
        .arg("selftest")
        .output()
        .expect("run terminal selftest");
    assert!(
        selftest.status.success(),
        "terminal selftest failed: {}",
        String::from_utf8_lossy(&selftest.stderr)
    );
    assert!(String::from_utf8_lossy(&selftest.stdout).contains("selftest: ok"));

    let validation = Command::new("sh")
        .arg("scripts/test-phase1-terminal.sh")
        .output()
        .expect("run terminal validation script");
    assert!(
        validation.status.success(),
        "terminal validation failed: {}",
        String::from_utf8_lossy(&validation.stderr)
    );
    assert!(String::from_utf8_lossy(&validation.stdout).contains("test-phase1-terminal: ok"));
}

#[test]
fn terminal_docs_point_to_start_phase1_as_source_of_truth() {
    let terminal_doc = fs::read_to_string("TERMINAL.md").expect("read TERMINAL.md");
    let readme = fs::read_to_string("terminal/README.md").expect("read terminal README");
    assert!(terminal_doc.contains("./start_phase1"));
    assert!(terminal_doc.contains("canonical launch path"));
    assert!(terminal_doc.contains("no full terminal-emulator claim"));
    assert!(readme.contains("delegates to ../../start_phase1"));
}
