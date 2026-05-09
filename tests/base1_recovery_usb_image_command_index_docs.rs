#[test]
fn recovery_usb_image_command_index_lists_docs_and_commands() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md")
        .expect("recovery usb image command index");

    assert!(
        doc.contains("Base1 recovery USB image provenance command index"),
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
        doc.contains("scripts/base1-recovery-usb-image-summary.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-image-validate.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-image-report.sh"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_image_command_index_preserves_provenance_fields() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md")
        .expect("recovery usb image command index");

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
    assert!(doc.contains("Rollback metadata compatibility"), "{doc}");
}

#[test]
fn recovery_usb_image_command_index_preserves_guardrails() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md")
        .expect("recovery usb image command index");

    assert!(doc.contains("Report downloads: no"), "{doc}");
    assert!(doc.contains("Report writes: no"), "{doc}");
    assert!(
        doc.contains("Do not download images automatically"),
        "{doc}"
    );
    assert!(doc.contains("Do not write USB media"), "{doc}");
    assert!(doc.contains("Do not run dd automatically"), "{doc}");
    assert!(doc.contains("Do not partition disks"), "{doc}");
    assert!(doc.contains("Do not format disks"), "{doc}");
    assert!(doc.contains("Do not install GRUB automatically"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("Do not accept missing checksums"), "{doc}");
    assert!(doc.contains("Do not accept checksum mismatch"), "{doc}");
    assert!(
        doc.contains("Do not accept hidden image provenance"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_image_surfaces_link_image_command_index() {
    let provenance = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_PROVENANCE.md")
        .expect("recovery usb image provenance");
    let summary = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_SUMMARY.md")
        .expect("recovery usb image summary");
    let index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        provenance.contains("RECOVERY_USB_IMAGE_COMMAND_INDEX.md"),
        "{provenance}"
    );
    assert!(
        summary.contains("RECOVERY_USB_IMAGE_COMMAND_INDEX.md"),
        "{summary}"
    );
    assert!(
        index.contains("RECOVERY_USB_IMAGE_COMMAND_INDEX.md"),
        "{index}"
    );
    assert!(
        readme.contains("base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md"),
        "{readme}"
    );
}
