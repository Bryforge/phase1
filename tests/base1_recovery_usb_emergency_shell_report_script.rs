use std::process::Command;

#[test]
fn recovery_usb_emergency_shell_report_script_prints_report_template() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-emergency-shell-report.sh")
        .output()
        .expect("run recovery USB emergency shell report script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(
        text.contains("base1 recovery USB emergency shell report"),
        "{text}"
    );
    assert!(text.contains("mode                  : read-only"), "{text}");
    assert!(text.contains("writes                : no"), "{text}");
    assert!(
        text.contains("firmware              : Libreboot expected"),
        "{text}"
    );
    assert!(
        text.contains("hardware              : X200-class expected"),
        "{text}"
    );
    assert!(
        text.contains("bootloader            : GRUB first"),
        "{text}"
    );
    assert!(
        text.contains("trust                 : no host trust escalation"),
        "{text}"
    );
}

#[test]
fn recovery_usb_emergency_shell_report_script_lists_behavior_and_commands() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-emergency-shell-report.sh")
        .output()
        .expect("run recovery USB emergency shell report script");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(
        text.contains("emergency shell entry path: unknown"),
        "{text}"
    );
    assert!(text.contains("keyboard availability: unknown"), "{text}");
    assert!(text.contains("display readability: unknown"), "{text}");
    assert!(
        text.contains("root/admin boundary: operator-visible"),
        "{text}"
    );
    assert!(
        text.contains("Phase1 state path: /state/phase1 preview"),
        "{text}"
    );
    assert!(
        text.contains("rollback metadata path: /recovery preview"),
        "{text}"
    );
    assert!(
        text.contains("emergency shell access must remain available"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-recovery-usb-image-validate.sh"),
        "{text}"
    );
    assert!(
        text.contains("emergency shell behavior not verified"),
        "{text}"
    );
}

#[test]
fn recovery_usb_emergency_shell_report_script_avoids_mutating_tools_and_sensitive_terms() {
    let script = std::fs::read_to_string("scripts/base1-recovery-usb-emergency-shell-report.sh")
        .expect("recovery USB emergency shell report script");

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
