# phase1-core

`phase1-core` is the reusable Rust library layer for Phase1.

It exists so Base1, validation tools, tests, and AI-assisted project management can depend on stable Phase1 internals without embedding the interactive terminal shell.

## Public API focus

The crate exposes:

- virtual kernel and VFS primitives
- simulated process scheduler types
- PCIe simulation types
- command registry metadata
- command capability reports
- policy helpers
- text/VFS helpers
- shell-independent persistent history helpers
- Phase1 Arena module access
- ops log helpers

## Design rules

- Keep deterministic reusable logic here.
- Keep terminal input loops in the `phase1` app crate.
- Keep host command execution behind policy in the app crate.
- Avoid shell-specific state such as `Phase1Shell` in this crate.
- Add `AI-NOTE:` comments only for important architectural guardrails.

## Git dependency example

```toml
phase1-core = { git = "https://github.com/Bryforge/phase1", branch = "feature/extract-phase1-core", package = "phase1-core" }
```

## Validation

From the repository root:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
```
