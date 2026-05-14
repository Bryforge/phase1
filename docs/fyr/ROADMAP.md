# Fyr language roadmap

Fyr is the Phase1-native language path for VFS automation, self-construction, and stable operator scripts. The language is intentionally small at first: it should be easy to read like Python, explicit like C, and safe-by-default like Rust.

## Current foundation

- `docs/project/PHASE1_NATIVE_LANGUAGE.md` names Fyr, reserves `.fyr`, and defines `fyr` as the command surface.
- Current Fyr visual assets live at `assets/fyr_symbol.png` and `assets/fyr_word.png`.
- `examples/fyr/hello.fyr` and `examples/fyr/self_check.fyr` document the first source shape.
- `fyr status`, `fyr spec`, and seed `fyr run <file.fyr>` support are wired into Phase1.
- The seed runner can execute simple print string literals from `.fyr` files stored in the Phase1 VFS.
- The 100% promotion gate is tracked in [`../status/FYR_PHASE1_100_COMPLETION_GATES.md`](../status/FYR_PHASE1_100_COMPLETION_GATES.md).
- F3 parser diagnostics now include package manifest, package entry-point, duplicate-main, missing-main, string, semicolon, return-value, boolean-condition, grouped-expression, division-by-zero, missing-operand, and boolean-operator checks.

## Roadmap

| Stage | Goal | Deliverables | Status |
| --- | --- | --- | --- |
| F0 — Identity | Establish the language name, file extension, command, and visual mark. | `assets/fyr_symbol.png`, `assets/fyr_word.png`, README reference, native language spec, roadmap. | Active |
| F1 — Seed runner | Let Phase1 run simple `.fyr` programs from the VFS. | `fyr status`, `fyr spec`, `fyr run`, print literal support, guard tests. | Active |
| F2 — Authoring loop | Make Fyr usable without manually echoing source code. | `fyr new <name>`, `fyr cat <file>`, `fyr self`, starter templates, safer VFS writes. | Active |
| F3 — Core syntax | Define stable parser behavior inspired by C, Rust, and Python. | Lexer, parser, function blocks, variables, return values, comments, diagnostics. | Active |
| F4 — Safe runtime | Execute useful scripts without exposing the host. | VFS reads/writes, command metadata, bounded runtime, redacted errors, deterministic tests. | Planned |
| F5 — Phase1 self-workflows | Use Fyr to help Phase1 inspect, copy, construct, and validate itself. | `fyr self`, repository manifest readers, docs sync helpers, checkpoint helpers. | Planned |
| F6 — Standard library | Provide a small Phase1-owned standard library. | `vfs`, `text`, `json-lite`, `audit`, `process`, `package`, and `doc` modules. | Planned |
| F7 — Compiler path | Decide whether Fyr remains interpreted, lowers to Rust, or targets WASI-lite. | Compiler design note, WASI-lite strategy, packaging contract. | Planned |

## 100% promotion gate

Fyr is not promoted to 100% because the name, command, or docs exist. It reaches 100% only when all F0-F7 stages are implemented, tested, documented, and reflected in public release evidence.

Required promotion evidence:

- F0-F2 remain stable after every parser/runtime change.
- F3 core syntax has deterministic parser, diagnostics, duplicate-main, and missing-main tests.
- F4 safe runtime stays VFS-only by default and has bounded runtime/error-redaction tests.
- F5 Phase1 self-workflows operate on deterministic fixture data.
- F6 standard library has smoke and failure-mode tests for each stable module.
- F7 packaging/compiler decision has a design note, compatibility policy, and versioned examples.
- `docs/status/FYR_PHASE1_100_COMPLETION_GATES.md` is satisfied before public status percentages are raised to 100%.

## Design rules

- Keep Fyr source readable on mobile terminals.
- Keep host access explicit and guarded.
- Prefer deterministic behavior over clever behavior.
- Make every language feature testable inside Phase1.
- Never require Python, C, or Rust syntax changes to keep Phase1 scripts alive.

## First working example

```fyr
fn main() -> i32 { print("Hello, hacker!"); return 0; }
```

Run inside Phase1:

```text
fyr run hello_hacker.fyr
```

Expected output:

```text
Hello, hacker!
```

## Visual reference

The current Fyr visual assets are:

- Symbol: [`assets/fyr_symbol.png`](../../assets/fyr_symbol.png)
- Word mark: [`assets/fyr_word.png`](../../assets/fyr_word.png)

Older references to `assets/fyr-flame.svg` are outdated.
