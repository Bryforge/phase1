use std::fs;

#[test]
fn updater_edge_stable_evidence_report_exists() {
    let doc = fs::read_to_string("docs/update/2026-05-10-updater-edge-stable-target.md").unwrap();
    assert!(doc.contains("Phase1 Updater Edge Stable Target Evidence"));
    assert!(doc.contains("updater fix validated"));
}

#[test]
fn updater_edge_stable_evidence_records_target_and_success_path() {
    let doc = fs::read_to_string("docs/update/2026-05-10-updater-edge-stable-target.md").unwrap();
    assert!(doc.contains("target     : origin/edge/stable"));
    assert!(doc.contains("git fetch           [ok]"));
    assert!(doc.contains("git checkout        [ok]"));
    assert!(doc.contains("git pull --ff-only  [ok]"));
    assert!(doc.contains("cargo build --release [ok]"));
    assert!(doc.contains("update: complete"));
}

#[test]
fn updater_edge_stable_evidence_preserves_non_claims() {
    let doc = fs::read_to_string("docs/update/2026-05-10-updater-edge-stable-target.md").unwrap();
    assert!(doc.contains("No installer readiness claim"));
    assert!(doc.contains("No hardware validation claim"));
    assert!(doc.contains("No daily-driver claim"));
    assert!(doc.contains("No destructive disk writes"));
    assert!(doc.contains("No real-device write path"));
}

#[test]
fn update_protocol_links_updater_edge_stable_evidence() {
    let protocol = fs::read_to_string("UPDATE_PROTOCOL.md").unwrap_or_default();
    assert!(protocol.contains("2026-05-10-updater-edge-stable-target.md"));
}
