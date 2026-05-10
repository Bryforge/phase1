#[test]
fn readme_links_base1_image_builder_design() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");

    assert!(
        readme.contains("docs/os/BASE1_IMAGE_BUILDER.md"),
        "{readme}"
    );
    assert!(readme.contains("Base1 image-builder design"), "{readme}");
}

#[test]
fn image_builder_doc_defines_stage1_boot_surface() {
    let doc = std::fs::read_to_string("docs/os/BASE1_IMAGE_BUILDER.md").expect("image builder doc");

    assert!(doc.contains("Base1 image-builder design"), "{doc}");
    assert!(doc.contains("Read-only base layer"), "{doc}");
    assert!(doc.contains("Writable user/data layer"), "{doc}");
    assert!(doc.contains("Phase1 auto-launch"), "{doc}");
    assert!(doc.contains("Emergency shell fallback"), "{doc}");
    assert!(doc.contains("Recovery boot path"), "{doc}");
    assert!(doc.contains("No silent host trust escalation"), "{doc}");
}

#[test]
fn os_roadmap_links_image_builder_design() {
    let roadmap = std::fs::read_to_string("docs/os/ROADMAP.md").expect("os roadmap");

    assert!(roadmap.contains("Base1 image-builder design"), "{roadmap}");
    assert!(roadmap.contains("BASE1_IMAGE_BUILDER.md"), "{roadmap}");
}
