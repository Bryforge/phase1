# Phase1 Native Language

<p align="center">
  <img src="assets/fyr-flame.svg" alt="Fyr flame mark with fyr written inside the flame" width="520">
</p>

Name: Fyr
Extension: .fyr
Command: fyr
Roadmap: [`docs/fyr/ROADMAP.md`](docs/fyr/ROADMAP.md)
Visual mark: [`assets/fyr-flame.svg`](assets/fyr-flame.svg)

Fyr is the planned Phase1-native language target. It is reserved for stable Phase1 automation, virtual file system workflows, future self-build scripts, and operator-owned tooling that should not stop working because an outside language changes direction.

Fyr is modeled after three familiar ideas:

- C-style direct control and explicit entry points.
- Rust-style safety posture, guarded capabilities, and predictable ownership boundaries.
- Python-style readability and fast scripting flow.

## Current milestone

- language specification
- examples
- guard tests
- `fyr status`
- `fyr spec`
- seed `fyr run <file.fyr>` support for print literals
- README/post flame mark
- dedicated language roadmap

## First working example

```fyr
fn main() -> i32 { print("Hello, hacker!"); return 0; }
```

Inside Phase1:

```text
fyr run hello_hacker.fyr
```

Expected output:

```text
Hello, hacker!
```

## Next milestones

- `fyr new <name>` for starter program creation.
- `fyr cat <file>` for native inspection from Phase1.
- `fyr self` for Phase1 self-check and construction workflows.
- Lexer and parser design.
- VFS-safe standard library modules.
- WASI-lite or compiler target decision.


## Authoring commands

```text
fyr new hello_hacker
fyr cat hello_hacker.fyr
fyr run hello_hacker.fyr
fyr self
```

These commands let Phase1 create, inspect, and run Fyr files from inside the Phase1 shell without manually echoing source code.
