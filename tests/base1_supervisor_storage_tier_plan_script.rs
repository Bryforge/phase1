use std::fs;
use std::path::Path;
use std::process::Command;

fn read_script() -> String {
    fs::read_to_string("scripts/base1-supervisor-storage-tier-plan.sh")
        .expect("read storage tier planner script")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}: {haystack}");
}

#[test]
fn base1_supervisor_storage_tier_plan_script_exists_and_has_valid_shell_syntax() {
    let script = read_script();
    assert_contains(&script, "Base1 supervisor storage-tier planner");
    assert_contains(&script, "does not mount filesystems");
    assert_contains(&script, "create swap");
    assert_contains(&script, "mutate disks");
    assert_contains(&script, "modify host boot settings");
    assert_contains(&script, "prove hardening");
    assert_contains(&script, "claim hypervisor readiness");

    let status = Command::new("sh")
        .arg("-n")
        .arg("scripts/base1-supervisor-storage-tier-plan.sh")
        .status()
        .expect("run sh -n");
    assert!(status.success(), "script should have valid shell syntax");
}

#[test]
fn base1_supervisor_storage_tier_plan_help_documents_scope() {
    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-storage-tier-plan.sh")
        .arg("--help")
        .output()
        .expect("run help");
    assert!(output.status.success(), "help should pass");
    let stdout = String::from_utf8_lossy(&output.stdout);

    for expected in [
        "real RAM",
        "small tmpfs",
        "zram",
        "SSD scratch",
        "swap backstop",
        "persistent evidence logs",
        "does not make Base1 bootable",
        "daily-driver ready",
    ] {
        assert_contains(&stdout, expected);
    }
}

#[test]
fn base1_supervisor_storage_tier_plan_writes_x200_report() {
    let out_dir = "build/base1-supervisor-storage-tier-test-x200";
    let report_path = Path::new(out_dir).join("supervisor-storage-tier.env");
    let _ = fs::remove_file(&report_path);

    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-storage-tier-plan.sh")
        .arg("--prepare")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .arg("--out")
        .arg(out_dir)
        .arg("--write-report")
        .output()
        .expect("run x200 prepare");

    assert!(output.status.success(), "x200 prepare should pass");
    assert!(report_path.exists(), "report should be written");
    let report = fs::read_to_string(report_path).expect("read report");
    assert_contains(&report, "BASE1_SUPERVISOR_STORAGE_PROFILE=x200-supervisor-lite");
    assert_contains(&report, "BASE1_SUPERVISOR_STORAGE_TARGET_RAM_MB=4096");
    assert_contains(&report, "BASE1_SUPERVISOR_STORAGE_TMPFS_MB=256");
    assert_contains(&report, "BASE1_SUPERVISOR_STORAGE_ZRAM_MB=1024");
    assert_contains(&report, "BASE1_SUPERVISOR_STORAGE_SSD_SCRATCH_MB=2048");
    assert_contains(&report, "BASE1_SUPERVISOR_STORAGE_DISK_IS_RAM_EQUIVALENT=no");
    assert_contains(&report, "BASE1_SUPERVISOR_STORAGE_CLAIM=not_claimed");
}

#[test]
fn base1_supervisor_storage_tier_plan_rejects_unknown_profile_and_non_build_out() {
    let bad_profile = Command::new("sh")
        .arg("scripts/base1-supervisor-storage-tier-plan.sh")
        .arg("--profile")
        .arg("unknown")
        .output()
        .expect("run bad profile");
    assert!(!bad_profile.status.success(), "unknown profile should fail");

    let bad_out = Command::new("sh")
        .arg("scripts/base1-supervisor-storage-tier-plan.sh")
        .arg("--out")
        .arg("/tmp/base1-storage")
        .output()
        .expect("run bad out");
    assert!(!bad_out.status.success(), "non-build out should fail");
}
