use std::process::Command;

fn run_script(args: &[&str]) -> (bool, String) {
    let output = Command::new("sh")
        .arg("scripts/base1-install-dry-run.sh")
        .args(args)
        .output()
        .expect("run base1 installer dry-run script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), text)
}

#[test]
fn installer_dry_run_refuses_without_dry_run_flag() {
    let (ok, text) = run_script(&["--target", "/dev/example"]);

    assert!(!ok, "{text}");
    assert!(text.contains("refusing: --dry-run is required"), "{text}");
}

#[test]
fn installer_dry_run_requires_target() {
    let (ok, text) = run_script(&["--dry-run"]);

    assert!(!ok, "{text}");
    assert!(text.contains("--target <disk> is required"), "{text}");
}

#[test]
fn installer_dry_run_reports_preview_without_writes() {
    let (ok, text) = run_script(&["--dry-run", "--target", "/dev/example"]);

    assert!(ok, "{text}");
    assert!(text.contains("base1 installer dry-run"), "{text}");
    assert!(text.contains("target   : /dev/example"), "{text}");
    assert!(text.contains("writes   : no"), "{text}");
    assert!(text.contains("boot     : preview only"), "{text}");
    assert!(
        text.contains("base1    : read-only layer preview"),
        "{text}"
    );
    assert!(
        text.contains("state    : writable phase1 state preview"),
        "{text}"
    );
    assert!(
        text.contains("recovery : emergency shell preview"),
        "{text}"
    );
    assert!(text.contains("rollback : metadata preview only"), "{text}");
    assert!(
        text.contains("trust    : no host trust escalation"),
        "{text}"
    );
}

#[test]
fn installer_dry_run_script_avoids_destructive_tools() {
    let script =
        std::fs::read_to_string("scripts/base1-install-dry-run.sh").expect("base1 dry-run script");

    let forbidden = [
        "mkfs",
        "fdisk",
        "parted",
        "sfdisk",
        "diskutil erase",
        "dd if=",
        "mount ",
        "umount ",
        "grub-install",
        "bootctl install",
    ];

    for token in forbidden {
        assert!(
            !script.contains(token),
            "forbidden token {token:?} found in script"
        );
    }
}
