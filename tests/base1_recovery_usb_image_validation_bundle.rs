use std::process::Command;

#[test]
fn recovery_usb_image_validation_bundle_runs_read_only_previews() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-image-validate.sh")
        .output()
        .expect("run recovery USB image validation bundle");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(
        text.contains("base1 recovery USB image provenance validation bundle"),
        "{text}"
    );
    assert!(text.contains("mode                 : read-only"), "{text}");
    assert!(text.contains("writes               : no"), "{text}");
    assert!(text.contains("downloads            : no"), "{text}");
    assert!(
        text.contains("firmware             : Libreboot expected"),
        "{text}"
    );
    assert!(
        text.contains("hardware             : X200-class expected"),
        "{text}"
    );
    assert!(text.contains("bootloader           : GRUB first"), "{text}");
    assert!(
        text.contains("checksum_rule        : exact match required before future writing"),
        "{text}"
    );
    assert!(
        text.contains("base1 recovery USB image provenance report"),
        "{text}"
    );
    assert!(
        text.contains("base1 recovery USB target selection summary"),
        "{text}"
    );
    assert!(
        text.contains("base1 recovery USB target selection validation bundle"),
        "{text}"
    );
    assert!(
        text.contains("base1 recovery USB image provenance validation bundle complete"),
        "{text}"
    );
}

#[test]
fn recovery_usb_image_validation_bundle_help_is_read_only() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-image-validate.sh")
        .arg("--help")
        .output()
        .expect("run recovery USB image validation help");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{text}");
    assert!(text.contains("read-only"), "{text}");
    assert!(text.contains("without downloading images"), "{text}");
    assert!(text.contains("host trust"), "{text}");
}

#[test]
fn recovery_usb_image_validation_bundle_avoids_mutating_tools_and_sensitive_terms() {
    let script = std::fs::read_to_string("scripts/base1-recovery-usb-image-validate.sh")
        .expect("recovery USB image validation bundle");

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
