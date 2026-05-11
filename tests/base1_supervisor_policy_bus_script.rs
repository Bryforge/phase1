use std::fs;
use std::path::Path;
use std::process::Command;

fn read_script() -> String {
    fs::read_to_string("scripts/base1-supervisor-policy-bus.sh")
        .expect("read supervisor policy bus script")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn base1_supervisor_policy_bus_script_exists_and_has_valid_shell_syntax() {
    let script = read_script();
    assert_contains(&script, "Base1 supervisor policy bus scaffold");
    assert_contains(&script, "does not boot kernels");
    assert_contains(&script, "modify host boot settings");
    assert_contains(&script, "prove hardening");
    assert_contains(&script, "claim hypervisor readiness");
    assert_contains(&script, "validate hardware");
    assert_contains(&script, "daily-driver readiness");

    let status = Command::new("sh")
        .arg("-n")
        .arg("scripts/base1-supervisor-policy-bus.sh")
        .status()
        .expect("run sh -n");
    assert!(status.success(), "script should have valid shell syntax");
}

#[test]
fn base1_supervisor_policy_bus_help_documents_policy_surface() {
    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-policy-bus.sh")
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
        "allow",
        "deny",
        "plan-only",
        "evidence-required",
        "profile-upgrade-required",
        "does not make Base1 bootable",
        "daily-driver ready",
    ] {
        assert_contains(&stdout, expected);
    }
}

#[test]
fn base1_supervisor_policy_bus_status_writes_x200_report() {
    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-policy-bus.sh")
        .arg("status")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .arg("--write-report")
        .output()
        .expect("run status");
    assert!(output.status.success(), "status should pass");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains(&stdout, "decision      : allow");
    assert_contains(&stdout, "profile       : x200-supervisor-lite");

    let report_path = Path::new("build/base1-supervisor-policy-bus/supervisor-policy-bus.env");
    assert!(report_path.exists(), "report should be written");
    let report = fs::read_to_string(report_path).expect("read report");
    assert_contains(&report, "BASE1_SUPERVISOR_POLICY_COMMAND=status");
    assert_contains(&report, "BASE1_SUPERVISOR_POLICY_PROFILE=x200-supervisor-lite");
    assert_contains(&report, "BASE1_SUPERVISOR_POLICY_DECISION=allow");
    assert_contains(&report, "BASE1_SUPERVISOR_POLICY_CLAIM=not_claimed");
}

#[test]
fn base1_supervisor_policy_bus_denies_x200_concurrent_mode() {
    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-policy-bus.sh")
        .arg("launch-preview")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .arg("--delivery-mode")
        .arg("supervisor-concurrent")
        .output()
        .expect("run x200 concurrent denial");
    assert!(output.status.success(), "policy denial is still a planned result");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains(&stdout, "decision      : deny");
    assert_contains(&stdout, "profile-upgrade-required");
    assert_contains(&stdout, "requested delivery mode is not allowed");
}

#[test]
fn base1_supervisor_policy_bus_allows_vm_concurrent_as_evidence_required() {
    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-policy-bus.sh")
        .arg("launch-preview")
        .arg("--profile")
        .arg("x86_64-vm-validation")
        .arg("--delivery-mode")
        .arg("supervisor-concurrent")
        .output()
        .expect("run vm concurrent policy");
    assert!(output.status.success(), "vm concurrent policy should pass");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains(&stdout, "decision      : evidence-required");
    assert_contains(&stdout, "profile       : x86_64-vm-validation");
    assert_contains(&stdout, "storage_policy: build-directory-scratch");
}

#[test]
fn base1_supervisor_policy_bus_rejects_unknown_command_profile_and_non_build_out() {
    let bad_command = Command::new("sh")
        .arg("scripts/base1-supervisor-policy-bus.sh")
        .arg("unknown")
        .output()
        .expect("run bad command");
    assert!(!bad_command.status.success(), "unknown command should fail");

    let bad_profile = Command::new("sh")
        .arg("scripts/base1-supervisor-policy-bus.sh")
        .arg("status")
        .arg("--profile")
        .arg("unknown")
        .output()
        .expect("run bad profile");
    assert!(!bad_profile.status.success(), "unknown profile should fail");

    let bad_out = Command::new("sh")
        .arg("scripts/base1-supervisor-policy-bus.sh")
        .arg("status")
        .arg("--out")
        .arg("/tmp/base1-policy")
        .output()
        .expect("run bad out");
    assert!(!bad_out.status.success(), "non-build out should fail");
}
