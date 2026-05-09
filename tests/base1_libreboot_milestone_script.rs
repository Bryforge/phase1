use std::process::Command;

#[test]
fn libreboot_milestone_script_prints_checkpoint_summary() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-milestone.sh")
        .output()
        .expect("run libreboot milestone script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "{text}");
    assert!(
        text.contains("base1 libreboot milestone checkpoint"),
        "{text}"
    );
    assert!(
        text.contains("firmware profile : Libreboot documented"),
        "{text}"
    );
    assert!(
        text.contains("hardware profile : X200-class documented"),
        "{text}"
    );
    assert!(
        text.contains("bootloader       : GRUB first documented"),
        "{text}"
    );
    assert!(
        text.contains("maturity         : documentation and read-only scripts"),
        "{text}"
    );
    assert!(text.contains("writes           : no"), "{text}");
    assert!(
        text.contains("trust            : no host trust escalation"),
        "{text}"
    );
}

#[test]
fn libreboot_milestone_script_lists_surfaces_and_non_claims() {
    let output = Command::new("sh")
        .arg("scripts/base1-libreboot-milestone.sh")
        .output()
        .expect("run libreboot milestone script");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(text.contains("base1/LIBREBOOT_MILESTONE.md"), "{text}");
    assert!(
        text.contains("sh scripts/base1-libreboot-docs.sh"),
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
    assert!(text.contains("bootable Base1 image readiness"), "{text}");
    assert!(text.contains("daily-driver readiness"), "{text}");
    assert!(
        text.contains("rollback validation on real hardware"),
        "{text}"
    );
}

#[test]
fn libreboot_milestone_script_avoids_mutating_tools_and_sensitive_terms() {
    let script = std::fs::read_to_string("scripts/base1-libreboot-milestone.sh")
        .expect("libreboot milestone script");

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
