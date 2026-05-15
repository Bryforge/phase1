use std::fs;

const GUIDANCE_DOC: &str = "docs/fyr/NATIVE_EXECUTION_GUIDANCE.md";
const SAFETY_DOC: &str = "docs/fyr/SAFETY_MODEL.md";
const TOOLCHAIN_DOC: &str = "docs/fyr/TOOLCHAIN.md";

#[test]
fn fyr_native_execution_guidance_doc_exists_and_defines_scope() {
    let doc =
        fs::read_to_string(GUIDANCE_DOC).expect("Fyr native execution guidance doc should exist");

    for required in [
        "# Fyr native execution guidance",
        "Status: active operator guidance",
        "safe use of Fyr inline/native execution",
        "file-backed `.fyr` source workflows",
        "Real Fyr work should be saved into `.fyr` source files and edited with an editor.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn fyr_native_execution_guidance_limits_inline_native_runs_to_short_tests() {
    let doc =
        fs::read_to_string(GUIDANCE_DOC).expect("Fyr native execution guidance doc should exist");

    for required in [
        "Use native or inline Fyr execution only for:",
        "short parser checks",
        "quick expression tests",
        "smoke tests",
        "command-surface verification",
        "tiny examples that do not need to be saved",
        "It should not become the normal place where operators write programs.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn fyr_native_execution_guidance_requires_file_backed_real_work() {
    let doc =
        fs::read_to_string(GUIDANCE_DOC).expect("Fyr native execution guidance doc should exist");

    for required in [
        "Use `.fyr` files for:",
        "real programs",
        "package work",
        "reusable scripts",
        "tests",
        "automation candidates",
        "review, diffing, recovery, or repeat execution",
        "fyr init app",
        "avim app/src/main.fyr",
        "fyr check app",
        "fyr test app",
        "fyr run app/src/main.fyr",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn fyr_native_execution_guidance_preserves_editor_and_package_expectations() {
    let doc =
        fs::read_to_string(GUIDANCE_DOC).expect("Fyr native execution guidance doc should exist");

    for required in [
        "Operators should use an editor for meaningful Fyr source.",
        "avim file.fyr",
        "ned file.fyr",
        "Package/check/build/test/run workflows should prefer file-backed sources.",
        "fyr.toml",
        "src/main.fyr",
        "tests/smoke.fyr",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn fyr_native_execution_guidance_preserves_safety_relationship_and_non_claims() {
    let doc =
        fs::read_to_string(GUIDANCE_DOC).expect("Fyr native execution guidance doc should exist");

    for required in [
        "VFS-first by default",
        "no host shell by default",
        "no host compiler by default",
        "no network by default",
        "deterministic outputs for check/build/test/run evidence",
        "does not claim Fyr is production-ready",
        "hardened",
        "audited",
        "general-purpose sandbox",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn fyr_native_execution_guidance_is_linked_from_safety_and_toolchain_docs() {
    let safety = fs::read_to_string(SAFETY_DOC).expect("Fyr safety model doc should exist");
    let toolchain = fs::read_to_string(TOOLCHAIN_DOC).expect("Fyr toolchain doc should exist");

    assert!(safety.contains("NATIVE_EXECUTION_GUIDANCE.md"), "{safety}");
    assert!(
        toolchain.contains("NATIVE_EXECUTION_GUIDANCE.md"),
        "{toolchain}"
    );
    assert!(
        safety.contains("Native or inline Fyr execution is for short tests and quick checks only."),
        "{safety}"
    );
    assert!(
        toolchain.contains("Use `.fyr` files edited with an editor for real work."),
        "{toolchain}"
    );
}
