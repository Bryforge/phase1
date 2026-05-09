#[test]
fn installer_dry_run_doc_exists_and_defines_safe_command() {
    let doc = std::fs::read_to_string("docs/os/BASE1_INSTALLER_DRY_RUN.md")
        .expect("installer dry-run doc");

    assert!(doc.contains("Base1 installer dry-run design"), "{doc}");
    assert!(
        doc.contains("base1 install --dry-run --target <disk>"),
        "{doc}"
    );
    assert!(
        doc.contains("must refuse to run without `--dry-run`"),
        "{doc}"
    );
    assert!(doc.contains("no disk writes occurred"), "{doc}");
    assert!(
        doc.contains("Partition disks") || doc.contains("partition disks"),
        "{doc}"
    );
}

#[test]
fn readme_links_installer_dry_run_design() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("docs/os/BASE1_INSTALLER_DRY_RUN.md"),
        "{readme}"
    );
    assert!(
        readme.contains("Base1 installer dry-run design"),
        "{readme}"
    );
}

#[test]
fn os_roadmap_links_installer_dry_run_design() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("os roadmap");

    assert!(roadmap.contains("BASE1_INSTALLER_DRY_RUN.md"), "{roadmap}");
    assert!(roadmap.contains("installer dry-run design"), "{roadmap}");
}
