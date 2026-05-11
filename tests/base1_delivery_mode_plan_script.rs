use std::process::Command;

#[test]
fn base1_delivery_mode_plan_script_exists_and_has_valid_shell_syntax() {
    let script = "scripts/base1-delivery-mode-plan.sh";
    let contents = std::fs::read_to_string(script).expect("delivery mode planner script");

    assert!(contents.contains("Base1 dual-path delivery mode planner"), "{contents}");
    assert!(contents.contains("set -eu"), "{contents}");
    assert!(contents.contains("require_build_out_dir"), "{contents}");
    assert!(contents.contains("PROFILE_FILE=\"$PROFILE_DIR/$PROFILE.env\""), "{contents}");
    assert!(contents.contains(". \"$PROFILE_FILE\""), "{contents}");
    assert!(contents.contains("BASE1_PROFILE_ALLOWED_DELIVERY_MODES"), "{contents}");
    assert!(contents.contains("direct-first"), "{contents}");
    assert!(contents.contains("supervisor-lite"), "{contents}");
    assert!(contents.contains("supervisor-concurrent"), "{contents}");
    assert!(contents.contains("workstation-supervisor"), "{contents}");

    let status = Command::new("sh")
        .arg("-n")
        .arg(script)
        .status()
        .expect("run sh -n on delivery mode planner");

    assert!(status.success(), "delivery mode planner should have valid shell syntax");
}

#[test]
fn base1_delivery_mode_plan_help_documents_modes_profiles_contract_and_non_claims() {
    let output = Command::new("sh")
        .arg("scripts/base1-delivery-mode-plan.sh")
        .arg("--help")
        .output()
        .expect("run delivery mode help");

    assert!(output.status.success(), "help should exit successfully");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "base1 dual-path delivery mode planner",
        "--dry-run",
        "--prepare",
        "--mode <name>",
        "--profile <name>",
        "--profile-dir <dir>",
        "direct-first",
        "supervisor-lite",
        "supervisor-concurrent",
        "workstation-supervisor",
        "fastest first-kernel/single-kernel route",
        "one active staged kernel plus Base1 control plane",
        "multiple staged kernels under orchestration",
        "profile source:",
        "profiles/base1/x200-supervisor-lite.env",
        "profiles/base1/x86_64-vm-validation.env",
        "profiles/base1/workstation-supervisor.env",
        "shared contract",
        "profile names",
        "policy vocabulary",
        "boot artifact IDs",
        "log paths",
        "storage-tier assumptions",
        "evidence states",
        "non-claims",
        "does not make Base1 bootable",
        "hypervisor-ready",
        "daily-driver ready",
    ] {
        assert!(stdout.contains(text), "missing help text {text}: {stdout}");
    }
}

#[test]
fn base1_delivery_mode_plan_dry_run_defaults_to_profile_default_x200_direct_first() {
    let output = Command::new("sh")
        .arg("scripts/base1-delivery-mode-plan.sh")
        .arg("--dry-run")
        .output()
        .expect("run delivery mode default dry-run");

    assert!(output.status.success(), "default dry-run should pass");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "BASE1 DUAL-PATH DELIVERY MODE PLAN",
        "delivery_mode    : direct-first",
        "mode_family      : direct",
        "profile          : x200-supervisor-lite",
        "profile_file     : profiles/base1/x200-supervisor-lite.env",
        "profile_class    : low-resource",
        "4GB-class low-resource target",
        "zram-plus-ssd-scratch-swap-backstop",
        "default_concur   : 1",
        "path_direct: enabled",
        "path_supervisor: enabled",
        "selected_plan: keep the shortest boot route",
        "no boot-ready claim",
        "no hypervisor claim",
    ] {
        assert!(stdout.contains(text), "missing default dry-run text {text}: {stdout}");
    }
}

