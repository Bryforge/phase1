# Fyr whitespace normalization

Status: language contract
Scope: insignificant whitespace between Fyr tokens, preservation of meaningful whitespace, and future parser/runtime expectations.

## Purpose

Fyr should be comfortable to write and edit in `.fyr` files without making spacing style fragile.

Whitespace between normal language tokens should not change program meaning.

Whitespace inside strings, text literals, comments, and any future indentation-sensitive syntax must be preserved according to that construct's rules.

## Core rule

These forms should be equivalent when they use the same tokens:

```fyr
let x=1+2;
let x = 1 + 2;
let   x   =   1   +   2;
```

The parser should treat insignificant whitespace as a separator or visual formatting aid, not as semantic content.

## Must normalize

Fyr should ignore insignificant whitespace around:

- assignment operators;
- arithmetic operators;
- comparison operators;
- boolean operators;
- grouping parentheses;
- statement separators;
- function call argument separators;
- braces when they are used as block delimiters.

Examples:

```fyr
assert_eq(answer,42);
assert_eq( answer , 42 );

if(answer>40&&answer<50){return answer;}
if ( answer > 40 && answer < 50 ) { return answer; }
```

## Must preserve

Fyr must preserve meaningful whitespace inside:

- string literals;
- future text literals;
- comments;
- any future indentation-sensitive syntax if such syntax is introduced deliberately.

Examples:

```fyr
print("hello world");
print("hello   world");
```

Those strings are not equivalent because the spaces are literal content.

## Current implementation expectation

Current Fyr already accepts compact and spaced expression forms across many arithmetic, boolean, grouping, and control-flow tests.

This document preserves the language contract and gives future parser work a clear boundary:

- normalize insignificant token spacing;
- preserve literal/comment spacing;
- keep diagnostics deterministic;
- keep file-backed `.fyr` workflows preferred for real work.

## Relationship to native execution guidance

Whitespace normalization supports editor-first `.fyr` workflows.

Operators should be free to format `.fyr` files for readability while preserving program meaning.

Native or inline execution should remain for short tests and quick checks only. Real work should be saved in `.fyr` files and edited with an editor.

See [`NATIVE_EXECUTION_GUIDANCE.md`](NATIVE_EXECUTION_GUIDANCE.md).

## Non-claims

This document does not claim that Fyr has a complete formatter, complete parser, complete language grammar, production readiness, or hardened sandboxing.

It defines the whitespace behavior that current and future Fyr parser work should preserve.
