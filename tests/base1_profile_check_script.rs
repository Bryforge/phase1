use std::process::Command;

#[test]
fn base1_profile_check_script_exists_and_has_valid_shell_syntax() {
    let script = "scripts/base1-profile-check.sh";
    let contents = std::fs::read_to_string(script).expect("profile checker script");

    assert!(contents.contains("Base1 profile checker"), "{contents}");
    assert!(contents.contains("set -eu"), "{contents}");
    assert!(contents.contains("x200-supervisor-lite"), "{contents}");
    assert!(contents.contains("x86_64-vm-validation"), "{contents}");
    assert!(contents.contains("workstation-supervisor"), "{contents}");
    assert!(contents.contains("BASE1_PROFILE_NON_CLAIM_HYPERVISOR"), "{contents}");

    let status = Command::new("sh")
        .arg("-n")
        .arg(script)
        .status()
        .expect("run sh -n");
    assert!(status.success(), "profile checker should have valid shell syntax");
}

#[test]
fn base1_profile_check_help_documents_contract_and_non_claims() {
    let output = Command::new("sh")
        .arg("scripts/base1-profile-check.sh")
        .arg("--help")
        .output()
        .expect("run profile checker help");

    assert!(output.status.success(), "help should pass");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "base1 profile checker",
        "--profile <name>",
        "--all",
        "--profile-dir <dir>",
        "x200-supervisor-lite",
        "x86_64-vm-validation",
        "workstation-supervisor",
        "required fields:",
        "BASE1_PROFILE_NAME",
        "BASE1_PROFILE_DEFAULT_DELIVERY_MODE",
        "BASE1_PROFILE_ALLOWED_DELIVERY_MODES",
        "BASE1_PROFILE_STORAGE_TIER_POLICY",
        "BASE1_PROFILE_NON_CLAIM_HYPERVISOR",
        "profile shape only",
        "does not make Base1 bootable",
        "hypervisor-ready",
        "daily-driver ready",
    ] {
        assert!(stdout.contains(text), "missing help text {text}: {stdout}");
    }
}

#[test]
fn base1_profile_check_all_profiles_pass_and_write_report() {
    let out_dir = "build/test-base1-profile-check";
    let _ = std::fs::remove_dir_all(out_dir);

    let output = Command::new("sh")
        .arg("scripts/base1-profile-check.sh")
        .arg("--all")
        .arg("--write-report")
        .arg("--out")
        .arg(out_dir)
        .output()
        .expect("run profile checker all");

    assert!(output.status.success(), "all profiles should pass");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "BASE1 PROFILE CHECK",
        "profile: x200-supervisor-lite",
        "profile: x86_64-vm-validation",
        "profile: workstation-supervisor",
        "profile_result: pass",
        "checked_profiles: 3",
        "failed_profiles: 0",
        "result: pass",
        "written_report:",
        "no boot-ready claim",
        "no hypervisor claim",
    ] {
        assert!(stdout.contains(text), "missing stdout text {text}: {stdout}");
    }

    let report = std::fs::read_to_string(format!("{out_dir}/profile-check.env"))
        .expect("profile check report");

    for text in [
        "BASE1_PROFILE_CHECK_MODE=all",
        "BASE1_PROFILE_CHECK_RESULT=pass",
        "BASE1_PROFILE_CHECK_COUNT=3",
        "BASE1_PROFILE_CHECK_FAILED=0",
        "BASE1_PROFILE_CHECK_CLAIM=not_claimed",
        "BASE1_PROFILE_CHECK_NON_CLAIM_BOOTABLE=1",
        "BASE1_PROFILE_CHECK_NON_CLAIM_INSTALLER=1",
        "BASE1_PROFILE_CHECK_NON_CLAIM_HARDENED=1",
        "BASE1_PROFILE_CHECK_NON_CLAIM_HYPERVISOR=1",
        "BASE1_PROFILE_CHECK_NON_CLAIM_HARDWARE=1",
        "BASE1_PROFILE_CHECK_NON_CLAIM_DAILY_DRIVER=1",
    ] {
        assert!(report.contains(text), "missing report text {text}: {report}");
    }
}

#[test]
fn base1_profile_check_single_x200_profile_passes() {
    let output = Command::new("sh")
        .arg("scripts/base1-profile-check.sh")
        .arg("--profile")
        .arg("x200-supervisor-lite")
        .output()
        .expect("run x200 profile check");

    assert!(output.status.success(), "x200 profile should pass");
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");

    for text in [
        "profile: x200-supervisor-lite",
        "file   : profiles/base1/x200-supervisor-lite.env",
        "profile_result: pass",
        "checked_profiles: 1",
        "result: pass",
    ] {
        assert!(stdout.contains(text), "missing x200 check text {text}: {stdout}");
    }
}

#[test]
fn base1_profile_files_preserve_expected_claim_boundaries_and_resource_profiles() {
    for (path, expected_name, expected_mode) in [
        ("profiles/base1/x200-supervisor-lite.env", "BASE1_PROFILE_NAME=x200-supervisor-lite", "BASE1_PROFILE_DEFAULT_DELIVERY_MODE=direct-first"),
        ("profiles/base1/x86_64-vm-validation.env", "BASE1_PROFILE_NAME=x86_64-vm-validation", "BASE1_PROFILE_DEFAULT_DELIVERY_MODE=supervisor-lite"),
        ("profiles/base1/workstation-supervisor.env", "BASE1_PROFILE_NAME=workstation-supervisor", "BASE1_PROFILE_DEFAULT_DELIVERY_MODE=workstation-supervisor"),
    ] {
        let profile = std::fs::read_to_string(path).expect("profile file");

        for text in [
            expected_name,
            expected_mode,
            "BASE1_PROFILE_SECURITY_POSTURE=hardened-requested-evidence-bound",
            "BASE1_PROFILE_CLAIM=not_claimed",
            "BASE1_PROFILE_NON_CLAIM_BOOTABLE=1",
            "BASE1_PROFILE_NON_CLAIM_INSTALLER=1",
            "BASE1_PROFILE_NON_CLAIM_HARDENED=1",
            "BASE1_PROFILE_NON_CLAIM_HYPERVISOR=1",
            "BASE1_PROFILE_NON_CLAIM_HARDWARE=1",
            "BASE1_PROFILE_NON_CLAIM_DAILY_DRIVER=1",
        ] {
            assert!(profile.contains(text), "missing profile text {text} in {path}: {profile}");
        }
    }
}

#[test]
fn base1_profile_check_rejects_unknown_profile_and_non_build_out() {
    let bad_profile = Command::new("sh")
        .arg("scripts/base1-profile-check.sh")
        .arg("--profile")
        .arg("unknown-profile")
        .output()
        .expect("run bad profile");
    assert!(!bad_profile.status.success(), "unknown profile should fail");

    let bad_out = Command::new("sh")
        .arg("scripts/base1-profile-check.sh")
        .arg("--out")
        .arg("/tmp/base1-profile-check")
        .output()
        .expect("run bad out");
    assert!(!bad_out.status.success(), "non-build output should fail");
}
