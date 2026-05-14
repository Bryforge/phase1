# Fyrlings operator course

Issue: #323  
Status: learning contract  
Scope: Rustlings-style Fyr course for Phase1 operators  
Non-claim: this document defines the course path; it does not claim the runtime course command is implemented yet.

Fyrlings is the guided learning path for Fyr operators. It should teach Fyr by small, deterministic exercises that can run inside Phase1 without host tools, network access, Cargo, Rust compiler invocation, or live-system mutation.

## Operator-facing command surface

The preferred Phase1 shell surface is:

```text
fyr learn
fyr learn list
fyr learn run <lesson>
fyr learn hint <lesson>
fyr learn verify
fyr learn reset <lesson>
fyr book
fyr book list
fyr book read <chapter>
fyr book next
fyr book search <term>
```

A short alias may exist later:

```text
fyrlings
```

The canonical path must remain discoverable through `fyr help` and the in-Phase1 Fyr book.

## Learning model

Each lesson should have:

- lesson id;
- title;
- goal;
- starter code or scenario;
- hint text;
- expected fix or expected command sequence;
- validation rule;
- pass output;
- failure output;
- next lesson guidance;
- safety boundary row.

## Course sequence

| Lesson | Title | Teaches | Validation style |
| --- | --- | --- | --- |
| 001 | Orientation | Fyr purpose, VFS-only boundary, commands | read-only quiz/check row |
| 002 | Hello Fyr | `fyr new`, `fyr run`, string output | run output includes literal |
| 003 | Main function | `fn main() -> i32` shape | parser accepts valid entrypoint |
| 004 | Printing | `print("literal")` | output matches expected text |
| 005 | Returning | `return <integer>` | deterministic exit/result row |
| 006 | Fix diagnostics | missing semicolon/string/return recovery | parser diagnostic matches expected |
| 007 | Bindings | `let` bindings where supported | parser/evaluator validates value |
| 008 | Expressions | integer expressions where supported | evaluator validates result |
| 009 | Branching | `if` handling where supported | branch output is deterministic |
| 010 | Equality asserts | `assert_eq` | pass/fail diagnostics are deterministic |
| 011 | Boolean asserts | `assert(true)` and `assert(false)` | assertion result is deterministic |
| 012 | Comparisons | `==`, `!=`, `>`, `<` assertions | comparison diagnostics are deterministic |
| 013 | Packages | `fyr init`, manifest, `src/main.fyr` | package check passes |
| 014 | Modules | module discovery and duplicate main recovery | resolver diagnostics are deterministic |
| 015 | Highlighting | `fyr color` / `fyr highlight` | ANSI and no-color fallback both readable |
| 016 | Staged mode | `fyr staged` / `black_arts` non-live mode | no host/network/live markers appear |
| 017 | Validation demo | full operator validation walkthrough | validation demo summary passes |

## User considerations

Fyrlings must support:

- first-time users who start with no Fyr context;
- keyboard-only use;
- tab-completion discovery;
- copy/paste lesson commands;
- mobile and small terminal use;
- compact output;
- no-color terminals;
- ASCII fallback;
- low-vision readability through text labels;
- unknown-command recovery;
- safe-mode and guarded-host boundaries;
- explicit non-claims for unfinished Fyr behavior.

## Control schemes

Each runtime learning command should remain usable through:

```text
direct command entry
help-first discovery
tab-completion guidance
copy/paste command flow
mobile terminal flow
compact terminal flow
no-color output
ASCII fallback
unknown-command recovery
```

## Safety boundaries

Fyrlings must not perform or imply:

```text
host shell execution
network access
Cargo invocation from Fyr learning commands
Rust compiler invocation from Fyr learning commands
live-system writes
autonomous promotion
autonomous mutation
self-hosting completion
production OS replacement claims
```

## Progress and state

The first implementation should use deterministic VFS-only lesson files. Progress may be held in Phase1 state later, but the first course should work without persistent state.

Required state rows:

```text
course        : fyrlings
mode          : deterministic
runtime       : VFS-only
host-tools    : blocked
network       : blocked
live-system   : untouched
claim-boundary: learning-contract-only
```

## Runtime implementation gates

Fyrlings should be implemented in phases:

1. contract and fixtures;
2. docs/book chapters;
3. lesson registry;
4. `fyr learn list` and `fyr book list`;
5. `fyr book read <chapter>`;
6. `fyr learn run <lesson>` for the first lesson;
7. lesson validation and hints;
8. full course verification;
9. status integration after tests prove behavior.

Do not raise Fyr public completion percentage for this plan alone.
