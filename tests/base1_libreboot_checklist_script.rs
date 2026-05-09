use std::process::Command;

#[test]
fn libreboot_checklist_script_reports_operator_summary() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-checklist.sh")
        .output()
        .expect("run libreboot checklist script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(
        text.contains("base1 libreboot operator checklist"),
        "{text}"
    );
    assert!(text.contains("firmware       : Libreboot"), "{text}");
    assert!(text.contains("hardware       : X200-class"), "{text}");
    assert!(text.contains("bootloader     : GRUB first"), "{text}");
    assert!(text.contains("secureboot     : not assumed"), "{text}");
    assert!(text.contains("tpm            : not assumed"), "{text}");
    assert!(
        text.contains("recovery_media : external USB recommended"),
        "{text}"
    );
    assert!(
        text.contains("emergency      : shell access required"),
        "{text}"
    );
    assert!(text.contains("writes         : no"), "{text}");
}

#[test]
fn libreboot_checklist_script_lists_required_dry_runs() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-checklist.sh")
        .output()
        .expect("run libreboot checklist script");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(
        text.contains("sh scripts/base1-libreboot-preflight.sh"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-grub-recovery-dry-run.sh --dry-run"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-recovery-dry-run.sh --dry-run"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-rollback-metadata-dry-run.sh --dry-run"),
        "{text}"
    );
    assert!(
        text.contains("sh scripts/base1-install-dry-run.sh --dry-run --target /dev/example"),
        "{text}"
    );
}

#[test]
fn libreboot_checklist_script_avoids_mutating_tools_and_secret_terms() {
    let script = std::fs::read_to_string("scripts/base1-libreboot-checklist.sh")
        .expect("libreboot checklist script");

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
