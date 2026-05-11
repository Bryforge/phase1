# Phase1 release documentation

Status: active release documentation index
Scope: organized release notes, checkpoint notes, compatibility mirrors, and release-facing validation references

## Purpose

This directory is the preferred organized home for release documentation.

Root-level release notes and checkpoint files may remain as compatibility paths when they are public, heavily linked, or referenced by existing workflows. This index provides a minimalist place for organized release documentation without breaking current paths.

## Reorganization policy

Release documentation reorganization is preservation-first.

Rules:

- Do not delete root-level release notes unless a future move map explicitly approves it.
- Prefer adding organized mirrors or indexes before moving release files.
- Keep old path -> new path mappings when files are mirrored or moved.
- Preserve Base1 checkpoint and recovery release references.
- Update tests when release paths become required navigation paths.

## Planned structure

```text
docs/releases/
  README.md
  phase1/
  base1/
  checkpoints/
```

Create subdirectories only when they contain real release documents or indexes.

## Suggested categories

| Category | Purpose |
| --- | --- |
| `phase1/` | Phase1 stable, edge, and milestone release notes. |
| `base1/` | Base1 read-only checkpoint and recovery release notes. |
| `checkpoints/` | Verified checkpoint summaries and compatibility maps. |

## Validation

Before and after release documentation reorganization, run:

```bash
sh scripts/quality-check.sh quick
```

For Base1 release/checkpoint documentation, also run:

```bash
sh scripts/quality-check.sh base1-docs
```

## Non-claims

This index does not move release files by itself, remove root compatibility paths, publish a release, or prove release readiness.

It creates an organized destination for future release documentation work.
