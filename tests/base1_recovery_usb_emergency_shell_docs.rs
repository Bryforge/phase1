#[test]
fn recovery_usb_emergency_shell_defines_read_only_behavior_path() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL.md")
        .expect("recovery usb emergency shell design");

    assert!(
        doc.contains("Base1 recovery USB emergency shell behavior design"),
        "{doc}"
    );
    assert!(doc.contains("advisory and read-only"), "{doc}");
    assert!(doc.contains("does not write USB media"), "{doc}");
    assert!(doc.contains("does not launch a shell"), "{doc}");
    assert!(doc.contains("Emergency shell entry path"), "{doc}");
    assert!(doc.contains("Keyboard availability"), "{doc}");
    assert!(doc.contains("Display readability"), "{doc}");
    assert!(doc.contains("Root/admin boundary"), "{doc}");
    assert!(doc.contains("Rollback metadata path"), "{doc}");
}

#[test]
fn recovery_usb_emergency_shell_preserves_required_behavior_and_guardrails() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL.md")
        .expect("recovery usb emergency shell design");

    assert!(
        doc.contains("must not remove emergency shell access"),
        "{doc}"
    );
    assert!(
        doc.contains("Do not launch privileged shells automatically"),
        "{doc}"
    );
    assert!(
        doc.contains("Do not remove emergency shell access"),
        "{doc}"
    );
    assert!(doc.contains("Do not hide root/admin boundaries"), "{doc}");
    assert!(doc.contains("Do not write USB media"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("Do not store passwords"), "{doc}");
}

#[test]
fn recovery_usb_emergency_shell_links_existing_surfaces() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_EMERGENCY_SHELL.md")
        .expect("recovery usb emergency shell design");

    assert!(
        doc.contains("sh scripts/base1-recovery-usb-image-summary.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-image-validate.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-target-summary.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-recovery-dry-run.sh --dry-run"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_surfaces_link_emergency_shell_design() {
    let image = std::fs::read_to_string("base1/RECOVERY_USB_IMAGE_SUMMARY.md")
        .expect("recovery usb image summary");
    let command_index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery usb command index");
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(image.contains("RECOVERY_USB_EMERGENCY_SHELL.md"), "{image}");
    assert!(
        command_index.contains("RECOVERY_USB_EMERGENCY_SHELL.md"),
        "{command_index}"
    );
    assert!(
        readme.contains("base1/RECOVERY_USB_EMERGENCY_SHELL.md"),
        "{readme}"
    );
}
