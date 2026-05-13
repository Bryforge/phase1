#[test]
fn website_docs_reference_current_public_asset_components() {
    let website =
        std::fs::read_to_string("docs/website/README.md").expect("website documentation index");

    for asset in [
        "../../assets/phase1_base_fyr_banner1.png",
        "../../assets/phase1_word.png",
        "../../assets/fyr_symbol.png",
        "../../assets/fyr_word.png",
    ] {
        assert!(
            website.contains(asset),
            "website docs missing current asset component {asset}: {website}"
        );
    }
}

#[test]
fn website_docs_mark_outdated_visual_assets_without_using_them_as_current() {
    let website =
        std::fs::read_to_string("docs/website/README.md").expect("website documentation index");

    for text in [
        "Older references to `assets/phase1-splash.png`, `assets/phase1-splash.svg`, and `assets/phase1-logo.svg` are outdated",
        "Older references to `assets/fyr-flame.svg` are outdated",
        "current Phase1 word-mark/splash PNG",
    ] {
        assert!(website.contains(text), "missing website asset note {text}: {website}");
    }
}

#[test]
fn asset_index_and_readme_agree_on_current_public_assets() {
    let assets = std::fs::read_to_string("assets/README.md").expect("asset index");
    let readme = std::fs::read_to_string("README.md").expect("README");

    for asset in [
        "assets/phase1_base_fyr_banner1.png",
        "assets/phase1_word.png",
        "assets/fyr_symbol.png",
        "assets/fyr_word.png",
    ] {
        assert!(
            assets.contains(asset),
            "asset index missing {asset}: {assets}"
        );
        assert!(readme.contains(asset), "README missing {asset}: {readme}");
    }
}

#[test]
fn current_public_asset_files_exist() {
    for asset in [
        "assets/phase1_base_fyr_banner1.png",
        "assets/phase1_word.png",
        "assets/fyr_symbol.png",
        "assets/fyr_word.png",
    ] {
        let metadata = std::fs::metadata(asset).expect(asset);
        assert!(metadata.len() > 0, "asset should not be empty: {asset}");
    }
}
