#[test]
fn quality_check_runs_full_base1_docs_gate_in_quick_gate() {
    let script = std::fs::read_to_string("scripts/quality-check.sh").expect("quality-check script");

    assert!(script.contains("check_base1_docs()"), "{script}");
    assert!(
        script.contains("run sh scripts/base1-doc-integrity.sh"),
        "{script}"
    );
    assert!(
        script.contains("run sh scripts/base1-link-check.sh"),
        "{script}"
    );
    assert!(
        script.contains("run sh scripts/base1-test-inventory-verify.sh"),
        "{script}"
    );
    assert!(script.contains("check_base1_docs"), "{script}");
    assert!(script.contains("base1-docs"), "{script}");
}

#[test]
fn quality_check_exposes_base1_reorganization_gate() {
    let script = std::fs::read_to_string("scripts/quality-check.sh").expect("quality-check script");

    assert!(script.contains("check_base1_reorganization()"), "{script}");
    assert!(
        script.contains("run sh scripts/base1-reorganization-verify.sh"),
        "{script}"
    );
    assert!(script.contains("base1-reorg"), "{script}");
    assert!(script.contains("base1-reorganization"), "{script}");
}

#[test]
fn quality_docs_describe_current_base1_docs_gate() {
    let quality =
        std::fs::read_to_string("docs/quality/QUALITY.md").expect("docs/quality/QUALITY.md");

    assert!(
        quality.contains("sh scripts/quality-check.sh base1-docs"),
        "{quality}"
    );
    assert!(
        quality.contains("sh scripts/base1-doc-integrity.sh"),
        "{quality}"
    );
    assert!(
        quality.contains("sh scripts/base1-link-check.sh"),
        "{quality}"
    );
    assert!(
        quality.contains("sh scripts/base1-test-inventory-verify.sh"),
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
        quality.contains("local Markdown links resolve"),
        "{quality}"
    );
    assert!(
        quality.contains("reported Base1 test files are listed in `docs/base1/TEST_INVENTORY.md`"),
        "{quality}"
    );
    assert!(
        quality.contains("Base1 organization keeps compatibility paths recoverable"),
        "{quality}"
    );
}

#[test]
fn quality_docs_describe_base1_reorganization_gate() {
    let quality =
        std::fs::read_to_string("docs/quality/QUALITY.md").expect("docs/quality/QUALITY.md");

    assert!(
        quality.contains("sh scripts/quality-check.sh base1-reorg"),
        "{quality}"
    );
    assert!(
        quality.contains("sh scripts/quality-check.sh base1-reorganization"),
        "{quality}"
    );
    assert!(
        quality.contains("sh scripts/base1-reorganization-verify.sh"),
        "{quality}"
    );
    assert!(
        quality.contains("Cargo tests must still run on a Rust-capable host"),
        "{quality}"
    );
}

#[test]
fn quality_required_scripts_include_base1_docs_gate_tools() {
    let quality =
        std::fs::read_to_string("docs/quality/QUALITY.md").expect("docs/quality/QUALITY.md");

    for script in [
        "scripts/base1-doc-integrity.sh",
        "scripts/base1-link-check.sh",
        "scripts/base1-test-inventory.sh",
        "scripts/base1-test-inventory-verify.sh",
        "scripts/base1-reorganization-verify.sh",
    ] {
        assert!(
            quality.contains(script),
            "missing quality script {script}: {quality}"
        );
    }

    assert!(quality.contains("Base1 integrity"), "{quality}");
    assert!(quality.contains("Base1 reorganization"), "{quality}");
}
