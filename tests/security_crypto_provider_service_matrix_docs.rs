use std::fs;

#[test]
fn crypto_provider_service_matrix_exists_and_preserves_scope() {
    let doc = fs::read_to_string("docs/security/CRYPTO_PROVIDER_SERVICE_MATRIX.md")
        .expect("read crypto provider service matrix");

    for required in [
        "Phase1 and Base1",
        "major reviewed open-source cryptographic systems",
        "operator-selectable policy layer",
        "documentation-first",
        "does not approve runtime cryptographic protection",
        "No provider, algorithm, profile, or service is runtime-approved",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn crypto_provider_service_matrix_lists_service_and_provider_families() {
    let doc = fs::read_to_string("docs/security/CRYPTO_PROVIDER_SERVICE_MATRIX.md")
        .expect("read crypto provider service matrix");

    for required in [
        "identity",
        "storage",
        "transport",
        "update",
        "logs-evidence",
        "recovery",
        "analysis",
        "developer",
        "RustCrypto ecosystem",
        "OpenSSL/LibreSSL/BoringSSL family",
        "libsodium family",
        "post-quantum provider family",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}

#[test]
fn crypto_provider_service_matrix_preserves_guardrails_and_non_claims() {
    let doc = fs::read_to_string("docs/security/CRYPTO_PROVIDER_SERVICE_MATRIX.md")
        .expect("read crypto provider service matrix");

    for required in [
        "Prefer reviewed open-source providers",
        "Reject custom security-critical primitives",
        "Reject unknown providers",
        "Fail closed",
        "Avoid silent provider substitution",
        "Require test vectors",
        "does not claim",
        "approved national-security crypto",
        "production-ready cryptographic enforcement",
    ] {
        assert!(doc.contains(required), "missing {required:?}");
    }
}
