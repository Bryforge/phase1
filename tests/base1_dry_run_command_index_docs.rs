#[test]
fn dry_run_command_index_documents_all_current_scripts() {
    let doc = std::fs::read_to_string("docs/os/BASE1_DRY_RUN_COMMANDS.md")
        .expect("dry-run command index");

    assert!(doc.contains("Base1 dry-run command index"), "{doc}");
    assert!(doc.contains("scripts/base1-install-dry-run.sh"), "{doc}");
    assert!(doc.contains("scripts/base1-recovery-dry-run.sh"), "{doc}");
    assert!(
        doc.contains("scripts/base1-storage-layout-dry-run.sh"),
        "{doc}"
    );
    assert!(
        doc.contains("scripts/base1-rollback-metadata-dry-run.sh"),
        "{doc}"
    );
    assert!(doc.contains("Require `--dry-run`"), "{doc}");
    assert!(doc.contains("Report `writes: no`"), "{doc}");
    assert!(doc.contains("Avoid destructive disk tools"), "{doc}");
    assert!(doc.contains("Avoid host trust escalation"), "{doc}");
}

#[test]
fn readme_links_dry_run_command_index() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("docs/os/BASE1_DRY_RUN_COMMANDS.md"),
        "{readme}"
    );
    assert!(readme.contains("Base1 dry-run command index"), "{readme}");
}

#[test]
fn os_roadmap_links_dry_run_command_index() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("os roadmap");

    assert!(roadmap.contains("BASE1_DRY_RUN_COMMANDS.md"), "{roadmap}");
    assert!(roadmap.contains("dry-run command index"), "{roadmap}");
}
