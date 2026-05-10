use std::{fs, process::Command};

const SCRIPT: &str = "scripts/base1-preview-inputs.sh";

#[test]
fn base1_preview_inputs_script_exists_and_documents_boundary() {
    let script =
        fs::read_to_string(SCRIPT).expect("Base1 preview inputs script should be readable");

    for expected in [
        "Base1 preview inputs checker",
        "Read-only checker",
        "candidate kernel path",
        "candidate initrd path",
        "next safe commands",
        "does not start the emulator",
        "does not claim Base1 is",
        "bootable",
    ] {
        assert!(
            script.contains(expected),
            "missing preview input text: {expected}"
        );
    }
}

#[test]
fn base1_preview_inputs_avoids_mutating_tools() {
    let script =
        fs::read_to_string(SCRIPT).expect("Base1 preview inputs script should be readable");

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
            "preview inputs script must not contain mutating tooling pattern: {forbidden}"
        );
    }
}

#[test]
fn base1_preview_inputs_reports_missing_required_paths() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg("build/base1-preview-inputs-missing")
        .arg("--no-qemu-check")
        .output()
        .expect("Base1 preview inputs script should execute");

    assert!(
        !output.status.success(),
        "missing kernel/initrd should fail"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);

    for expected in [
        "BASE1 PREVIEW INPUTS",
        "PASS  bundle path is under build/",
        "FAIL  kernel path not provided",
        "FAIL  initrd path not provided",
        "WARN  qemu executable check skipped",
        "result: failed",
        "no emulator started",
    ] {
        assert!(
            stdout.contains(expected),
            "missing output: {expected}\n{stdout}"
        );
    }
}

#[test]
fn base1_preview_inputs_passes_with_placeholder_files() {
    let kernel = format!("build/base1-preview-inputs-kernel-{}", std::process::id());
    let initrd = format!("build/base1-preview-inputs-initrd-{}", std::process::id());
    fs::create_dir_all("build").expect("build directory should be creatable");
    fs::write(&kernel, "kernel placeholder").expect("kernel placeholder should be writable");
    fs::write(&initrd, "initrd placeholder").expect("initrd placeholder should be writable");

    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg("build/base1-emulator-preview")
        .arg("--kernel")
        .arg(&kernel)
        .arg("--initrd")
        .arg(&initrd)
        .arg("--no-qemu-check")
        .output()
        .expect("Base1 preview inputs script should execute");

    assert!(
        output.status.success(),
        "preview inputs checker failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    for expected in [
        "BASE1 PREVIEW INPUTS",
        "PASS  bundle path is under build/",
        "PASS  kernel file exists",
        "PASS  initrd file exists",
        "WARN  qemu executable check skipped",
        "scripts/base1-emulator-preview.sh --out build/base1-emulator-preview",
        "scripts/base1-emulator-doctor.sh --bundle build/base1-emulator-preview",
        "scripts/base1-preview-gate.sh --bundle build/base1-emulator-preview --dry-run",
        "result: pass-with-notes",
        "no emulator started",
        "no image created",
    ] {
        assert!(
            stdout.contains(expected),
            "missing output: {expected}\n{stdout}"
        );
    }

    let _ = fs::remove_file(&kernel);
    let _ = fs::remove_file(&initrd);
}

#[test]
fn base1_preview_inputs_warns_when_bundle_outside_build() {
    let kernel = format!(
        "build/base1-preview-inputs-kernel-outside-{}",
        std::process::id()
    );
    let initrd = format!(
        "build/base1-preview-inputs-initrd-outside-{}",
        std::process::id()
    );
    fs::create_dir_all("build").expect("build directory should be creatable");
    fs::write(&kernel, "kernel placeholder").expect("kernel placeholder should be writable");
    fs::write(&initrd, "initrd placeholder").expect("initrd placeholder should be writable");

    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg("/tmp/base1-emulator-preview")
        .arg("--kernel")
        .arg(&kernel)
        .arg("--initrd")
        .arg(&initrd)
        .arg("--no-qemu-check")
        .output()
        .expect("Base1 preview inputs script should execute");

    assert!(
        output.status.success(),
        "outside-build warning should not fail valid inputs"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("WARN  bundle path is outside build/"),
        "missing outside-build warning: {stdout}"
    );
    assert!(
        stdout.contains("result: pass-with-notes"),
        "missing pass-with-notes result: {stdout}"
    );

    let _ = fs::remove_file(&kernel);
    let _ = fs::remove_file(&initrd);
}
