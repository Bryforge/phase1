#[test]
fn base1_readme_links_network_lockdown_preview() {
    let readme = std::fs::read_to_string("base1/README.md").expect("base1 readme");

    assert!(readme.contains("NETWORK_LOCKDOWN_DRY_RUN.md"), "{readme}");
    assert!(
        readme.contains("base1-network-lockdown-dry-run.sh"),
        "{readme}"
    );
    assert!(readme.contains("writes: no"), "{readme}");
}

#[test]
fn network_lockdown_doc_defines_read_only_contract() {
    let doc = std::fs::read_to_string("base1/NETWORK_LOCKDOWN_DRY_RUN.md")
        .expect("network lockdown dry-run doc");

    assert!(doc.contains("Base1 network lockdown dry-run"), "{doc}");
    assert!(doc.contains("Read-only guarantee"), "{doc}");
    assert!(doc.contains("secure-default"), "{doc}");
    assert!(doc.contains("offline"), "{doc}");
    assert!(doc.contains("appliance"), "{doc}");
    assert!(doc.contains("dev"), "{doc}");
    assert!(doc.contains("Recovery path"), "{doc}");
    assert!(doc.contains("Rollback path"), "{doc}");
    assert!(doc.contains("remote access can be lost"), "{doc}");
}

#[test]
fn network_lockdown_script_requires_dry_run_and_reports_no_writes() {
    let script = std::fs::read_to_string("scripts/base1-network-lockdown-dry-run.sh")
        .expect("network lockdown dry-run script");

    assert!(
        script.contains("refusing to run without --dry-run"),
        "{script}"
    );
    assert!(script.contains("info 'mode: dry-run'"), "{script}");
    assert!(script.contains("info 'writes: no'"), "{script}");
    assert!(script.contains("planned-inbound: deny"), "{script}");
    assert!(script.contains("planned-outbound"), "{script}");
    assert!(
        script.contains("planned-phase1-host-tools: denied"),
        "{script}"
    );
}

#[test]
fn dry_run_command_index_includes_network_lockdown() {
    let index = std::fs::read_to_string("docs/os/BASE1_DRY_RUN_COMMANDS.md")
        .expect("base1 dry-run command index");

    assert!(
        index.contains("sh scripts/base1-network-lockdown-dry-run.sh --dry-run"),
        "{index}"
    );
    assert!(
        index.contains("network lockdown policy without changing firewall or service state"),
        "{index}"
    );
}
