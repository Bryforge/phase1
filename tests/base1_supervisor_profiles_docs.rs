#[test]
fn base1_supervisor_profiles_doc_defines_purpose_and_profile_set() {
    let doc = std::fs::read_to_string("docs/os/BASE1_SUPERVISOR_PROFILES.md")
        .expect("Base1 supervisor profiles doc");

    for text in [
        "Base1 supervisor profiles",
        "profile contract scaffold",
        "direct-first path and supervisor orchestration path first-class",
        "profiles/base1/",
        "x200-supervisor-lite",
        "x86_64-vm-validation",
        "workstation-supervisor",
        "low-resource",
        "vm-validation",
        "workstation",
    ] {
        assert!(
            doc.contains(text),
            "missing purpose/profile-set text {text}: {doc}"
        );
    }
}

#[test]
fn base1_supervisor_profiles_doc_lists_required_fields() {
    let doc = std::fs::read_to_string("docs/os/BASE1_SUPERVISOR_PROFILES.md")
        .expect("Base1 supervisor profiles doc");

    for field in [
        "BASE1_PROFILE_NAME",
        "BASE1_PROFILE_CLASS",
        "BASE1_PROFILE_TARGET_RAM_MB",
        "BASE1_PROFILE_DEFAULT_DELIVERY_MODE",
        "BASE1_PROFILE_ALLOWED_DELIVERY_MODES",
        "BASE1_PROFILE_DEFAULT_CONCURRENCY",
        "BASE1_PROFILE_MAX_CONCURRENCY",
        "BASE1_PROFILE_DISPLAY_POLICY",
        "BASE1_PROFILE_VM_MEMORY_MB",
        "BASE1_PROFILE_OPENBSD_MEMORY_MB",
        "BASE1_PROFILE_UEFI_MEMORY_MB",
        "BASE1_PROFILE_STORAGE_TIER_POLICY",
        "BASE1_PROFILE_TMPFS_MB",
        "BASE1_PROFILE_ZRAM_MB",
        "BASE1_PROFILE_SWAP_MB",
        "BASE1_PROFILE_SSD_SCRATCH_MB",
        "BASE1_PROFILE_SECURITY_POSTURE",
        "BASE1_PROFILE_CLAIM",
        "BASE1_PROFILE_NON_CLAIM_BOOTABLE",
        "BASE1_PROFILE_NON_CLAIM_INSTALLER",
        "BASE1_PROFILE_NON_CLAIM_HARDENED",
        "BASE1_PROFILE_NON_CLAIM_HYPERVISOR",
        "BASE1_PROFILE_NON_CLAIM_HARDWARE",
        "BASE1_PROFILE_NON_CLAIM_DAILY_DRIVER",
    ] {
        assert!(doc.contains(field), "missing required field {field}: {doc}");
    }
}

#[test]
fn base1_supervisor_profiles_doc_defines_x200_vm_and_workstation_intents() {
    let doc = std::fs::read_to_string("docs/os/BASE1_SUPERVISOR_PROFILES.md")
        .expect("Base1 supervisor profiles doc");

    for text in [
        "The X200 profile is the low-resource anchor.",
        "one active staged kernel by default",
        "serial/headless execution",
        "direct-first delivery when boot delivery matters",
        "supervisor-lite when staged evidence matters",
        "zram plus SSD scratch plus swap backstop",
        "The VM validation profile is the deterministic evidence profile.",
        "serial capture",
        "explicit artifacts",
        "B3 evidence review",
        "no generalized hardware claim",
        "The workstation profile is for larger-memory development and parallel validation.",
        "requires explicit evidence and reviewed logs before any claim strengthens",
    ] {
        assert!(
            doc.contains(text),
            "missing profile intent text {text}: {doc}"
        );
    }
}

#[test]
fn base1_supervisor_profiles_doc_preserves_compatibility_rules_and_non_claims() {
    let doc = std::fs::read_to_string("docs/os/BASE1_SUPERVISOR_PROFILES.md")
        .expect("Base1 supervisor profiles doc");

    for text in [
        "Direct-first and supervisor paths must consume the same profile vocabulary.",
        "No profile may imply boot readiness by itself.",
        "No profile may imply hardening proof by itself.",
        "No profile may imply hypervisor readiness by itself.",
        "No profile may imply physical hardware validation by itself.",
        "Storage-backed swap is an OOM backstop, not true RAM.",
        "X200-class profiles must keep concurrency conservative.",
        "scripts/base1-delivery-mode-plan.sh",
        "scripts/base1-storage-tier-plan.sh",
        "scripts/base1-b3-vm-validate.sh",
        "scripts/base1-b3-log-bundle-review.sh",
        "do not make Base1 bootable",
        "hypervisor-ready",
        "daily-driver ready",
    ] {
        assert!(
            doc.contains(text),
            "missing compatibility/non-claim text {text}: {doc}"
        );
    }
}
