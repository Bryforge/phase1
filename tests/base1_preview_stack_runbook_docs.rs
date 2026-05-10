use std::fs;

const RUNBOOK: &str = "docs/base1/PREVIEW_STACK_RUNBOOK.md";
const INDEX: &str = "docs/base1/README.md";

#[test]
fn base1_preview_stack_runbook_exists() {
    assert!(
        fs::metadata(RUNBOOK).is_ok(),
        "missing Base1 preview stack runbook"
    );
}

#[test]
fn base1_index_links_preview_stack_runbook() {
    let index = fs::read_to_string(INDEX).expect("Base1 docs index should be readable");

    assert!(
        index.contains("PREVIEW_STACK_RUNBOOK.md"),
        "Base1 index should link the preview stack runbook"
    );
    assert!(
        index.contains("safe emulator-preview stack"),
        "Base1 index should explain the preview stack boundary"
    );
}

#[test]
fn base1_preview_stack_runbook_lists_safe_stack_order() {
    let runbook =
        fs::read_to_string(RUNBOOK).expect("Base1 preview stack runbook should be readable");

    for expected in [
        "scripts/base1-preview-inputs.sh",
        "scripts/base1-emulator-preview.sh",
        "scripts/base1-emulator-doctor.sh",
        "scripts/base1-preview-gate.sh --dry-run",
        "scripts/base1-preview-provenance.sh",
        "scripts/base1-preview-verify.sh",
        "scripts/base1-preview-stack.sh",
        "reports/provenance.env",
        "reports/SHA256SUMS",
    ] {
        assert!(
            runbook.contains(expected),
            "runbook missing stack item: {expected}"
        );
    }
}

#[test]
fn base1_preview_stack_runbook_preserves_non_claims() {
    let runbook =
        fs::read_to_string(RUNBOOK).expect("Base1 preview stack runbook should be readable");

    for expected in [
        "does not claim that Base1 is bootable",
        "not prove Base1 boots",
        "not claim",
        "released OS image",
        "secure OS replacement",
        "daily-driver ready",
        "safe to install on hardware",
        "Recovery USB behavior is complete",
        "Real hardware has been validated",
        "emulator was launched",
    ] {
        assert!(
            runbook.contains(expected),
            "runbook missing non-claim: {expected}"
        );
    }
}

#[test]
fn base1_preview_stack_runbook_preserves_promotion_rule() {
    let runbook =
        fs::read_to_string(RUNBOOK).expect("Base1 preview stack runbook should be readable");

    for expected in [
        "Exact source commit",
        "Exact kernel and initrd source",
        "Generated `reports/provenance.env`",
        "Generated `reports/SHA256SUMS`",
        "Passing `scripts/base1-preview-verify.sh` output",
        "validation report under `docs/base1/validation/`",
        "what was not validated",
    ] {
        assert!(
            runbook.contains(expected),
            "runbook missing promotion rule: {expected}"
        );
    }
}
