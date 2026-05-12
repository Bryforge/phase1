#[test]
fn asset_index_documents_current_public_assets() {
    let index = std::fs::read_to_string("assets/README.md").expect("assets index");

    assert!(index.contains("Phase1 assets"), "{index}");
    assert!(index.contains("current asset filenames"), "{index}");

    for asset in [
        "assets/phase1_base_fyr_banner1.png",
        "assets/phase1-splash.png",
        "assets/fyr_symbol.png",
        "assets/fyr_word.png",
    ] {
        assert!(
            index.contains(asset),
            "missing asset reference {asset}: {index}"
        );
    }
}

#[test]
fn asset_index_marks_old_phase1_splash_svg_reference_outdated() {
    let index = std::fs::read_to_string("assets/README.md").expect("assets index");

    assert!(
        index.contains("Older references to `phase1-splash.svg` are outdated"),
        "{index}"
    );
    assert!(
        index.contains("should be replaced with the current PNG asset above"),
        "{index}"
    );
}

#[test]
fn asset_index_marks_old_fyr_flame_reference_outdated() {
    let index = std::fs::read_to_string("assets/README.md").expect("assets index");

    assert!(
        index.contains("Older references to `fyr-flame.svg` are outdated"),
        "{index}"
    );
    assert!(
        index.contains("should be replaced with the current PNG assets"),
        "{index}"
    );
}

#[test]
fn asset_index_defines_naming_update_and_safety_rules() {
    let index = std::fs::read_to_string("assets/README.md").expect("assets index");

    for text in [
        "Naming policy",
        "phase1_*.png",
        "base1_*.png",
        "fyr_*.png",
        "bryforge_*.png",
        "Documentation update rule",
        "Do not commit private, unrevised, accidental, credential-bearing, or sensitive screenshots.",
        "Public assets should not contain secrets, tokens, private keys, recovery codes, personal credentials, private logs, or unrevised device/account screenshots.",
    ] {
        assert!(index.contains(text), "missing asset policy text {text}: {index}");
    }
}

#[test]
fn asset_index_referenced_from_readme_and_website_docs() {
    let readme = std::fs::read_to_string("README.md").expect("README.md");
    let website = std::fs::read_to_string("docs/website/README.md").expect("website docs index");

    assert!(website.contains("../../assets/README.md"), "{website}");
    assert!(readme.contains("Public assets and branding"), "{readme}");

    for asset in [
        "assets/phase1_base_fyr_banner1.png",
        "assets/phase1-splash.png",
        "assets/fyr_symbol.png",
        "assets/fyr_word.png",
    ] {
        assert!(
            readme.contains(asset),
            "README missing asset {asset}: {readme}"
        );
    }

    assert!(
        website.contains("../../assets/phase1-splash.png"),
        "website docs should reference current Phase1 splash PNG: {website}"
    );
}

#[test]
fn current_asset_files_exist_and_are_non_empty() {
    for asset in [
        "assets/phase1_base_fyr_banner1.png",
        "assets/phase1-splash.png",
        "assets/fyr_symbol.png",
        "assets/fyr_word.png",
    ] {
        let metadata = std::fs::metadata(asset).expect(asset);
        assert!(metadata.len() > 0, "asset should not be empty: {asset}");
    }
}
