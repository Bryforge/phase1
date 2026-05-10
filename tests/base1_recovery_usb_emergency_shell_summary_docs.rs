#[test]
fn recovery_usb_emergency_shell_summary_lists_core_docs_and_commands() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md")
        .expect("recovery usb emergency shell summary");

    assert!(
        doc.contains("Base1 recovery USB emergency shell summary"),
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
        doc.contains("sh scripts/base1-recovery-usb-emergency-shell-validate.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-emergency-shell-report.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-recovery-dry-run.sh --dry-run"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_emergency_shell_summary_records_behavior_fields() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md")
        .expect("recovery usb emergency shell summary");

    assert!(doc.contains("Emergency shell entry path"), "{doc}");
    assert!(doc.contains("Keyboard availability"), "{doc}");
    assert!(doc.contains("Display readability"), "{doc}");
    assert!(doc.contains("Root/admin boundary"), "{doc}");
    assert!(doc.contains("Phase1 auto-launch status"), "{doc}");
    assert!(doc.contains("Rollback metadata path"), "{doc}");
    assert!(doc.contains("Log collection path"), "{doc}");
}

#[test]
fn recovery_usb_emergency_shell_summary_preserves_non_claims_and_promotion_rule() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md")
        .expect("recovery usb emergency shell summary");

    assert!(doc.contains("Emergency shell execution readiness"), "{doc}");
    assert!(doc.contains("Privileged shell launch support"), "{doc}");
    assert!(doc.contains("USB media writing readiness"), "{doc}");
    assert!(doc.contains("Bootable Base1 image readiness"), "{doc}");
    assert!(doc.contains("Automatic recovery readiness"), "{doc}");
    assert!(doc.contains("Real-hardware recovery completion"), "{doc}");
    assert!(doc.contains("Real-hardware rollback completion"), "{doc}");
    assert!(doc.contains("must remain read-only"), "{doc}");
}

#[test]
fn recovery_usb_emergency_shell_surfaces_link_summary() {
    let emergency = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL.md")
        .expect("recovery usb emergency shell");
    let image = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_SUMMARY.md")
        .expect("recovery usb image summary");
    let command_index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        emergency.contains("RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md"),
        "{emergency}"
    );
    assert!(
        image.contains("RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md"),
        "{image}"
    );
    assert!(
        command_index.contains("RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md"),
        "{command_index}"
    );
    assert!(
        readme.contains("base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md"),
        "{readme}"
    );
}
