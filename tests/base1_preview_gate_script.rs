use std::{fs, process::Command};

const GATE: &str = "scripts/base1-preview-gate.sh";
const BUNDLE: &str = "scripts/base1-emulator-preview.sh";

#[test]
fn base1_preview_gate_exists_and_documents_boundary() {
    let script = fs::read_to_string(GATE).expect("Base1 preview gate should be readable");

    for expected in [
        "Base1 preview gate",
        "Default mode is dry-run",
        "explicit confirmation phrase",
        "RUN BASE1 EMULATOR PREVIEW",
        "emulator-only",
        "not an installer",
        "hardware validation",
        "daily-driver readiness proof",
    ] {
        assert!(script.contains(expected), "missing preview gate text: {expected}");
    }
}

#[test]
fn base1_preview_gate_avoids_real_device_tools() {
    let script = fs::read_to_string(GATE).expect("Base1 preview gate should be readable");

    for forbidden in [
        "dd if=",
        "dd of=",
        "\ndd ",
        " mkfs",
        "\nmkfs",
        "parted",
        "sfdisk",
        "wipefs",
        "sgdisk",
        "losetup",
        "qemu-img",
        "\nmount ",
        " mount -",
        "sudo ",
        "apt-get",
        "dnf ",
        "pacman ",
        "brew install",
    ] {
        assert!(
            !script.contains(forbidden),
            "preview gate must not contain real-device tooling pattern: {forbidden}"
        );
    }
}

#[test]
fn base1_preview_gate_reports_missing_bundle() {
    let output = Command::new("sh")
        .arg(GATE)
        .arg("--bundle")
        .arg("build/base1-preview-gate-missing")
        .output()
        .expect("Base1 preview gate should execute");

    assert!(!output.status.success(), "missing bundle should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("bundle directory does not exist"), "unexpected stderr: {stderr}");
}

#[test]
fn base1_preview_gate_dry_run_refuses_missing_boot_inputs() {
    let out = format!("build/base1-preview-gate-missing-boot-{}", std::process::id());
    let _ = fs::remove_dir_all(&out);

    let build = Command::new("sh")
        .arg(BUNDLE)
        .arg("--out")
        .arg(&out)
        .arg("--image-mb")
        .arg("1")
        .output()
        .expect("Base1 emulator bundle script should execute");

    assert!(
        build.status.success(),
        "bundle generator failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let gate = Command::new("sh")
        .arg(GATE)
        .arg("--bundle")
        .arg(&out)
        .arg("--dry-run")
        .output()
        .expect("Base1 preview gate should execute");

    assert!(!gate.status.success(), "missing kernel/initrd should fail gate");
    let stderr = String::from_utf8_lossy(&gate.stderr);
    assert!(stderr.contains("kernel missing"), "unexpected stderr: {stderr}");

    let _ = fs::remove_dir_all(&out);
}

#[test]
fn base1_preview_gate_dry_run_passes_when_boot_inputs_exist() {
    let out = format!("build/base1-preview-gate-ready-{}", std::process::id());
    let kernel = format!("build/base1-preview-gate-kernel-{}", std::process::id());
    let initrd = format!("build/base1-preview-gate-initrd-{}", std::process::id());
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all("build").expect("build directory should be creatable");
    fs::write(&kernel, "kernel placeholder").expect("kernel placeholder should be writable");
    fs::write(&initrd, "initrd placeholder").expect("initrd placeholder should be writable");

    let build = Command::new("sh")
        .arg(BUNDLE)
        .arg("--out")
        .arg(&out)
        .arg("--image-mb")
        .arg("1")
        .arg("--kernel")
        .arg(&kernel)
        .arg("--initrd")
        .arg(&initrd)
        .output()
        .expect("Base1 emulator bundle script should execute");

    assert!(
        build.status.success(),
        "bundle generator failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let gate = Command::new("sh")
        .arg(GATE)
        .arg("--bundle")
        .arg(&out)
        .arg("--dry-run")
        .output()
        .expect("Base1 preview gate should execute");

    assert!(
        gate.status.success(),
        "gate failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&gate.stdout),
        String::from_utf8_lossy(&gate.stderr)
    );

    let stdout = String::from_utf8_lossy(&gate.stdout);
    for expected in [
        "mode: dry-run",
        "running read-only doctor",
        "doctor passed and boot inputs are present",
        "handoff: sh",
        "dry-run complete; QEMU was not started",
        "non-claim: emulator preview was not executed",
    ] {
        assert!(stdout.contains(expected), "gate output missing: {expected}\n{stdout}");
    }

    let _ = fs::remove_dir_all(&out);
    let _ = fs::remove_file(&kernel);
    let _ = fs::remove_file(&initrd);
}

#[test]
fn base1_preview_gate_execute_requires_confirmation() {
    let out = format!("build/base1-preview-gate-confirm-{}", std::process::id());
    let kernel = format!("build/base1-preview-gate-confirm-kernel-{}", std::process::id());
    let initrd = format!("build/base1-preview-gate-confirm-initrd-{}", std::process::id());
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all("build").expect("build directory should be creatable");
    fs::write(&kernel, "kernel placeholder").expect("kernel placeholder should be writable");
    fs::write(&initrd, "initrd placeholder").expect("initrd placeholder should be writable");

    let build = Command::new("sh")
        .arg(BUNDLE)
        .arg("--out")
        .arg(&out)
        .arg("--image-mb")
        .arg("1")
        .arg("--kernel")
        .arg(&kernel)
        .arg("--initrd")
        .arg(&initrd)
        .output()
        .expect("Base1 emulator bundle script should execute");

    assert!(build.status.success(), "bundle generator should pass");

    let gate = Command::new("sh")
        .arg(GATE)
        .arg("--bundle")
        .arg(&out)
        .arg("--execute")
        .output()
        .expect("Base1 preview gate should execute");

    assert!(!gate.status.success(), "execute without confirmation should fail");
    let stderr = String::from_utf8_lossy(&gate.stderr);
    assert!(stderr.contains("--execute requires --confirm"), "unexpected stderr: {stderr}");

    let _ = fs::remove_dir_all(&out);
    let _ = fs::remove_file(&kernel);
    let _ = fs::remove_file(&initrd);
}
