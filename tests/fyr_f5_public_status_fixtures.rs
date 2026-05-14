use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_f5_public_status_fixture_has_required_shape() {
    let path = "docs/fyr/fixtures/public-status-ok.txt";

    for field in [
        "name: fyr-public-status-fixture",
        "kind: public-status",
        "status-kind: estimated-roadmap-progress",
        "project: Fyr native language",
        "claim: fixture-only",
        "required-fields:",
        "repository",
        "branch",
        "project",
        "estimated-completion",
        "next-milestone",
        "checks:",
        "deterministic",
        "evidence-bound",
        "non-claim-aware",
    ] {
        assert_contains(path, field);
    }
}

#[test]
fn fyr_self_workflows_doc_links_public_status_fixture() {
    assert_contains(
        "docs/fyr/SELF_WORKFLOWS.md",
        "docs/fyr/fixtures/public-status-ok.txt",
    );
}

#[test]
fn fyr_public_status_reader_remains_documented_as_pending() {
    let doc = read("docs/fyr/SELF_WORKFLOWS.md");

    assert!(doc.contains("Public status reader"), "doc should name public status reader");
    assert!(
        doc.contains("not yet a full reader command") || doc.contains("documented deferral"),
        "doc should avoid claiming completed public status reader"
    );
}
