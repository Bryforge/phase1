use std::process::Command;

#[test]
fn recovery_usb_emergency_shell_summary_script_prints_summary() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-emergency-shell-summary.sh")
        .output()
        .expect("run recovery USB emergency shell summary script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(
        text.contains("base1 recovery USB emergency shell summary"),
        "{text}"
    );
    assert!(text.contains("mode                  : read-only"), "{text}");
    assert!(text.contains("writes                : no"), "{text}");
    assert!(text.contains("shell_launch          : no"), "{text}");
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
        text.contains("emergency_access      : must remain available"),
        "{text}"
    );
}

#[test]
fn recovery_usb_emergency_shell_summary_script_lists_docs_commands_and_non_claims() {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-usb-emergency-shell-summary.sh")
        .output()
        .expect("run recovery USB emergency shell summary script");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(
        text.contains("base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md"),
        "{text}"
    );
    assert!(
        text.contains("base1/RECOVERY_USB_EMERGENCY_SHELL.md"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-recovery-usb-emergency-shell-summary.sh"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-recovery-usb-emergency-shell-validate.sh"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-recovery-usb-emergency-shell-report.sh"),
        "{text}"
    );
    assert!(
        text.contains("emergency shell execution readiness"),
        "{text}"
    );
    assert!(text.contains("privileged shell launch support"), "{text}");
    assert!(text.contains("automatic recovery readiness"), "{text}");
    assert!(text.contains("real-hardware rollback completion"), "{text}");
}

#[test]
fn recovery_usb_emergency_shell_summary_script_avoids_mutating_tools_and_sensitive_terms() {
    let script = std::fs::read_to_string("scripts/base1-recovery-usb-emergency-shell-summary.sh")
        .expect("recovery USB emergency shell summary script");

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
