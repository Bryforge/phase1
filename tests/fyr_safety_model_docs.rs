use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_safety_model_preserves_no_host_boundary() {
    let path = "docs/fyr/SAFETY_MODEL.md";

    for boundary in [
        "VFS-only behavior by default.",
        "No host shell.",
        "No network.",
        "No host compiler.",
        "No Cargo invocation from Fyr commands.",
        "Deterministic check/build/test/run output.",
        "host    : none",
    ] {
        assert_contains(path, boundary);
    }
}

#[test]
fn fyr_safety_model_keeps_non_claims_visible() {
    let path = "docs/fyr/SAFETY_MODEL.md";

    assert_contains(path, "does not make Fyr production-ready");
    assert_contains(path, "hardened");
    assert_contains(path, "audited");
    assert_contains(path, "Do not use this wording yet:");
    assert_contains(path, "Fyr is a hardened sandbox.");
}

#[test]
fn fyr_safety_model_lists_f4_promotion_evidence() {
    let path = "docs/fyr/SAFETY_MODEL.md";

    for gate in [
        "VFS read/write happy path tests.",
        "Guard failure tests.",
        "Bounded runtime tests.",
        "Error-redaction tests.",
        "Deterministic output tests.",
    ] {
        assert_contains(path, gate);
    }
}

#[test]
fn fyr_toolchain_links_to_safety_model() {
    assert_contains(
        "docs/fyr/TOOLCHAIN.md",
        "[`SAFETY_MODEL.md`](SAFETY_MODEL.md)",
    );
    assert_contains("docs/fyr/TOOLCHAIN.md", "No host shell.");
    assert_contains("docs/fyr/TOOLCHAIN.md", "No network.");
    assert_contains("docs/fyr/TOOLCHAIN.md", "No host compiler.");
}
