use std::fs;

#[test]
fn homepage_includes_phase_b_landing_sections() {
    let html = read("index.html");
    assert_contains_all(
        &html,
        &[
            "Try the phase1 prompt before you clone it.",
            "Clone &amp; run",
            "Try the console",
            "Everything feels like a console",
            "From landing page to full public front door.",
            "Fuel phase1 and the Bryforge roadmap.",
            "phase1 is a terminal-first advanced operator kernel",
        ],
    );
}

#[test]
fn homepage_preserves_project_identity_and_metadata() {
    let html = read("index.html");
    assert_contains_all(
        &html,
        &[
            "name\": \"phase1\"",
            "Chase Bryan",
            "Bryforge",
            "GPL-3.0",
            "Rust",
            "secure · private · powerful · open",
            "https://github.com/Bryforge/phase1",
            "https://www.buymeacoffee.com/bryforge",
            "https://bryforge.github.io/phase1/",
            "SoftwareSourceCode",
        ],
    );
}

#[test]
fn homepage_keeps_static_offline_friendly_dependency_posture() {
    let html = read("index.html");
    assert_contains_all(&html, &["./styles.css", "./site.js", "./button-fix.css"]);
    assert_not_contains_any(
        &html,
        &[
            "cdn.tailwindcss.com",
            "unpkg.com",
            "cdnjs.cloudflare.com",
            "jsdelivr.net",
            "https://fonts.googleapis.com",
        ],
    );
}

#[test]
fn styles_cover_terminal_roadmap_mobile_and_reveal_states() {
    let css = read("styles.css");
    assert_contains_all(
        &css,
        &[
            ".terminal-output",
            ".quick-commands",
            ".timeline li.active",
            ".nav-toggle",
            ".reveal.is-visible",
            "prefers-reduced-motion",
            "@media (max-width: 860px)",
        ],
    );
}

#[test]
fn website_mobile_fix_prevents_fragmented_headings_and_duplicate_creator_labels() {
    let fix_css = read("button-fix.css");
    assert_contains_all(
        &fix_css,
        &[
            "Creator-section cleanup",
            "Created by Chase Bryan",
            ".profile-label,",
            ".founder-copy > .eyebrow",
            "overflow-wrap: normal",
            "word-break: normal",
            "text-wrap: balance",
            "Mobile readability",
        ],
    );
    assert_not_contains_any(&fix_css, &["Builder profile", "builder profile"]);
}

#[test]
fn site_js_implements_canvas_terminal_progressive_enhancement_and_performance_guards() {
    let js = read("site.js");
    assert_contains_all(
        &js,
        &[
            "setupTerminalDemo",
            "demoResponses",
            "wiki-quick",
            "phase1 // advanced operator kernel",
            "stable: v4.0.0",
            "previous stable: v3.10.9",
            "safe mode: on",
            "setupNavigation",
            "setupReveals",
            "IntersectionObserver",
            "prefers-reduced-motion: reduce",
            "scheduleResize",
            "handleVisibilityChange",
            "document.hidden",
            "startAnimation",
            "stopAnimation",
            "desktop ? 180 : 210",
        ],
    );
    assert_not_contains_any(&js, &["edge: v4.0.0-dev"]);
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains_all(text: &str, needles: &[&str]) {
    for needle in needles {
        assert!(text.contains(needle), "missing {needle:?}");
    }
}

fn assert_not_contains_any(text: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            !text.contains(needle),
            "unexpected external dependency {needle:?}"
        );
    }
}
