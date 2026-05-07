use std::fs;

const STABLE_VERSION: &str = "v4.0.0";
const PREVIOUS_STABLE: &str = "v3.10.9";
const COMPATIBILITY_BASE: &str = "v3.6.0";

#[test]
fn cargo_metadata_is_promoted_to_stable() {
    let cargo_toml = read("Cargo.toml");
    let cargo_lock = read("Cargo.lock");
    assert!(
        cargo_toml.contains("version = \"4.0.0\""),
        "Cargo.toml must be promoted to 4.0.0"
    );
    assert!(
        cargo_lock.contains("version = \"4.0.0\""),
        "Cargo.lock must be promoted to 4.0.0"
    );
    assert!(
        !cargo_toml.contains("4.0.0-dev"),
        "Cargo.toml must not contain a dev suffix for stable"
    );
    assert!(
        !cargo_lock.contains("4.0.0-dev"),
        "Cargo.lock must not contain a dev suffix for stable"
    );
}

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
            text.contains(STABLE_VERSION),
            "{path} is missing stable version {STABLE_VERSION}"
        );
        assert!(
            text.contains(PREVIOUS_STABLE),
            "{path} is missing previous stable {PREVIOUS_STABLE}"
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
        "site.js",
    ] {
        let text = read(path);
        assert!(
            text.contains(COMPATIBILITY_BASE),
            "{path} is missing compatibility base {COMPATIBILITY_BASE}"
        );
    }
}

#[test]
fn website_demo_reports_stable_release() {
    let js = read("site.js");
    assert!(js.contains("stable: v4.0.0"));
    assert!(js.contains("previous stable: v3.10.9"));
    assert!(!js.contains("edge: v4.0.0-dev"));
}

#[test]
fn stale_release_lines_are_removed_from_release_facing_files() {
    for path in [
        "Cargo.toml",
        "Cargo.lock",
        "README.md",
        "site.js",
        "docs/wiki/Home.md",
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
        assert!(
            !text.contains("v4.0.0-dev"),
            "{path} still references development version v4.0.0-dev"
        );
        assert!(
            !text.contains("4.0.0-dev"),
            "{path} still references development version 4.0.0-dev"
        );
    }
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}
