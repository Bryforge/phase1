use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(
        text.contains(needle),
        "{path} should contain required public B6 X200 marker text: {needle}"
    );
}

#[test]
fn public_status_trail_links_b6_x200_checkpoint() {
    assert_contains("README.md", "docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md");
    assert_contains(
        "docs/status/PROJECT_STATUS.md",
        "docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md",
    );
    assert_contains(
        "docs/os/BOOT_READINESS_STATUS.md",
        "../checkpoints/B6_X200_MARKER_CHECKPOINT.md",
    );
    assert_contains(
        "docs/wiki/Current-Status.md",
        "../checkpoints/B6_X200_MARKER_CHECKPOINT.md",
    );
}

#[test]
fn public_status_trail_preserves_b6_marker_values() {
    for path in [
        "docs/status/PROJECT_STATUS.md",
        "docs/os/BOOT_READINESS_STATUS.md",
        "docs/wiki/Current-Status.md",
        "site/status.json",
    ] {
        assert_contains(path, "phase1_marker_seen");
        assert_contains(path, "not_claimed");
    }

    assert_contains(
        "docs/status/PROJECT_STATUS.md",
        "095786e808d3908d27c045f04f3de0b5cd538ab9",
    );
    assert_contains(
        "docs/status/PROJECT_STATUS.md",
        "688518c1437003c7b8325b1d5d479bc97f77c3404c8fd27dace6d823d406b79b",
    );
    assert_contains("site/status.json", "B6 X200 marker checkpoint");
}

#[test]
fn public_status_trail_keeps_non_claims_visible() {
    for path in [
        "README.md",
        "docs/status/PROJECT_STATUS.md",
        "docs/os/BOOT_READINESS_STATUS.md",
        "docs/wiki/Current-Status.md",
        "site/status.json",
    ] {
        assert_contains(path, "installer");
        assert_contains(path, "hardening");
        assert_contains(path, "daily-driver");
    }
}
