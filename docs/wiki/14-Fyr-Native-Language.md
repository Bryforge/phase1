# Fyr Native Language

![Fyr](https://img.shields.io/badge/Fyr-native%20language-ff5a00) ![Scripts](https://img.shields.io/badge/scripts-.fyr-00d8ff) ![Phase1](https://img.shields.io/badge/Phase1-owned-39ff88)

Fyr is the Phase1-native language track for VFS automation, operator-owned scripts, and future self-construction work. It is designed around C-style explicit control, a Rust-style safety posture, and Python-style readability while staying owned by Phase1.

## What Fyr is for

| Area | Purpose |
| --- | --- |
| First scripts | Run `.fyr` files directly inside Phase1. |
| Operator automation | Build small repeatable workflows without leaving the Phase1 environment. |
| VFS-first tasks | Work naturally with Phase1 files and command output. |
| Testing | Assert behavior through simple language-level checks. |
| Future system work | Grow toward deeper Phase1-owned automation and self-hosted tools. |

## Current language surface

Current Fyr work includes:

- `.fyr` script files
- `print` output
- `return` values
- `let` bindings
- arithmetic expressions
- grouped expressions
- comparisons
- boolean chains
- simple `if` return statements
- assertions
- package checks
- test runner flow
- syntax-color output

> [!IMPORTANT]
> Fyr is growing. Treat unsupported language features as roadmap items, not hidden behavior.

## First script

Inside Phase1:

```text
echo 'fn main() -> i32 { print("Hello, hacker!"); return 0; }' > hello_hacker.fyr
fyr run hello_hacker.fyr
```

Expected output:

```text
Hello, hacker!
```

## AVIM workflow

```text
avim hello.fyr
```

Inside AVIM:

```text
i
fn main() -> i32 { print("hello from Fyr"); return 0; }
Esc
:wq
```

Run it:

```text
fyr run hello.fyr
```

## Host quick check

From the repository root, use normal Phase1 validation before changing Fyr-facing behavior:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
sh scripts/quality-check.sh quick
```

## Documentation sources

Start with these repository docs:

- `docs/project/PHASE1_NATIVE_LANGUAGE.md` — Phase1 native-language overview.
- `docs/fyr/ROADMAP.md` — Fyr roadmap.
- `assets/fyr_symbol.png` — Fyr symbol.
- `assets/fyr_word.png` — Fyr word mark.

## Good wiki examples

Use short, copyable examples:

```text
fn main() -> i32 {
    let x = 40 + 2;
    print(x);
    return 0;
}
```

Use explicit expected output:

```text
42
```

Use current command names:

```text
fyr run example.fyr
```

## Avoid in public examples

Do not use Fyr examples that:

- imply unimplemented language features
- require private secrets or credentials
- disable Phase1 safety gates without explaining why
- claim kernel-level or hardened-host behavior
- mix Base1 boot claims into ordinary Fyr scripting examples

## Roadmap language

Use conservative language for future features:

| Better wording | Avoid |
| --- | --- |
| `planned` | `supported` |
| `target` | `guaranteed` |
| `prototype` | `production-ready` |
| `Phase1-native scripting` | `complete systems language` |
| `future self-hosting path` | `self-hosted today` |

## Next pages

- [Quick Start](01-Quick-Start.md)
- [Command Manual](04-Command-Manual.md)
- [Language Runtimes](07-Language-Runtimes.md)
- [Base1 OS Track](13-Base1-OS-Track.md)
