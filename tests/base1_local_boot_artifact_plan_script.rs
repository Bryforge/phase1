use std::fs;
use std::path::Path;
use std::process::Command;

fn read_script() -> String {
    fs::read_to_string("scripts/base1-local-boot-artifact-plan.sh")
        .expect("read local boot artifact planner")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn base1_local_boot_artifact_plan_script_exists_and_has_valid_shell_syntax() {
    let script = read_script();
    assert_contains(&script, "Base1 local boot artifact planner");
    assert_contains(&script, "without writing disks");
    assert_contains(&script, "modifying host boot settings");
    assert_contains(&script, "formatting filesystems");
    assert_contains(&script, "installing bootloaders");
    assert_contains(&script, "validating hardware");
    assert_contains(&script, "daily-driver readiness");

    let status = Command::new("sh")
        .arg("-n")
        .arg("scripts/base1-local-boot-artifact-plan.sh")
        .status()
        .expect("run sh -n");
    assert!(status.success(), "script should have valid shell syntax");
}

#[test]
fn base1_local_boot_artifact_plan_help_documents_scope() {
    let output = Command::new("sh")
        .arg("scripts/base1-local-boot-artifact-plan.sh")
        .arg("--help")
        .output()
        .expect("run help");
    assert!(output.status.success(), "help should pass");
    let stdout = String::from_utf8_lossy(&output.stdout);

    for expected in [
        "base1 local boot artifact planner",
        "--artifact <path>",
        "build/phase1-uefi.img",
        "local boot artifact report",
        "does not make Base1 bootable on hardware",
        "installer-ready",
        "hardware-validated",
        "daily-driver ready",
    ] {
        assert_contains(&stdout, expected);
    }
}

#[test]
fn base1_local_boot_artifact_plan_prepare_writes_report() {
    let out_dir = "build/base1-local-boot-artifact-test";
    let report_path = Path::new(out_dir).join("local-boot-artifact-plan.env");
    let _ = fs::remove_file(&report_path);

    let output = Command::new("sh")
        .arg("scripts/base1-local-boot-artifact-plan.sh")
        .arg("--prepare")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .arg("--artifact")
        .arg("build/phase1-uefi.img")
        .arg("--out")
        .arg(out_dir)
        .arg("--write-report")
        .output()
        .expect("run prepare");

    assert!(output.status.success(), "prepare should pass");
    assert!(report_path.exists(), "report should be written");
    let report = fs::read_to_string(report_path).expect("read report");
    assert_contains(&report, "BASE1_LOCAL_BOOT_ARTIFACT_MODE=prepare");
    assert_contains(&report, "BASE1_LOCAL_BOOT_ARTIFACT_PROFILE=x200-supervisor-lite");
    assert_contains(&report, "BASE1_LOCAL_BOOT_ARTIFACT_PATH=build/phase1-uefi.img");
    assert_contains(&report, "BASE1_LOCAL_BOOT_ARTIFACT_LOCAL_ONLY=1");
    assert_contains(&report, "BASE1_LOCAL_BOOT_ARTIFACT_RECOVERY_EVIDENCE_REQUIRED=1");
    assert_contains(&report, "BASE1_LOCAL_BOOT_ARTIFACT_CLAIM=not_claimed");
}

#[test]
fn base1_local_boot_artifact_plan_rejects_unknown_profile_and_non_build_paths() {
    let bad_profile = Command::new("sh")
        .arg("scripts/base1-local-boot-artifact-plan.sh")
        .arg("--profile")
        .arg("unknown")
        .output()
        .expect("run bad profile");
    assert!(!bad_profile.status.success(), "unknown profile should fail");

    let bad_out = Command::new("sh")
        .arg("scripts/base1-local-boot-artifact-plan.sh")
        .arg("--out")
        .arg("/tmp/base1-local-boot")
        .output()
        .expect("run bad out");
    assert!(!bad_out.status.success(), "non-build out should fail");

    let bad_artifact = Command::new("sh")
        .arg("scripts/base1-local-boot-artifact-plan.sh")
        .arg("--artifact")
        .arg("/tmp/phase1-uefi.img")
        .output()
        .expect("run bad artifact");
    assert!(!bad_artifact.status.success(), "non-build artifact should fail");
}
