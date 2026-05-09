use std::process::Command;

#[test]
fn recovery_usb_target_report_script_prints_report_template() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-target-report.sh")
        .output()
        .expect("run recovery USB target report script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(
        text.contains("base1 recovery USB target selection report"),
        "{text}"
    );
    assert!(text.contains("mode               : read-only"), "{text}");
    assert!(text.contains("writes             : no"), "{text}");
    assert!(
        text.contains("firmware           : Libreboot expected"),
        "{text}"
    );
    assert!(
        text.contains("hardware           : X200-class expected"),
        "{text}"
    );
    assert!(text.contains("bootloader         : GRUB first"), "{text}");
    assert!(
        text.contains("trust              : no host trust escalation"),
        "{text}"
    );
}

#[test]
fn recovery_usb_target_report_script_lists_identity_fields_and_commands() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-target-report.sh")
        .output()
        .expect("run recovery USB target report script");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(text.contains("device path: unknown"), "{text}");
    assert!(text.contains("device model/name: unknown"), "{text}");
    assert!(text.contains("device size: unknown"), "{text}");
    assert!(text.contains("removable status: unknown"), "{text}");
    assert!(
        text.contains("current attachment status: unknown"),
        "{text}"
    );
    assert!(text.contains("internal disk status: unknown"), "{text}");
    assert!(
        text.contains("physical USB label status: unknown"),
        "{text}"
    );
    assert!(
        text.contains("I understand this will write recovery USB media to the selected device"),
        "{text}"
    );
    assert!(
        text.contains(
            "sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example"
        ),
        "{text}"
    );
    assert!(text.contains("target selection not completed"), "{text}");
}

#[test]
fn recovery_usb_target_report_script_avoids_mutating_tools_and_sensitive_terms() {
    let script = std::fs::read_to_string("scripts/base1-recovery-usb-target-report.sh")
        .expect("recovery USB target report script");

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
