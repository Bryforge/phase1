#[test]
fn reorganization_verifier_is_read_only() {
    let script = std::fs::read_to_string("scripts/base1-reorganization-verify.sh")
        .expect("Base1 reorganization verifier");

    assert!(script.contains("Base1 reorganization verification bundle"), "{script}");
    assert!(script.contains("mode: read-only"), "{script}");
    assert!(
        script.contains("verification complete; no files were changed"),
        "{script}"
    );
}

#[test]
fn reorganization_verifier_runs_required_gates() {
    let script = std::fs::read_to_string("scripts/base1-reorganization-verify.sh")
        .expect("Base1 reorganization verifier");

    for command in [
        "scripts/base1-doc-integrity.sh",
        "scripts/base1-link-check.sh",
        "scripts/base1-test-inventory-verify.sh",
        "cargo test --all-targets",
    ] {
        assert!(script.contains(command), "missing verifier command {command}: {script}");
    }
}

#[test]
fn reorganization_verifier_handles_missing_cargo_explicitly() {
    let script = std::fs::read_to_string("scripts/base1-reorganization-verify.sh")
        .expect("Base1 reorganization verifier");

    assert!(script.contains("command -v cargo"), "{script}");
    assert!(
        script.contains("cargo not found; skipping cargo test --all-targets"),
        "{script}"
    );
    assert!(
        script.contains("run cargo test --all-targets on a Rust-capable host before claiming full readiness"),
        "{script}"
    );
}

#[test]
fn reorganization_verifier_is_integrity_checked() {
    let integrity = std::fs::read_to_string("scripts/base1-doc-integrity.sh")
        .expect("Base1 integrity gate");

    assert!(
        integrity.contains("scripts/base1-reorganization-verify.sh"),
        "{integrity}"
    );
    assert!(
        integrity.contains("Base1 organization readiness"),
        "{integrity}"
    );
    assert!(
        integrity.contains("cargo test --all-targets"),
        "{integrity}"
    );
}
