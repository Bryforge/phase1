use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn docs_sync_script_and_outputs_are_present() {
    let script = read("scripts/update-docs.py");
    assert!(script.contains("base/v4.2.0"));
    assert!(script.contains("edge/stable"));
    assert!(script.contains("upsert(\"README.md\", \"repo-model\""));
    assert!(script.contains("docs/wiki/Repository-Model.md"));
    assert!(script.contains("docs/wiki/Current-Status.md"));
}

#[test]
fn generated_docs_preserve_stable_base_and_edge_path() {
    let docs = [
        "README.md",
        "docs/repo/REPO_DOCTRINE.md",
        "docs/repo/EDGE_STABLE_CHECKPOINT.md",
        "docs/project/FEATURE_STATUS.md",
        "docs/repo/EDGE.md",
        "docs/project/WIKI_ROADMAP.md",
        "docs/wiki/Repository-Model.md",
        "docs/wiki/Current-Status.md",
    ];

    for path in docs {
        let body = read(path);
        assert!(
            body.contains("base/v4.2.0"),
            "{path} lost the frozen stable base"
        );
        assert!(
            body.contains("edge/stable"),
            "{path} lost the active edge path"
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
