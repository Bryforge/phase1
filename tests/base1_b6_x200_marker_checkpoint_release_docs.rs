use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn b6_x200_marker_release_note_exists_and_links_checkpoint() {
    let path = "docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md";

    assert_contains(path, "Base1 B6 X200 marker checkpoint v1");
    assert_contains(path, "../../checkpoints/B6_X200_MARKER_CHECKPOINT.md");
    assert_contains(path, "d4cd1e13d429662f6713466f57a41233d8238416");
    assert_contains(path, "095786e808d3908d27c045f04f3de0b5cd538ab9");
    assert_contains(path, "phase1_marker_seen");
    assert_contains(path, "not_claimed");
    assert_contains(
        path,
        "688518c1437003c7b8325b1d5d479bc97f77c3404c8fd27dace6d823d406b79b",
    );
}

#[test]
fn b6_x200_marker_release_note_is_indexed() {
    assert_contains(
        "docs/base1/releases/README.md",
        "RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md",
    );

    assert_contains(
        "docs/base1/INVENTORY.md",
        "RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md",
    );
}

#[test]
fn b6_x200_marker_release_note_preserves_non_claims() {
    let path = "docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md";

    assert_contains(path, "installer readiness");
    assert_contains(path, "recovery completion");
    assert_contains(path, "hardening");
    assert_contains(path, "release-candidate readiness");
    assert_contains(path, "daily-driver readiness");
    assert_contains(path, "broad hardware validation");
}
