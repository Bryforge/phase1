#[test]
fn recovery_usb_hardware_checklist_defines_target_and_read_only_path() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_HARDWARE_CHECKLIST.md")
        .expect("recovery usb hardware checklist");

    assert!(
        doc.contains("Base1 recovery USB hardware validation checklist"),
        "{doc}"
    );
    assert!(doc.contains("advisory and read-only"), "{doc}");
    assert!(doc.contains("Firmware profile: Libreboot"), "{doc}");
    assert!(
        doc.contains("Hardware profile: ThinkPad X200-class"),
        "{doc}"
    );
    assert!(doc.contains("Bootloader expectation: GRUB first"), "{doc}");
    assert!(doc.contains("Recovery media: external USB"), "{doc}");
    assert!(
        doc.contains("Current maturity: operator checklist only"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-validate.sh"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_hardware_checklist_records_hardware_observations() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_HARDWARE_CHECKLIST.md")
        .expect("recovery usb hardware checklist");

    assert!(doc.contains("GRUB menu reachable"), "{doc}");
    assert!(doc.contains("USB boot option visible"), "{doc}");
    assert!(doc.contains("Keyboard works in boot menu"), "{doc}");
    assert!(doc.contains("Display is readable"), "{doc}");
    assert!(doc.contains("Emergency shell path is known"), "{doc}");
    assert!(doc.contains("Rollback metadata path is known"), "{doc}");
}

#[test]
fn recovery_usb_hardware_checklist_preserves_guardrails() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_HARDWARE_CHECKLIST.md")
        .expect("recovery usb hardware checklist");

    assert!(doc.contains("Do not write USB media"), "{doc}");
    assert!(doc.contains("Do not run dd automatically"), "{doc}");
    assert!(doc.contains("Do not partition disks"), "{doc}");
    assert!(doc.contains("Do not format disks"), "{doc}");
    assert!(doc.contains("Do not install GRUB automatically"), "{doc}");
    assert!(doc.contains("Do not edit grub.cfg automatically"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("Do not change boot order"), "{doc}");
    assert!(doc.contains("Do not flash firmware"), "{doc}");
    assert!(doc.contains("Do not store passwords"), "{doc}");
}

#[test]
fn recovery_usb_surfaces_link_hardware_checklist() {
    let index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let report = std::fs::read_to_string("base1/RECOVERY_USB_VALIDATION_REPORT.md")
        .expect("recovery usb validation report");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        index.contains("RECOVERY_USB_HARDWARE_CHECKLIST.md"),
        "{index}"
    );
    assert!(
        index.contains("Recovery USB hardware validation checklist"),
        "{index}"
    );
    assert!(
        report.contains("RECOVERY_USB_HARDWARE_CHECKLIST.md"),
        "{report}"
    );
    assert!(
        readme.contains("base1/RECOVERY_USB_HARDWARE_CHECKLIST.md"),
        "{readme}"
    );
}
