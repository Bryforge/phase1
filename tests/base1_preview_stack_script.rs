use std::{fs, process::Command};

const SCRIPT: &str = "scripts/base1-preview-stack.sh";

#[test]
fn base1_preview_stack_exists_and_documents_boundary() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 preview stack should be readable");

    for expected in [
        "Base1 preview stack",
        "inputs -> bundle -> doctor -> gate dry-run",
        "does not launch QEMU",
        "does not claim Base1 is bootable",
        "emulator-prep only",
        "It does not claim Base1 is",
        "safe flow",
    ] {
        assert!(script.contains(expected), "missing preview stack text: {expected}");
    }
}

#[test]
fn base1_preview_stack_avoids_real_device_tools() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 preview stack should be readable");

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
            "preview stack must not contain mutating tooling pattern: {forbidden}"
        );
    }
}

#[test]
fn base1_preview_stack_requires_kernel_and_initrd() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg("build/base1-preview-stack-missing")
        .arg("--no-qemu-check")
        .output()
        .expect("Base1 preview stack should execute");

    assert!(!output.status.success(), "missing kernel/initrd should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--kernel is required"), "unexpected stderr: {stderr}");
}

#[test]
fn base1_preview_stack_runs_full_safe_flow_with_placeholder_inputs() {
    let pid = std::process::id();
    let out = format!("build/base1-preview-stack-test-{pid}");
    let kernel = format!("build/base1-preview-stack-kernel-{pid}");
    let initrd = format!("build/base1-preview-stack-initrd-{pid}");

    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all("build").expect("build directory should be creatable");
    fs::write(&kernel, "kernel placeholder").expect("kernel placeholder should be writable");
    fs::write(&initrd, "initrd placeholder").expect("initrd placeholder should be writable");

    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg(&out)
        .arg("--kernel")
        .arg(&kernel)
        .arg("--initrd")
        .arg(&initrd)
        .arg("--image-mb")
        .arg("1")
        .arg("--no-qemu-check")
        .output()
        .expect("Base1 preview stack should execute");

    assert!(
        output.status.success(),
        "preview stack failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    for expected in [
        "step 1/4: checking preview inputs",
        "step 2/4: generating emulator preview bundle",
        "step 3/4: running read-only bundle doctor",
        "step 4/4: running guarded dry-run gate",
        "dry-run complete; QEMU was not started",
        "complete: safe preview stack passed",
        "non-claim: no emulator launched",
    ] {
        assert!(stdout.contains(expected), "missing output: {expected}\n{stdout}");
    }

    for path in [
        "manifest.env",
        "staging/manifest.env",
        "staging/boot/vmlinuz",
        "staging/boot/initrd.img",
        "run-qemu-bundle.sh",
        "base1-sandbox.raw",
    ] {
        assert!(fs::metadata(format!("{out}/{path}")).is_ok(), "missing generated path: {path}");
    }

    let _ = fs::remove_dir_all(&out);
    let _ = fs::remove_file(&kernel);
    let _ = fs::remove_file(&initrd);
}
