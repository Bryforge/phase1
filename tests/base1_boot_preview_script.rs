use std::{fs, process::Command};

const SCRIPT: &str = "scripts/base1-boot-preview.sh";

#[test]
fn base1_boot_preview_script_exists_and_documents_boundary() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 boot preview script should be readable");

    for expected in [
        "Base1 boot preview generator",
        "staging-only",
        "manifest.env",
        "rootfs/opt/phase1",
        "boot/grub/grub.cfg",
        "run-qemu-preview.sh",
        "not a bootable Base1 image",
        "not an installer",
        "not hardware validated",
    ] {
        assert!(
            script.contains(expected),
            "missing preview boundary text: {expected}"
        );
    }
}

#[test]
fn base1_boot_preview_script_avoids_destructive_tools() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 boot preview script should be readable");

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
            "boot preview script must not contain destructive tooling pattern: {forbidden}"
        );
    }
}

#[test]
fn base1_boot_preview_script_refuses_unsafe_output_roots() {
    let script = fs::read_to_string(SCRIPT).expect("Base1 boot preview script should be readable");

    for expected in ["safe_out_dir", "/dev", "/proc", "/sys", "/run"] {
        assert!(
            script.contains(expected),
            "missing unsafe output guard: {expected}"
        );
    }

    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--out")
        .arg("/")
        .output()
        .expect("base1 boot preview script should execute");

    assert!(
        !output.status.success(),
        "unsafe output directory should be rejected"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("refusing unsafe output directory"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn base1_boot_preview_generates_staging_tree() {
    let out = std::env::temp_dir().join(format!(
        "phase1-base1-boot-preview-test-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&out);

    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--out")
        .arg(&out)
        .arg("--profile")
        .arg("test-safe")
        .arg("--target")
        .arg("emulator-x86_64")
        .output()
        .expect("base1 boot preview script should execute");

    assert!(
        output.status.success(),
        "script failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    for path in [
        "manifest.env",
        "rootfs/opt/phase1/README.txt",
        "rootfs/var/lib/phase1/workspace/.keep",
        "rootfs/usr/local/bin/base1-phase1-run.sh",
        "rootfs/etc/systemd/system/phase1-base1.service",
        "boot/README.txt",
        "boot/grub/grub.cfg",
        "boot/vmlinuz.MISSING",
        "boot/initrd.img.MISSING",
        "run-qemu-preview.sh",
        "reports/README.txt",
    ] {
        assert!(
            out.join(path).exists(),
            "missing generated preview path: {path}"
        );
    }

    let manifest =
        fs::read_to_string(out.join("manifest.env")).expect("manifest should be readable");
    for expected in [
        "BASE1_PREVIEW_STATUS=staging-only",
        "BASE1_PROFILE=test-safe",
        "BASE1_HARDWARE_TARGET=emulator-x86_64",
        "PHASE1_SAFE_MODE=1",
        "PHASE1_ALLOW_HOST_TOOLS=0",
        "BASE1_NON_CLAIM_BOOTABLE=1",
    ] {
        assert!(manifest.contains(expected), "manifest missing: {expected}");
    }

    let qemu = fs::read_to_string(out.join("run-qemu-preview.sh"))
        .expect("qemu script should be readable");
    assert!(qemu.contains("missing boot/vmlinuz or boot/initrd.img"));
    assert!(qemu.contains("base1.preview=1"));
    assert!(qemu.contains("phase1.safe=1"));
    assert!(qemu.contains("phase1.host_tools=0"));

    let _ = fs::remove_dir_all(&out);
}
