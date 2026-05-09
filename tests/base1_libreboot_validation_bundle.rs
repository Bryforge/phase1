use std::process::Command;

#[test]
fn libreboot_validation_bundle_runs_all_read_only_previews() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-validate.sh")
        .output()
        .expect("run libreboot validation bundle");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(text.contains("base1 libreboot validation bundle"), "{text}");
    assert!(text.contains("firmware : Libreboot expected"), "{text}");
    assert!(text.contains("hardware : X200-class expected"), "{text}");
    assert!(text.contains("boot     : GRUB first"), "{text}");
    assert!(text.contains("writes   : no"), "{text}");
    assert!(text.contains("base1 libreboot command index"), "{text}");
    assert!(
        text.contains("base1 libreboot operator checklist"),
        "{text}"
    );
    assert!(text.contains("base1 grub recovery dry-run"), "{text}");
    assert!(text.contains("base1 installer dry-run"), "{text}");
}

#[test]
fn libreboot_validation_bundle_help_is_read_only() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-validate.sh")
        .arg("--help")
        .output()
        .expect("run libreboot validation help");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{text}");
    assert!(text.contains("read-only"), "{text}");
    assert!(text.contains("without changing firmware"), "{text}");
    assert!(text.contains("host trust"), "{text}");
}

#[test]
fn libreboot_validation_bundle_avoids_mutating_tools_and_secret_terms() {
    let script = std::fs::read_to_string("scripts/base1-libreboot-validate.sh")
        .expect("libreboot validation bundle");

    let forbidden = [
        "flashrom",
        "grub-install",
        "grub-mkconfig",
        "update-grub",
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
        "password",
        "token",
        "private key",
    ];

    for token in forbidden {
        assert!(
            !script.contains(token),
            "forbidden token {token:?} found in script"
        );
    }
}
