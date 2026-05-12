use std::process::Command;

const SCRIPT: &str = "scripts/base1-b3-openbsd-stage.sh";

#[test]
fn base1_b3_openbsd_stage_script_exists_and_has_valid_shell_syntax() {
    let metadata = std::fs::metadata(SCRIPT).expect("B3 OpenBSD stage script exists");
    assert!(
        metadata.len() > 0,
        "B3 OpenBSD stage script should not be empty"
    );

    let output = Command::new("sh")
        .arg("-n")
        .arg(SCRIPT)
        .output()
        .expect("run sh -n on B3 OpenBSD stage script");

    assert!(
        output.status.success(),
        "script syntax should pass sh -n\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn base1_b3_openbsd_stage_help_documents_stage_model() {
    let output = Command::new("sh")
        .arg(SCRIPT)
        .arg("--help")
        .output()
        .expect("run B3 OpenBSD stage help");

    assert!(output.status.success(), "--help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);

    for text in [
        "base1 B3 OpenBSD stage",
        "--iso <path>",
        "--img <path>",
        "--prepare",
        "--dry-run",
        "--check",
        "--timeout <seconds>",
        "--expect <text>",
        "default: OpenBSD",
        "OpenBSD boot artifact as a B3 staging point",
        "separate from the GNU/Linux kernel/initrd handoff",
        "<out>/openbsd-stage.env",
        "<out>/reports/openbsd-qemu-boot.log",
        "<out>/reports/openbsd-qemu-summary.env",
        "emulator-only OpenBSD staging evidence",
        "does not make Base1 an",
        "validate physical hardware",
        "prove daily-driver readiness",
    ] {
        assert!(stdout.contains(text), "missing help text {text}: {stdout}");
    }
}

#[test]
fn base1_b3_openbsd_stage_requires_local_artifact_and_rejects_unknown_arguments() {
    let no_args = Command::new("sh")
        .arg(SCRIPT)
        .output()
        .expect("run B3 OpenBSD stage without artifact");
    assert!(
        !no_args.status.success(),
        "script should require --iso or --img"
    );
    let stderr = String::from_utf8_lossy(&no_args.stderr);
    assert!(
        stderr.contains("provide --iso <path> or --img <path> for the OpenBSD stage"),
        "stderr was: {stderr}"
    );

    let unknown = Command::new("sh")
        .arg(SCRIPT)
        .arg("--unknown")
        .output()
        .expect("run B3 OpenBSD stage with unknown option");
    assert!(
        !unknown.status.success(),
        "script should reject unknown args"
    );
    let unknown_stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        unknown_stderr.contains("unknown option: --unknown"),
        "stderr was: {unknown_stderr}"
    );
}

#[test]
fn base1_b3_openbsd_stage_uses_expected_qemu_shapes() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 OpenBSD stage source");

    for text in [
        "OPENBSD_ISO=${BASE1_OPENBSD_ISO:-}",
        "OPENBSD_IMG=${BASE1_OPENBSD_IMG:-}",
        "EXPECT=${BASE1_B3_OPENBSD_MARKER:-OpenBSD}",
        "build/base1-b3-openbsd-stage",
        "openbsd-stage.env",
        "reports/openbsd-qemu-boot.log",
        "reports/openbsd-qemu-summary.env",
        "-cdrom $ARTIFACT -boot d",
        "-drive file=$ARTIFACT,format=raw,if=virtio -boot c",
        "-display none -serial file:$LOG",
        "grep -F \"$EXPECT\" \"$LOG\"",
        "BASE1_B3_OPENBSD_RESULT=$result",
        "BASE1_B3_OPENBSD_CLAIM=not_claimed",
    ] {
        assert!(
            script.contains(text),
            "missing OpenBSD QEMU/evidence text {text}: {script}"
        );
    }
}

#[test]
fn base1_b3_openbsd_stage_preserves_local_only_non_claims() {
    let script = std::fs::read_to_string(SCRIPT).expect("B3 OpenBSD stage source");

    for text in [
        "require_build_out_dir",
        "output directory must be under build/",
        "does not download OpenBSD",
        "install Base1",
        "modify host boot",
        "settings",
        "partition disks",
        "claim hardware readiness",
        "no installer",
        "no hardware validation",
        "no hardening proof",
        "no daily-driver claim",
    ] {
        assert!(
            script.contains(text),
            "missing OpenBSD non-claim text {text}: {script}"
        );
    }
}
