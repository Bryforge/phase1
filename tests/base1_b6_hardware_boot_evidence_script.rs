use std::fs;
use std::path::Path;
use std::process::Command;

fn read_script() -> String {
    fs::read_to_string("scripts/base1-b6-hardware-boot-evidence.sh")
        .expect("read B6 hardware boot evidence script")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn base1_b6_hardware_boot_evidence_script_exists_and_has_valid_shell_syntax() {
    let script = read_script();
    assert_contains(&script, "Base1 B6 hardware boot evidence capture scaffold");
    assert_contains(&script, "does not write");
    assert_contains(&script, "format filesystems");
    assert_contains(&script, "install bootloaders");
    assert_contains(&script, "modify host boot settings");
    assert_contains(&script, "prove hardening");
    assert_contains(&script, "daily-driver readiness");

    let status = Command::new("sh")
        .arg("-n")
        .arg("scripts/base1-b6-hardware-boot-evidence.sh")
        .status()
        .expect("run sh -n");
    assert!(status.success(), "script should have valid shell syntax");
}

#[test]
fn base1_b6_hardware_boot_evidence_help_documents_result_states() {
    let output = Command::new("sh")
        .arg("scripts/base1-b6-hardware-boot-evidence.sh")
        .arg("--help")
        .output()
        .expect("run help");
    assert!(output.status.success(), "help should pass");
    let stdout = String::from_utf8_lossy(&output.stdout);

    for expected in [
        "not_attempted",
        "boot_menu_seen",
        "boot_started",
        "phase1_marker_seen",
        "blocked",
        "failed",
        "does not make Base1 installer-ready",
        "daily-driver ready",
    ] {
        assert_contains(&stdout, expected);
    }
}

#[test]
fn base1_b6_hardware_boot_evidence_records_not_attempted_report() {
    let out_dir = "build/base1-b6-hardware-boot-evidence-test";
    let report_path = Path::new(out_dir).join("b6-hardware-boot-evidence.env");
    let _ = fs::remove_file(&report_path);

    let output = Command::new("sh")
        .arg("scripts/base1-b6-hardware-boot-evidence.sh")
        .arg("--record")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .arg("--machine")
        .arg("Chases-MacBook-Air")
        .arg("--artifact")
        .arg("build/phase1-uefi.img")
        .arg("--result")
        .arg("not_attempted")
        .arg("--out")
        .arg(out_dir)
        .arg("--write-report")
        .output()
        .expect("run B6 record");

    assert!(output.status.success(), "B6 record should pass");
    assert!(report_path.exists(), "report should be written");
    let report = fs::read_to_string(report_path).expect("read report");
    assert_contains(&report, "BASE1_B6_HARDWARE_BOOT_MODE=record");
    assert_contains(
        &report,
        "BASE1_B6_HARDWARE_BOOT_PROFILE=x200-supervisor-lite",
    );
    assert_contains(&report, "BASE1_B6_HARDWARE_BOOT_MACHINE=Chases-MacBook-Air");
    assert_contains(&report, "BASE1_B6_HARDWARE_BOOT_RESULT=not_attempted");
    assert_contains(&report, "BASE1_B6_HARDWARE_BOOT_CLAIM=not_claimed");
}

#[test]
fn base1_b6_hardware_boot_evidence_rejects_bad_inputs() {
    let bad_profile = Command::new("sh")
        .arg("scripts/base1-b6-hardware-boot-evidence.sh")
        .arg("--profile")
        .arg("unknown")
        .arg("--machine")
        .arg("test-machine")
        .output()
        .expect("run bad profile");
    assert!(!bad_profile.status.success(), "unknown profile should fail");

    let bad_result = Command::new("sh")
        .arg("scripts/base1-b6-hardware-boot-evidence.sh")
        .arg("--result")
        .arg("booted_daily_driver")
        .arg("--machine")
        .arg("test-machine")
        .output()
        .expect("run bad result");
    assert!(!bad_result.status.success(), "unknown result should fail");

    let bad_out = Command::new("sh")
        .arg("scripts/base1-b6-hardware-boot-evidence.sh")
        .arg("--out")
        .arg("/tmp/base1-b6")
        .arg("--machine")
        .arg("test-machine")
        .output()
        .expect("run bad out");
    assert!(!bad_out.status.success(), "non-build out should fail");

    let bad_artifact = Command::new("sh")
        .arg("scripts/base1-b6-hardware-boot-evidence.sh")
        .arg("--artifact")
        .arg("/tmp/phase1-uefi.img")
        .arg("--machine")
        .arg("test-machine")
        .output()
        .expect("run bad artifact");
    assert!(
        !bad_artifact.status.success(),
        "non-build artifact should fail"
    );
}
