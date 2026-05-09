#[test]
fn recovery_command_doc_exists_and_defines_safe_surface() {
    let doc =
        std::fs::read_to_string("docs/os/BASE1_RECOVERY_COMMAND.md").expect("recovery command doc");

    assert!(doc.contains("Base1 recovery command design"), "{doc}");
    assert!(doc.contains("base1 recovery --dry-run"), "{doc}");
    assert!(doc.contains("base1 recovery status"), "{doc}");
    assert!(doc.contains("base1 recovery plan"), "{doc}");
    assert!(doc.contains("must be read-only"), "{doc}");
    assert!(doc.contains("no recovery changes were made"), "{doc}");
    assert!(
        doc.contains("Modify bootloader entries") || doc.contains("modify bootloader entries"),
        "{doc}"
    );
}

#[test]
fn readme_links_recovery_command_design() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("docs/os/BASE1_RECOVERY_COMMAND.md"),
        "{readme}"
    );
    assert!(readme.contains("Base1 recovery command design"), "{readme}");
}

#[test]
fn os_roadmap_links_recovery_command_design() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("os roadmap");

    assert!(roadmap.contains("BASE1_RECOVERY_COMMAND.md"), "{roadmap}");
    assert!(roadmap.contains("recovery command design"), "{roadmap}");
}
