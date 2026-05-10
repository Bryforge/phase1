#[test]
fn theme_status_alias_and_edge_defaults_use_crimson() {
    let operator = std::fs::read_to_string("src/operator.rs").expect("operator.rs");

    assert!(
        operator.contains(r#"None | Some("show") | Some("status") => theme_status(shell)"#),
        "{operator}"
    );
    assert!(operator.contains("ThemePalette::Crimson"), "{operator}");
    assert!(
        operator.contains("theme: reset to crimson edge default"),
        "{operator}"
    );
    assert!(
        operator.contains("theme: crimson edge default enabled"),
        "{operator}"
    );
    assert!(
        !operator.contains("theme: reset to bleeding-edge default"),
        "{operator}"
    );
    assert!(
        !operator.contains("theme: bleeding-edge default enabled"),
        "{operator}"
    );
}
