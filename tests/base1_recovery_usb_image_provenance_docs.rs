#[test]
fn recovery_usb_image_provenance_defines_read_only_verification_path() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_PROVENANCE.md")
        .expect("recovery usb image provenance design");

    assert!(
        doc.contains("Base1 recovery USB image provenance and checksum design"),
        "{doc}"
    );
    assert!(doc.contains("advisory and read-only"), "{doc}");
    assert!(doc.contains("does not download images"), "{doc}");
    assert!(doc.contains("Image filename"), "{doc}");
    assert!(
        doc.contains("Image source URL or local source path"),
        "{doc}"
    );
    assert!(doc.contains("Image build commit"), "{doc}");
    assert!(doc.contains("Expected SHA256 checksum"), "{doc}");
    assert!(doc.contains("Observed SHA256 checksum"), "{doc}");
    assert!(doc.contains("Signature status"), "{doc}");
}

#[test]
fn recovery_usb_image_provenance_requires_checksum_rule() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_PROVENANCE.md")
        .expect("recovery usb image provenance design");

    assert!(doc.contains("Checksum rule"), "{doc}");
    assert!(
        doc.contains("refuse to write unless the observed checksum exactly matches"),
        "{doc}"
    );
    assert!(doc.contains("does not implement image downloads"), "{doc}");
}

#[test]
fn recovery_usb_image_provenance_preserves_guardrails() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_PROVENANCE.md")
        .expect("recovery usb image provenance design");

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
fn recovery_usb_surfaces_link_image_provenance_design() {
    let target = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_SUMMARY.md")
        .expect("recovery usb target summary");
    let index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        target.contains("RECOVERY_USB_IMAGE_PROVENANCE.md"),
        "{target}"
    );
    assert!(
        index.contains("RECOVERY_USB_IMAGE_PROVENANCE.md"),
        "{index}"
    );
    assert!(
        readme.contains("base1/RECOVERY_USB_IMAGE_PROVENANCE.md"),
        "{readme}"
    );
}
