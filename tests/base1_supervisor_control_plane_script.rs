use std::fs;
use std::path::Path;
use std::process::Command;

fn read_script() -> String {
    fs::read_to_string("scripts/base1-supervisor-control-plane.sh")
        .expect("read supervisor control-plane script")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn base1_supervisor_control_plane_script_exists_and_has_valid_shell_syntax() {
    let script = read_script();
    assert_contains(&script, "Base1 supervisor control-plane scaffold");
    assert_contains(&script, "does not boot kernels");
    assert_contains(&script, "modify host boot settings");
    assert_contains(&script, "prove hardening");
    assert_contains(&script, "claim hypervisor readiness");
    assert_contains(&script, "validate hardware");
    assert_contains(&script, "daily-driver readiness");

    let status = Command::new("sh")
        .arg("-n")
        .arg("scripts/base1-supervisor-control-plane.sh")
        .status()
        .expect("run sh -n");
    assert!(status.success(), "script should have valid shell syntax");
}

#[test]
fn base1_supervisor_control_plane_help_documents_commands_profiles_and_non_claims() {
    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-control-plane.sh")
        .arg("--help")
        .output()
        .expect("run help");
    assert!(output.status.success(), "help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);

    for expected in [
        "status",
        "plan",
        "stage-artifact",
        "validate-artifact",
        "launch-preview",
        "capture-evidence",
        "request-recovery",
        "stop",
        "--profile <name>",
        "profiles/base1",
        "supervisor-control-plane.env",
        "does not make Base1 bootable",
        "installer-ready",
        "hypervisor-ready",
        "daily-driver ready",
    ] {
        assert_contains(&stdout, expected);
    }
}

#[test]
fn base1_supervisor_control_plane_status_writes_report_for_x200_profile() {
    let out_dir = "build/base1-supervisor-control-plane-test-basic-status";
    let report_path = Path::new(out_dir).join("supervisor-control-plane.env");
    let _ = fs::remove_file(&report_path);

    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-control-plane.sh")
        .arg("status")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .arg("--out")
        .arg(out_dir)
        .arg("--write-report")
        .output()
        .expect("run status");

    assert!(output.status.success(), "status should pass");
    assert!(report_path.exists(), "report should be written");

    let report = fs::read_to_string(report_path).expect("read report");
    assert_contains(&report, "BASE1_SUPERVISOR_CONTROL_COMMAND=status");
    assert_contains(&report, "BASE1_SUPERVISOR_CONTROL_PROFILE=x200-supervisor-lite");
    assert_contains(&report, "BASE1_SUPERVISOR_CONTROL_PROFILE_CLASS=low-resource");
    assert_contains(&report, "BASE1_SUPERVISOR_CONTROL_MAX_CONCURRENCY=1");
    assert_contains(&report, "BASE1_SUPERVISOR_CONTROL_CLAIM=not_claimed");
}

#[test]
fn base1_supervisor_control_plane_supports_vm_capture_evidence_profile() {
    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-control-plane.sh")
        .arg("capture-evidence")
        .arg("--profile")
        .arg("x86_64-vm-validation")
        .output()
        .expect("run capture-evidence");
    assert!(output.status.success(), "capture-evidence should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains(&stdout, "command       : capture-evidence");
    assert_contains(&stdout, "action        : plan evidence capture paths");
    assert_contains(&stdout, "profile       : x86_64-vm-validation");
    assert_contains(&stdout, "storage_policy: build-directory-scratch");
    assert_contains(&stdout, "BASE1_SUPERVISOR_CONTROL_MAX_CONCURRENCY=3");
}

#[test]
fn base1_supervisor_control_plane_rejects_unknown_command_profile_and_non_build_out() {
    let bad_command = Command::new("sh")
        .arg("scripts/base1-supervisor-control-plane.sh")
        .arg("explode")
        .output()
        .expect("run bad command");
    assert!(!bad_command.status.success(), "unknown command should fail");

    let bad_profile = Command::new("sh")
        .arg("scripts/base1-supervisor-control-plane.sh")
        .arg("status")
        .arg("--profile")
        .arg("unknown")
        .output()
        .expect("run bad profile");
    assert!(!bad_profile.status.success(), "unknown profile should fail");

    let bad_out = Command::new("sh")
        .arg("scripts/base1-supervisor-control-plane.sh")
        .arg("status")
        .arg("--out")
        .arg("/tmp/base1-control")
        .output()
        .expect("run bad out");
    assert!(!bad_out.status.success(), "non-build out should fail");
}

#[test]
fn base1_supervisor_control_plane_blocks_x200_concurrent_launch_preview_by_policy() {
    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-control-plane.sh")
        .arg("launch-preview")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .arg("--delivery-mode")
        .arg("supervisor-concurrent")
        .arg("--out")
        .arg("build/base1-supervisor-control-plane-test-block")
        .output()
        .expect("run x200 blocked launch-preview");

    assert!(!output.status.success(), "x200 supervisor-concurrent launch-preview should be policy-blocked");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");
    assert_contains(&combined, "decision");
    assert_contains(&combined, "deny");
    assert_contains(&combined, "profile-upgrade-required");
}

#[test]
fn base1_supervisor_control_plane_records_policy_decision_for_status_report() {
    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-control-plane.sh")
        .arg("status")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .arg("--out")
        .arg("build/base1-supervisor-control-plane-test-status")
        .arg("--write-report")
        .output()
        .expect("run policy-gated status");

    assert!(output.status.success(), "status should pass");
    let report_path = Path::new("build/base1-supervisor-control-plane-test-status/supervisor-control-plane.env");
    assert!(report_path.exists(), "report should be written");
    let report = fs::read_to_string(report_path).expect("read report");
    assert_contains(&report, "BASE1_SUPERVISOR_CONTROL_POLICY_DECISION=allow");
    assert_contains(&report, "BASE1_SUPERVISOR_CONTROL_POLICY_REASON=profile allows read-only planning command");
    assert_contains(&report, "BASE1_SUPERVISOR_CONTROL_REQUESTED_MODE=direct-first");
}
