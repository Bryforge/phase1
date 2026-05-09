use std::fs;

#[test]
fn fyr_command_stub_is_wired() {
    let main = fs::read_to_string("src/main.rs").expect("main source exists");
    assert!(main.contains("\"fyr\" => print!(\"{}\", fyr_command(shell, args))"));
    assert!(main.contains("fn fyr_command(shell: &Phase1Shell"));
    assert!(main.contains("fn fyr_run(shell: &Phase1Shell"));
    assert!(main.contains("fn fyr_print_output"));
    assert!(main.contains("Hello, hacker!"));

    let registry = fs::read_to_string("src/registry.rs").expect("registry source exists");
    assert!(registry.contains("cmd!(\"fyr\""));
    assert!(registry.contains("phase1lang"));
    assert!(registry.contains("forge"));
}
