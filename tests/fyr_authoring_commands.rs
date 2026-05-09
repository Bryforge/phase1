use std::fs;

#[test]
fn fyr_authoring_commands_are_wired() {
    let main = fs::read_to_string("src/main.rs").expect("main source exists");
    assert!(main.contains("Some(\"new\") => fyr_new(shell, &args[1..])"));
    assert!(main.contains("Some(\"init\") => fyr_init(shell, &args[1..])"));
    assert!(main.contains("Some(\"cat\") => fyr_cat(shell, &args[1..])"));
    assert!(main.contains("Some(\"check\") => fyr_check(shell, &args[1..])"));
    assert!(main.contains("Some(\"build\") => fyr_build(shell, &args[1..])"));
    assert!(main.contains("Some(\"self\") => fyr_self()"));
    assert!(main.contains("fn fyr_new(shell: &mut Phase1Shell"));
    assert!(main.contains("fn fyr_init(shell: &mut Phase1Shell"));
    assert!(main.contains("fn fyr_cat(shell: &mut Phase1Shell"));
    assert!(main.contains("fn fyr_check(shell: &mut Phase1Shell"));
    assert!(main.contains("fn fyr_build(shell: &mut Phase1Shell"));
    assert!(main.contains("fn fyr_self()"));
    assert!(main.contains("fn fyr_file_name(raw: &str)"));
    assert!(main.contains("sys_write(&path, source, false)"));

    let registry = fs::read_to_string("src/registry.rs").expect("registry source exists");
    assert!(registry.contains("fyr [status|spec|new|init|cat|check|build|self|run <file.fyr>]"));

    let spec = fs::read_to_string("PHASE1_NATIVE_LANGUAGE.md").expect("Fyr spec exists");
    assert!(spec.contains("fyr new hello_hacker"));
    assert!(spec.contains("fyr cat hello_hacker.fyr"));
    assert!(spec.contains("fyr self"));
}
