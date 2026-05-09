# Fyr toolchain checkpoint — v4.4.0-dev

This checkpoint records the current Phase1-owned Fyr toolchain surface.

## Implemented

- `fyr init <package>`
- `fyr check <file.fyr|package>`
- `fyr build <file.fyr|package>`
- `fyr test <package>`
- `fyr color <file.fyr>`
- `fyr highlight <file.fyr>` alias
- package manifest validation
- package source/module discovery
- duplicate `fn main` diagnostics
- parser diagnostics for missing main, unterminated strings, missing semicolons, and invalid returns
- test assertions:
  - `assert(true);`
  - `assert(false);`
  - `assert_eq(1, 1);`
  - integer comparisons with `==`, `!=`, `>`, `<`
- ANSI color output for Fyr keywords, builtins, strings, numbers, and booleans

## Boundary

Fyr remains VFS-only and host-independent. The current build command is still a dry-run interpreted artifact path; it does not invoke Cargo or host tools.

## Next direction

- richer expressions
- variables and bindings
- functions beyond `main`
- real package graph metadata
- Fyr-to-Fyr build artifacts
- eventual Phase1 self-construction path
