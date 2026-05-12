# Phase1 / Base1 / Fyr tracker

Status: active project reset and reorganization tracker  
Active branch model: `testing` -> `stable`  
Current phase: repository healing, file-structure cleanup, and focused rebuild

## Current decision

The project is being reorganized into three logical systems:

```text
phase1/  runtime, shell, operator console, UI, local tools
base1/   boot foundation, hardware targets, X200/QEMU paths, recovery, media prep
fyr/     Phase1-native language, examples, parser/runtime work, package model
```

Everything else is preserved, but no longer drives the active direction.

## Branch model

```text
testing  active development and repair branch
stable   known-good promoted branch
```

Legacy branches such as `black-phase1` and `edge/stable` are historical and should not be the active operating model after this reset.

## Main objective

Build one cohesive project instead of many scattered tracks:

```text
Base1 working foundation
  -> Phase1 working runtime on top
  -> Fyr working language path after Phase1/Base1 are coherent
```

## Active order of work

1. Repository structure cleanup.
2. Preserve all valuable files in logical locations.
3. Move aged/unclear/duplicate files into `junk/` for later review instead of deleting them.
4. Rebuild Base1 as the boot/hardware foundation.
5. Rebuild Phase1 as the operator/runtime system.
6. Rebuild Fyr as the Phase1-native language track.
7. Fix all broken tests, paths, scripts, and docs caused by the reorganization.
8. Promote known-good results from `testing` to `stable`.

## Current project lanes

| Lane | Status | Rule |
| --- | --- | --- |
| Base1 | Active first | Boot, X200, QEMU, recovery, media prep, hardware evidence. |
| Phase1 | Active second | Runtime, shell, UI, renderer, operator console, local tools. |
| Fyr | Preserved, then active third | Language work resumes after Base1/Phase1 are coherent. |
| Website | Frozen | Only fixes/pointers until core project is organized. |
| Crypto docs | Frozen | Preserve; only safety/privacy fixes unless needed by Phase1/Base1. |
| Community/support docs | Frozen | Preserve; no expansion during cleanup. |
| Historical B-series scripts | Preserved/junk review | Keep evidence, but do not let old experiments drive active work. |

## Current evidence retained

```text
QEMU framebuffer card: seen
QEMU Japanese pixels: seen
X200 framebuffer card: not claimed
X200 Japanese pixels: not claimed
X200 terminal Phase1 boot: seen
X200 stable/safe color terminal: seen
```

## Reorganization rules

- Do not delete valuable work.
- Prefer `git mv` over remove/recreate.
- Move aged or confusing work to `junk/` with enough path history to recover it.
- Keep root-level files minimal.
- Keep public claims conservative.
- Keep one active router for day-to-day work.
- Fix breakage after the structural move, not during an endless pre-cleanup loop.

## Target root layout

```text
README.md
TRACKER.md
Cargo.toml
Cargo.lock
LICENSE
phase1/
base1/
fyr/
shared/
docs/
scripts/
tests/
junk/
.github/
```

Root should stop being a dumping ground for old docs, one-off scripts, and experiments.

## Immediate next command

Dry-run the reorganization first:

```sh
sh scripts/reorganize-phase1-base1-fyr.sh --dry-run
```

Apply only after reviewing the plan:

```sh
sh scripts/reorganize-phase1-base1-fyr.sh --apply
```

## Success criteria for cleanup

```text
root directory is understandable
phase1/base1/fyr are clearly separated
aged files are preserved in junk/
active docs point to TRACKER.md
active scripts are discoverable
breakage list is explicit
no evidence is lost
```
