use std::fs;
use std::path::Path;
use std::process::Command;

const SCRIPT: &str = "scripts/base1-real-phase1-initrd-preview.sh";

#[test]
fn real_phase1_initrd_preview_script_exists() {
    assert!(Path::new(SCRIPT).exists(), "script should exist");
}

#[test]
fn real_phase1_initrd_preview_documents_boundary() {
    let script = fs::read_to_string(SCRIPT).expect("script should be readable");

    for expected in [
        "preview initramfs only",
        "no installer",
        "hardware validation",
        "release image",
        "refusing output outside build/",
    ] {
        assert!(script.contains(expected), "missing boundary text: {expected}");
    }
}

#[test]
fn real_phase1_initrd_preview_requires_base_initrd_and_binary() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--base-initrd")
        .arg("build/missing-initrd.img")
        .arg("--phase1-bin")
        .arg("build/missing-phase1")
        .arg("--out")
        .arg("build/base1-real-boot/test.img")
        .output()
        .expect("script should run");

    assert!(
        !output.status.success(),
        "script should fail when inputs are missing"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing base initrd"),
        "stderr should explain missing base initrd: {stderr}"
    );
}

#[test]
fn real_phase1_initrd_preview_refuses_output_outside_build() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--out")
        .arg("/tmp/base1-real-phase1.img")
        .output()
        .expect("script should run");

    assert!(
        !output.status.success(),
        "script should refuse output outside build/"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("refusing output outside build/"),
        "stderr should explain output boundary: {stderr}"
    );
}
