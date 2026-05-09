use std::process::Command;

#[test]
fn libreboot_preflight_script_reports_read_only_grub_first_notes() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-preflight.sh")
        .output()
        .expect("run libreboot preflight script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(text.contains("base1 libreboot preflight"), "{text}");
    assert!(text.contains("firmware  : Libreboot expected"), "{text}");
    assert!(text.contains("hardware  : X200-class expected"), "{text}");
    assert!(text.contains("bootloader: GRUB first"), "{text}");
    assert!(text.contains("secureboot: not required"), "{text}");
    assert!(text.contains("tpm       : not required"), "{text}");
    assert!(text.contains("writes    : no"), "{text}");
    assert!(
        text.contains("trust     : no host trust escalation"),
        "{text}"
    );
}

#[test]
fn libreboot_preflight_script_avoids_mutating_tools() {
    let script = std::fs::read_to_string("scripts/base1-libreboot-preflight.sh")
        .expect("libreboot preflight script");

    let forbidden = [
        "flashrom",
        "grub-install",
        "bootctl install",
        "efibootmgr",
        "mkfs",
        "fdisk",
        "parted",
        "sfdisk",
        "diskutil erase",
        "dd if=",
        "mount ",
        "umount ",
        "rm -rf",
    ];

    for token in forbidden {
        assert!(
            !script.contains(token),
            "forbidden token {token:?} found in script"
        );
    }
}

#[test]
fn libreboot_preflight_script_help_is_read_only() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-preflight.sh")
        .arg("--help")
        .output()
        .expect("run libreboot preflight help");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{text}");
    assert!(text.contains("read-only"), "{text}");
    assert!(text.contains("does not flash firmware"), "{text}");
    assert!(text.contains("install GRUB"), "{text}");
}
