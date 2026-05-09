use std::process::Command;

fn run_script(args: &[&str]) -> (bool, String) {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-dry-run.sh")
        .args(args)
        .output()
        .expect("run recovery USB dry-run script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), text)
}

#[test]
fn recovery_usb_dry_run_requires_dry_run_flag() {
    let (ok, text) = run_script(&["--target", "/dev/example"]);

    assert!(!ok, "{text}");
    assert!(text.contains("refusing: --dry-run is required"), "{text}");
}

#[test]
fn recovery_usb_dry_run_requires_target() {
    let (ok, text) = run_script(&["--dry-run"]);

    assert!(!ok, "{text}");
    assert!(text.contains("--target <usb-device> is required"), "{text}");
}

#[test]
fn recovery_usb_dry_run_rejects_non_device_target() {
    let (ok, text) = run_script(&["--dry-run", "--target", "usb0"]);

    assert!(!ok, "{text}");
    assert!(
        text.contains("target must look like a device path under /dev"),
        "{text}"
    );
}

#[test]
fn recovery_usb_dry_run_reports_preview_without_writes() {
    let (ok, text) = run_script(&["--dry-run", "--target", "/dev/example"]);

    assert!(ok, "{text}");
    assert!(text.contains("base1 recovery USB dry-run"), "{text}");
    assert!(text.contains("target       : /dev/example"), "{text}");
    assert!(text.contains("writes       : no"), "{text}");
    assert!(text.contains("firmware     : Libreboot expected"), "{text}");
    assert!(
        text.contains("hardware     : X200-class expected"),
        "{text}"
    );
    assert!(text.contains("bootloader   : GRUB first"), "{text}");
    assert!(text.contains("usb_media    : preview only"), "{text}");
    assert!(text.contains("image        : not created"), "{text}");
    assert!(text.contains("boot_order   : no change"), "{text}");
    assert!(
        text.contains("trust        : no host trust escalation"),
        "{text}"
    );
}

#[test]
fn recovery_usb_dry_run_script_avoids_mutating_tools_and_sensitive_terms() {
    let script = std::fs::read_to_string("scripts/base1-recovery-usb-dry-run.sh")
        .expect("recovery USB dry-run script");

    let forbidden = [
        "flashrom",
        "grub-install",
        "grub-mkconfig",
        "update-grub",
        "bootctl install",
        "efibootmgr",
        "mkfs",
        "fdisk",
        "parted",
        "sfdisk",
        "diskutil erase",
        "dd if=",
        "mount ",
        "umount ",
        "rm -rf",
        "password",
        "token",
        "private key",
    ];

    for token in forbidden {
        assert!(
            !script.contains(token),
            "forbidden token {token:?} found in script"
        );
    }
}
