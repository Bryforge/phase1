#[test]
fn installer_recovery_doc_exists_and_defines_contracts() {
    let doc =
        std::fs::read_to_string("docs/os/INSTALLER_RECOVERY.md").expect("installer recovery doc");

    assert!(doc.contains("Base1 installer and recovery design"), "{doc}");
    assert!(doc.contains("Non-destructive dry-run mode"), "{doc}");
    assert!(doc.contains("Explicit target-disk selection"), "{doc}");
    assert!(doc.contains("Emergency shell access"), "{doc}");
    assert!(doc.contains("Disable Phase1 auto-launch"), "{doc}");
    assert!(
        doc.contains("Do not run destructive disk commands"),
        "{doc}"
    );
}

#[test]
fn readme_links_installer_recovery_design() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(readme.contains("docs/os/INSTALLER_RECOVERY.md"), "{readme}");
    assert!(
        readme.contains("Base1 installer and recovery design"),
        "{readme}"
    );
}

#[test]
fn os_roadmap_mentions_installer_recovery_design() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("os roadmap");

    assert!(roadmap.contains("INSTALLER_RECOVERY.md"), "{roadmap}");
    assert!(
        roadmap.contains("installer and recovery design"),
        "{roadmap}"
    );
}
