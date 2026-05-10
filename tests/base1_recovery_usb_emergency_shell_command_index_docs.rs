#[test]
fn recovery_usb_emergency_shell_command_index_lists_docs_and_commands() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md")
        .expect("recovery usb emergency shell command index");

    assert!(
        doc.contains("Base1 recovery USB emergency shell command index"),
        "{doc}"
    );
    assert!(doc.contains("RECOVERY_USB_EMERGENCY_SHELL.md"), "{doc}");
    assert!(
        doc.contains("RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md"),
        "{doc}"
    );
    assert!(doc.contains("RECOVERY_USB_IMAGE_SUMMARY.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_IMAGE_COMMAND_INDEX.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_COMMAND_INDEX.md"), "{doc}");
    assert!(
        doc.contains("scripts/base1-recovery-usb-emergency-shell-summary.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-emergency-shell-validate.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-emergency-shell-report.sh"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_emergency_shell_command_index_preserves_behavior_fields() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md")
        .expect("recovery usb emergency shell command index");

    assert!(doc.contains("Emergency shell entry path"), "{doc}");
    assert!(doc.contains("Keyboard availability"), "{doc}");
    assert!(doc.contains("Display readability"), "{doc}");
    assert!(doc.contains("Root/admin boundary"), "{doc}");
    assert!(doc.contains("Phase1 auto-launch status"), "{doc}");
    assert!(doc.contains("Rollback metadata path"), "{doc}");
    assert!(doc.contains("Log collection path"), "{doc}");
    assert!(doc.contains("Exit/reboot path"), "{doc}");
}

#[test]
fn recovery_usb_emergency_shell_command_index_preserves_guardrails() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md")
        .expect("recovery usb emergency shell command index");

    assert!(doc.contains("Report shell launch: no"), "{doc}");
    assert!(doc.contains("Report writes: no"), "{doc}");
    assert!(
        doc.contains("Keep emergency shell access available"),
        "{doc}"
    );
    assert!(
        doc.contains("Keep root/admin boundaries operator-visible"),
        "{doc}"
    );
    assert!(
        doc.contains("Do not launch privileged shells automatically"),
        "{doc}"
    );
    assert!(
        doc.contains("Do not remove emergency shell access"),
        "{doc}"
    );
    assert!(doc.contains("Do not hide root/admin boundaries"), "{doc}");
    assert!(doc.contains("Do not write USB media"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("Do not store passwords"), "{doc}");
}

#[test]
fn recovery_usb_emergency_shell_surfaces_link_command_index() {
    let emergency = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL.md")
        .expect("recovery usb emergency shell");
    let summary = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md")
        .expect("recovery usb emergency shell summary");
    let command_index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        emergency.contains("RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md"),
        "{emergency}"
    );
    assert!(
        summary.contains("RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md"),
        "{summary}"
    );
    assert!(
        command_index.contains("RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md"),
        "{command_index}"
    );
    assert!(
        readme.contains("base1/RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md"),
        "{readme}"
    );
}
