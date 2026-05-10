#[test]
fn help_dispatch_uses_topic_aware_registry_help() {
    let commands = std::fs::read_to_string("src/commands.rs").expect("commands.rs");

    assert!(
        commands.contains("\"help\" => print!(\"{}\", registry::help(args))"),
        "{commands}"
    );
}

#[test]
fn registry_help_exposes_modern_operator_hud() {
    let registry = std::fs::read_to_string("src/registry.rs").expect("registry.rs");

    assert!(
        registry.contains("phase1 help // operator HUD"),
        "{registry}"
    );
    assert!(registry.contains("help --compact"), "{registry}");
    assert!(registry.contains("help <category>"), "{registry}");
    assert!(registry.contains("help <command>"), "{registry}");
    assert!(registry.contains("quick routes"), "{registry}");
    assert!(registry.contains("category_help"), "{registry}");
    assert!(registry.contains("compact_command_map"), "{registry}");
}

#[test]
fn boot_live_ops_points_to_new_help_routes() {
    let boot = std::fs::read_to_string("src/boot_ui_static.rs").expect("boot ui");

    assert!(
        boot.contains("help --compact | help sys | help host"),
        "{boot}"
    );
    assert!(
        boot.contains("dash | sysinfo | security | opslog"),
        "{boot}"
    );
    assert!(
        boot.contains("theme list | tips | update protocol"),
        "{boot}"
    );
}
