# Phase1 repository reorganization plan

Status: planning roadmap
Scope: repository structure, navigation, compatibility paths, staged moves, and minimalist top-level organization

## Purpose

This plan defines how Phase1 should reorganize the repository without breaking users, contributors, documentation links, scripts, release references, or Base1 compatibility paths.

The goal is a minimalist visible structure where the top level is easy to understand, while preserving compatibility for existing public paths.

## Reorganization goal

Phase1 should move toward a cleaner repository shape:

```text
.github/        GitHub templates, workflows, and automation
assets/         Public images, logos, icons, and branding files
base1/          Base1 compatibility entry points and root-level Base1 docs
docs/           Manuals, roadmaps, support docs, security docs, and navigation
examples/       Example scripts, walkthroughs, demo inputs, and learning samples
scripts/        Operator and maintainer scripts
tests/          Rust and documentation guard tests
tools/          Internal helper tooling and future maintainer utilities
src/            Phase1 Rust source
phase1-core/    Core package workspace member
xtask/          Repository validation helper
```

If the repository cannot safely reach this exact shape without breaking compatibility, continue using the current preservation-first organization and add indexes, mirrors, and navigation instead of moving files aggressively.

## Minimalist view principle

The top level should answer four questions quickly:

1. How do I run it?
2. How do I contribute?
3. Where are the docs?
4. Where are source, scripts, tests, and assets?

Top-level files should be limited to high-value entry points, release/legal/security files, and compatibility paths.

## Preserve-first rule

Do not delete or move public/root compatibility files until all of these exist:

- documented old path -> new path map;
- compatibility shim or mirror decision;
- link-check coverage;
- tests for required links;
- rollback plan;
- maintainer approval.

When in doubt, add an index before moving a file.

## Proposed destination map

| Content type | Preferred destination |
| --- | --- |
| Public entry point | `README.md` |
| Contribution rules | `CONTRIBUTING.md` |
| Security policy | `SECURITY.md` |
| Quality system | `QUALITY.md` and `QUALITY_SCORECARD.md` |
| License | `LICENSE` |
| Feature status | `FEATURE_STATUS.md` |
| Release notes | `docs/releases/` with root compatibility files when needed |
| Manual pages | `docs/` |
| Security and crypto docs | `docs/security/` |
| Community/support docs | `docs/community/` |
| Base1 organized docs | `docs/base1/` |
| Base1 compatibility docs | `base1/` |
| Fyr docs | `docs/fyr/` |
| Operator docs | `docs/operators/` |
| Developer docs | `docs/developers/` |
| Recovery docs | `docs/recovery/` |
| Website docs | `docs/website/` or `docs/wiki/` depending on current use |
| Examples | `examples/` |
| Scripts | `scripts/` |
| Internal helper tools | `tools/` |
| Rust source | `src/`, `phase1-core/`, `xtask/` |
| Tests | `tests/` |
| Assets | `assets/` |
| GitHub templates/workflows | `.github/` |

## Create-a-place checklist

Before reorganizing a file, identify exactly one destination category:

- docs;
- source;
- tests;
- scripts;
- assets;
- examples;
- tools;
- release notes;
- compatibility path;
- GitHub automation.

If none fits, create an index or destination folder first, then move later.

## Root-level policy

Root should stay readable and small.

Preferred root-level files:

```text
README.md
CONTRIBUTING.md
SECURITY.md
QUALITY.md
QUALITY_SCORECARD.md
FEATURE_STATUS.md
CHANGELOG.md
LICENSE
Cargo.toml
Cargo.lock
phase1
```

Root-level compatibility files may remain when they are public, release-facing, or heavily linked. They should be mirrored or indexed from `docs/` when possible.

## Docs organization policy

The docs tree should be the primary home for explanation and planning:

