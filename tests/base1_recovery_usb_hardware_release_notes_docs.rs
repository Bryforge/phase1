const HARDWARE_RELEASE_NOTES: &str =
    "docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md";

#[test]
fn recovery_usb_hardware_release_notes_record_checkpoint_status() {
    let doc = std::fs::read_to_string(HARDWARE_RELEASE_NOTES)
        .expect("recovery usb hardware release notes");

    assert!(
        doc.contains("Base1 recovery USB hardware read-only checkpoint v1"),
        "{doc}"
    );
    assert!(
        doc.contains("checkpoint/base1-recovery-usb-hardware-readonly-v1"),
        "{doc}"
    );
    assert!(
        doc.contains("base1-recovery-usb-hardware-readonly-v1"),
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
        doc.contains("documentation, checklist, reports, and read-only dry-runs"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_hardware_release_notes_list_surfaces_and_non_claims() {
    let doc = std::fs::read_to_string(HARDWARE_RELEASE_NOTES)
        .expect("recovery usb hardware release notes");

    assert!(
        doc.contains("base1/RECOVERY_USB_HARDWARE_SUMMARY.md"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-hardware-summary.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-hardware-validate.sh"),
        "{doc}"
    );
    assert!(doc.contains("USB media writing readiness"), "{doc}");
    assert!(doc.contains("Bootable Base1 image readiness"), "{doc}");
    assert!(doc.contains("Real-hardware recovery completion"), "{doc}");
    assert!(doc.contains("Real-hardware rollback completion"), "{doc}");
}

#[test]
fn recovery_usb_hardware_surfaces_link_release_notes() {
    let summary = std::fs::read_to_string("base1/RECOVERY_USB_HARDWARE_SUMMARY.md")
        .expect("recovery usb hardware summary");
    let index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");
    let releases = std::fs::read_to_string("docs/base1/releases/README.md")
        .expect("base1 release notes index");

    assert!(
        summary.contains("RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md"),
        "{summary}"
    );
    assert!(index.contains(HARDWARE_RELEASE_NOTES), "{index}");
    assert!(
        readme.contains("RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md"),
        "{readme}"
    );
    assert!(
        releases.contains("RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md"),
        "{releases}"
    );
}
