#[test]
fn recovery_usb_validation_report_template_defines_safe_target_summary() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_VALIDATION_REPORT.md")
        .expect("recovery USB validation report");

    assert!(
        doc.contains("Base1 recovery USB validation report"),
        "{doc}"
    );
    assert!(
        doc.contains("Recovery media: external USB planned"),
        "{doc}"
    );
    assert!(
        doc.contains("Firmware profile: Libreboot expected"),
        "{doc}"
    );
    assert!(
        doc.contains("Hardware profile: X200-class expected"),
        "{doc}"
    );
    assert!(doc.contains("Bootloader expectation: GRUB first"), "{doc}");
    assert!(doc.contains("Emergency shell: required"), "{doc}");
    assert!(
        doc.contains("Phase1 state path: /state/phase1 preview"),
        "{doc}"
    );
    assert!(
        doc.contains("Rollback metadata path: /recovery preview"),
        "{doc}"
    );
    assert!(
        doc.contains("sh scripts/base1-recovery-usb-validate.sh"),
        "{doc}"
    );
}

#[test]
fn recovery_usb_validation_report_preserves_guardrails() {
    let doc = std::fs::read_to_string("base1/RECOVERY_USB_VALIDATION_REPORT.md")
        .expect("recovery USB validation report");

    assert!(doc.contains("Do not write USB media"), "{doc}");
    assert!(doc.contains("Do not run dd automatically"), "{doc}");
    assert!(doc.contains("Do not partition disks"), "{doc}");
    assert!(doc.contains("Do not format disks"), "{doc}");
    assert!(doc.contains("Do not install GRUB automatically"), "{doc}");
    assert!(doc.contains("Do not write to /boot"), "{doc}");
    assert!(doc.contains("Do not flash firmware"), "{doc}");
    assert!(doc.contains("Do not store secrets"), "{doc}");
}

#[test]
fn recovery_usb_command_index_links_validation_report() {
    let index = std::fs::read_to_string("base1/RECOVERY_USB_COMMAND_INDEX.md")
        .expect("recovery USB command index");

    assert!(
        index.contains("RECOVERY_USB_VALIDATION_REPORT.md"),
        "{index}"
    );
    assert!(index.contains("Recovery USB validation report"), "{index}");
}

#[test]
fn readme_links_recovery_usb_validation_report() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("base1/RECOVERY_USB_VALIDATION_REPORT.md"),
        "{readme}"
    );
    assert!(
        readme.contains("Recovery USB validation report"),
        "{readme}"
    );
}
