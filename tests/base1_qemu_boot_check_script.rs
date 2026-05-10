use std::{fs, process::Command};

const SCRIPT: &str = "scripts/base1-qemu-boot-check.sh";

fn make_fake_bundle(name: &str) -> String {
    let pid = std::process::id();
    let bundle = format!("build/{name}-{pid}");
    let _ = fs::remove_dir_all(&bundle);
    fs::create_dir_all(format!("{bundle}/staging/boot")).expect("staging boot dir should be creatable");
    fs::create_dir_all(format!("{bundle}/reports")).expect("reports dir should be creatable");
    fs::write(format!("{bundle}/staging/boot/vmlinuz"), "kernel placeholder\n")
        .expect("kernel placeholder should be writable");
    fs::write(format!("{bundle}/staging/boot/initrd.img"), "initrd placeholder\n")
        .expect("initrd placeholder should be writable");
    fs::write(
        format!("{bundle}/run-qemu-bundle.sh"),
        "#!/usr/bin/env sh\nprintf 'phase1 6.0.0 ready\\n'\n",
    )
    .expect("fake qemu scaffold should be writable");
    bundle
}

#[test]
fn base1_qemu_boot_check_exists_and_documents_boundary() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 qemu boot checker should be readable");

    for expected in [
        "Base1 guarded QEMU boot checker",
        "defaults to dry-run",
        "--execute plus the confirmation phrase",
        "launch-qemu-base1-preview",
        "qemu-boot.log",
        "qemu-boot-summary.env",
        "phase1 6.0.0 ready",
        "timeout or gtimeout",
        "does not install Base1",
        "validate real hardware",
        "daily-driver",
    ] {
        assert!(script.contains(expected), "missing qemu boot checker text: {expected}");
    }
}

#[test]
fn base1_qemu_boot_check_refuses_bundle_outside_build() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg("/tmp/base1-qemu-boot-check")
        .output()
        .expect("Base1 qemu boot checker should execute");

    assert!(!output.status.success(), "outside-build bundle should be rejected");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("bundle must be under build/"), "unexpected stderr: {stderr}");
}

#[test]
fn base1_qemu_boot_check_reports_missing_bundle() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg("build/base1-qemu-boot-missing")
        .output()
        .expect("Base1 qemu boot checker should execute");

    assert!(!output.status.success(), "missing bundle should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("bundle directory does not exist"), "unexpected stderr: {stderr}");
}

#[test]
fn base1_qemu_boot_check_dry_run_does_not_launch() {
    let bundle = make_fake_bundle("base1-qemu-boot-dry-run");
    fs::write(std::path::Path::new(&bundle).join("base1-sandbox.raw"), b"").expect("sandbox placeholder should be writable");

    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg(&bundle)
        .arg("--dry-run")
        .output()
        .expect("Base1 qemu boot checker should execute");

    assert!(
        output.status.success(),
        "dry-run should pass\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    for expected in [
        "BASE1 QEMU BOOT CHECK",
        "mode   : dry-run",
        "plan: sh build/",
        "run-qemu-bundle.sh",
        "result: dry-run",
        "no emulator launched",
        "no hardware validation",
    ] {
        assert!(stdout.contains(expected), "missing dry-run output: {expected}\n{stdout}");
    }

    assert!(
        fs::metadata(format!("{bundle}/reports/qemu-boot.log")).is_err(),
        "dry-run should not create a qemu log"
    );

    let _ = fs::remove_dir_all(&bundle);
}

#[test]
fn base1_qemu_boot_check_execute_requires_confirmation() {
    let bundle = make_fake_bundle("base1-qemu-boot-confirmation");
    fs::write(std::path::Path::new(&bundle).join("base1-sandbox.raw"), b"").expect("sandbox placeholder should be writable");

    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg(&bundle)
        .arg("--execute")
        .arg("--timeout")
        .arg("1")
        .output()
        .expect("Base1 qemu boot checker should execute");

    assert!(!output.status.success(), "execute without confirmation should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--execute requires --confirm launch-qemu-base1-preview"),
        "unexpected stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(&bundle);
}
