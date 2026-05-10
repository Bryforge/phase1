use std::{fs, process::Command};

const SCRIPT: &str = "scripts/base1-emulator-preview.sh";

#[test]
fn base1_emulator_preview_script_exists_and_documents_boundary() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 emulator preview script should be readable");

    for expected in [
        "Base1 emulator preview bundle",
        "emulator-only preview",
        "build/",
        "manifest.env",
        "staging/",
        "base1-rootfs-preview.tar",
        "base1-sandbox.raw",
        "run-qemu-bundle.sh",
        "not a released Base1 image",
        "not an installer",
        "not hardware validated",
        "not daily-driver ready",
    ] {
        assert!(script.contains(expected), "missing boundary text: {expected}");
    }
}

#[test]
fn base1_emulator_preview_script_avoids_real_device_tools() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 emulator preview script should be readable");

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
            "emulator preview script must not contain real-device tooling pattern: {forbidden}"
        );
    }
}

#[test]
fn base1_emulator_preview_refuses_output_outside_build() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--out")
        .arg("/tmp/base1-emulator-preview-test")
        .output()
        .expect("base1 emulator preview script should execute");

    assert!(!output.status.success(), "output outside build/ should be rejected");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("output directory must be under build/"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn base1_emulator_preview_generates_bundle_under_build() {
    let out = format!("build/base1-emulator-preview-test-{}", std::process::id());
    let _ = fs::remove_dir_all(&out);

    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--out")
        .arg(&out)
        .arg("--profile")
        .arg("test-safe")
        .arg("--target")
        .arg("emulator-x86_64")
        .arg("--image-mb")
        .arg("1")
        .output()
        .expect("base1 emulator preview script should execute");

    assert!(
        output.status.success(),
        "script failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    for path in [
        "manifest.env",
        "README.txt",
        "staging/manifest.env",
        "staging/rootfs/opt/phase1/README.txt",
        "staging/rootfs/usr/local/bin/base1-phase1-run.sh",
        "staging/rootfs/etc/systemd/system/phase1-base1.service",
        "staging/boot/grub/grub.cfg",
        "staging/run-qemu-preview.sh",
        "base1-sandbox.raw",
        "run-qemu-bundle.sh",
    ] {
        assert!(fs::metadata(format!("{out}/{path}")).is_ok(), "missing generated path: {path}");
    }

    assert!(
        fs::metadata(format!("{out}/base1-rootfs-preview.tar")).is_ok()
            || fs::metadata(format!("{out}/base1-rootfs-preview.tar.MISSING")).is_ok(),
        "missing rootfs archive or archive-missing marker"
    );

    let manifest = fs::read_to_string(format!("{out}/manifest.env")).expect("manifest should be readable");
    for expected in [
        "BASE1_EMULATOR_PREVIEW_STATUS=staging-only",
        "BASE1_PROFILE=test-safe",
        "BASE1_HARDWARE_TARGET=emulator-x86_64",
        "BASE1_EMULATOR_IMAGE_MB=1",
        "BASE1_ROOTFS_TAR=base1-rootfs-preview.tar",
        "BASE1_SANDBOX_RAW=base1-sandbox.raw",
        "BASE1_NON_CLAIM_BOOTABLE_RELEASE=1",
        "BASE1_NON_CLAIM_INSTALLER=1",
        "BASE1_NON_CLAIM_HARDWARE_VALIDATED=1",
    ] {
        assert!(manifest.contains(expected), "manifest missing: {expected}");
    }

    let readme = fs::read_to_string(format!("{out}/README.txt")).expect("bundle README should be readable");
    for expected in [
        "emulator-only staging artifacts",
        "not a released Base1 image",
        "not an installer",
        "not hardware validated",
        "not daily-driver ready",
    ] {
        assert!(readme.contains(expected), "README missing: {expected}");
    }

    let qemu = fs::read_to_string(format!("{out}/run-qemu-bundle.sh")).expect("qemu script should be readable");
    assert!(qemu.contains("missing staging/boot/vmlinuz or staging/boot/initrd.img"));
    assert!(qemu.contains("base1.preview=1"));
    assert!(qemu.contains("base1.emulator=1"));
    assert!(qemu.contains("phase1.safe=1"));
    assert!(qemu.contains("phase1.host_tools=0"));
    assert!(qemu.contains("base1-sandbox.raw"));

    let _ = fs::remove_dir_all(&out);
}
