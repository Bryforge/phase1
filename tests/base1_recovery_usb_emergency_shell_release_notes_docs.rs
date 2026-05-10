#[test]
fn recovery_usb_emergency_shell_release_notes_record_checkpoint_status() {
    let doc = std::fs::read_to_string("RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md")
        .expect("recovery usb emergency shell release notes");

    assert!(
        doc.contains("Base1 recovery USB emergency shell read-only checkpoint v1"),
        "{doc}"
    );
    assert!(
        doc.contains("checkpoint/base1-recovery-usb-emergency-shell-readonly-v1"),
        "{doc}"
    );
    assert!(
        doc.contains("base1-recovery-usb-emergency-shell-readonly-v1"),
        "{doc}"
    );
    assert!(
        doc.contains("Firmware profile: Libreboot expected"),
        "{doc}"
    );
    assert!(
        doc.contains("Hardware profile: ThinkPad X200-class expected"),
        "{doc}"
    );
    assert!(doc.contains("Bootloader expectation: GRUB first"), "{doc}");
    assert!(
        doc.contains("Emergency shell access: must remain available"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_emergency_shell_release_notes_list_surfaces_and_non_claims() {
    let doc = std::fs::read_to_string("RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md")
        .expect("recovery usb emergency shell release notes");

    assert!(
        doc.contains("base1/RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-emergency-shell-summary.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-emergency-shell-validate.sh"),
        "{doc}"
    );
    assert!(doc.contains("Emergency shell execution readiness"), "{doc}");
    assert!(doc.contains("Privileged shell launch support"), "{doc}");
    assert!(doc.contains("Automatic recovery readiness"), "{doc}");
    assert!(doc.contains("Real-hardware rollback completion"), "{doc}");
}

#[test]
fn recovery_usb_emergency_shell_surfaces_link_release_notes() {
    let summary = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md")
        .expect("recovery usb emergency shell summary");
    let index = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md")
        .expect("recovery usb emergency shell command index");
    let command_index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        summary.contains("RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md"),
        "{summary}"
    );
    assert!(
        index.contains("RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md"),
        "{index}"
    );
    assert!(
        command_index.contains("RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md"),
        "{command_index}"
    );
    assert!(
        readme.contains("RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md"),
        "{readme}"
    );
}
