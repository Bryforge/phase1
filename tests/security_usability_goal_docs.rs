#[test]
fn security_policy_defines_security_usability_goal() {
    let security = std::fs::read_to_string("SECURITY.md").expect("SECURITY.md");

    assert!(security.contains("Security goal"), "{security}");
    assert!(
        security.contains("as secure as possible while maintaining practical usability"),
        "{security}"
    );
    assert!(
        security.contains("secure defaults with intentional opt-in escape hatches"),
        "{security}"
    );
    assert!(
        security.contains("read-only and dry-run flows before real mutation"),
        "{security}"
    );
}

#[test]
fn security_policy_links_crypto_policy_goal() {
    let security = std::fs::read_to_string("SECURITY.md").expect("SECURITY.md");

    assert!(security.contains("Cryptographic policy goal"), "{security}");
    assert!(
        security.contains("docs/security/CRYPTO_POLICY_ROADMAP.md"),
        "{security}"
    );
    assert!(
        security.contains("advanced operators inspect and intentionally select approved cryptographic profiles by control point"),
        "{security}"
    );
    assert!(
        security.contains("reject custom security-critical primitives"),
        "{security}"
    );
    assert!(
        security.contains("does not make Phase1 cryptographically complete"),
        "{security}"
    );
}

#[test]
fn trust_model_defines_security_and_usability_principle() {
    let trust = std::fs::read_to_string("docs/security/TRUST_MODEL.md")
        .expect("trust model");

    assert!(
        trust.contains("Security and usability principle"),
        "{trust}"
    );
    assert!(
        trust.contains("as secure as possible while maintaining practical usability"),
        "{trust}"
    );
    assert!(
        trust.contains("secure defaults with usable opt-in paths"),
        "{trust}"
    );
    assert!(
        trust.contains("Security that blocks normal safe work without adding meaningful protection should be redesigned"),
        "{trust}"
    );
}

#[test]
fn security_review_checklist_includes_usability_review() {
    let trust = std::fs::read_to_string("docs/security/TRUST_MODEL.md")
        .expect("trust model");

    assert!(
        trust.contains("Security controls preserve practical usability or explain why restriction is necessary."),
        "{trust}"
    );
}

#[test]
fn main_readme_links_security_crypto_policy() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(readme.contains("docs/security/CRYPTO_POLICY_ROADMAP.md"), "{readme}");
    assert!(
        readme.contains("approved cryptographic profiles by control point"),
        "{readme}"
    );
    assert!(
        readme.contains("normal users keep safe defaults"),
        "{readme}"
    );
}
