use std::fs;

#[test]
fn fyr_brand_and_roadmap_are_documented() {
    let readme = fs::read_to_string("README.md").expect("README exists");
    assert!(readme.contains("assets/fyr_symbol.png"));
    assert!(readme.contains("assets/fyr_word.png"));
    assert!(readme.contains("docs/project/PHASE1_NATIVE_LANGUAGE.md"));
    assert!(readme.contains("docs/fyr/ROADMAP.md"));
    assert!(readme.contains("fyr run hello_hacker.fyr"));

    let spec =
        fs::read_to_string("docs/project/PHASE1_NATIVE_LANGUAGE.md").expect("Fyr spec exists");
    assert!(spec.contains("Name: Fyr"));
    assert!(spec.contains("Extension: .fyr"));
    assert!(spec.contains("Visual symbol: [`assets/fyr_symbol.png`](assets/fyr_symbol.png)"));
    assert!(spec.contains("Word mark: [`assets/fyr_word.png`](assets/fyr_word.png)"));
    assert!(spec.contains("Roadmap: [`docs/fyr/ROADMAP.md`](docs/fyr/ROADMAP.md)"));

    let roadmap = fs::read_to_string("docs/fyr/ROADMAP.md").expect("Fyr roadmap exists");
    assert!(roadmap.contains("Fyr language roadmap"));
    assert!(roadmap.contains("F0 — Identity"));
    assert!(roadmap.contains("F1 — Seed runner"));
    assert!(roadmap.contains("F2 — Authoring loop"));
    assert!(roadmap.contains("F7 — Compiler path"));
    assert!(roadmap.contains("assets/fyr_symbol.png"));
    assert!(roadmap.contains("assets/fyr_word.png"));

    let symbol = fs::metadata("assets/fyr_symbol.png").expect("Fyr symbol image exists");
    let word = fs::metadata("assets/fyr_word.png").expect("Fyr word mark image exists");
    assert!(symbol.len() > 0, "Fyr symbol image should not be empty");
    assert!(word.len() > 0, "Fyr word mark image should not be empty");
}
