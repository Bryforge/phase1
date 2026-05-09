#[test]
fn recovery_usb_target_selection_defines_read_only_device_identity_path() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_SELECTION.md")
        .expect("recovery usb target selection design");

    assert!(
        doc.contains("Base1 recovery USB target-device selection design"),
        "{doc}"
    );
    assert!(doc.contains("advisory and read-only"), "{doc}");
    assert!(doc.contains("Device path"), "{doc}");
    assert!(doc.contains("Device model/name"), "{doc}");
    assert!(doc.contains("Device size"), "{doc}");
    assert!(doc.contains("Removable status"), "{doc}");
    assert!(doc.contains("Current mount status"), "{doc}");
    assert!(doc.contains("physically labeled the USB device"), "{doc}");
}

#[test]
fn recovery_usb_target_selection_requires_future_confirmation_phrase() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_SELECTION.md")
        .expect("recovery usb target selection design");

    assert!(doc.contains("Required confirmation language"), "{doc}");
    assert!(
        doc.contains("I understand this will write recovery USB media to the selected device"),
        "{doc}"
    );
    assert!(
        doc.contains("does not implement that mutating command"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_target_selection_preserves_guardrails() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_SELECTION.md")
        .expect("recovery usb target selection design");

    assert!(doc.contains("Do not write USB media"), "{doc}");
    assert!(doc.contains("Do not run dd automatically"), "{doc}");
    assert!(doc.contains("Do not partition disks"), "{doc}");
    assert!(doc.contains("Do not format disks"), "{doc}");
    assert!(doc.contains("Do not install GRUB automatically"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(
        doc.contains("Do not allow hidden target selection"),
        "{doc}"
    );
    assert!(
        doc.contains("Do not default to the internal system disk"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_surfaces_link_target_selection_design() {
    let summary = std::fs::read_to_string("base1/RECOVERY_USB_HARDWARE_SUMMARY.md")
        .expect("recovery usb hardware summary");
    let index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        summary.contains("RECOVERY_USB_TARGET_SELECTION.md"),
        "{summary}"
    );
    assert!(
        index.contains("RECOVERY_USB_TARGET_SELECTION.md"),
        "{index}"
    );
    assert!(
        readme.contains("base1/RECOVERY_USB_TARGET_SELECTION.md"),
        "{readme}"
    );
}
