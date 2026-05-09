use std::process::Command;

#[test]
fn recovery_usb_image_report_script_prints_report_template() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-image-report.sh")
        .output()
        .expect("run recovery USB image report script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(
        text.contains("base1 recovery USB image provenance report"),
        "{text}"
    );
    assert!(text.contains("mode                 : read-only"), "{text}");
    assert!(text.contains("writes               : no"), "{text}");
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
        text.contains("trust                : no host trust escalation"),
        "{text}"
    );
}

#[test]
fn recovery_usb_image_report_script_lists_provenance_and_checksums() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-image-report.sh")
        .output()
        .expect("run recovery USB image report script");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(text.contains("image filename: unknown"), "{text}");
    assert!(
        text.contains("image source URL or local path: unknown"),
        "{text}"
    );
    assert!(text.contains("image build commit: unknown"), "{text}");
    assert!(text.contains("expected SHA256 checksum: unknown"), "{text}");
    assert!(text.contains("observed SHA256 checksum: unknown"), "{text}");
    assert!(text.contains("checksum match: unknown"), "{text}");
    assert!(text.contains("signature status: unknown"), "{text}");
    assert!(text.contains("future media writing must refuse"), "{text}");
    assert!(text.contains("image provenance not verified"), "{text}");
}

#[test]
fn recovery_usb_image_report_script_avoids_mutating_tools_and_sensitive_terms() {
    let script = std::fs::read_to_string("scripts/base1-recovery-usb-image-report.sh")
        .expect("recovery USB image report script");

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
