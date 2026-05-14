use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_f6_stdlib_contract_lists_planned_modules() {
    let path = "docs/fyr/STDLIB.md";

    for module in [
        "`vfs`",
        "`text`",
        "`json-lite`",
        "`audit`",
        "`package`",
        "`doc`",
    ] {
        assert_contains(path, module);
    }

    assert_contains(path, "not implemented as a complete standard library yet");
    assert_contains(path, "smoke tests");
    assert_contains(path, "failure-mode tests");
}

#[test]
fn fyr_f6_stdlib_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/stdlib-modules-ok.txt";

    for field in [
        "name: fyr-stdlib-modules",
        "kind: standard-library-fixture",
        "modules:",
        "vfs",
        "text",
        "json-lite",
        "audit",
        "package",
        "doc",
        "required-evidence:",
        "smoke-tests",
        "failure-mode-tests",
        "docs-examples",
        "safety-notes",
        "claim: fixture-only",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn fyr_f6_stdlib_contract_links_fixture() {
    assert_contains(
        "docs/fyr/STDLIB.md",
        "docs/fyr/fixtures/stdlib-modules-ok.txt",
    );
}

#[test]
fn fyr_roadmap_tracks_f6_stdlib_fixture_evidence() {
    assert_contains(
        "docs/fyr/ROADMAP.md",
        "F6 standard-library fixture evidence",
    );
}
