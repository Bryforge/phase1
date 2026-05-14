#[test]
fn system_diagnostics_contract_exists() {
    let doc = std::fs::read_to_string("docs/diagnostics/SYSTEM_DIAGNOSTICS.md")
        .expect("system diagnostics contract");

    for text in [
        "Phase1 system diagnostics",
        "local, sanitized diagnostics report",
        "Non-claim: this is not telemetry",
        "sh scripts/phase1-system-diagnostics.sh --quick",
        "build/diagnostics/latest.md",
    ] {
        assert!(doc.contains(text), "missing contract text {text}: {doc}");
    }
}

#[test]
fn system_diagnostics_script_is_local_only_and_sanitized() {
    let script = std::fs::read_to_string("scripts/phase1-system-diagnostics.sh")
        .expect("system diagnostics script");

    for text in [
        "local-only",
        "upload: no",
        "git_mutation: no",
        "secrets_collection: no",
        "sanitize_stream",
        "build/diagnostics",
        "does not upload, add, commit, push",
        "--repo-copy",
    ] {
        assert!(
            script.contains(text),
            "missing safety text {text}: {script}"
        );
    }

    for forbidden in [
        "git push origin",
        "git add .",
        "git add -A",
        "git commit -m",
        "curl ",
        "wget ",
        "scp ",
        "rsync ",
        "dd if=",
        "dd of=",
        "mkfs",
        "parted",
        "fdisk",
        "flashrom",
    ] {
        assert!(
            !script.contains(forbidden),
            "diagnostics script must not contain forbidden operation {forbidden}: {script}"
        );
    }
}

#[test]
fn system_diagnostics_script_has_valid_shell_syntax() {
    let status = std::process::Command::new("sh")
        .arg("-n")
        .arg("scripts/phase1-system-diagnostics.sh")
        .status()
        .expect("run sh -n");

    assert!(
        status.success(),
        "system diagnostics script shell syntax failed"
    );
}

#[test]
fn system_diagnostics_help_reports_modes_and_boundaries() {
    let output = std::process::Command::new("sh")
        .arg("scripts/phase1-system-diagnostics.sh")
        .arg("--help")
        .output()
        .expect("run diagnostics help");

    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(output.status.success(), "diagnostics help failed: {text}");

    for row in [
        "--quick",
        "--full",
        "--no-tests",
        "--repo-copy",
        "no upload",
        "no git add/commit/push",
        "no secret collection",
    ] {
        assert!(text.contains(row), "missing help row {row}: {text}");
    }
}
