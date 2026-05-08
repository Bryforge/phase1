use std::fs;
use std::process::Command;

#[test]
fn launch_scripts_exist_and_have_valid_shell_syntax() {
    for path in ["start_phase1", "scripts/configure-phase1.sh", "scripts/test-phase1-launch.sh"] {
        assert!(fs::metadata(path).is_ok(), "missing launch script: {path}");
        let status = Command::new("sh")
            .arg("-n")
            .arg(path)
            .status()
            .expect("run sh -n");
        assert!(status.success(), "script has invalid shell syntax: {path}");
    }
}

#[test]
fn launch_help_displays_simple_command() {
    let output = Command::new("sh")
        .arg("start_phase1")
        .arg("--help")
        .output()
        .expect("run start help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("./start_phase1"));
    assert!(stdout.contains("Simple launch command"));
}

#[test]
fn launch_doctor_reports_gina_base1_and_config() {
    let output = Command::new("sh")
        .arg("start_phase1")
        .arg("--doctor")
        .output()
        .expect("run start doctor");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Phase1 launch doctor"));
    assert!(stdout.contains("gina"));
    assert!(stdout.contains("base1"));
    assert!(stdout.contains("config"));
}

#[test]
fn configure_dry_run_is_safe_and_mentions_launch_command() {
    let output = Command::new("sh")
        .arg("scripts/configure-phase1.sh")
        .arg("--dry-run")
        .output()
        .expect("run configure dry-run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Phase1 absolute configuration"));
    assert!(stdout.contains("Launch command: ./start_phase1"));
    assert!(stdout.contains("dry-run"));
}

#[test]
fn gina_is_configured_for_phase1_operations() {
    let gina = fs::read_to_string("plugins/gina.wasi").expect("read gina manifest");
    assert!(gina.contains("Phase1 AI operations assistant"));
    assert!(gina.contains("cybersecurity"));
    assert!(gina.contains("Base1"));
    assert!(gina.contains("quality"));
    assert!(gina.contains("./start_phase1"));
    assert!(gina.contains("offline"));
}
