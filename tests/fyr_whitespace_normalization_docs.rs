use std::fs;

const WHITESPACE_DOC: &str = "docs/fyr/WHITESPACE_NORMALIZATION.md";
const LANGUAGE_BOOK: &str = "docs/fyr/LANGUAGE_BOOK.md";

#[test]
fn fyr_whitespace_normalization_doc_exists_and_defines_scope() {
    let doc = fs::read_to_string(WHITESPACE_DOC)
        .expect("Fyr whitespace normalization doc should exist");

    for required in [
        "# Fyr whitespace normalization",
        "Status: language contract",
        "insignificant whitespace between Fyr tokens",
        "preservation of meaningful whitespace",
        "Whitespace between normal language tokens should not change program meaning.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn fyr_whitespace_normalization_preserves_core_equivalence_rule() {
    let doc = fs::read_to_string(WHITESPACE_DOC)
        .expect("Fyr whitespace normalization doc should exist");

    for required in [
        "let x=1+2;",
        "let x = 1 + 2;",
        "let   x   =   1   +   2;",
        "The parser should treat insignificant whitespace as a separator or visual formatting aid, not as semantic content.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn fyr_whitespace_normalization_lists_token_spacing_surfaces() {
    let doc = fs::read_to_string(WHITESPACE_DOC)
        .expect("Fyr whitespace normalization doc should exist");

    for required in [
        "assignment operators",
        "arithmetic operators",
        "comparison operators",
        "boolean operators",
        "grouping parentheses",
        "statement separators",
        "function call argument separators",
        "braces when they are used as block delimiters",
        "assert_eq(answer,42);",
        "assert_eq( answer , 42 );",
        "if(answer>40&&answer<50){return answer;}",
        "if ( answer > 40 && answer < 50 ) { return answer; }",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn fyr_whitespace_normalization_preserves_meaningful_whitespace() {
    let doc = fs::read_to_string(WHITESPACE_DOC)
        .expect("Fyr whitespace normalization doc should exist");

    for required in [
        "string literals",
        "future text literals",
        "comments",
        "future indentation-sensitive syntax",
        "print(\"hello world\");",
        "print(\"hello   world\");",
        "Those strings are not equivalent because the spaces are literal content.",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn fyr_whitespace_normalization_links_file_backed_workflows_and_non_claims() {
    let doc = fs::read_to_string(WHITESPACE_DOC)
        .expect("Fyr whitespace normalization doc should exist");

    for required in [
        "NATIVE_EXECUTION_GUIDANCE.md",
        "editor-first `.fyr` workflows",
        "Native or inline execution should remain for short tests and quick checks only.",
        "does not claim that Fyr has a complete formatter",
        "complete parser",
        "complete language grammar",
        "production readiness",
        "hardened sandboxing",
    ] {
        assert!(doc.contains(required), "missing {required:?}: {doc}");
    }
}

#[test]
fn fyr_language_book_links_whitespace_normalization_contract() {
    let book = fs::read_to_string(LANGUAGE_BOOK).expect("Fyr language book should exist");

    for required in [
        "## 6. Whitespace",
        "WHITESPACE_NORMALIZATION.md",
        "spaces between normal tokens should not change program meaning",
        "spaces inside strings, text literals, comments, and any future indentation-sensitive syntax must be preserved",
        "let x=1+2;",
        "let x = 1 + 2;",
        "let   x   =   1   +   2;",
    ] {
        assert!(book.contains(required), "missing {required:?}: {book}");
    }
}
