use std::fs;

#[test]
fn repository_model_documents_stable_base_and_edge_default() {
    let checkpoint =
        fs::read_to_string("EDGE_STABLE_CHECKPOINT.md").expect("EDGE_STABLE_CHECKPOINT.md exists");

    assert!(checkpoint.contains("base/v4.2.0"));
    assert!(checkpoint.contains("frozen stable base"));
    assert!(checkpoint.contains("edge/stable"));
    assert!(checkpoint.contains("active default development path"));
    assert!(checkpoint.contains("feature branches now target edge/stable"));
}

#[test]
fn release_docs_keep_4_2_0_as_stability_base() {
    let release = fs::read_to_string("RELEASE_v4.2.0.md").expect("RELEASE_v4.2.0.md exists");

    assert!(
        release.contains("4.2.0") || release.contains("v4.2.0"),
        "v4.2.0 release notes must remain present as the stability base"
    );
}
