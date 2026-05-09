use std::fs;
use std::process::Command;

#[test]
fn removed_personal_reference_does_not_return() {
    let output = Command::new("git")
        .args(["ls-files"])
        .output()
        .expect("git ls-files works");

    assert!(output.status.success());

    let forbidden = ["je", "sse"].concat().to_lowercase();
    let files = String::from_utf8_lossy(&output.stdout);

    for path in files.lines() {
        if path == "tests/no_personal_references.rs" {
            continue;
        }

        let Ok(bytes) = fs::read(path) else {
            continue;
        };

        let text = String::from_utf8_lossy(&bytes).to_lowercase();
        assert!(
            !text.contains(&forbidden),
            "removed personal reference remains in {path}"
        );
    }
}
