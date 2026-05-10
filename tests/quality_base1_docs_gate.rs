#[test]
fn quality_check_runs_base1_docs_integrity_in_quick_gate() {
    let script = std::fs::read_to_string("scripts/quality-check.sh")
        .expect("quality-check script");

    assert!(script.contains("check_base1_docs()"), "{script}");
    assert!(
        script.contains("run sh scripts/base1-doc-integrity.sh"),
        "{script}"
    );
    assert!(script.contains("check_base1_docs"), "{script}");
    assert!(script.contains("base1-docs"), "{script}");
}

#[test]
fn quality_docs_describe_base1_docs_gate() {
    let quality = std::fs::read_to_string("QUALITY.md").expect("QUALITY.md");

    assert!(
        quality.contains("sh scripts/quality-check.sh base1-docs"),
        "{quality}"
    );
    assert!(
        quality.contains("sh scripts/base1-doc-integrity.sh"),
        "{quality}"
    );
    assert!(
        quality.contains("root checkpoint-note compatibility files are present"),
        "{quality}"
    );
    assert!(
        quality.contains("organized release mirrors are present"),
        "{quality}"
    );
    assert!(
        quality.contains("Base1 organization keeps compatibility paths recoverable"),
        "{quality}"
    );
}

#[test]
fn quality_required_scripts_include_base1_integrity_gate() {
    let quality = std::fs::read_to_string("QUALITY.md").expect("QUALITY.md");

    assert!(
        quality.contains("scripts/base1-doc-integrity.sh"),
        "{quality}"
    );
    assert!(
        quality.contains("Base1 integrity"),
        "{quality}"
    );
}
