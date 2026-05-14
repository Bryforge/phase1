#[test]
fn theme_status_alias_and_edge_defaults_use_neotokyo() {
    let operator = std::fs::read_to_string("src/operator.rs").expect("operator.rs");

    assert!(
        operator.contains(r#"None | Some("show") | Some("status") => theme_status(shell)"#),
        "{operator}"
    );
    assert!(operator.contains("ThemePalette::NeoTokyo"), "{operator}");
    assert!(
        operator.contains("theme: reset to neo-tokyo default"),
        "{operator}"
    );
    assert!(
        operator.contains("theme: neo-tokyo default enabled"),
        "{operator}"
    );
    assert!(
        !operator.contains("theme: reset to crimson edge default"),
        "{operator}"
    );
    assert!(
        !operator.contains("theme: crimson edge default enabled"),
        "{operator}"
    );
}
