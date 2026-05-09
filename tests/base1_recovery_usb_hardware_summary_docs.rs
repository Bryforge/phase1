#[test]
fn recovery_usb_hardware_summary_lists_core_docs_and_commands() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_HARDWARE_SUMMARY.md")
        .expect("recovery USB hardware summary");

    assert!(
        doc.contains("Base1 recovery USB hardware validation summary"),
        "{doc}"
    );
    assert!(doc.contains("RECOVERY_USB_DESIGN.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_COMMAND_INDEX.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_HARDWARE_CHECKLIST.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_VALIDATION_REPORT.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_MILESTONE.md"), "{doc}");
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-hardware-validate.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-hardware-report.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_hardware_summary_preserves_guardrails_and_non_claims() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_HARDWARE_SUMMARY.md")
        .expect("recovery USB hardware summary");

    assert!(doc.contains("does not write USB media"), "{doc}");
    assert!(
        doc.contains("does not write USB media") || doc.contains("USB media writing readiness"),
        "{doc}"
    );
    assert!(doc.contains("Bootable Base1 image readiness"), "{doc}");
    assert!(doc.contains("Automatic GRUB repair"), "{doc}");
    assert!(doc.contains("Real-hardware recovery completion"), "{doc}");
    assert!(doc.contains("Real-hardware rollback completion"), "{doc}");
    assert!(doc.contains("remain read-only"), "{doc}");
}

#[test]
fn recovery_usb_indexes_link_hardware_summary() {
    let index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let checklist = std::fs::read_to_string("base1/RECOVERY_USB_HARDWARE_CHECKLIST.md")
        .expect("recovery usb hardware checklist");
    let report = std::fs::read_to_string("base1/RECOVERY_USB_VALIDATION_REPORT.md")
        .expect("recovery usb validation report");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        index.contains("RECOVERY_USB_HARDWARE_SUMMARY.md"),
        "{index}"
    );
    assert!(
        index.contains("Recovery USB hardware validation summary"),
        "{index}"
    );
    assert!(
        checklist.contains("RECOVERY_USB_HARDWARE_SUMMARY.md"),
        "{checklist}"
    );
    assert!(
        report.contains("RECOVERY_USB_HARDWARE_SUMMARY.md"),
        "{report}"
    );
    assert!(
        readme.contains("base1/RECOVERY_USB_HARDWARE_SUMMARY.md"),
        "{readme}"
    );
}
