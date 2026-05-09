use std::process::Command;

fn run_script(args: &[&str]) -> (bool, String) {
    let output = Command::new("sh")
        .arg("scripts/base1-recovery-dry-run.sh")
        .args(args)
        .output()
        .expect("run base1 recovery dry-run script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), text)
}

#[test]
fn recovery_dry_run_refuses_without_dry_run_flag() {
    let (ok, text) = run_script(&[]);

    assert!(!ok, "{text}");
    assert!(text.contains("refusing: --dry-run is required"), "{text}");
}

#[test]
fn recovery_dry_run_reports_preview_without_writes() {
    let (ok, text) = run_script(&["--dry-run"]);

    assert!(ok, "{text}");
    assert!(text.contains("base1 recovery dry-run"), "{text}");
    assert!(text.contains("writes      : no"), "{text}");
    assert!(
        text.contains("boot        : recovery preview only"),
        "{text}"
    );
    assert!(text.contains("auto-launch : no change"), "{text}");
    assert!(text.contains("emergency fallback preview"), "{text}");
    assert!(text.contains("metadata preview only"), "{text}");
}

#[test]
fn recovery_dry_run_accepts_explicit_target_preview() {
    let (ok, text) = run_script(&["--dry-run", "--target", "/dev/example"]);

    assert!(ok, "{text}");
    assert!(text.contains("target      : /dev/example"), "{text}");
    assert!(
        text.contains("state       : /state/phase1 export preview only"),
        "{text}"
    );
    assert!(
        text.contains("trust       : no host trust escalation"),
        "{text}"
    );
}

#[test]
fn recovery_dry_run_script_avoids_destructive_tools() {
    let script = std::fs::read_to_string("scripts/base1-recovery-dry-run.sh")
        .expect("base1 recovery dry-run script");

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
        "rm -rf",
    ];

    for token in forbidden {
        assert!(
            !script.contains(token),
            "forbidden token {token:?} found in script"
        );
    }
}
