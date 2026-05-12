use std::fs;
use std::process::Command;

#[test]
fn quality_docs_exist() {
    for path in [
        "docs/quality/QUALITY.md",
        "docs/quality/QUALITY_SCORECARD.md",
        "README.md",
        "SECURITY.md",
        "docs/security/SECURITY_REVIEW.md",
        "docs/releases/UPDATE_PROTOCOL.md",
    ] {
        assert!(
            fs::metadata(path).is_ok(),
            "missing required quality doc: {path}"
        );
    }
}

#[test]
fn quality_scripts_exist_and_are_valid_shell() {
    for path in ["scripts/quality-check.sh", "scripts/quality-score.sh"] {
        assert!(fs::metadata(path).is_ok(), "missing quality script: {path}");
        let status = Command::new("sh")
            .arg("-n")
            .arg(path)
            .status()
            .expect("run sh -n");
        assert!(
            status.success(),
            "quality script has shell syntax errors: {path}"
        );
    }
}

#[test]
fn quality_score_reports_score_and_rating() {
    let output = Command::new("sh")
        .arg("scripts/quality-score.sh")
        .output()
        .expect("run quality score");
    assert!(output.status.success(), "quality score script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Phase1 Quality Scorecard"));
    assert!(stdout.contains("score"));
    assert!(stdout.contains("rating"));
}

#[test]
fn quality_check_file_gate_passes() {
    let output = Command::new("sh")
        .arg("scripts/quality-check.sh")
        .arg("files")
        .output()
        .expect("run quality file gate");
    assert!(
        output.status.success(),
        "quality file gate failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn quality_workflow_exists() {
    assert!(
        fs::metadata(".github/workflows/quality.yml").is_ok(),
        "missing quality workflow"
    );
}
