use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn docs_sync_script_and_outputs_are_present() {
    let script = read("scripts/update-docs.py");
    assert!(script.contains("base/v5.0.0"), "{script}");
    assert!(script.contains("v6.0.0"), "{script}");
    assert!(script.contains("edge/stable"), "{script}");
    assert!(
        script.contains("upsert(\"README.md\", \"repo-model\""),
        "{script}"
    );
    assert!(script.contains("docs/repo/EDGE.md"), "{script}");
    assert!(script.contains("docs/wiki/Repository-Model.md"), "{script}");
    assert!(script.contains("docs/wiki/Current-Status.md"), "{script}");
    assert!(
        script.contains("root-level generated notes are intentionally not created"),
        "{script}"
    );
}

#[test]
fn generated_docs_preserve_current_stable_base_edge_version_and_edge_path() {
    let docs = [
        "README.md",
        "docs/repo/EDGE.md",
        "docs/wiki/Repository-Model.md",
        "docs/wiki/Current-Status.md",
    ];

    for path in docs {
        let body = read(path);
        assert!(
            body.contains("base/v5.0.0"),
            "{path} lost the current stable base"
        );
        assert!(
            body.contains("edge/stable"),
            "{path} lost the active edge path"
        );
    }

    let readme = read("README.md");
    assert!(
        readme.contains("Current edge version: `v6.0.0`"),
        "README lost v6 current status: {readme}"
    );
}

#[test]
fn old_root_generated_docs_are_not_required_or_regenerated() {
    let script = read("scripts/update-docs.py");

    for old_root_path in [
        "EDGE_STABLE_CHECKPOINT.md",
        "FEATURE_STATUS.md",
        "REPO_DOCTRINE.md",
        "WIKI_ROADMAP.md",
    ] {
        let old_upsert = format!("upsert(\"{old_root_path}\"");
        assert!(
            !script.contains(&old_upsert),
            "update-docs.py should not regenerate old root note {old_root_path}: {script}"
        );
    }
}

#[test]
fn generated_docs_have_managed_blocks() {
    let readme = read("README.md");
    assert!(readme.contains("<!-- phase1:auto:repo-model:start -->"));
    assert!(readme.contains("<!-- phase1:auto:repo-model:end -->"));
    assert!(readme.contains("<!-- phase1:auto:current-status:start -->"));
    assert!(readme.contains("<!-- phase1:auto:current-status:end -->"));
}
