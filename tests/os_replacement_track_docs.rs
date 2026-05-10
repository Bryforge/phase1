#[test]
fn readme_links_phase1_operating_system_track() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(readme.contains("docs/os/ROADMAP.md"), "{readme}");
    assert!(readme.contains("Phase1 operating-system track"), "{readme}");
    assert!(readme.contains("not yet a kernel"), "{readme}");
    assert!(
        readme.contains("not a current drop-in OS replacement"),
        "{readme}"
    );
}

#[test]
fn os_track_roadmap_documents_guarded_stages() {
    let doc = std::fs::read_to_string("docs/os/ROADMAP.md").expect("docs/os/ROADMAP.md");

    assert!(doc.contains("Phase1 operating-system track"), "{doc}");
    assert!(
        doc.contains("Base1 provides the minimal trusted host foundation"),
        "{doc}"
    );
    assert!(doc.contains("Stage 1: Base1 bootable foundation"), "{doc}");
    assert!(doc.contains("Stage 2: Installer and recovery"), "{doc}");
    assert!(doc.contains("Stage 3: Daily-driver basics"), "{doc}");
    assert!(
        doc.contains("Stage 4: Phase1-owned system surface"),
        "{doc}"
    );
    assert!(doc.contains("Stage 5: Hardware targets"), "{doc}");
    assert!(
        doc.contains("No claim that the current terminal console replaces a kernel"),
        "{doc}"
    );
}
