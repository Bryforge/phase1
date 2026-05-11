#[test]
fn base1_dual_path_delivery_defines_purpose_and_paths() {
    let doc = std::fs::read_to_string("docs/os/BASE1_DUAL_PATH_DELIVERY.md")
        .expect("dual-path delivery design doc");

    for text in [
        "Base1 dual-path delivery design",
        "preserving a fast first-kernel delivery path while Base1 grows into a supervisor orchestration system",
        "Direct path",
        "Supervisor path",
        "Shared contract",
        "minimal delivery path",
        "long-term architecture path",
        "compatibility layer",
    ] {
        assert!(doc.contains(text), "missing dual-path purpose/path text {text}: {doc}");
    }
}

#[test]
fn base1_dual_path_delivery_preserves_direct_first_kernel_path() {
    let doc = std::fs::read_to_string("docs/os/BASE1_DUAL_PATH_DELIVERY.md")
        .expect("dual-path delivery design doc");

    for text in [
        "Path A: direct first-kernel delivery",
        "firmware / QEMU / hardware profile",
        "bootloader",
        "Base1 kernel or first supported kernel payload",
        "reduce boot path complexity",
        "keep memory use low",
        "keep X200-class targets viable",
        "support fast B3/B4 iteration",
        "make one boot artifact explicit",
        "Direct path does not mean insecure path",
    ] {
        assert!(doc.contains(text), "missing direct path text {text}: {doc}");
    }
}

#[test]
fn base1_dual_path_delivery_preserves_supervisor_orchestration_path() {
    let doc = std::fs::read_to_string("docs/os/BASE1_DUAL_PATH_DELIVERY.md")
        .expect("dual-path delivery design doc");

    for text in [
        "Path B: supervisor orchestration delivery",
        "Base1 supervisor/control plane",
        "staged GNU/Linux kernel environment",
        "staged OpenBSD boot environment",
        "Base1 runtime/service environment",
        "shared policy/log/evidence bus",
        "isolate staged kernels and workflows",
        "enable policy-controlled execution",
        "grow toward supervisor/hypervisor behavior only when evidence supports it",
        "must not be described as a hypervisor",
    ] {
        assert!(doc.contains(text), "missing supervisor path text {text}: {doc}");
    }
}

#[test]
fn base1_dual_path_delivery_defines_shared_contract_and_profiles() {
    let doc = std::fs::read_to_string("docs/os/BASE1_DUAL_PATH_DELIVERY.md")
        .expect("dual-path delivery design doc");

    for text in [
        "Profile names",
        "x86_64-vm-validation",
        "x200-supervisor-lite",
        "Evidence state",
        "not_claimed",
        "evidence-incomplete",
        "evidence-present",
        "reviewed",
        "Boot artifact",
        "VM/hardware profile",
        "Storage tiers",
        "Do not call storage true RAM",
        "Non-claims",
        "Compatibility",
    ] {
        assert!(doc.contains(text), "missing shared contract text {text}: {doc}");
    }
}

#[test]
fn base1_dual_path_delivery_defines_x200_and_delivery_modes() {
    let doc = std::fs::read_to_string("docs/os/BASE1_DUAL_PATH_DELIVERY.md")
        .expect("dual-path delivery design doc");

    for text in [
        "X200 policy",
        "4 GB ThinkPad X200-class target",
        "profile: x200-supervisor-lite",
        "mode: direct-first when boot delivery matters",
        "mode: supervisor-lite when staged evidence matters",
        "concurrency: 1 active staged kernel by default",
        "direct-first",
        "supervisor-lite",
        "supervisor-concurrent",
        "workstation-supervisor",
    ] {
        assert!(doc.contains(text), "missing X200/mode text {text}: {doc}");
    }
}

#[test]
fn base1_dual_path_delivery_preserves_non_claims() {
    let doc = std::fs::read_to_string("docs/os/BASE1_DUAL_PATH_DELIVERY.md")
        .expect("dual-path delivery design doc");

    for text in [
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "hypervisor-ready",
        "daily-driver ready",
        "first-kernel delivery and supervisor orchestration",
        "without fragmenting the project or slowing Phase1 growth",
    ] {
        assert!(doc.contains(text), "missing non-claim text {text}: {doc}");
    }
}
