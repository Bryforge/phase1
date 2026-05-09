use std::fs;

#[test]
fn fyr_toolchain_bootstrap_docs_define_owned_build_surface() {
    let docs = fs::read_to_string("docs/fyr/TOOLCHAIN.md").expect("Fyr toolchain docs exist");

    assert!(docs.contains("Fyr toolchain bootstrap"));
    assert!(docs.contains("fyr init <package>"));
    assert!(docs.contains("fyr check <file.fyr|package>"));
    assert!(docs.contains("fyr build <file.fyr|package>"));
    assert!(docs.contains("VFS-only"));
    assert!(docs.contains("No Cargo invocation"));
    assert!(docs.contains("No host shell"));
    assert!(docs.contains("No network"));
    assert!(docs.contains("backend : seed/interpreted"));
    assert!(docs.contains("status  : dry-run artifact ready"));
}

#[test]
fn fyr_runtime_surface_is_still_host_independent() {
    let registry = fs::read_to_string("src/registry.rs").expect("registry source exists");
    let main = fs::read_to_string("src/main.rs").expect("main source exists");

    assert!(registry.contains("cmd!(\"fyr\""));
    assert!(registry.contains("Phase1-native language target"));
    assert!(registry.contains("\"none\")"));
    assert!(main.contains("fn fyr_command"));
    assert!(main.contains("fn fyr_new"));
    assert!(main.contains("fn fyr_run"));
    assert!(main.contains("fn fyr_self"));
}
