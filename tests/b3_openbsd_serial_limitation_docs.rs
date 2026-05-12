#[test]
fn b3_openbsd_serial_limitation_doc_defines_scope_and_purpose() {
    let doc = std::fs::read_to_string("docs/os/B3_OPENBSD_SERIAL_LIMITATION.md")
        .expect("B3 OpenBSD serial limitation doc");

    for text in [
        "Base1 B3 OpenBSD serial marker limitation",
        "Status: documented limitation",
        "OpenBSD B3 stage serial-marker evidence boundary",
        "bounded OpenBSD QEMU launch can be staged and checked",
        "expected OpenBSD serial marker is not yet captured",
        "OpenBSD currently has launch-check evidence, not serial-marker boot evidence",
    ] {
        assert!(
            doc.contains(text),
            "missing scope/purpose text {text}: {doc}"
        );
    }
}

#[test]
fn b3_openbsd_serial_limitation_doc_records_observed_behavior() {
    let doc = std::fs::read_to_string("docs/os/B3_OPENBSD_SERIAL_LIMITATION.md")
        .expect("B3 OpenBSD serial limitation doc");

    for text in [
        "marker",
        "launch",
        "requires the expected marker",
        "OpenBSD",
        "captured serial log",
        "OpenBSD launch-check: pass",
        "OpenBSD serial marker-check: not yet captured",
    ] {
        assert!(
            doc.contains(text),
            "missing observed behavior text {text}: {doc}"
        );
    }
}

#[test]
fn b3_openbsd_serial_limitation_doc_defines_interpretation_boundary() {
    let doc = std::fs::read_to_string("docs/os/B3_OPENBSD_SERIAL_LIMITATION.md")
        .expect("B3 OpenBSD serial limitation doc");

    for text in [
        "weaker than marker-check evidence",
        "the local OpenBSD artifact path was accepted",
        "generated a guarded QEMU command",
        "QEMU launched within a bounded timeout",
        "local build/report evidence boundary",
        "does not prove that OpenBSD reached its installer",
        "kernel",
        "userland",
        "expected serial marker",
    ] {
        assert!(
            doc.contains(text),
            "missing interpretation boundary text {text}: {doc}"
        );
    }
}

#[test]
fn b3_openbsd_serial_limitation_doc_lists_tuning_candidates() {
    let doc = std::fs::read_to_string("docs/os/B3_OPENBSD_SERIAL_LIMITATION.md")
        .expect("B3 OpenBSD serial limitation doc");

    for text in [
        "booting an OpenBSD ISO instead of the miniroot raw image",
        "-display cocoa",
        "-serial stdio",
        "-nographic",
        "set tty com0",
        "captures `OpenBSD` in the serial log",
    ] {
        assert!(
            doc.contains(text),
            "missing tuning candidate text {text}: {doc}"
        );
    }
}

#[test]
fn b3_openbsd_serial_limitation_doc_preserves_status_impact_and_non_claims() {
    let doc = std::fs::read_to_string("docs/os/B3_OPENBSD_SERIAL_LIMITATION.md")
        .expect("B3 OpenBSD serial limitation doc");

    for text in [
        "OpenBSD serial marker evidence is captured or documented as a limitation.",
        "does not satisfy a stronger OpenBSD boot validation claim",
        "reviewed log bundle",
        "final validation report",
        "does not make Base1 bootable",
        "installer-ready",
        "recovery-complete",
        "hardened",
        "hardware-validated",
        "release-candidate ready",
        "daily-driver ready",
        "OpenBSD launch-check evidence exists while OpenBSD serial-marker evidence remains a known limitation",
    ] {
        assert!(doc.contains(text), "missing status impact/non-claim text {text}: {doc}");
    }
}
