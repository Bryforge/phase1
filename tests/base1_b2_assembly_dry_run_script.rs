use std::process::Command;

const SCRIPT: &str = "scripts/base1-b2-assembly-dry-run.sh";

#[test]
fn base1_b2_assembly_dry_run_script_exists_and_has_valid_shell_syntax() {
    let metadata = std::fs::metadata(SCRIPT).expect("B2 dry-run assembly script exists");
    assert!(metadata.len() > 0, "B2 dry-run assembly script should not be empty");

    let output = Command::new("sh")
        .arg("-n")
        .arg(SCRIPT)
        .output()
        .expect("run sh -n on B2 dry-run assembly script");

    assert!(
        output.status.success(),
        "script syntax should pass sh -n\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn base1_b2_assembly_dry_run_requires_dry_run() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--profile")
        .arg("x86_64-vm-validation")
        .output()
        .expect("run B2 dry-run assembly without --dry-run");

    assert!(
        !output.status.success(),
        "B2 dry-run assembly must fail without --dry-run"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--dry-run is required"), "stderr was: {stderr}");
}

#[test]
fn base1_b2_assembly_dry_run_requires_profile() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--dry-run")
        .output()
        .expect("run B2 dry-run assembly without --profile");

    assert!(
        !output.status.success(),
        "B2 dry-run assembly must fail without --profile"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--profile is required"), "stderr was: {stderr}");
}

#[test]
fn base1_b2_assembly_dry_run_rejects_unknown_profile() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--dry-run")
        .arg("--profile")
        .arg("unknown-profile")
        .output()
        .expect("run B2 dry-run assembly with unknown profile");

    assert!(
        !output.status.success(),
        "B2 dry-run assembly must fail for unknown profile"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported profile"), "stderr was: {stderr}");
}

#[test]
fn base1_b2_assembly_dry_run_reports_expected_sections() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--dry-run")
        .arg("--profile")
        .arg("x86_64-vm-validation")
        .output()
        .expect("run B2 dry-run assembly with VM profile");

    assert!(
        output.status.success(),
        "B2 dry-run assembly should succeed with --dry-run and a valid profile\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    for text in [
        "base1_b2_assembly_dry_run: start",
        "status: B2 dry-run assembly preview",
        "boot_readiness_level: B2",
        "writes: no",
        "mutation: no",
        "network: no",
        "profile: x86_64-vm-validation",
        "b1_detection_summary:",
        "profile_assumptions:",
        "image_builder_preview:",
        "boot_handoff_preview:",
        "installer_preview:",
        "recovery_preview:",
        "rollback_preview:",
        "validation_bundle:",
        "known_limitations:",
        "next_validation_step:",
        "base1_b2_assembly_dry_run: complete",
    ] {
        assert!(stdout.contains(text), "missing dry-run output {text}: {stdout}");
    }
}

#[test]
fn base1_b2_assembly_dry_run_accepts_planned_profiles() {
    for profile in [
        "x86_64-uefi-generic",
        "x86_64-bios-generic",
        "x86_64-libreboot-grub",
        "x86_64-vm-validation",
        "x86_64-recovery-usb",
    ] {
        let output = Command::new("sh")
            .arg(SCRIPT)
            .arg("--dry-run")
            .arg("--profile")
            .arg(profile)
            .output()
            .unwrap_or_else(|error| panic!("run B2 dry-run assembly for {profile}: {error}"));

        assert!(
            output.status.success(),
            "B2 dry-run assembly should accept profile {profile}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains(&format!("profile: {profile}")), "stdout was: {stdout}");
    }
}

#[test]
fn base1_b2_assembly_dry_run_source_preserves_non_mutation_scope() {
    let script = std::fs::read_to_string(SCRIPT).expect("B2 dry-run assembly source");

    for required in [
        "B2 boot-readiness slice",
        "--dry-run is required",
        "--profile is required",
        "writes: no",
        "mutation: no",
        "network: no",
        "bootable: not_claimed",
        "installer_ready: not_claimed",
        "hardware_validated: not_claimed",
        "hardened: not_claimed",
        "release_candidate: not_claimed",
    ] {
        assert!(script.contains(required), "missing required source text {required}: {script}");
    }

    for forbidden in [
        "grub-install",
        "mkfs",
        "parted",
        "sfdisk",
        "fdisk ",
        "efibootmgr -c",
        "efibootmgr --create",
        "mount -o remount,rw",
        "apt install",
        "dnf install",
        "pacman -S",
        "curl ",
        "wget ",
    ] {
        assert!(
            !script.contains(forbidden),
            "B2 dry-run assembly should not contain mutating or network command pattern {forbidden}: {script}"
        );
    }
}

#[test]
fn base1_b2_assembly_dry_run_is_documented_from_plan_and_status() {
    let plan = std::fs::read_to_string("docs/os/B2_DRY_RUN_ASSEMBLY_PLAN.md")
        .expect("B2 dry-run assembly plan");
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status");
    let race = std::fs::read_to_string("docs/os/BOOT_READINESS_RACE_PLAN.md")
        .expect("boot readiness race plan");
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("OS roadmap");

    for doc in [&plan, &status, &race, &roadmap] {
        assert!(doc.contains(SCRIPT), "missing script path in doc: {doc}");
        assert!(doc.contains("--dry-run"), "missing dry-run reference in doc: {doc}");
        assert!(doc.contains("x86_64-vm-validation"), "missing VM profile reference in doc: {doc}");
    }
}
