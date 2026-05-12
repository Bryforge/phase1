use std::process::Command;

#[test]
fn base1_supervisor_orchestration_plan_script_exists_and_has_valid_shell_syntax() {
    let script = "scripts/base1-supervisor-orchestration-plan.sh";
    let contents =
        std::fs::read_to_string(script).expect("supervisor orchestration planner script");

    for text in [
        "Base1 supervisor orchestration planner",
        "profiles/base1",
        "BASE1_PROFILE_ALLOWED_DELIVERY_MODES",
        "direct-first",
        "supervisor-lite",
        "supervisor-concurrent",
        "BASE1_SUPERVISOR_CONTROL_PLANE=planned",
        "BASE1_SUPERVISOR_POLICY_BUS=planned",
        "BASE1_SUPERVISOR_EVIDENCE_BUS=planned",
        "BASE1_SUPERVISOR_RECOVERY_HOOKS=planned",
        "BASE1_SUPERVISOR_NON_CLAIM_HYPERVISOR=1",
        "does not boot kernels",
        "launch QEMU",
        "install Base1",
        "mutate disks",
        "modify host boot settings",
        "prove hardening",
        "claim hypervisor readiness",
        "validate hardware",
    ] {
        assert!(
            contents.contains(text),
            "missing script text {text}: {contents}"
        );
    }

    let status = Command::new("sh")
        .arg("-n")
        .arg(script)
        .status()
        .expect("run sh -n");

    assert!(status.success(), "script should have valid shell syntax");
}

#[test]
fn base1_supervisor_orchestration_plan_defaults_to_x200_safe_shape() {
    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-orchestration-plan.sh")
        .arg("--dry-run")
        .output()
        .expect("run supervisor orchestration dry-run");

    assert!(output.status.success(), "dry-run should pass");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "BASE1 SUPERVISOR ORCHESTRATION PLAN",
        "profile       : x200-supervisor-lite",
        "profile_class : low-resource",
        "active_kernels: 1",
        "supervisor_lite: preferred for X200-class systems",
        "supervisor_concurrent: profile-gated",
        "BASE1_SUPERVISOR_ACTIVE_KERNELS=1",
        "BASE1_SUPERVISOR_CLAIM=not_claimed",
        "no hypervisor claim",
        "no hardware validation",
    ] {
        assert!(
            stdout.contains(text),
            "missing dry-run text {text}: {stdout}"
        );
    }
}

#[test]
fn base1_supervisor_orchestration_plan_prepare_writes_report() {
    let out_dir = "build/test-base1-supervisor-orchestration";
    let _ = std::fs::remove_dir_all(out_dir);

    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-orchestration-plan.sh")
        .arg("--prepare")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .arg("--out")
        .arg(out_dir)
        .output()
        .expect("run supervisor orchestration prepare");

    assert!(output.status.success(), "prepare should pass");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.contains("result: prepared"), "{stdout}");

    let report = std::fs::read_to_string(format!("{out_dir}/supervisor-orchestration-plan.env"))
        .expect("supervisor orchestration report");

    for text in [
        "BASE1_SUPERVISOR_PLAN_MODE=prepare",
        "BASE1_SUPERVISOR_PROFILE=x200-supervisor-lite",
        "BASE1_SUPERVISOR_PROFILE_CLASS=low-resource",
        "BASE1_SUPERVISOR_DEFAULT_DELIVERY_MODE=direct-first",
        "BASE1_SUPERVISOR_ACTIVE_KERNELS=1",
        "BASE1_SUPERVISOR_STORAGE_TIER_POLICY=zram-plus-ssd-scratch-swap-backstop",
        "BASE1_SUPERVISOR_CONTROL_PLANE=planned",
        "BASE1_SUPERVISOR_POLICY_BUS=planned",
        "BASE1_SUPERVISOR_EVIDENCE_BUS=planned",
        "BASE1_SUPERVISOR_RECOVERY_HOOKS=planned",
        "BASE1_SUPERVISOR_CLAIM=not_claimed",
        "BASE1_SUPERVISOR_NON_CLAIM_HARDENED=1",
        "BASE1_SUPERVISOR_NON_CLAIM_HYPERVISOR=1",
        "BASE1_SUPERVISOR_NON_CLAIM_HARDWARE=1",
    ] {
        assert!(
            report.contains(text),
            "missing report text {text}: {report}"
        );
    }
}

#[test]
fn base1_supervisor_orchestration_plan_supports_vm_concurrent_profile_shape() {
    let output = Command::new("sh")
        .arg("scripts/base1-supervisor-orchestration-plan.sh")
        .arg("--dry-run")
        .arg("--profile")
        .arg("x86_64-vm-validation")
        .output()
        .expect("run vm validation supervisor plan");

    assert!(output.status.success(), "vm validation dry-run should pass");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "profile       : x86_64-vm-validation",
        "profile_class : vm-validation",
        "allowed_modes : direct-first,supervisor-lite,supervisor-concurrent",
        "BASE1_SUPERVISOR_MAX_CONCURRENCY=3",
        "BASE1_SUPERVISOR_POLICY_BUS=planned",
        "BASE1_SUPERVISOR_EVIDENCE_BUS=planned",
    ] {
        assert!(
            stdout.contains(text),
            "missing vm profile text {text}: {stdout}"
        );
    }
}

#[test]
fn base1_supervisor_orchestration_plan_rejects_unknown_profile_and_non_build_out() {
    let bad_profile = Command::new("sh")
        .arg("scripts/base1-supervisor-orchestration-plan.sh")
        .arg("--profile")
        .arg("unknown")
        .output()
        .expect("run bad profile");
    assert!(!bad_profile.status.success(), "unknown profile should fail");

    let bad_out = Command::new("sh")
        .arg("scripts/base1-supervisor-orchestration-plan.sh")
        .arg("--out")
        .arg("/tmp/base1-supervisor")
        .output()
        .expect("run bad out");
    assert!(!bad_out.status.success(), "non-build out should fail");
}