#[test]
fn base1_delivery_mode_plan_prepare_writes_report_from_profile_contract() {
    let out_dir = "build/test-base1-delivery-mode-plan";
    let _ = std::fs::remove_dir_all(out_dir);

    let output = Command::new("sh")
        .arg("scripts/base1-delivery-mode-plan.sh")
        .arg("--prepare")
        .arg("--mode")
        .arg("supervisor-lite")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .arg("--out")
        .arg(out_dir)
        .output()
        .expect("run delivery mode prepare");

    assert!(output.status.success(), "prepare should pass");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.contains("result: prepared"), "{stdout}");
    assert!(stdout.contains("written_report:"), "{stdout}");

    let report = std::fs::read_to_string(format!("{out_dir}/delivery-mode-plan.env"))
        .expect("delivery mode report");

    for text in [
        "BASE1_DELIVERY_MODE_PLANNER_MODE=prepare",
        "BASE1_DELIVERY_MODE=supervisor-lite",
        "BASE1_DELIVERY_MODE_FAMILY=supervisor",
        "BASE1_DELIVERY_PROFILE=x200-supervisor-lite",
        "BASE1_DELIVERY_PROFILE_FILE=profiles/base1/x200-supervisor-lite.env",
        "BASE1_DELIVERY_PROFILE_CLASS=low-resource",
        "BASE1_DELIVERY_PROFILE_TARGET_RAM_MB=4096",
        "BASE1_DELIVERY_PROFILE_DEFAULT_MODE=direct-first",
        "BASE1_DELIVERY_PROFILE_ALLOWED_MODES=direct-first,supervisor-lite",
        "BASE1_DELIVERY_DEFAULT_CONCURRENCY=1",
        "BASE1_DELIVERY_PROFILE_MAX_CONCURRENCY=1",
        "BASE1_DELIVERY_PROFILE_ZRAM_MB=1024",
        "BASE1_DELIVERY_PROFILE_SWAP_MB=1024",
        "BASE1_DELIVERY_PROFILE_SSD_SCRATCH_MB=2048",
        "BASE1_DELIVERY_DIRECT_PATH=enabled",
        "BASE1_DELIVERY_SUPERVISOR_PATH=enabled",
        "BASE1_DELIVERY_SHARED_CONTRACT=profiles,policy,artifacts,logs,evidence,storage,non_claims",
        "BASE1_DELIVERY_CLAIM=not_claimed",
        "BASE1_DELIVERY_NON_CLAIM_BOOTABLE=1",
        "BASE1_DELIVERY_NON_CLAIM_INSTALLER=1",
        "BASE1_DELIVERY_NON_CLAIM_RECOVERY=1",
        "BASE1_DELIVERY_NON_CLAIM_HARDENED=1",
        "BASE1_DELIVERY_NON_CLAIM_HYPERVISOR=1",
        "BASE1_DELIVERY_NON_CLAIM_HARDWARE=1",
        "BASE1_DELIVERY_NON_CLAIM_RELEASE_CANDIDATE=1",
        "BASE1_DELIVERY_NON_CLAIM_DAILY_DRIVER=1",
    ] {
        assert!(report.contains(text), "missing report text {text}: {report}");
    }
}

#[test]
fn base1_delivery_mode_plan_supports_concurrent_when_profile_allows_it() {
    let output = Command::new("sh")
        .arg("scripts/base1-delivery-mode-plan.sh")
        .arg("--dry-run")
        .arg("--mode")
        .arg("supervisor-concurrent")
        .arg("--profile")
        .arg("x86_64-vm-validation")
        .output()
        .expect("run delivery mode concurrent dry-run");

    assert!(output.status.success(), "concurrent dry-run should pass");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "delivery_mode    : supervisor-concurrent",
        "mode_family      : supervisor",
        "profile_file     : profiles/base1/x86_64-vm-validation.env",
        "profile_class    : vm-validation",
        "multiple staged kernels under Base1 orchestration",
        "default_concur   : 3",
        "VM evidence only until reviewed",
        "no hypervisor claim",
        "no hardware validation",
    ] {
        assert!(stdout.contains(text), "missing concurrent dry-run text {text}: {stdout}");
    }
}

#[test]
fn base1_delivery_mode_plan_rejects_mode_disallowed_by_profile() {
    let output = Command::new("sh")
        .arg("scripts/base1-delivery-mode-plan.sh")
        .arg("--dry-run")
        .arg("--mode")
        .arg("supervisor-concurrent")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .output()
        .expect("run disallowed mode");

    assert!(!output.status.success(), "x200 profile should reject supervisor-concurrent");
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("delivery mode supervisor-concurrent is not allowed by profile x200-supervisor-lite"), "{stderr}");
}

#[test]
fn base1_delivery_mode_plan_rejects_unknown_args_profiles_and_non_build_out() {
    let bad_arg = Command::new("sh")
        .arg("scripts/base1-delivery-mode-plan.sh")
        .arg("--wat")
        .output()
        .expect("run bad arg");
    assert!(!bad_arg.status.success(), "unknown args should fail");

    let bad_mode = Command::new("sh")
        .arg("scripts/base1-delivery-mode-plan.sh")
        .arg("--mode")
        .arg("magic")
        .arg("--profile")
        .arg("workstation-supervisor")
        .output()
        .expect("run bad mode");
    assert!(!bad_mode.status.success(), "unknown mode should fail");

    let bad_profile = Command::new("sh")
        .arg("scripts/base1-delivery-mode-plan.sh")
        .arg("--profile")
        .arg("unknown-box")
        .output()
        .expect("run bad profile");
    assert!(!bad_profile.status.success(), "unknown profile should fail");

    let bad_out = Command::new("sh")
        .arg("scripts/base1-delivery-mode-plan.sh")
        .arg("--out")
        .arg("/tmp/base1-delivery-mode")
        .output()
        .expect("run bad out");
    assert!(!bad_out.status.success(), "non-build out should fail");
}

#[test]
fn base1_delivery_mode_plan_preserves_boundaries_and_best_of_both_worlds() {
    let contents = std::fs::read_to_string("scripts/base1-delivery-mode-plan.sh")
        .expect("delivery mode planner script");

    for text in [
        "direct first-kernel delivery",
        "supervisor orchestration",
        "without fragmenting Base1",
        "profiles/base1/*.env",
        "does not boot kernels",
        "launch QEMU",
        "install Base1",
        "mutate disks",
        "modify host boot settings",
        "validate hardware",
        "prove hardening",
        "claim daily-",
        "BASE1_DELIVERY_DIRECT_PATH=enabled",
        "BASE1_DELIVERY_SUPERVISOR_PATH=enabled",
        "BASE1_DELIVERY_CLAIM=not_claimed",
        "BASE1_DELIVERY_NON_CLAIM_HYPERVISOR=1",
    ] {
        assert!(contents.contains(text), "missing boundary/best-of-both-worlds text {text}: {contents}");
    }
}
