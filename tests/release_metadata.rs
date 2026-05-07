use std::fs;

const EDGE_VERSION: &str = "v4.0.0-dev";
const STABLE_VERSION: &str = "v3.10.9";
const COMPATIBILITY_BASE: &str = "v3.6.0";

#[test]
fn release_metadata_is_consistent_across_public_docs() {
    for path in [
        "README.md",
        "docs/wiki/Home.md",
        "docs/wiki/02-Version-Guide.md",
        "docs/wiki/08-Updates-Releases-and-Validation.md",
        "plugins/wiki-version.wasi",
        "plugins/wiki-updates.wasi",
    ] {
        let text = read(path);
        assert!(
            text.contains(EDGE_VERSION),
            "{path} is missing edge version {EDGE_VERSION}"
        );
        assert!(
            text.contains(STABLE_VERSION),
            "{path} is missing stable version {STABLE_VERSION}"
        );
    }
}

#[test]
fn compatibility_base_remains_documented() {
    for path in [
        "README.md",
        "docs/wiki/Home.md",
        "docs/wiki/02-Version-Guide.md",
        "plugins/wiki-version.wasi",
    ] {
        let text = read(path);
        assert!(
            text.contains(COMPATIBILITY_BASE),
            "{path} is missing compatibility base {COMPATIBILITY_BASE}"
        );
    }
}

#[test]
fn stale_stable_release_lines_are_removed() {
    for path in [
        "docs/wiki/02-Version-Guide.md",
        "docs/wiki/08-Updates-Releases-and-Validation.md",
        "plugins/wiki-version.wasi",
        "plugins/wiki-updates.wasi",
    ] {
        let text = read(path);
        assert!(
            !text.contains("v3.10.7"),
            "{path} still references old stable v3.10.7"
        );
        assert!(
            !text.contains("v3.10.9-dev"),
            "{path} still references old edge v3.10.9-dev"
        );
    }
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}
