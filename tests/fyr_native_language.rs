use std::fs;

#[test]
fn fyr_language_spec_and_examples_exist() {
    let spec =
        fs::read_to_string("PHASE1_NATIVE_LANGUAGE.md").expect("native language spec exists");
    assert!(spec.contains("Name: Fyr"));
    assert!(spec.contains("Extension: .fyr"));
    assert!(spec.contains("Command: fyr"));
    assert!(spec.contains("Phase1-native language target"));

    let hello = fs::read_to_string("examples/fyr/hello.fyr").expect("fyr hello example exists");
    assert!(hello.contains("fn main() -> i32"));
    assert!(hello.contains("Hello from"));

    let self_check =
        fs::read_to_string("examples/fyr/self_check.fyr").expect("fyr self check example exists");
    assert!(self_check.contains("Phase1"));
    assert!(self_check.contains("Fyr"));
}
