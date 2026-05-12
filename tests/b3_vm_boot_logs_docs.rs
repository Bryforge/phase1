#[test]
fn b3_vm_boot_logs_define_scope_and_entry_gate() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_LOGS.md")
        .expect("B3 VM boot log capture notes");

    assert!(doc.contains("Base1 B3 VM boot log capture notes"), "{doc}");
    assert!(
        doc.contains("log evidence expected for future B3 VM boot validation"),
        "{doc}"
    );
    assert!(
        doc.contains("Do not treat B3 logs as validation evidence until the B2 focused test suite has passed locally or in CI."),
        "{doc}"
    );
    assert!(doc.contains("B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md"), "{doc}");
}

#[test]
fn b3_vm_boot_logs_define_initial_profile() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_LOGS.md")
        .expect("B3 VM boot log capture notes");

    assert!(doc.contains("x86_64-vm-validation"), "{doc}");
}

#[test]
fn b3_vm_boot_logs_list_planned_log_categories() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_LOGS.md")
        .expect("B3 VM boot log capture notes");

    for text in [
        "VM runtime name and version",
        "VM profile name",
        "architecture mode",
        "firmware mode",
        "boot artifact identifier",
        "command or run configuration used for the VM validation",
        "console boot log",
        "kernel or init handoff notes",
        "Phase1 launch result",
        "emergency fallback result or limitation",
        "failure mode notes if the VM does not boot",
        "known limitations",
    ] {
        assert!(
            doc.contains(text),
            "missing planned log category {text}: {doc}"
        );
    }
}

#[test]
fn b3_vm_boot_logs_preserve_safe_log_handling_rules() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_LOGS.md")
        .expect("B3 VM boot log capture notes");

    for text in [
        "Logs must not include:",
        "private keys",
        "tokens",
        "passwords",
        "recovery codes",
        "personal credentials",
        "private local file contents",
        "unredacted environment variables",
        "private network addresses unless intentionally part of a public test fixture",
        "personal account identifiers",
        "secret-bearing kernel command-line values",
    ] {
        assert!(
            doc.contains(text),
            "missing safe log handling text {text}: {doc}"
        );
    }
}

#[test]
fn b3_vm_boot_logs_define_future_paths_as_examples_only() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_LOGS.md")
        .expect("B3 VM boot log capture notes");

    for text in [
        "docs/os/validation/b3-x86_64-vm-validation.md",
        "docs/os/validation/logs/b3-x86_64-vm-validation-console.txt",
        "These paths are examples only until a real validation run exists.",
    ] {
        assert!(doc.contains(text), "missing future path text {text}: {doc}");
    }
}

#[test]
fn b3_vm_boot_logs_preserve_review_checklist() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_LOGS.md")
        .expect("B3 VM boot log capture notes");

    for text in [
        "B2 focused test suite passed locally or in CI",
        "VM profile is explicit",
        "VM runtime is explicit",
        "boot artifact is explicit",
        "run configuration is documented",
        "console log is captured or linked",
        "Phase1 launch result is recorded",
        "emergency fallback result or limitation is recorded",
        "logs are reviewed for secrets",
        "limitations are documented",
        "non-claims are preserved",
    ] {
        assert!(
            doc.contains(text),
            "missing review checklist text {text}: {doc}"
        );
    }
}

#[test]
fn b3_vm_boot_logs_link_related_docs() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_LOGS.md")
        .expect("B3 VM boot log capture notes");

    for link in [
        "BOOT_READINESS_STATUS.md",
        "B3_VM_BOOT_VALIDATION_PLAN.md",
        "B3_VM_BOOT_VALIDATION_LIMITATIONS.md",
        "B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md",
    ] {
        assert!(doc.contains(link), "missing related doc link {link}: {doc}");
    }
}

#[test]
fn b3_vm_boot_logs_are_linked_from_status_tracker() {
    let status = std::fs::read_to_string("docs/os/BOOT_READINESS_STATUS.md")
        .expect("boot readiness status tracker");

    assert!(status.contains("B3_VM_BOOT_LOGS.md"), "{status}");
    assert!(status.contains("B3 log capture notes exist."), "{status}");
}

#[test]
fn b3_vm_boot_logs_preserve_non_claims() {
    let doc = std::fs::read_to_string("docs/os/B3_VM_BOOT_LOGS.md")
        .expect("B3 VM boot log capture notes");

    for text in [
        "does not make Base1 bootable on physical hardware",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
    ] {
        assert!(doc.contains(text), "missing non-claim {text}: {doc}");
    }
}
