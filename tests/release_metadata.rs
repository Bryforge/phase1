use std::fs;

const EDGE_VERSION: &str = "v6.0.0";
const EDGE_PACKAGE_VERSION: &str = "6.0.0";
const STABLE_VERSION: &str = "v5.0.0";
const STABLE_PACKAGE_VERSION: &str = "5.0.0";
const PREVIOUS_STABLE: &str = "v4.4.0";
const COMPATIBILITY_BASE: &str = "v3.6.0";
const EDGE_CHECKPOINT: &str = "DEVELOPMENT_CHECKPOINT_EDGE_6_0_0.md";

#[test]
fn cargo_metadata_matches_current_track() {
    let cargo_toml = read("Cargo.toml");
    let cargo_lock = read("Cargo.lock");

    if is_edge_track(&cargo_toml) {
        assert!(
            cargo_toml.contains("version = \"6.0.0\""),
            "Cargo.toml must identify the edge package version as {EDGE_PACKAGE_VERSION}"
        );
        assert!(
            cargo_lock.contains("version = \"6.0.0\""),
            "Cargo.lock must identify the edge package version as {EDGE_PACKAGE_VERSION}"
        );
    } else {
        assert!(
            cargo_toml.contains("version = \"5.0.0\""),
            "Cargo.toml must be promoted to {STABLE_PACKAGE_VERSION} for stable"
        );
        assert!(
            cargo_lock.contains("version = \"5.0.0\""),
            "Cargo.lock must be promoted to {STABLE_PACKAGE_VERSION} for stable"
        );
        assert!(
            !cargo_toml.contains("-dev"),
            "stable Cargo.toml must not contain a dev suffix"
        );
        assert!(
            !cargo_lock.contains("-dev"),
            "stable Cargo.lock must not contain a dev suffix"
        );
    }
}

#[test]
fn release_metadata_is_consistent_across_public_docs() {
    for path in release_facing_files() {
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
fn edge_checkpoint_records_current_dev_boundary() {
    let cargo_toml = read("Cargo.toml");
    if !is_edge_track(&cargo_toml) {
        return;
    }

    let checkpoint = read(EDGE_CHECKPOINT);
    for expected in [
        EDGE_VERSION,
        EDGE_PACKAGE_VERSION,
        STABLE_VERSION,
        PREVIOUS_STABLE,
        COMPATIBILITY_BASE,
        "Guarded host runtime execution",
        "compact dynamic chips",
        "Mobile/narrow terminals",
        "PHASE1_COMPACT_PROMPT=0",
    ] {
        assert!(
            checkpoint.contains(expected),
            "{EDGE_CHECKPOINT} is missing checkpoint marker {expected}"
        );
    }
}

#[test]
fn compatibility_base_remains_documented() {
    for path in ["README.md", "site.js"] {
        let text = read(path);
        assert!(
            text.contains(COMPATIBILITY_BASE),
            "{path} is missing compatibility base {COMPATIBILITY_BASE}"
        );
    }
}

#[test]
fn website_demo_reports_current_stable_track() {
    let js = read("site.js");
    assert!(has_line(&js, "    \"stable: v5.0.0\","));
    assert!(has_line(&js, "    \"previous stable: v4.4.0\","));
    assert!(has_line(&js, "    \"next edge: v6.0.0\","));
    assert!(!has_line(&js, "    \"stable: v4.1.0\","));
    assert!(!has_line(&js, "    \"next edge: v4.2.0\","));
}

#[test]
fn stale_dev_release_lines_are_removed_from_release_facing_files() {
    for path in release_facing_files()
        .into_iter()
        .chain(["Cargo.toml", "Cargo.lock", "site.js"])
    {
        let text = read(path);
        assert!(
            !text.contains("v4.2.0-dev"),
            "{path} still references development version v4.2.0-dev"
        );
        assert!(
            !text.contains("4.2.0-dev"),
            "{path} still references development version 4.2.0-dev"
        );
        assert!(
            !text.contains("v4.4.0-dev"),
            "{path} still references development version v4.4.0-dev"
        );
        assert!(
            !text.contains("4.4.0-dev"),
            "{path} still references development version 4.4.0-dev"
        );
    }
}

fn is_edge_track(cargo_toml: &str) -> bool {
    cargo_toml.contains("version = \"6.0.0\"")
}

fn edge_facing_files() -> [&'static str; 5] {
    [
        "README.md",
        "EDGE.md",
        "CHANGELOG.md",
        "site.js",
        EDGE_CHECKPOINT,
    ]
}

fn release_facing_files() -> [&'static str; 3] {
    ["README.md", "RELEASE_v5.0.0.md", "site.js"]
}

fn has_line(text: &str, expected: &str) -> bool {
    text.lines().any(|line| line == expected)
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}
