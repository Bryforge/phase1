use std::fs;

#[test]
fn roadmap_points_to_next_design_implementation_docs() {
    let roadmap = read("docs/project/WIKI_ROADMAP.md");
    assert_contains_all(
        &roadmap,
        &[
            "docs/website/NEXT_ROADMAP_IMPLEMENTATION.md",
            "docs/website/PROJECT_AND_COMPANY_PAGES.md",
            "project.html",
            "company.html",
            "roadmap.html",
            "support.html",
            "Phase B — Product and company page foundation",
            "Start with **PR B1: phase1 project page**",
        ],
    );
}

#[test]
fn next_roadmap_implementation_has_clear_pr_sequence() {
    let plan = read("docs/website/NEXT_ROADMAP_IMPLEMENTATION.md");
    assert_contains_all(
        &plan,
        &[
            "phase1 product track",
            "Bryforge company track",
            "PR A: Project page foundation",
            "PR B: Company page foundation",
            "PR C: Roadmap page foundation",
            "PR D: Wiki hub expansion",
            "The next implementation should start with `project.html`",
        ],
    );
}

#[test]
fn project_and_company_pages_stay_distinct() {
    let plan = read("docs/website/PROJECT_AND_COMPANY_PAGES.md");
    assert_contains_all(
        &plan,
        &[
            "phase1 | The Rust virtual OS console",
            "Bryforge | The startup software development company behind phase1",
            "Chase Bryan | Founder, computer scientist, and developer",
            "project.html",
            "company.html",
            "Do not use multiple competing labels",
            "Avoid duplicate labels such as founder profile plus builder profile",
            "no `Builder profile` label appears",
        ],
    );
}

#[test]
fn direction_plan_preserves_static_security_posture() {
    for path in [
        "docs/project/WIKI_ROADMAP.md",
        "docs/website/NEXT_ROADMAP_IMPLEMENTATION.md",
        "docs/website/PROJECT_AND_COMPANY_PAGES.md",
    ] {
        let text = read(path);
        assert!(
            text.contains("No external JavaScript dependencies")
                || text.contains("no external CDN dependency")
                || text.contains("no external CDN dependencies"),
            "{path} should preserve the static/offline-friendly dependency posture"
        );
    }
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains_all(text: &str, needles: &[&str]) {
    for needle in needles {
        assert!(text.contains(needle), "missing {needle:?}");
    }
}
