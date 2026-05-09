#[test]
fn edge_v6_checkpoint_records_current_boundary() {
    let doc = std::fs::read_to_string("DEVELOPMENT_CHECKPOINT_EDGE_6_0_0.md")
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
fn release_metadata_points_to_v6_edge() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");
    let edge = std::fs::read_to_string("EDGE.md").expect("EDGE.md");
    let cargo = std::fs::read_to_string("Cargo.toml").expect("Cargo.toml");

    assert!(readme.contains("edge-v6.0.0"), "{readme}");
    assert!(
        readme.contains("Current edge version: `v6.0.0`"),
        "{readme}"
    );
    assert!(edge.contains("Current package version | `6.0.0`"), "{edge}");
    assert!(edge.contains("Current edge label | `v6.0.0`"), "{edge}");
    assert!(cargo.contains("version = \"6.0.0\""), "{cargo}");
}
