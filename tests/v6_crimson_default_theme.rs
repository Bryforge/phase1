#[test]
fn v7_edge_defaults_to_neotokyo_theme_when_theme_is_missing_or_empty() {
    let boot = std::fs::read_to_string("src/boot_ui_static.rs").expect("boot ui");

    assert!(boot.contains("ThemePalette::NeoTokyo.name()"), "{boot}");
    assert!(boot.contains("theme.trim().is_empty()"), "{boot}");
    assert!(boot.contains("unwrap_or(true)"), "{boot}");
}

#[test]
fn v7_edge_selector_should_default_display_to_neotokyo() {
    let boot = std::fs::read_to_string("src/boot_ui_static.rs").expect("boot ui");

    assert!(boot.contains("neo-tokyo"), "{boot}");
    assert!(!boot.contains("display    crimson"), "{boot}");
    assert!(!boot.contains("display    bleeding-edge"), "{boot}");
}

#[test]
fn bleeding_edge_theme_remains_available_as_legacy_manual_palette() {
    let boot = std::fs::read_to_string("src/boot_ui_static.rs").expect("boot ui");

    assert!(boot.contains("Self::BleedingEdge"), "{boot}");
    assert!(
        boot.contains("legacy edge-only blue/magenta update channel console"),
        "{boot}"
    );
}
