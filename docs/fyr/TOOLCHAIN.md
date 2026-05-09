# Fyr toolchain bootstrap

Fyr is gaining a Phase1-owned toolchain surface so Phase1 can gradually rely less on outside build tools for operator scripts and future self-construction workflows.

This does not replace Rust today. Rust remains the implementation language for Phase1. The goal is to make the Fyr command interface stable and host-independent first.

## Current commands

```text
fyr init <package>
fyr check <file.fyr|package>
fyr build <file.fyr|package>
```

## Package layout

```text
fyr.toml
src/
  main.fyr
tests/
  smoke.fyr
```

## Safety model

- VFS-only.
- No Cargo invocation.
- No host shell.
- No network.
- No host compiler.
- Deterministic dry-run build output.

## Example

```text
fyr init hello
fyr check hello
fyr build hello
fyr run hello/src/main.fyr
```

Expected shape:

```text
fyr init: created package hello
fyr check: ok hello/src/main.fyr
fyr build
package : hello
backend : seed/interpreted
host    : none
status  : dry-run artifact ready
Hello from Fyr package
```
