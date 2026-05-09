#[test]
fn recovery_usb_target_command_index_lists_docs_and_commands() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md")
        .expect("recovery usb target command index");

    assert!(
        doc.contains("Base1 recovery USB target selection command index"),
        "{doc}"
    );
    assert!(doc.contains("RECOVERY_USB_TARGET_SELECTION.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_HARDWARE_SUMMARY.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_HARDWARE_CHECKLIST.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_COMMAND_INDEX.md"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_VALIDATION_REPORT.md"), "{doc}");
    assert!(
        doc.contains(
            "scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example"
        ),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-hardware-validate.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_target_command_index_preserves_identity_fields() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md")
        .expect("recovery usb target command index");

    assert!(doc.contains("Device path"), "{doc}");
    assert!(doc.contains("Device model/name"), "{doc}");
    assert!(doc.contains("Device size"), "{doc}");
    assert!(doc.contains("Removable status"), "{doc}");
    assert!(doc.contains("Current mount status"), "{doc}");
    assert!(doc.contains("Internal disk status"), "{doc}");
    assert!(doc.contains("Physical USB label status"), "{doc}");
    assert!(doc.contains("Confirmation status"), "{doc}");
}

#[test]
fn recovery_usb_target_command_index_preserves_guardrails() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md")
        .expect("recovery usb target command index");

    assert!(doc.contains("Require dry-run mode"), "{doc}");
    assert!(
        doc.contains("Require an explicit target device path"),
        "{doc}"
    );
    assert!(doc.contains("Report writes: no"), "{doc}");
    assert!(doc.contains("Do not write USB media"), "{doc}");
    assert!(doc.contains("Do not run dd automatically"), "{doc}");
    assert!(doc.contains("Do not partition disks"), "{doc}");
    assert!(doc.contains("Do not format disks"), "{doc}");
    assert!(doc.contains("Do not install GRUB automatically"), "{doc}");
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
fn recovery_usb_surfaces_link_target_command_index() {
    let target = std::fs::read_to_string("base1/RECOVERY_USB_TARGET_SELECTION.md")
        .expect("recovery usb target selection");
    let index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let summary = std::fs::read_to_string("base1/RECOVERY_USB_HARDWARE_SUMMARY.md")
        .expect("recovery usb hardware summary");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        target.contains("RECOVERY_USB_TARGET_COMMAND_INDEX.md"),
        "{target}"
    );
    assert!(
        index.contains("RECOVERY_USB_TARGET_COMMAND_INDEX.md"),
        "{index}"
    );
    assert!(
        summary.contains("RECOVERY_USB_TARGET_COMMAND_INDEX.md"),
        "{summary}"
    );
    assert!(
        readme.contains("base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md"),
        "{readme}"
    );
}
