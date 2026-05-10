use std::{fs, process::Command};

const DOCTOR: &str = "scripts/base1-emulator-doctor.sh";
const BUNDLE: &str = "scripts/base1-emulator-preview.sh";

#[test]
fn base1_emulator_doctor_exists_and_documents_boundary() {
    let script = fs::read_to_string(DOCTOR).expect("Base1 emulator doctor should be readable");

    for expected in [
        "Base1 emulator doctor",
        "Read-only checker",
        "does not launch QEMU",
        "does not launch the emulator",
        "does not claim Base1 is bootable",
        "read-only bundle inspection tool",
        "no emulator launched",
        "no disk image created",
        "no installer or hardware validation performed",
    ] {
        assert!(
            script.contains(expected),
            "missing doctor boundary text: {expected}"
        );
    }
}

#[test]
fn base1_emulator_doctor_avoids_mutating_tools() {
    let script = fs::read_to_string(DOCTOR).expect("Base1 emulator doctor should be readable");

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
            "doctor script must not contain mutating tooling pattern: {forbidden}"
        );
    }
}

#[test]
fn base1_emulator_doctor_reports_missing_bundle() {
    let output = Command::new("sh")
        .arg(DOCTOR)
        .arg("--bundle")
        .arg("build/base1-doctor-missing-bundle")
        .arg("--no-qemu-check")
        .output()
        .expect("Base1 emulator doctor should execute");

    assert!(
        !output.status.success(),
        "missing bundle should fail doctor"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("BASE1 EMULATOR DOCTOR"));
    assert!(stdout.contains("FAIL  bundle directory does not exist"));
    assert!(stdout.contains("result: failed"));
}

#[test]
fn base1_emulator_doctor_checks_generated_bundle_without_launching() {
    let out = format!("build/base1-emulator-doctor-test-{}", std::process::id());
    let _ = fs::remove_dir_all(&out);

    let build = Command::new("sh")
        .arg(BUNDLE)
        .arg("--out")
        .arg(&out)
        .arg("--profile")
        .arg("doctor-safe")
        .arg("--target")
        .arg("emulator-x86_64")
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

    let doctor = Command::new("sh")
        .arg(DOCTOR)
        .arg("--bundle")
        .arg(&out)
        .arg("--no-qemu-check")
        .output()
        .expect("Base1 emulator doctor should execute");

    assert!(
        doctor.status.success(),
        "doctor failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&doctor.stdout),
        String::from_utf8_lossy(&doctor.stderr)
    );

    let stdout = String::from_utf8_lossy(&doctor.stdout);
    for expected in [
        "BASE1 EMULATOR DOCTOR",
        "PASS  bundle path is under build/",
        "PASS  bundle manifest: manifest.env",
        "PASS  staging manifest: staging/manifest.env",
        "PASS  Phase1 launcher: staging/rootfs/usr/local/bin/base1-phase1-run.sh",
        "PASS  bundle QEMU scaffold: run-qemu-bundle.sh",
        "WARN  kernel for direct-kernel preview missing: staging/boot/vmlinuz",
        "WARN  initrd for direct-kernel preview missing: staging/boot/initrd.img",
        "WARN  qemu executable check skipped",
        "result: pass-with-notes",
        "non-claims: no emulator launched; no disk image created; no installer or hardware validation performed",
    ] {
        assert!(stdout.contains(expected), "doctor output missing: {expected}\n{stdout}");
    }

    let _ = fs::remove_dir_all(&out);
}
