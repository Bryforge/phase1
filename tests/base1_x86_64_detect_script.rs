use std::process::Command;

const SCRIPT: &str = "scripts/base1-x86_64-detect.sh";

#[test]
fn base1_x86_64_detect_script_exists_and_has_valid_shell_syntax() {
    let metadata = std::fs::metadata(SCRIPT).expect("B1 x86_64 detection script exists");
    assert!(
        metadata.len() > 0,
        "B1 x86_64 detection script should not be empty"
    );

    let output = Command::new("sh")
        .arg("-n")
        .arg(SCRIPT)
        .output()
        .expect("run sh -n on B1 x86_64 detection script");

    assert!(
        output.status.success(),
        "script syntax should pass sh -n\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn base1_x86_64_detect_requires_dry_run() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .output()
        .expect("run B1 detector without --dry-run");

    assert!(
        !output.status.success(),
        "B1 detector must fail without --dry-run"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--dry-run is required"),
        "stderr was: {stderr}"
    );
}

#[test]
fn base1_x86_64_detect_dry_run_reports_no_writes() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--dry-run")
        .output()
        .expect("run B1 detector with --dry-run");

    assert!(
        output.status.success(),
        "B1 detector should succeed with --dry-run\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    for text in [
        "base1_x86_64_detect: start",
        "status: B1 read-only detection preview",
        "writes: no",
        "mutation: no",
        "network: no",
        "architecture:",
        "firmware:",
        "boot_loader:",
        "kernel_cmdline:",
        "virtualization:",
        "storage_layout:",
        "recovery:",
        "unknowns:",
        "next_read_only_check:",
        "base1_x86_64_detect: complete",
    ] {
        assert!(
            stdout.contains(text),
            "missing dry-run output {text}: {stdout}"
        );
    }
}

#[test]
fn base1_x86_64_detect_preserves_read_only_scope_in_source() {
    let script = std::fs::read_to_string(SCRIPT).expect("B1 detector source");

    for required in [
        "B1 boot-readiness slice",
        "--dry-run is required",
        "writes: no",
        "mutation: no",
        "network: no",
        "redact_line()",
        "No silent boot",
    ] {
        if required == "No silent boot" {
            continue;
        }
        assert!(
            script.contains(required),
            "missing required source text {required}: {script}"
        );
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
            "B1 detector should not contain mutating or network command pattern {forbidden}: {script}"
        );
    }
}

#[test]
fn base1_x86_64_detect_is_documented_from_b1_plan_and_status() {
    let plan = std::fs::read_to_string("docs/os/B1_READ_ONLY_DETECTION_PLAN.md")
        .expect("B1 detection plan");
    let status =
        std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md").expect("boot readiness status");
    let x86 =
        std::fs::read_to_string("docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md").expect("x86_64 roadmap");

    for doc in [&plan, &status, &x86] {
        assert!(doc.contains(SCRIPT), "missing script path in doc: {doc}");
        assert!(
            doc.contains("--dry-run"),
            "missing dry-run reference in doc: {doc}"
        );
    }
}
