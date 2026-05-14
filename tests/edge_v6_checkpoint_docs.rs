#[test]
fn edge_v6_checkpoint_records_current_boundary() {
    let doc =
        std::fs::read_to_string("docs/archive/checkpoints/DEVELOPMENT_CHECKPOINT_EDGE_6_0_0.md")
            .expect("edge v6 checkpoint");

    assert!(doc.contains("Edge 6.0.0"), "{doc}");
    assert!(doc.contains("v6.0.0"), "{doc}");
    assert!(doc.contains("Stable release point"), "{doc}");
    assert!(doc.contains("v5.0.0"), "{doc}");
    assert!(doc.contains("Base1 recovery USB"), "{doc}");
    assert!(
        doc.contains("does not claim a finished OS replacement"),
        "{doc}"
    );
}

#[test]
fn release_metadata_points_to_current_v7_edge() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");
    let edge = std::fs::read_to_string("docs/repo/EDGE.md").expect("docs/repo/EDGE.md");
    let cargo = std::fs::read_to_string("Cargo.toml").expect("Cargo.toml");

    assert!(readme.contains("current%20edge-v7.0.1"), "{readme}");
    assert!(
        readme.contains("Current edge version: `v7.0.1`"),
        "{readme}"
    );
    assert!(readme.contains("Stable base: `base/v6.0.0`"), "{readme}");
    assert!(edge.contains("Current package version | `7.0.1`"), "{edge}");
    assert!(edge.contains("Current edge label | `v7.0.1`"), "{edge}");
    assert!(cargo.contains("version = \"7.0.1\""), "{cargo}");
}
