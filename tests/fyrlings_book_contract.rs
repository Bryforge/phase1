use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyrlings_course_contract_defines_operator_command_surface() {
    let path = "docs/fyr/FYRLINGS_OPERATOR_COURSE.md";

    for command in [
        "fyr learn",
        "fyr learn list",
        "fyr learn run <lesson>",
        "fyr learn hint <lesson>",
        "fyr learn verify",
        "fyr learn reset <lesson>",
        "fyr book",
        "fyr book list",
        "fyr book read <chapter>",
        "fyr book search <term>",
        "fyrlings",
    ] {
        assert_contains(path, command);
    }
}

#[test]
fn fyrlings_course_covers_required_lessons() {
    let path = "docs/fyr/FYRLINGS_OPERATOR_COURSE.md";

    for lesson in [
        "Orientation",
        "Hello Fyr",
        "Main function",
        "Printing",
        "Returning",
        "Fix diagnostics",
        "Bindings",
        "Expressions",
        "Branching",
        "Equality asserts",
        "Boolean asserts",
        "Comparisons",
        "Packages",
        "Modules",
        "Highlighting",
        "Staged mode",
        "Validation demo",
    ] {
        assert_contains(path, lesson);
    }
}

#[test]
fn fyr_book_contract_defines_in_phase1_reader_surface() {
    let path = "docs/fyr/FYR_BOOK_IN_PHASE1.md";

    for command in [
        "fyr book",
        "fyr book list",
        "fyr book read <chapter>",
        "fyr book next",
        "fyr book prev",
        "fyr book search <term>",
        "fyr book help",
    ] {
        assert_contains(path, command);
    }
}

#[test]
fn fyr_book_contract_covers_initial_chapter_set() {
    let path = "docs/fyr/FYR_BOOK_IN_PHASE1.md";

    for chapter in [
        "What is Fyr?",
        "Your first Fyr program",
        "Main functions",
        "Printing and returning",
        "Fixing parser errors",
        "Values and expressions",
        "Testing with assertions",
        "Packages and modules",
        "Reading Fyr code",
        "Staged candidate mode",
        "Operator validation",
    ] {
        assert_contains(path, chapter);
    }
}

#[test]
fn first_book_chapter_is_readable_inside_phase1_constraints() {
    let path = "docs/fyr/book/00-what-is-fyr.txt";

    for row in [
        "book          : Fyr",
        "chapter       : 00",
        "title         : What is Fyr?",
        "controls      : direct command | tab-complete | help-first | paste-safe | mobile-safe",
        "fallback      : ascii | no-color | compact-terminal",
        "runtime       : VFS-only",
        "host-tools    : blocked",
        "network       : blocked",
        "live-system   : untouched",
        "claim-boundary: book-contract-only",
        "fyr book read 01",
        "fyr learn run 001",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn first_fyrlings_lesson_fixture_has_guided_learning_shape() {
    let path = "docs/fyr/fixtures/fyrlings-lesson-001-ok.txt";

    for row in [
        "course        : fyrlings",
        "lesson        : 001",
        "title         : Orientation",
        "goal:",
        "starter commands:",
        "exercise:",
        "hint:",
        "expected answer:",
        "validation:",
        "pass output:",
        "failure output:",
        "next:",
        "host-tools    : blocked",
        "network       : blocked",
        "live-system   : untouched",
    ] {
        assert_contains(path, row);
    }
}

#[test]
fn fyrlings_and_book_contracts_block_unsafe_or_overclaiming_behavior() {
    for path in [
        "docs/fyr/FYRLINGS_OPERATOR_COURSE.md",
        "docs/fyr/FYR_BOOK_IN_PHASE1.md",
    ] {
        for blocked in [
            "host shell execution",
            "network access",
            "Cargo invocation",
            "Rust compiler invocation",
            "live-system writes",
            "autonomous promotion",
            "autonomous mutation",
            "self-hosting completion",
            "production OS replacement claims",
        ] {
            assert_contains(path, blocked);
        }
    }
}

#[test]
fn fyrlings_and_book_contracts_preserve_non_claim_boundary() {
    assert_contains(
        "docs/fyr/FYRLINGS_OPERATOR_COURSE.md",
        "Non-claim: this document defines the course path; it does not claim the runtime course command is implemented yet.",
    );
    assert_contains(
        "docs/fyr/FYR_BOOK_IN_PHASE1.md",
        "Non-claim: this document defines the book reader contract; it does not claim the runtime `fyr book` command is implemented yet.",
    );
    assert_contains(
        "docs/fyr/FYRLINGS_OPERATOR_COURSE.md",
        "Do not raise Fyr public completion percentage for this plan alone.",
    );
    assert_contains(
        "docs/fyr/FYR_BOOK_IN_PHASE1.md",
        "Do not update public Fyr completion percentage until runtime behavior and tests land.",
    );
}
