#[test]
fn recovery_usb_design_doc_exists_and_defines_read_only_contract() {
    let doc =
        std::fs::read_to_string("base1/RECOVERY_USB_DESIGN.md").expect("recovery usb design doc");

    assert!(doc.contains("Base1 recovery USB design"), "{doc}");
    assert!(doc.contains("documentation-only and read-only"), "{doc}");
    assert!(
        doc.contains("Libreboot-backed ThinkPad X200-class"),
        "{doc}"
    );
    assert!(doc.contains("GRUB-first boot path"), "{doc}");
    assert!(doc.contains("External USB recovery media"), "{doc}");
    assert!(doc.contains("Emergency shell path"), "{doc}");
    assert!(doc.contains("Rollback metadata path"), "{doc}");
}

#[test]
fn recovery_usb_design_preserves_safety_guardrails() {
    let doc =
        std::fs::read_to_string("base1/RECOVERY_USB_DESIGN.md").expect("recovery usb design doc");

    assert!(doc.contains("Do not write USB media"), "{doc}");
    assert!(doc.contains("Do not run dd automatically"), "{doc}");
    assert!(doc.contains("Do not partition disks"), "{doc}");
    assert!(doc.contains("Do not format disks"), "{doc}");
    assert!(doc.contains("Do not install GRUB automatically"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("Do not flash firmware"), "{doc}");
    assert!(doc.contains("Do not store passwords"), "{doc}");
}

#[test]
fn readme_links_recovery_usb_design() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(readme.contains("base1/RECOVERY_USB_DESIGN.md"), "{readme}");
    assert!(readme.contains("Recovery USB design"), "{readme}");
}

#[test]
fn os_roadmap_links_recovery_usb_design() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("os roadmap");

    assert!(roadmap.contains("RECOVERY_USB_DESIGN.md"), "{roadmap}");
    assert!(roadmap.contains("recovery USB design"), "{roadmap}");
}
