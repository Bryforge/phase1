use std::process::Command;

#[test]
fn recovery_usb_hardware_checklist_script_prints_read_only_checklist() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-hardware-checklist.sh")
        .output()
        .expect("run recovery USB hardware checklist script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(
        text.contains("base1 recovery USB hardware checklist"),
        "{text}"
    );
    assert!(text.contains("mode       : read-only"), "{text}");
    assert!(text.contains("writes     : no"), "{text}");
    assert!(text.contains("firmware   : Libreboot expected"), "{text}");
    assert!(text.contains("hardware   : X200-class expected"), "{text}");
    assert!(text.contains("bootloader : GRUB first"), "{text}");
    assert!(text.contains("media      : external USB planned"), "{text}");
    assert!(
        text.contains("trust      : no host trust escalation"),
        "{text}"
    );
}

#[test]
fn recovery_usb_hardware_checklist_script_lists_observations() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-hardware-checklist.sh")
        .output()
        .expect("run recovery USB hardware checklist script");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(text.contains("GRUB menu reachable: unknown"), "{text}");
    assert!(text.contains("USB boot option visible: unknown"), "{text}");
    assert!(
        text.contains("external USB device labeled: unknown"),
        "{text}"
    );
    assert!(
        text.contains("keyboard works in boot menu: unknown"),
        "{text}"
    );
    assert!(
        text.contains("display readable in recovery mode: unknown"),
        "{text}"
    );
    assert!(
        text.contains("emergency shell path known: unknown"),
        "{text}"
    );
    assert!(
        text.contains("rollback metadata path known: unknown"),
        "{text}"
    );
    assert!(
        text.contains("checklist not completed on hardware"),
        "{text}"
    );
}

#[test]
fn recovery_usb_hardware_checklist_script_avoids_mutating_tools_and_sensitive_terms() {
    let script = std::fs::read_to_string("scripts/base1-recovery-usb-hardware-checklist.sh")
        .expect("recovery USB hardware checklist script");

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
