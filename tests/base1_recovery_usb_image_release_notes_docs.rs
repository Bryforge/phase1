const IMAGE_RELEASE_NOTES: &str =
    "docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md";

#[test]
fn recovery_usb_image_release_notes_record_checkpoint_status() {
    let doc =
        std::fs::read_to_string(IMAGE_RELEASE_NOTES).expect("recovery usb image release notes");

    assert!(
        doc.contains("Base1 recovery USB image provenance read-only checkpoint v1"),
        "{doc}"
    );
    assert!(
        doc.contains("checkpoint/base1-recovery-usb-image-readonly-v1"),
        "{doc}"
    );
    assert!(
        doc.contains("base1-recovery-usb-image-readonly-v1"),
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
        doc.contains("Checksum rule: exact match required before future writing"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_image_release_notes_list_surfaces_and_non_claims() {
    let doc =
        std::fs::read_to_string(IMAGE_RELEASE_NOTES).expect("recovery usb image release notes");

    assert!(
        doc.contains("base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-image-summary.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-image-validate.sh"),
        "{doc}"
    );
    assert!(doc.contains("Image download readiness"), "{doc}");
    assert!(
        doc.contains("Signature verification implementation"),
        "{doc}"
    );
    assert!(doc.contains("Hidden image provenance safety"), "{doc}");
    assert!(doc.contains("Real-hardware rollback completion"), "{doc}");
}

#[test]
fn recovery_usb_image_surfaces_link_release_notes() {
    let summary = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_SUMMARY.md")
        .expect("recovery usb image summary");
    let index = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md")
        .expect("recovery usb image command index");
    let command_index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");
    let releases = std::fs::read_to_string("docs/base1/releases/README.md")
        .expect("base1 release notes index");

    assert!(
        summary.contains("RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md"),
        "{summary}"
    );
    assert!(
        index.contains("RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md"),
        "{index}"
    );
    assert!(
        command_index.contains(IMAGE_RELEASE_NOTES),
        "{command_index}"
    );
    assert!(
        readme.contains("RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md"),
        "{readme}"
    );
    assert!(
        releases.contains("RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md"),
        "{releases}"
    );
}
