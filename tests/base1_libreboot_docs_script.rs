use std::process::Command;

#[test]
fn libreboot_docs_script_prints_docs_path() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-docs.sh")
        .output()
        .expect("run libreboot docs script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(text.contains("base1 libreboot docs"), "{text}");
    assert!(text.contains("firmware : Libreboot"), "{text}");
    assert!(text.contains("hardware : X200-class"), "{text}");
    assert!(text.contains("boot     : GRUB first"), "{text}");
    assert!(text.contains("writes   : no"), "{text}");
    assert!(text.contains("LIBREBOOT_DOCS_SUMMARY.md"), "{text}");
    assert!(text.contains("LIBREBOOT_QUICKSTART.md"), "{text}");
    assert!(text.contains("LIBREBOOT_VALIDATION_REPORT.md"), "{text}");
}

#[test]
fn libreboot_docs_script_lists_first_commands() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-docs.sh")
        .output()
        .expect("run libreboot docs script");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(
        text.contains("sh scripts/base1-libreboot-index.sh"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-libreboot-checklist.sh"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-libreboot-preflight.sh"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-grub-recovery-dry-run.sh --dry-run"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-libreboot-validate.sh"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-libreboot-report.sh"),
        "{text}"
    );
}

#[test]
fn libreboot_docs_script_avoids_mutating_tools_and_secret_terms() {
    let script =
        std::fs::read_to_string("scripts/base1-libreboot-docs.sh").expect("libreboot docs script");

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
