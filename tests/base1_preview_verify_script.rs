use std::{fs, process::Command};

const SCRIPT: &str = "scripts/base1-preview-verify.sh";
const STACK: &str = "scripts/base1-preview-stack.sh";

#[test]
fn base1_preview_verify_exists_and_documents_boundary() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 preview verifier should be readable");

    for expected in [
        "Base1 preview provenance verifier",
        "Read-only verifier",
        "reports/SHA256SUMS",
        "each listed file hash matches",
        "does not claim Base1 is bootable",
        "no bootable Base1 release claim",
        "result: %s",
    ] {
        assert!(
            script.contains(expected),
            "missing verifier text: {expected}"
        );
    }
}

#[test]
fn base1_preview_verify_avoids_real_device_tools() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 preview verifier should be readable");

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
            "verifier must not contain real-device tooling pattern: {forbidden}"
        );
    }
}

#[test]
fn base1_preview_verify_refuses_bundle_outside_build() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg("/tmp/base1-preview-verify")
        .output()
        .expect("Base1 preview verifier should execute");

    assert!(
        !output.status.success(),
        "outside-build bundle should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("bundle must be under build/"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn base1_preview_verify_reports_missing_bundle() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg("build/base1-preview-verify-missing-bundle")
        .output()
        .expect("Base1 preview verifier should execute");

    assert!(!output.status.success(), "missing bundle should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("bundle directory does not exist"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn base1_preview_verify_passes_generated_stack_bundle() {
    let pid = std::process::id();
    let out = format!("build/base1-preview-verify-pass-{pid}");
    let kernel = format!("build/base1-preview-verify-kernel-{pid}");
    let initrd = format!("build/base1-preview-verify-initrd-{pid}");

    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all("build").expect("build directory should be creatable");
    fs::write(&kernel, "kernel placeholder").expect("kernel placeholder should be writable");
    fs::write(&initrd, "initrd placeholder").expect("initrd placeholder should be writable");

    let stack = Command::new("sh")
        .arg(STACK)
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
        stack.status.success(),
        "stack failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&stack.stdout),
        String::from_utf8_lossy(&stack.stderr)
    );

    let verify = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg(&out)
        .output()
        .expect("Base1 preview verifier should execute");

    assert!(
        verify.status.success(),
        "verify failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&verify.stdout),
        String::from_utf8_lossy(&verify.stderr)
    );

    let stdout = String::from_utf8_lossy(&verify.stdout);
    for expected in [
        "BASE1 PREVIEW VERIFY",
        "mode   : read-only",
        "PASS  sha256 ok: manifest.env",
        "PASS  sha256 ok: base1-sandbox.raw",
        "PASS  sha256 ok: run-qemu-bundle.sh",
        "PASS  sha256 ok: staging/boot/vmlinuz",
        "PASS  sha256 ok: staging/boot/initrd.img",
        "result: pass",
        "no bootable Base1 release claim",
    ] {
        assert!(
            stdout.contains(expected),
            "missing verifier output: {expected}\n{stdout}"
        );
    }

    let _ = fs::remove_dir_all(&out);
    let _ = fs::remove_file(&kernel);
    let _ = fs::remove_file(&initrd);
}

#[test]
fn base1_preview_verify_fails_after_bundle_drift() {
    let pid = std::process::id();
    let out = format!("build/base1-preview-verify-drift-{pid}");
    let kernel = format!("build/base1-preview-verify-drift-kernel-{pid}");
    let initrd = format!("build/base1-preview-verify-drift-initrd-{pid}");

    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all("build").expect("build directory should be creatable");
    fs::write(&kernel, "kernel placeholder").expect("kernel placeholder should be writable");
    fs::write(&initrd, "initrd placeholder").expect("initrd placeholder should be writable");

    let stack = Command::new("sh")
        .arg(STACK)
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
        stack.status.success(),
        "stack should pass before drift test"
    );

    fs::write(format!("{out}/README.txt"), "tampered preview bundle\n")
        .expect("README tamper should be writable in test bundle");

    let verify = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg(&out)
        .output()
        .expect("Base1 preview verifier should execute");

    assert!(
        !verify.status.success(),
        "tampered bundle should fail verification"
    );
    let stdout = String::from_utf8_lossy(&verify.stdout);
    assert!(
        stdout.contains("FAIL  checksum mismatch: README.txt"),
        "missing mismatch output: {stdout}"
    );
    assert!(
        stdout.contains("result: failed"),
        "missing failed result: {stdout}"
    );

    let _ = fs::remove_dir_all(&out);
    let _ = fs::remove_file(&kernel);
    let _ = fs::remove_file(&initrd);
}
