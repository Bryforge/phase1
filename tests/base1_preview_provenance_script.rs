use std::{fs, process::Command};

const SCRIPT: &str = "scripts/base1-preview-provenance.sh";
const BUNDLE: &str = "scripts/base1-emulator-preview.sh";

#[test]
fn base1_preview_provenance_exists_and_documents_boundary() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 preview provenance script should be readable");

    for expected in [
        "Base1 preview provenance",
        "Records checksums",
        "reports/provenance.env",
        "reports/SHA256SUMS",
        "does not launch the emulator",
        "does not claim Base1 is bootable",
        "preview-bundle provenance only",
        "BASE1_NON_CLAIM_BOOTABLE=1",
    ] {
        assert!(script.contains(expected), "missing provenance script text: {expected}");
    }
}

#[test]
fn base1_preview_provenance_avoids_real_device_tools() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 preview provenance script should be readable");

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
            "provenance script must not contain real-device tooling pattern: {forbidden}"
        );
    }
}

#[test]
fn base1_preview_provenance_refuses_bundle_outside_build() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg("/tmp/base1-provenance")
        .output()
        .expect("Base1 preview provenance script should execute");

    assert!(!output.status.success(), "outside-build bundle should be rejected");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("bundle must be under build/"), "unexpected stderr: {stderr}");
}

#[test]
fn base1_preview_provenance_reports_missing_bundle() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg("build/base1-provenance-missing-bundle")
        .output()
        .expect("Base1 preview provenance script should execute");

    assert!(!output.status.success(), "missing bundle should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("bundle directory does not exist"), "unexpected stderr: {stderr}");
}

#[test]
fn base1_preview_provenance_records_hashes_for_generated_bundle() {
    let pid = std::process::id();
    let out = format!("build/base1-provenance-test-{pid}");
    let kernel = format!("build/base1-provenance-kernel-{pid}");
    let initrd = format!("build/base1-provenance-initrd-{pid}");

    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all("build").expect("build directory should be creatable");
    fs::write(&kernel, "kernel placeholder").expect("kernel placeholder should be writable");
    fs::write(&initrd, "initrd placeholder").expect("initrd placeholder should be writable");

    let build = Command::new("sh")
        .arg(BUNDLE)
        .arg("--out")
        .arg(&out)
        .arg("--kernel")
        .arg(&kernel)
        .arg("--initrd")
        .arg(&initrd)
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

    let provenance = Command::new("sh")
        .arg(SCRIPT)
        .arg("--bundle")
        .arg(&out)
        .output()
        .expect("Base1 preview provenance script should execute");

    assert!(
        provenance.status.success(),
        "provenance script failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&provenance.stdout),
        String::from_utf8_lossy(&provenance.stderr)
    );

    let stdout = String::from_utf8_lossy(&provenance.stdout);
    for expected in [
        "tracked manifest.env",
        "tracked base1-sandbox.raw",
        "tracked run-qemu-bundle.sh",
        "tracked staging/manifest.env",
        "tracked staging/boot/vmlinuz",
        "tracked staging/boot/initrd.img",
        "result: pass",
        "non-claim: provenance was recorded for a preview bundle only",
    ] {
        assert!(stdout.contains(expected), "missing stdout: {expected}\n{stdout}");
    }

    let prov_path = format!("{out}/reports/provenance.env");
    let sums_path = format!("{out}/reports/SHA256SUMS");
    assert!(fs::metadata(&prov_path).is_ok(), "missing provenance.env");
    assert!(fs::metadata(&sums_path).is_ok(), "missing SHA256SUMS");

    let prov = fs::read_to_string(&prov_path).expect("provenance.env should be readable");
    for expected in [
        "BASE1_PREVIEW_PROVENANCE_STATUS=preview-only",
        "BASE1_NON_CLAIM_BOOTABLE=1",
        "BASE1_NON_CLAIM_RELEASED_IMAGE=1",
        "BASE1_NON_CLAIM_HARDWARE_VALIDATED=1",
        "file=manifest.env sha256=",
        "file=base1-sandbox.raw sha256=",
        "file=staging/boot/vmlinuz sha256=",
        "file=staging/boot/initrd.img sha256=",
        "result=pass",
        "no bootable Base1 release claim",
    ] {
        assert!(prov.contains(expected), "missing provenance field: {expected}\n{prov}");
    }

    let sums = fs::read_to_string(&sums_path).expect("SHA256SUMS should be readable");
    for expected in [
        "  manifest.env",
        "  base1-sandbox.raw",
        "  run-qemu-bundle.sh",
        "  staging/manifest.env",
        "  staging/boot/grub/grub.cfg",
        "  staging/boot/vmlinuz",
        "  staging/boot/initrd.img",
    ] {
        assert!(sums.contains(expected), "missing checksum entry: {expected}\n{sums}");
    }

    let _ = fs::remove_dir_all(&out);
    let _ = fs::remove_file(&kernel);
    let _ = fs::remove_file(&initrd);
}
