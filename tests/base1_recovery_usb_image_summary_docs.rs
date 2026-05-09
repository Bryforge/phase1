#[test]
fn recovery_usb_image_summary_lists_core_docs_and_commands() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_SUMMARY.md")
        .expect("recovery usb image summary");

    assert!(
        doc.contains("Base1 recovery USB image provenance summary"),
        "{doc}"
    );
    assert!(doc.contains("RECOVERY_USB_IMAGE_PROVENANCE.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_IMAGE_SUMMARY.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_TARGET_SUMMARY.md"), "{doc}");
    assert!(
        doc.contains("RECOVERY_USB_TARGET_COMMAND_INDEX.md"),
        "{doc}"
    );
    assert!(doc.contains("RECOVERY_USB_COMMAND_INDEX.md"), "{doc}");
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-image-validate.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-image-report.sh"),
        "{doc}"
    );
    assert!(
        doc.contains(
            "sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example"
        ),
        "{doc}"
    );
}

#[test]
fn recovery_usb_image_summary_records_provenance_fields_and_rules() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_SUMMARY.md")
        .expect("recovery usb image summary");

    assert!(doc.contains("Image filename"), "{doc}");
    assert!(
        doc.contains("Image source URL or local source path"),
        "{doc}"
    );
    assert!(doc.contains("Image build commit"), "{doc}");
    assert!(doc.contains("Expected SHA256 checksum"), "{doc}");
    assert!(doc.contains("Observed SHA256 checksum"), "{doc}");
    assert!(doc.contains("Checksum match status"), "{doc}");
    assert!(doc.contains("Signature status"), "{doc}");
    assert!(doc.contains("Future media writing must refuse"), "{doc}");
}

#[test]
fn recovery_usb_image_summary_preserves_non_claims_and_promotion_rule() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_SUMMARY.md")
        .expect("recovery usb image summary");

    assert!(doc.contains("Image download readiness"), "{doc}");
    assert!(
        doc.contains("Signature verification implementation"),
        "{doc}"
    );
    assert!(doc.contains("USB media writing readiness"), "{doc}");
    assert!(doc.contains("Bootable Base1 image readiness"), "{doc}");
    assert!(doc.contains("Destructive installer readiness"), "{doc}");
    assert!(doc.contains("Hidden image provenance safety"), "{doc}");
    assert!(doc.contains("Real-hardware recovery completion"), "{doc}");
    assert!(doc.contains("Real-hardware rollback completion"), "{doc}");
    assert!(doc.contains("must remain read-only"), "{doc}");
}

#[test]
fn recovery_usb_image_surfaces_link_image_summary() {
    let provenance = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_PROVENANCE.md")
        .expect("recovery usb image provenance");
    let target = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_SUMMARY.md")
        .expect("recovery usb target summary");
    let index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        provenance.contains("RECOVERY_USB_IMAGE_SUMMARY.md"),
        "{provenance}"
    );
    assert!(target.contains("RECOVERY_USB_IMAGE_SUMMARY.md"), "{target}");
    assert!(index.contains("RECOVERY_USB_IMAGE_SUMMARY.md"), "{index}");
    assert!(
        readme.contains("base1/RECOVERY_USB_IMAGE_SUMMARY.md"),
        "{readme}"
    );
}
