use std::env;
use std::fs;
use std::process::Command;

#[test]
fn terminal_wrapper_files_exist_and_have_valid_shell_syntax() {
    for path in [
        "phase1",
        "terminal/bin/phase1-terminal",
        "scripts/install-phase1-command.sh",
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
fn simple_phase1_command_delegates_to_phase1() {
    let help = Command::new("sh")
        .arg("phase1")
        .arg("help")
        .output()
        .expect("run phase1 help");
    assert!(help.status.success());
    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(help_stdout.contains("Fresh clone shortcut"));
    assert!(help_stdout.contains("sh phase1"));
    assert!(help_stdout.contains("install-phase1-command"));

    let doctor = Command::new("sh")
        .arg("phase1")
        .arg("doctor")
        .output()
        .expect("run phase1 doctor");
    assert!(doctor.status.success());
    let doctor_stdout = String::from_utf8_lossy(&doctor.stdout);
    assert!(doctor_stdout.contains("Phase1 launch doctor"));
    assert!(doctor_stdout.contains("launcher"));
}

#[test]
fn install_phase1_command_creates_executable_wrapper() {
    let temp_dir = env::temp_dir().join(format!("phase1-command-test-{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("create temp command dir");

    let install = Command::new("sh")
        .arg("scripts/install-phase1-command.sh")
        .env("PHASE1_BIN_DIR", &temp_dir)
        .output()
        .expect("run installer");
    assert!(
        install.status.success(),
        "installer failed: {}",
        String::from_utf8_lossy(&install.stderr)
    );

    let installed = temp_dir.join("phase1");
    assert!(
        installed.exists(),
        "installer did not create phase1 command"
    );

    let version = Command::new(&installed)
        .arg("version")
        .output()
        .expect("run installed phase1 command");
    assert!(version.status.success());
    assert!(String::from_utf8_lossy(&version.stdout).contains("Phase1"));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp command dir");
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
fn terminal_doctor_delegates_to_phase1() {
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
fn terminal_docs_point_to_simple_phase1_command_and_source_launcher() {
    let terminal_doc =
        fs::read_to_string("docs/operators/TERMINAL.md").expect("read docs/operators/TERMINAL.md");
    let readme = fs::read_to_string("terminal/README.md").expect("read terminal README");
    let root_readme = fs::read_to_string("README.md").expect("read README.md");
    assert!(terminal_doc.contains("./phase1"));
    assert!(terminal_doc.contains("./phase1"));
    assert!(terminal_doc.contains("no full terminal-emulator claim"));
    assert!(readme.contains("delegates to `./phase1`"));
    assert!(root_readme.contains("sh phase1"));
    assert!(root_readme.contains("install-phase1-command"));
}
