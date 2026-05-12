#[test]
fn reorganization_report_template_defines_metadata_and_commands() {
    let doc = std::fs::read_to_string("docs/base1/REORGANIZATION_VERIFICATION_REPORT_TEMPLATE.md")
        .expect("Base1 reorganization verification report template");

    assert!(
        doc.contains("Base1 reorganization verification report template"),
        "{doc}"
    );

    for text in [
        "Report metadata",
        "Branch:",
        "Commit:",
        "Rust/Cargo available: yes/no",
        "sh scripts/base1-test-inventory-verify.sh",
        "sh scripts/quality-check.sh base1-docs",
        "sh scripts/quality-check.sh base1-reorg",
        "cargo test --all-targets",
    ] {
        assert!(
            doc.contains(text),
            "missing template field or command {text}: {doc}"
        );
    }
}

#[test]
fn reorganization_report_template_lists_expected_passing_conditions() {
    let doc = std::fs::read_to_string("docs/base1/REORGANIZATION_VERIFICATION_REPORT_TEMPLATE.md")
        .expect("Base1 reorganization verification report template");

    for condition in [
        "scripts/base1-test-inventory-verify.sh` passes.",
        "scripts/base1-doc-integrity.sh` passes.",
        "scripts/base1-link-check.sh` passes.",
        "scripts/base1-reorganization-verify.sh` passes on a Rust-capable host.",
        "cargo test --all-targets` passes.",
        "No compatibility paths were removed.",
        "Root release/checkpoint files remain present.",
        "Organized release/checkpoint mirrors remain present.",
        "Base1 non-claims remain visible.",
    ] {
        assert!(
            doc.contains(condition),
            "missing passing condition {condition}: {doc}"
        );
    }
}

#[test]
fn reorganization_report_template_includes_compatibility_review_and_decision() {
    let doc = std::fs::read_to_string("docs/base1/REORGANIZATION_VERIFICATION_REPORT_TEMPLATE.md")
        .expect("Base1 reorganization verification report template");

    for text in [
        "Compatibility review",
        "Root checkpoint-note files remain present.",
        "Organized release/checkpoint mirrors remain present.",
        "docs/base1/ROOT_COMPATIBILITY_MAP.md` remains accurate.",
        "docs/base1/releases/PRE_MOVE_CHECKS.md` remains accurate.",
        "Existing `scripts/base1-*.sh` paths remain stable.",
        "Test inventory matches reporter output.",
        "Decision",
        "Not ready for broader organization.",
        "Ready for one small preservation-first group move.",
        "Ready for broader organization planning only.",
    ] {
        assert!(
            doc.contains(text),
            "missing compatibility or decision text {text}: {doc}"
        );
    }
}

#[test]
fn reorganization_report_template_is_linked_and_integrity_checked() {
    let manual = std::fs::read_to_string("docs/base1/README.md").expect("Base1 manual");
    let map = std::fs::read_to_string("docs/base1/DOCUMENTATION_MAP.md")
        .expect("Base1 documentation map");
    let integrity =
        std::fs::read_to_string("scripts/base1-doc-integrity.sh").expect("Base1 integrity gate");

    for doc in [&manual, &map, &integrity] {
        assert!(
            doc.contains("REORGANIZATION_VERIFICATION_REPORT_TEMPLATE.md"),
            "{doc}"
        );
    }
}

#[test]
fn reorganization_report_template_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/base1/REORGANIZATION_VERIFICATION_REPORT_TEMPLATE.md")
        .expect("Base1 reorganization verification report template");

    assert!(doc.contains("does not make Base1 installer-ready"), "{doc}");
    assert!(doc.contains("hardware-validated"), "{doc}");
    assert!(doc.contains("daily-driver ready"), "{doc}");
}
