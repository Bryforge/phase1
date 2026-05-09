use std::process::Command;

fn run_script(args: &[&str]) -> (bool, String) {
    let output = Command::new("sh")
        .arg("scripts/base1-grub-recovery-dry-run.sh")
        .args(args)
        .output()
        .expect("run grub recovery dry-run script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), text)
}

#[test]
fn grub_recovery_dry_run_requires_dry_run_flag() {
    let (ok, text) = run_script(&[]);

    assert!(!ok, "{text}");
    assert!(text.contains("refusing: --dry-run is required"), "{text}");
}

#[test]
fn grub_recovery_dry_run_reports_read_only_recovery_plan() {
    let (ok, text) = run_script(&["--dry-run"]);

    assert!(ok, "{text}");
    assert!(text.contains("base1 grub recovery dry-run"), "{text}");
    assert_eq!(
        text.matches("base1 grub recovery dry-run").count(),
        1,
        "{text}"
    );
    assert!(text.contains("firmware    : Libreboot expected"), "{text}");
    assert!(text.contains("hardware    : X200-class expected"), "{text}");
    assert!(text.contains("bootloader  : GRUB first"), "{text}");
    assert!(text.contains("writes      : no"), "{text}");
    assert!(text.contains("boot_order  : no change"), "{text}");
    assert!(
        text.contains("boot_config : grub.cfg preview only"),
        "{text}"
    );
    assert!(text.contains("boot_path   : /boot preview only"), "{text}");
    assert!(
        text.contains("emergency   : shell access required"),
        "{text}"
    );
}

#[test]
fn grub_recovery_dry_run_script_avoids_mutating_tools() {
    let script = std::fs::read_to_string("scripts/base1-grub-recovery-dry-run.sh")
        .expect("grub recovery dry-run script");

    let forbidden = [
        "grub-install",
        "grub-mkconfig",
        "update-grub",
        "bootctl install",
        "efibootmgr",
        "flashrom",
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
fn grub_recovery_dry_run_help_is_read_only() {
    let output = Command::new("sh")
        .arg("scripts/base1-grub-recovery-dry-run.sh")
        .arg("--help")
        .output()
        .expect("run grub recovery help");

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{text}");
    assert!(text.contains("read-only"), "{text}");
    assert!(text.contains("does not edit GRUB config"), "{text}");
    assert!(text.contains("write to /boot"), "{text}");
}
