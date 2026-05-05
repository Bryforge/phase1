# phase1 operator shell upgrade

This document tracks the implemented foundation for the next phase of the project: turning phase1 from a basic terminal simulator into a structured virtual OS console.

## Implemented in this pass

### Command registry

`src/registry.rs` is now the metadata source for commands. It records:

- command name
- aliases
- category
- usage
- description
- required capability label

This lets the project move toward generated help, generated man pages, completion, permissions, and command discovery from one source of truth.

### Generated help and man metadata

- `help` / `commands` now render the grouped command map from the registry.
- `man <command>` now resolves pages from registry metadata instead of a hand-written static match table.

### Completion surface

`complete <prefix>` returns registry-backed command and alias suggestions. This is intentionally small and dependency-free now, but it creates a clean upgrade path to a real line editor such as Reedline later.

Examples:

```text
complete p
complete br
complete wi
```

### Mobile-first operator UI

`src/ui.rs` owns the boot card and command map UI. The boot screen is constrained for narrow phone terminals and keeps startup output compact.

### CI quality wall

`.github/workflows/rust-ci.yml` now performs:

```bash
cargo generate-lockfile
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

`deny.toml` adds dependency policy defaults for future dependencies.

## Next implementation targets

1. Replace stdin line reading with a true line editor.
2. Add persistent command history on disk.
3. Add registry-backed aliases in dispatch so aliases do not need separate match arms.
4. Add a syscall boundary between shell commands and kernel state.
5. Add a capability checker that uses the registry capability field.
6. Add structured command output so pipelines can later support `where`, `sort`, `get`, and `table`.
7. Replace Python plugins with a sandboxed WASM/WASI runtime.
