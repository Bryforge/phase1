#[test]
fn registry_help_includes_visual_palette_and_flows() {
    let registry = std::fs::read_to_string("src/registry.rs").expect("registry.rs");

    assert!(registry.contains("command_palette"), "{registry}");
    assert!(registry.contains("operator_flows"), "{registry}");
    assert!(registry.contains("help ui // v6"), "{registry}");
    assert!(registry.contains("hot zones"), "{registry}");
    assert!(registry.contains("launch examples"), "{registry}");
    assert!(registry.contains("phase1 help // workflows"), "{registry}");
}

#[test]
fn command_map_promotes_visual_help_routes() {
    let registry = std::fs::read_to_string("src/registry.rs").expect("registry.rs");

    assert!(
        registry.contains("help ui          visual command palette"),
        "{registry}"
    );
    assert!(
        registry.contains("help flows       workflow launch paths"),
        "{registry}"
    );
    assert!(
        registry.contains("launch the command palette"),
        "{registry}"
    );
}

#[test]
fn boot_live_ops_points_to_visual_help() {
    let boot = std::fs::read_to_string("src/boot_ui_static.rs").expect("boot ui");

    assert!(
        boot.contains("help ui | help --compact | help host"),
        "{boot}"
    );
    assert!(
        boot.contains("help flows | theme list | update protocol"),
        "{boot}"
    );
}

#[test]
fn help_dispatch_passes_arguments_to_registry_help() {
    let commands = std::fs::read_to_string("src/commands.rs").expect("commands.rs");

    assert!(
        commands.contains("\"help\" => print!(\"{}\", registry::help(args))"),
        "{commands}"
    );
}

#[test]
fn runtime_main_help_dispatch_passes_arguments_to_registry_help() {
    let main = std::fs::read_to_string("src/main.rs").expect("main.rs");

    assert!(main.contains("registry::help(args)"), "{main}");
    assert!(!main.contains("ui::print_help()"), "{main}");
}
