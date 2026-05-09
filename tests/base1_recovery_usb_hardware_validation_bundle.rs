use std::process::Command;

#[test]
fn recovery_usb_hardware_validation_bundle_runs_read_only_previews() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-hardware-validate.sh")
        .output()
        .expect("run recovery USB hardware validation bundle");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(
        text.contains("base1 recovery USB hardware validation bundle"),
        "{text}"
    );
    assert!(text.contains("mode       : read-only"), "{text}");
    assert!(text.contains("writes     : no"), "{text}");
    assert!(text.contains("firmware   : Libreboot expected"), "{text}");
    assert!(text.contains("hardware   : X200-class expected"), "{text}");
    assert!(text.contains("bootloader : GRUB first"), "{text}");
    assert!(
        text.contains("base1 recovery USB hardware checklist"),
        "{text}"
    );
    assert!(text.contains("base1 recovery USB command index"), "{text}");
    assert!(
        text.contains("base1 recovery USB validation bundle"),
        "{text}"
    );
    assert!(text.contains("base1 recovery USB dry-run"), "{text}");
    assert!(
        text.contains("base1 recovery USB hardware validation bundle complete"),
        "{text}"
    );
}

#[test]
fn recovery_usb_hardware_validation_bundle_help_is_read_only() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-hardware-validate.sh")
        .arg("--help")
        .output()
        .expect("run recovery USB hardware validation help");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{text}");
    assert!(text.contains("read-only"), "{text}");
    assert!(text.contains("without writing USB media"), "{text}");
    assert!(text.contains("flashing firmware"), "{text}");
    assert!(text.contains("host trust"), "{text}");
}

#[test]
fn recovery_usb_hardware_validation_bundle_avoids_mutating_tools_and_sensitive_terms() {
    let script = std::fs::read_to_string("scripts/base1-recovery-usb-hardware-validate.sh")
        .expect("recovery USB hardware validation bundle");

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
