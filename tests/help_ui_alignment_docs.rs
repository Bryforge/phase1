#[test]
fn help_ui_palette_uses_boot_card_style_helpers() {
    let registry = std::fs::read_to_string("src/registry.rs").expect("registry.rs");

    assert!(registry.contains("fn palette_top"), "{registry}");
    assert!(registry.contains("fn palette_rule"), "{registry}");
    assert!(registry.contains("fn palette_row"), "{registry}");
    assert!(registry.contains("fn palette_bottom"), "{registry}");
    assert!(registry.contains("fn fit_visible"), "{registry}");
    assert!(registry.contains("fn visible_cell_width"), "{registry}");
    assert!(registry.contains("fn char_cell_width"), "{registry}");
    assert!(registry.contains("fn is_wide_cell"), "{registry}");
}

#[test]
fn help_ui_palette_title_is_inside_card_not_border() {
    let registry = std::fs::read_to_string("src/registry.rs").expect("registry.rs");

    assert!(
        registry.contains("out.push_str(&palette_row(\"help ui // v6\", width))"),
        "{registry}"
    );
    assert!(
        registry.contains("out.push_str(&palette_rule(width))"),
        "{registry}"
    );
    assert!(!registry.contains("╭─ help ui // v6"), "{registry}");
    assert!(!registry.contains("format!(\"╭─{title}"), "{registry}");
}

#[test]
fn help_ui_palette_has_mobile_safe_width_and_margin_rows() {
    let registry = std::fs::read_to_string("src/registry.rs").expect("registry.rs");

    assert!(registry.contains("let width = 38"), "{registry}");
    assert!(
        registry.contains("let inner = width.saturating_sub(2)"),
        "{registry}"
    );
    assert!(
        registry.contains("format!(\"│ {fitted}{padding} │\\\\n\")"),
        "{registry}"
    );
    assert!(
        registry.contains("(\"inspect\", \"help <cmd> | man <cmd>\")"),
        "{registry}"
    );
    assert!(
        registry.contains("(\"recover\", \"help host | update protocol\")"),
        "{registry}"
    );
}

#[test]
fn help_ui_palette_starts_on_fresh_line_after_prompt() {
    let registry = std::fs::read_to_string("src/registry.rs").expect("registry.rs");

    assert!(
        registry.contains("let mut out = String::from(\"\\\\n\")"),
        "{registry}"
    );
    assert!(registry.contains("Start on a fresh line"), "{registry}");
}

#[test]
fn help_ui_hot_zones_are_compact_for_mobile_terminals() {
    let registry = std::fs::read_to_string("src/registry.rs").expect("registry.rs");

    assert!(
        registry.contains("CORE   dash sysinfo security opslog"),
        "{registry}"
    );
    assert!(
        registry.contains("BUILD  dev repo fyr lang cargo rustc"),
        "{registry}"
    );
    assert!(
        registry.contains("SAFE   capabilities sandbox audit update"),
        "{registry}"
    );
    assert!(
        !registry.contains("capabilities sandbox audit update protocol"),
        "{registry}"
    );
}
