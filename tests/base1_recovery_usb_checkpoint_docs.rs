#[test]
fn recovery_usb_development_checkpoint_records_current_boundary() {
    let doc = std::fs::read_to_string("DEVELOPMENT_CHECKPOINT_BASE1_RECOVERY_USB_READONLY_V1.md")
        .expect("recovery usb development checkpoint");

    assert!(
        doc.contains("Base1 recovery USB read-only development checkpoint v1"),
        "{doc}"
    );
    assert!(
        doc.contains("Libreboot-backed ThinkPad X200-class"),
        "{doc}"
    );
    assert!(doc.contains("GRUB first"), "{doc}");
    assert!(
        doc.contains("Edge line after this checkpoint: v5.1.0"),
        "{doc}"
    );
    assert!(doc.contains("Stable promotion target: v5.0.0"), "{doc}");
    assert!(
        doc.contains("scripts/base1-recovery-usb-validate.sh"),
        "{doc}"
    );
    assert!(doc.contains("Do not write USB media"), "{doc}");
    assert!(
        doc.contains("does not claim bootable Base1 image readiness"),
        "{doc}"
    );
}

#[test]
fn release_metadata_points_to_v5_tracks() {
    let release = std::fs::read_to_string("RELEASE_v5.0.0.md").expect("release v5.0.0");

    assert!(release.contains("Phase1 v5.0.0 Stable"), "{release}");
    assert!(release.contains("Stable version: v5.0.0"), "{release}");
    assert!(release.contains("Previous stable: v4.4.0"), "{release}");
    assert!(release.contains("Next edge: v5.1.0"), "{release}");
    assert!(release.contains("Base1 recovery USB design"), "{release}");
}
