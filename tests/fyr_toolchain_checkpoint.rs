use std::fs;

#[test]
fn fyr_toolchain_checkpoint_documents_current_surface() {
    let doc =
        fs::read_to_string("docs/fyr/CHECKPOINT_v4_4_0.md").expect("checkpoint doc should exist");

    for expected in [
        "fyr init <package>",
        "fyr check <file.fyr|package>",
        "fyr build <file.fyr|package>",
        "fyr test <package>",
        "fyr color <file.fyr>",
        "fyr highlight <file.fyr>",
        "assert_eq(1, 1);",
        "integer comparisons",
        "VFS-only and host-independent",
        "does not invoke Cargo or host tools",
    ] {
        assert!(
            doc.contains(expected),
            "missing checkpoint line: {expected}"
        );
    }
}
