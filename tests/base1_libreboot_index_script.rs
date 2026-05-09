use std::process::Command;

#[test]
fn libreboot_index_script_prints_docs_and_scripts() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-index.sh")
        .output()
        .expect("run libreboot index script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(text.contains("base1 libreboot command index"), "{text}");
    assert!(text.contains("firmware : Libreboot"), "{text}");
    assert!(text.contains("hardware : X200-class"), "{text}");
    assert!(text.contains("boot     : GRUB first"), "{text}");
    assert!(text.contains("writes   : no"), "{text}");
    assert!(text.contains("base1/LIBREBOOT_PROFILE.md"), "{text}");
    assert!(text.contains("base1/LIBREBOOT_COMMAND_INDEX.md"), "{text}");
    assert!(
        text.contains("sh scripts/base1-libreboot-checklist.sh"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-grub-recovery-dry-run.sh --dry-run"),
        "{text}"
    );
}

#[test]
fn libreboot_index_script_help_is_read_only() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-index.sh")
        .arg("--help")
        .output()
        .expect("run libreboot index help");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{text}");
    assert!(text.contains("read-only"), "{text}");
    assert!(text.contains("does not change firmware"), "{text}");
    assert!(text.contains("host trust"), "{text}");
}

#[test]
fn libreboot_index_script_avoids_mutating_tools_and_secret_terms() {
    let script = std::fs::read_to_string("scripts/base1-libreboot-index.sh")
        .expect("libreboot index script");

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
