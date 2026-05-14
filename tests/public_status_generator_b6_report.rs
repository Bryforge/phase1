use std::fs;
use std::process::Command;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn public_status_generator_preserves_b6_report_state() {
    let output = Command::new("python3")
        .arg("scripts/update-public-status.py")
        .output()
        .expect("failed to run public status generator");

    assert!(
        output.status.success(),
        "generator failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert_contains(
        "site/status.json",
        "\"overall_estimated_completion_percent\": 66",
    );
    assert_contains("site/status.json", "\"estimated_completion_percent\": 40");
    assert_contains("site/status.json", "\"estimated_completion_percent\": 44");
    assert_contains("site/status.json", "\"estimated_completion_percent\": 88");
    assert_contains("site/status.json", "current_public_report");
    assert_contains("site/status.json", "phase1_marker_seen");
    assert_contains("site/status.json", "not_claimed");
    assert_contains(
        "site/status.json",
        "RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md",
    );

    assert_contains(
        "docs/status/PROJECT_STATUS.md",
        "Overall estimated roadmap completion: **66%**.",
    );
    assert_contains(
        "docs/status/PROJECT_STATUS.md",
        "B6 X200 marker chain is now published",
    );
}
