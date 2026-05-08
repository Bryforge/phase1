use std::fs;

const EDGE_VERSION: &str = "v4.2.5-dev";
const EDGE_PACKAGE_VERSION: &str = "4.2.5-dev";
const STABLE_VERSION: &str = "v4.0.0";
const STABLE_PACKAGE_VERSION: &str = "4.0.0";
const PREVIOUS_STABLE: &str = "v3.10.9";
const COMPATIBILITY_BASE: &str = "v3.6.0";

#[test]
fn cargo_metadata_matches_current_track() {
    let cargo_toml = read("Cargo.toml");
    let cargo_lock = read("Cargo.lock");

    if is_edge_track(&cargo_toml) {
        assert!(
            cargo_toml.contains("version = \"4.2.5-dev\""),
            "Cargo.toml must identify the edge package version as {EDGE_PACKAGE_VERSION}"
        );
        assert!(
            cargo_lock.contains("version = \"4.2.5-dev\""),
            "Cargo.lock must identify the edge package version as {EDGE_PACKAGE_VERSION}"
        );
    } else {
        assert!(
            cargo_toml.contains("version = \"4.0.0\""),
            "Cargo.toml must be promoted to {STABLE_PACKAGE_VERSION} for stable"
        );
        assert!(
            cargo_lock.contains("version = \"4.0.0\""),
            "Cargo.lock must be promoted to {STABLE_PACKAGE_VERSION} for stable"
        );
        assert!(!cargo_toml.contains("-dev"), "stable Cargo.toml must not contain a dev suffix");
        assert!(!cargo_lock.contains("-dev"), "stable Cargo.lock must not contain a dev suffix");
    }
}

#[test]
fn release_metadata_is_consistent_across_public_docs() {
    for path in release_facing_files() {
        let text = read(path);
        assert!(text.contains(STABLE_VERSION), "{path} is missing stable version {STABLE_VERSION}");
        assert!(text.contains(PREVIOUS_STABLE), "{path} is missing previous stable {PREVIOUS_STABLE}");
    }
}

#[test]
fn edge_track_is_documented_when_package_is_dev() {
    let cargo_toml = read("Cargo.toml");
    if !is_edge_track(&cargo_toml) {
        return;
    }

    for path in edge_facing_files() {
        let text = read(path);
        assert!(
            text.contains(EDGE_VERSION) || text.contains(EDGE_PACKAGE_VERSION),
            "{path} is missing edge version {EDGE_VERSION}"
        );
    }
}

#[test]
fn compatibility_base_remains_documented() {
    for path in [
        "README.md",
        "site.js",
        "docs/wiki/Home.md",
        "docs/wiki/02-Version-Guide.md",
        "docs/wiki/12-v4-Edge-Manual.md",
        "plugins/wiki.wasi",
        "plugins/wiki-quick.wasi",
        "plugins/wiki-version.wasi",
    ] {
        let text = read(path);
        assert!(text.contains(COMPATIBILITY_BASE), "{path} is missing compatibility base {COMPATIBILITY_BASE}");
    }
}

#[test]
fn stale_release_lines_are_removed_from_release_facing_files() {
    for path in release_facing_files().into_iter().chain(["Cargo.toml", "Cargo.lock", "start_phase1"]) {
        let text = read(path);
        assert!(!text.contains("v3.10.7"), "{path} still references old stable v3.10.7");
        assert!(!text.contains("v3.10.9-dev"), "{path} still references old edge v3.10.9-dev");
        assert!(!text.contains("v4.0.0-dev"), "{path} still references development version v4.0.0-dev");
        assert!(!text.contains("4.0.0-dev"), "{path} still references development version 4.0.0-dev");
    }
}

fn is_edge_track(cargo_toml: &str) -> bool {
    cargo_toml.contains("version = \"4.2.5-dev\"")
}

fn edge_facing_files() -> [&'static str; 3] {
    ["Cargo.toml", "Cargo.lock", "EDGE.md"]
}

fn release_facing_files() -> [&'static str; 13] {
    [
        "README.md",
        "RELEASE_v4.0.0.md",
        "docs/wiki/Home.md",
        "docs/wiki/02-Version-Guide.md",
        "docs/wiki/08-Updates-Releases-and-Validation.md",
        "docs/wiki/10-Publish-to-GitHub-Wiki.md",
        "docs/wiki/11-Tutorials.md",
        "docs/wiki/12-v4-Edge-Manual.md",
        "plugins/wiki.wasi",
        "plugins/wiki-quick.wasi",
        "plugins/wiki-version.wasi",
        "plugins/wiki-updates.wasi",
        "site.js",
    ]
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}
