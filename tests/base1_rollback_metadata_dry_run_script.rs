use std::process::Command;

fn run_script(args: &[&str]) -> (bool, String) {
    let output = Command::new("sh")
        .arg("scripts/base1-rollback-metadata-dry-run.sh")
        .args(args)
        .output()
        .expect("run base1 rollback metadata dry-run script");

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), text)
}

#[test]
fn rollback_metadata_dry_run_refuses_without_dry_run_flag() {
    let (ok, text) = run_script(&[]);

    assert!(!ok, "{text}");
    assert!(text.contains("refusing: --dry-run is required"), "{text}");
}

#[test]
fn rollback_metadata_dry_run_reports_preview_without_writes() {
    let (ok, text) = run_script(&["--dry-run"]);

    assert!(ok, "{text}");
    assert!(text.contains("base1 rollback metadata dry-run"), "{text}");
    assert!(text.contains("writes             : no"), "{text}");
    assert!(
        text.contains("base1_version      : foundation preview"),
        "{text}"
    );
    assert!(
        text.contains("phase1_version     : v5.0.0 preview"),
        "{text}"
    );
    assert!(
        text.contains("stable_version     : v4.4.0 preview"),
        "{text}"
    );
    assert!(
        text.contains("previous_stable    : v4.3.0 preview"),
        "{text}"
    );
}

#[test]
fn rollback_metadata_dry_run_accepts_target_preview() {
    let (ok, text) = run_script(&["--dry-run", "--target", "/dev/example"]);

    assert!(ok, "{text}");
    assert!(text.contains("target             : /dev/example"), "{text}");
    assert!(text.contains("operator_confirmed : no"), "{text}");
    assert!(
        text.contains("trust              : no host trust escalation"),
        "{text}"
    );
}

#[test]
fn rollback_metadata_dry_run_script_avoids_destructive_tools_and_secrets() {
    let script = std::fs::read_to_string("scripts/base1-rollback-metadata-dry-run.sh")
        .expect("base1 rollback metadata dry-run script");

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