```text
docs/
  README.md
  REPOSITORY_NAVIGATION.md
  REORGANIZATION_PLAN.md
  MANUAL_ROADMAP.md
  phase1/
  base1/
  fyr/
  operators/
  developers/
  community/
  recovery/
  security/
  templates/
  releases/
  website/
```

If a folder is not ready yet, add it only when a real document needs it.

## Issue and support organization policy

GitHub templates live under:

```text
.github/ISSUE_TEMPLATE/
```

Current planned templates:

- bug report;
- support request;
- feature request;
- documentation issue;
- future crypto/security-policy proposal.

Community planning lives under:

```text
docs/community/
```

## Base1 compatibility policy

Base1 is special because it has many root-level and release/checkpoint references.

Rules:

- keep existing Base1 compatibility paths unless a tested wrapper or mirror exists;
- prefer organized mirrors under `docs/base1/`;
- maintain root compatibility maps;
- run Base1 docs gates before and after Base1 moves.

Required gate:

```bash
sh scripts/quality-check.sh base1-docs
```

For broad Base1 movement:

```bash
sh scripts/quality-check.sh base1-reorg
```

## Security and crypto organization policy

Security and crypto docs stay under:

```text
docs/security/
```

Crypto docs must remain documentation-first until implementation, tests, review, and validation exist.

Required gate:

```bash
sh scripts/quality-check.sh security-crypto-docs
```

Do not move or rename security/crypto files without updating the integrity gate and tests.

## Suggested phases

### Phase 1: navigation and indexes

- Create repository navigation guide.
- Create this reorganization plan.
- Link both from `docs/README.md` and `README.md` where appropriate.
- Ensure issue templates, contribution guide, support docs, and quality gates are easy to find.

### Phase 2: destination folders

Create only folders that are actively needed:

- `docs/releases/`
- `docs/website/`
- `examples/`
- `tools/`

Do not create empty folders unless tooling requires placeholders.

### Phase 3: docs mirrors and indexes

- Add indexes before moving files.
- Mirror or link high-traffic root docs into organized docs locations.
- Add tests for required links.

### Phase 4: low-risk moves

Move only low-risk docs that are not public compatibility paths.

Each move needs:

- old path -> new path map;
- link update;
- test update;
- rollback note.

### Phase 5: assets and examples

- Put logos, banners, symbols, and generated public visuals under `assets/`.
- Put runnable examples under `examples/`.
- Avoid mixing examples with source, docs, or release files.

### Phase 6: script/tool boundary

- Keep user-facing scripts in `scripts/`.
- Move internal-only helper utilities to `tools/` only after script references are checked.
- Preserve wrappers for renamed scripts.

### Phase 7: broad cleanup

Only after tests and link checks pass:

- reduce root clutter;
- replace duplicate docs with indexes or compatibility pointers;
- keep release/checkpoint references stable.

## Move map template

Use this table before moving files:

| Current path | Proposed path | Keep old path? | Compatibility method | Gate |
| --- | --- | --- | --- | --- |
|  |  | yes/no | mirror/index/wrapper/link |  |

## Rollback rule

Every reorganization PR should be reversible.

A reorganization PR should state:

- files moved;
- links updated;
- compatibility paths kept;
- validation run;
- rollback plan.

## Validation plan

Before and after reorganization work, run:

```bash
sh scripts/quality-check.sh quick
```

For Base1-related movement:

```bash
sh scripts/quality-check.sh base1-docs
```

For crypto/security docs movement:

```bash
sh scripts/quality-check.sh security-crypto-docs
```

For broad Base1 reorganization:

```bash
sh scripts/quality-check.sh base1-reorg
```

## Readiness estimate

Current status:

- documentation/support reorganization: mostly ready;
- physical file movement: staged readiness only;
- broad deletion or root cleanup: not ready until move maps and compatibility shims exist.

## Non-claims

This plan does not move files by itself, prove repository quality, remove compatibility obligations, or make Phase1, Base1, or Fyr production-ready.

It defines a minimalist target structure and preservation-first path for future repository reorganization.
