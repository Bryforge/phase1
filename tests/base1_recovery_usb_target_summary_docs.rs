#[test]
fn recovery_usb_target_summary_lists_core_docs_and_commands() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_SUMMARY.md")
        .expect("recovery usb target summary");

    assert!(
        doc.contains("Base1 recovery USB target selection summary"),
        "{doc}"
    );
    assert!(doc.contains("RECOVERY_USB_TARGET_SELECTION.md"), "{doc}");
    assert!(
        doc.contains("RECOVERY_USB_TARGET_COMMAND_INDEX.md"),
        "{doc}"
    );
    assert!(doc.contains("RECOVERY_USB_HARDWARE_SUMMARY.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_COMMAND_INDEX.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_VALIDATION_REPORT.md"), "{doc}");
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-target-validate.sh"),
        "{doc}"
    );
    assert!(
        doc.contains(
            "sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example"
        ),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-target-report.sh"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_target_summary_records_identity_fields_and_confirmation() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_SUMMARY.md")
        .expect("recovery usb target summary");

    assert!(doc.contains("Device path"), "{doc}");
    assert!(doc.contains("Device model/name"), "{doc}");
    assert!(doc.contains("Device size"), "{doc}");
    assert!(doc.contains("Removable status"), "{doc}");
    assert!(doc.contains("Current attachment status"), "{doc}");
    assert!(doc.contains("Internal disk status"), "{doc}");
    assert!(doc.contains("Operator confirmation status"), "{doc}");
    assert!(
        doc.contains("I understand this will write recovery USB media to the selected device"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_target_summary_preserves_non_claims_and_promotion_rule() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_SUMMARY.md")
        .expect("recovery usb target summary");

    assert!(doc.contains("USB media writing readiness"), "{doc}");
    assert!(doc.contains("Bootable Base1 image readiness"), "{doc}");
    assert!(doc.contains("Destructive installer readiness"), "{doc}");
    assert!(doc.contains("Hidden target discovery safety"), "{doc}");
    assert!(doc.contains("Real-hardware recovery completion"), "{doc}");
    assert!(doc.contains("Real-hardware rollback completion"), "{doc}");
    assert!(doc.contains("must remain read-only"), "{doc}");
}

#[test]
fn recovery_usb_target_surfaces_link_target_summary() {
    let target = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_SELECTION.md")
        .expect("recovery usb target selection");
    let index = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md")
        .expect("recovery usb target command index");
    let hardware = std::fs::read_to_string("base1/RECOVERY_USB_HARDWARE_SUMMARY.md")
        .expect("recovery usb hardware summary");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        target.contains("RECOVERY_USB_TARGET_SUMMARY.md"),
        "{target}"
    );
    assert!(index.contains("RECOVERY_USB_TARGET_SUMMARY.md"), "{index}");
    assert!(
        hardware.contains("RECOVERY_USB_TARGET_SUMMARY.md"),
        "{hardware}"
    );
    assert!(
        readme.contains("base1/RECOVERY_USB_TARGET_SUMMARY.md"),
        "{readme}"
    );
}
