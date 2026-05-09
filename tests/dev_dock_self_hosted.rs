use std::fs;

#[test]
fn dev_dock_exposes_self_hosted_workflow_commands() {
    let dev = fs::read_to_string("plugins/dev.py").expect("plugins/dev.py exists");
    assert!(dev.contains("dev docs"));
    assert!(dev.contains("dev checkpoint"));
    assert!(dev.contains("no changes"));
    assert!(dev.contains("scripts/update-docs.py"));
}

#[test]
fn dev_dock_docs_explain_inside_phase1_workflow() {
    let docs = fs::read_to_string("DEV_DOCK.md").expect("DEV_DOCK.md exists");
    assert!(docs.contains("dev docs"));
    assert!(docs.contains("dev checkpoint"));
    assert!(docs.contains("inside Phase1"));
}
