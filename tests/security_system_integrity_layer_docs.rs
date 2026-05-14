use std::fs;

const INTEGRITY_DOC: &str = "docs/security/SYSTEM_INTEGRITY_LAYER.md";

#[test]
fn system_integrity_layer_doc_exists_and_defines_scope() {
    let doc = fs::read_to_string(INTEGRITY_DOC).expect("system integrity layer doc should exist");

    for required in [
        "Phase1 System Integrity Layer",
        "Status: planning",
        "native file integrity records",
        "SHA-256 manifests",
        "read-only validation",
        "operator-visible integrity reports",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn system_integrity_layer_doc_defines_manifest_model() {
    let doc = fs::read_to_string(INTEGRITY_DOC).expect("system integrity layer doc should exist");

    for required in [
        "expected SHA-256",
        "observed SHA-256",
        "expected byte length",
        "observed byte length",
        "manifest source",
        "validation scope",
        "release artifact review",
        "Base1 recovery bundle review",
        "future signed integrity reports",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn system_integrity_layer_doc_preserves_result_labels_and_safety_rules() {
    let doc = fs::read_to_string(INTEGRITY_DOC).expect("system integrity layer doc should exist");

    for required in [
        "ok",
        "changed",
        "missing",
        "extra",
        "unreadable",
        "manifest-invalid",
        "not-checked",
        "Do not silently repair changed files.",
        "Do not delete extra files.",
        "Keep validation read-only by default.",
        "Fail closed when a required manifest is missing or malformed.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn system_integrity_layer_doc_preserves_non_claims() {
    let doc = fs::read_to_string(INTEGRITY_DOC).expect("system integrity layer doc should exist");

    for required in [
        "does not claim total system integrity",
        "tamper-proof execution",
        "hardware-backed measurement",
        "secure boot",
        "production hardening",
        "certification",
        "complete compromise detection",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}
