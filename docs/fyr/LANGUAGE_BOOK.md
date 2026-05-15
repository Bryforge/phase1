# Fyr Language Book

> **Status:** Current-behavior language book for the Phase1-native Fyr track.
>
> **Validation:** Links to implemented examples, integration tests, toolchain docs, and roadmap gates.
>
> **Non-claims:** Fyr is not production-ready and is not a general replacement for Rust, Python, C, or shell.

This book is the Fyr section of **The Phase1 Codex**. It documents what Fyr does today, what is still planned, and which tests protect the current behavior.

## 1. Purpose and status

Fyr is the Phase1-native language path for VFS automation, self-construction, and stable operator scripts.

Current status:

- Identity, `.fyr` extension, `fyr` command surface, and visual references exist.
- Seed runner, authoring loop, package bootstrap, check/build/test commands, and print-literal run support exist.
- F3 core syntax work is active, not complete.
- F4-F7 remain planned and must not be described as complete until implementation, tests, docs, and release evidence agree.

## 2. Getting started

Create a single-file program:

```text
fyr new hello.fyr
fyr cat hello.fyr
fyr check hello.fyr
fyr build hello.fyr
fyr run hello.fyr
```

Create a package:

```text
fyr init hello
fyr check hello
fyr build hello
fyr test hello
fyr run hello/src/main.fyr
```

The package layout is:

```text
fyr.toml
src/
  main.fyr
tests/
  smoke.fyr
```

## 3. Program structure

The current implemented source shape is intentionally small:

```fyr
fn main() -> i32 { print("Hello, hacker!"); return 0; }
```

Implemented structure checks include:

- `fn main` entry-point detection.
- Missing-main diagnostics.
- Duplicate-main diagnostics for package modules.
- Package manifest and package main diagnostics.

## 4. Expressions and values

Implemented expression behavior includes:

- Integer literals.
- Integer `let` bindings.
- Basic arithmetic with grouped expressions.
- Comparisons used by assertions and `if` conditions.
- Boolean conditions with grouped boolean expressions.
- Deterministic diagnostics for unclosed parentheses, division by zero, and missing operands.

Example:

```fyr
fn main() -> i32 { let answer = ((6 + 8) * (9 - 3)) / 2; assert_eq(answer, 42); return answer; }
```

## 5. Bindings and control flow

Implemented statements include:

- `let name = integer_expression;`
- `print("literal");`
- `assert(condition);`
- `assert_eq(left, right);`
- `if condition { return value; }`
- `return integer_expression;`

Example:

```fyr
fn main() -> i32 { let answer = 42; if (answer > 40 && answer < 50) { return answer; } return 0; }
```

## 6. Whitespace

Fyr whitespace behavior is defined in [`WHITESPACE_NORMALIZATION.md`](WHITESPACE_NORMALIZATION.md).

Core rule:

- spaces between normal tokens should not change program meaning;
- spaces inside strings, text literals, comments, and any future indentation-sensitive syntax must be preserved.

Example equivalent forms:

```fyr
let x=1+2;
let x = 1 + 2;
let   x   =   1   +   2;
```

Example preserved literal spacing:

```fyr
print("hello world");
print("hello   world");
```

Those string literals are not equivalent because the spaces are literal content.

## 7. Diagnostics

Current diagnostics are designed to be deterministic and VFS-facing. Covered diagnostics include:

- Missing package manifest.
- Missing package main.
- Missing `fn main` entry point.
- Duplicate package `fn main`.
- Unterminated string literal.
- Missing semicolon after print.
- Invalid return value.
- Non-integer let binding value.
- Non-boolean `if` condition.
- Unclosed grouped integer expression.
- Division by zero.
- Missing right-hand integer operand.

Diagnostics should identify the Fyr file or package target where possible.

## 8. Tooling commands

Current commands:

```text
fyr status
fyr spec
fyr new <name>
fyr init <package>
fyr cat <file>
fyr color <file>
fyr check <file.fyr|package>
fyr build <file.fyr|package>
fyr test <package>
fyr self
fyr run <file.fyr>
fyr help
```

Safety model:

- VFS-only by default.
- No Cargo invocation.
- No host shell.
- No network.
- No host compiler.
- Deterministic dry-run build output.

## 9. Phase1 integration

Fyr is part of Phase1, not a separate production language. The intended path is:

1. Keep F0-F2 stable.
2. Finish F3 core syntax and diagnostics.
3. Add F4 safe runtime behavior without expanding host authority.
4. Use F5 self-workflows to help Phase1 inspect, copy, construct, and validate itself.
5. Add a small standard library only after the runtime is bounded and tested.

## 10. Examples

Current examples:

- [`../../examples/fyr/hello.fyr`](../../examples/fyr/hello.fyr)
- [`../../examples/fyr/self_check.fyr`](../../examples/fyr/self_check.fyr)

Current roadmap and toolchain docs:

- [`ROADMAP.md`](ROADMAP.md)
- [`TOOLCHAIN.md`](TOOLCHAIN.md)
- [`README.md`](README.md)
- [`NATIVE_EXECUTION_GUIDANCE.md`](NATIVE_EXECUTION_GUIDANCE.md)
- [`../status/FYR_PHASE1_100_COMPLETION_GATES.md`](../status/FYR_PHASE1_100_COMPLETION_GATES.md)

## 11. Validation links

Current Fyr-focused integration tests include:

- `tests/fyr_authoring_commands.rs`
- `tests/fyr_parser_diagnostics.rs`
- `tests/fyr_let_bindings.rs`
- `tests/fyr_if_statements.rs`
- `tests/fyr_parenthesized_expressions.rs`
- `tests/fyr_f3_package_diagnostics.rs`
- `tests/fyr_f3_expression_diagnostics.rs`

## 12. Contributor guide

When adding Fyr behavior:

1. Update implementation first.
2. Add deterministic tests for happy paths and failure paths.
3. Update this language book and the roadmap.
4. Do not raise public completion percentages manually.
5. Preserve non-claims unless release evidence supports changing them.

## 13. Roadmap

Fyr's 100% gate is evidence-bound. All F0-F7 gates must be satisfied before public wording can move from "Phase1-native language track" to "Phase1's native scripting language." See [`../status/FYR_PHASE1_100_COMPLETION_GATES.md`](../status/FYR_PHASE1_100_COMPLETION_GATES.md).

## Source links

- [`../MANUAL_ROADMAP.md`](../MANUAL_ROADMAP.md)
- [`../../docs/project/PHASE1_NATIVE_LANGUAGE.md`](../../docs/project/PHASE1_NATIVE_LANGUAGE.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`TOOLCHAIN.md`](TOOLCHAIN.md)
- [`WHITESPACE_NORMALIZATION.md`](WHITESPACE_NORMALIZATION.md)