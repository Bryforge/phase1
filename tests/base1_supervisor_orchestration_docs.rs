#[test]
fn base1_supervisor_orchestration_doc_defines_model_profiles_and_limits() {
    let doc = std::fs::read_to_string("docs/os/BASE1_SUPERVISOR_ORCHESTRATION.md")
        .expect("supervisor orchestration doc");

    for text in [
        "Base1 supervisor orchestration",
        "direct-first",
        "supervisor orchestration",
        "control-plane",
        "staged kernel",
        "x200-supervisor-lite",
        "x86_64-vm-validation",
        "workstation-supervisor",
        "4GB-class low-resource target",
        "one active staged kernel by default",
        "zram plus SSD scratch/swap backstop",
        "no hypervisor claim",
        "no hardware-ready claim",
        "no daily-driver claim",
        "Storage-tier rule",
        "no claim that storage is equivalent to RAM",
        "does not make Base1 bootable",
        "installer-ready",
        "hardened",
        "hardware-validated",
    ] {
        assert!(doc.contains(text), "missing doc text {text}: {doc}");
    }
}
