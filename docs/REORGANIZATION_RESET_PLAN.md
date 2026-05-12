# Phase1 / Base1 / Fyr reorganization reset plan

Status: active reset plan  
Branch: `testing`  
Purpose: heal repository structure before continuing implementation

## Decision

The repository is being reset around three coherent projects:

```text
phase1 = runtime/operator system
base1  = boot/hardware foundation
fyr    = Phase1-native language
```

The goal is not to destroy old work. The goal is to preserve it in a structure that makes it clear what is active, what is historical, and what is future work.

## Why this reset is necessary

The repository currently contains too many parallel historical paths:

- old release/edge terminology;
- black-phase1 rapid branch workflow;
- B-series scripts and docs mixed with active scripts;
- root-level docs competing with project docs;
- website, crypto, community, support, and language work moving in parallel;
- generated/local backup files leaking into status;
- repeated boot loops caused by unclear active paths.

This makes the project harder to reason about than the actual technical problems.

## New active project hierarchy

```text
phase1/
  README.md
  src/ or runtime docs/pointers
  docs/
  tools/

base1/
  README.md
  docs/
  scripts/
  evidence/
  targets/

fyr/
  README.md
  docs/
  examples/
  tests/

shared/
  assets/
  docs/
  tooling/

junk/
  README.md
  legacy/
  experiments/
  old-docs/
  old-scripts/
  generated-backups/
```

Root should become a control plane, not a dumping ground.

## Root files to keep

Keep these at root:

```text
README.md
TRACKER.md
FOCUS.md
Cargo.toml
Cargo.lock
LICENSE
.gitignore
.github/
phase1
```

`phase1` shell launcher remains at root for compatibility unless replaced by a compatibility shim.

## Root files to move or archive

Root markdown that is not a control-plane document should move to one of:

```text
phase1/docs/
base1/docs/
fyr/docs/
shared/docs/
junk/old-docs/
```

Root one-off scripts should move to:

```text
scripts/active/
scripts/base1/
scripts/phase1/
scripts/fyr/
junk/old-scripts/
```

Backup files such as `*.bak` should move to:

```text
junk/generated-backups/
```

## Junk policy

`junk/` is not trash. It is a preservation area for files that should not drive the active code path.

A file belongs in `junk/` when:

- its purpose is unclear;
- it duplicates a newer active path;
- it is a one-off experiment;
- it is a generated backup;
- it is historical evidence but not active implementation;
- it is blocking clarity.

Files in `junk/` can be revived later with a clear reason.

## Active router target

Use a single router for active system work:

```text
scripts/phase1-base1.sh
```

Future cleanup can split internals below it, but operators should use one front door.

## Implementation strategy

1. Create the new directories.
2. Add `README.md` files to new roots.
3. Move obvious backup files to `junk/generated-backups/`.
4. Move historical B-series docs/scripts to `junk/legacy/` unless currently active.
5. Keep active B47/QEMU/X200 framebuffer work under `base1/` or active scripts.
6. Move Fyr docs/examples under `fyr/`.
7. Move shared assets under `shared/assets/` only after link checks are ready.
8. Keep compatibility shims for old paths during transition.
9. Run tests and fix path breakage.
10. Promote clean structure to `stable` when coherent.

## Do not do in the first pass

- Do not mass-delete files.
- Do not move `.github` workflows yet.
- Do not rewrite every doc link manually in the same commit.
- Do not change Cargo workspace layout until files are organized.
- Do not move `src/` until a Rust module migration plan exists.

## First-pass script behavior

The first script should:

- default to dry-run;
- write a move plan;
- create needed directories;
- move only safe categories first;
- preserve old paths where required;
- create `junk/README.md`;
- create project root README stubs;
- report likely breakage.

## After first pass

Run:

```sh
git status --short
cargo check --all-targets
cargo test --all-targets
sh scripts/phase1-base1.sh status
```

Then fix breakage deliberately.
