use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(path: &str, needle: &str) {
    let text = read(path);
    assert!(text.contains(needle), "{path} should contain: {needle}");
}

#[test]
fn fyr_language_book_documents_current_behavior_without_production_claims() {
    let path = "docs/fyr/LANGUAGE_BOOK.md";

    assert_contains(path, "Current-behavior language book");
    assert_contains(path, "F3 core syntax work is active, not complete.");
    assert_contains(path, "F4-F7 remain planned");
    assert_contains(path, "Fyr is not production-ready");
    assert_contains(path, "not a general replacement for Rust, Python, C, or shell");
}

#[test]
fn fyr_language_book_lists_implemented_commands_and_safety_model() {
    let path = "docs/fyr/LANGUAGE_BOOK.md";

    for command in [
        "fyr status",
        "fyr spec",
        "fyr new <name>",
        "fyr init <package>",
        "fyr check <file.fyr|package>",
        "fyr build <file.fyr|package>",
        "fyr test <package>",
        "fyr run <file.fyr>",
    ] {
        assert_contains(path, command);
    }

    assert_contains(path, "VFS-only by default.");
    assert_contains(path, "No host shell.");
    assert_contains(path, "No network.");
    assert_contains(path, "No host compiler.");
}

#[test]
fn fyr_language_book_tracks_f3_diagnostics_and_tests() {
    let path = "docs/fyr/LANGUAGE_BOOK.md";

    for diagnostic in [
        "Missing package manifest.",
        "Missing package main.",
        "Missing `fn main` entry point.",
        "Duplicate package `fn main`.",
        "Unterminated string literal.",
        "Division by zero.",
        "Missing right-hand integer operand.",
    ] {
        assert_contains(path, diagnostic);
    }

    for test_file in [
        "tests/fyr_parser_diagnostics.rs",
        "tests/fyr_f3_package_diagnostics.rs",
        "tests/fyr_f3_expression_diagnostics.rs",
    ] {
        assert_contains(path, test_file);
    }
}
