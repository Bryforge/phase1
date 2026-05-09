#[test]
fn recovery_usb_command_index_lists_docs_and_commands() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");

    assert!(doc.contains("Base1 recovery USB command index"), "{doc}");
    assert!(doc.contains("RECOVERY_USB_DESIGN.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_MILESTONE.md"), "{doc}");
    assert!(doc.contains("LIBREBOOT_VALIDATION_REPORT.md"), "{doc}");
    assert!(
        doc.contains("RELEASE_BASE1_LIBREBOOT_READONLY_V1.md"),
        "{doc}"
    );
    assert!(
        doc.contains("RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example"),
        "{doc}"
    );
    assert!(doc.contains("scripts/base1-libreboot-validate.sh"), "{doc}");
    assert!(
        doc.contains("scripts/base1-grub-recovery-dry-run.sh --dry-run"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_command_index_preserves_guardrails() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");

    assert!(doc.contains("Require dry-run mode"), "{doc}");
    assert!(doc.contains("Make the target device visible"), "{doc}");
    assert!(doc.contains("Report writes: no"), "{doc}");
    assert!(doc.contains("Do not write USB media"), "{doc}");
    assert!(doc.contains("Do not run dd automatically"), "{doc}");
    assert!(doc.contains("Do not partition disks"), "{doc}");
    assert!(doc.contains("Do not format disks"), "{doc}");
    assert!(doc.contains("Do not install GRUB automatically"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("Do not store passwords"), "{doc}");
}

#[test]
fn recovery_usb_design_and_readme_link_command_index() {
    let design =
        std::fs::read_to_string("base1/RECOVERY_USB_DESIGN.md").expect("recovery usb design");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(design.contains("RECOVERY_USB_COMMAND_INDEX.md"), "{design}");
    assert!(design.contains("Recovery USB command index"), "{design}");
    assert!(
        readme.contains("base1/RECOVERY_USB_COMMAND_INDEX.md"),
        "{readme}"
    );
    assert!(readme.contains("Recovery USB command index"), "{readme}");
}
