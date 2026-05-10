use std::fs;

#[test]
fn base1_readiness_matrix_exists() {
    assert!(
        fs::metadata("docs/base1/READINESS_MATRIX.md").is_ok(),
        "missing Base1 readiness matrix"
    );
}

#[test]
fn base1_index_links_readiness_matrix() {
    let index =
        fs::read_to_string("docs/base1/README.md").expect("Base1 docs index should be readable");

    assert!(index.contains("READINESS_MATRIX.md"));
    assert!(index.contains("Promotion rule"));
}

#[test]
fn base1_readiness_matrix_preserves_evidence_levels() {
    let matrix = fs::read_to_string("docs/base1/READINESS_MATRIX.md")
        .expect("Base1 readiness matrix should be readable");

    for expected in ["Roadmap", "Design", "Dry-run", "Preview", "Validated"] {
        assert!(
            matrix.contains(expected),
            "missing evidence level: {expected}"
        );
    }
}

#[test]
fn base1_readiness_matrix_preserves_non_claims() {
    let matrix = fs::read_to_string("docs/base1/READINESS_MATRIX.md")
        .expect("Base1 readiness matrix should be readable");

    for expected in [
        "does not claim Base1 is bootable",
        "daily-driver ready",
        "installer-ready",
        "recovery-complete",
        "validated on real hardware",
    ] {
        assert!(matrix.contains(expected), "missing non-claim: {expected}");
    }
}
