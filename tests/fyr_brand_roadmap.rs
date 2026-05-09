use std::fs;

#[test]
fn fyr_brand_and_roadmap_are_documented() {
    let readme = fs::read_to_string("README.md").expect("README exists");
    assert!(readme.contains("assets/fyr-flame.svg"));
    assert!(readme.contains("PHASE1_NATIVE_LANGUAGE.md"));
    assert!(readme.contains("docs/fyr/ROADMAP.md"));
    assert!(readme.contains("fyr run hello_hacker.fyr"));

    let spec = fs::read_to_string("PHASE1_NATIVE_LANGUAGE.md").expect("Fyr spec exists");
    assert!(spec.contains("Name: Fyr"));
    assert!(spec.contains("Extension: .fyr"));
    assert!(spec.contains("Visual mark: [`assets/fyr-flame.svg`]"));
    assert!(spec.contains("Roadmap: [`docs/fyr/ROADMAP.md`]"));

    let roadmap = fs::read_to_string("docs/fyr/ROADMAP.md").expect("Fyr roadmap exists");
    assert!(roadmap.contains("Fyr language roadmap"));
    assert!(roadmap.contains("F0 — Identity"));
    assert!(roadmap.contains("F1 — Seed runner"));
    assert!(roadmap.contains("F2 — Authoring loop"));
    assert!(roadmap.contains("F7 — Compiler path"));

    let svg = fs::read_to_string("assets/fyr-flame.svg").expect("Fyr flame image exists");
    assert!(svg.contains("Fyr flame mark"));
    assert!(svg.contains(">fyr<"));
    assert!(svg.contains("Phase1 Native Language"));
}
