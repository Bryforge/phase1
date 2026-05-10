use std::fs;

#[test]
fn ai_collaboration_guardrails_doc_exists() {
    let doc = fs::read_to_string("docs/developer/AI_COLLABORATION_GUARDRAILS.md").unwrap();
    assert!(doc.contains("Phase1 AI Collaboration Guardrails"));
    assert!(doc.contains("Status: active developer guidance"));
    assert!(doc.contains("AI-assisted Phase1 development"));
}

#[test]
fn ai_collaboration_guardrails_separate_sources_and_inferences() {
    let doc = fs::read_to_string("docs/developer/AI_COLLABORATION_GUARDRAILS.md").unwrap();
    assert!(doc.contains("user instruction"));
    assert!(doc.contains("project fact"));
    assert!(doc.contains("observed evidence"));
    assert!(doc.contains("agent inference"));
    assert!(doc.contains("recommendation"));
    assert!(doc.contains("unvalidated hypothesis"));
}

#[test]
fn ai_collaboration_guardrails_preserve_phase1_practice() {
    let doc = fs::read_to_string("docs/developer/AI_COLLABORATION_GUARDRAILS.md").unwrap();
    assert!(doc.contains("record validated commands"));
    assert!(doc.contains("record non-claims"));
    assert!(doc.contains("separate planning from promotion"));
    assert!(doc.contains("prefer dry-run/read-only validation"));
}

#[test]
fn ai_collaboration_guardrails_include_agent_and_config_rules() {
    let doc = fs::read_to_string("docs/developer/AI_COLLABORATION_GUARDRAILS.md").unwrap();
    assert!(doc.contains("Config Visibility"));
    assert!(doc.contains("Agent Prompt Self-Sufficiency"));
    assert!(doc.contains("Bounded Agent Output"));
    assert!(doc.contains("One Change, One Validation"));
}

#[test]
fn developer_index_links_ai_collaboration_guardrails() {
    let index = fs::read_to_string("docs/developer/README.md").unwrap_or_default();
    assert!(index.contains("AI_COLLABORATION_GUARDRAILS.md"));
}
