# Phase1 tools

Status: active tools index
Scope: internal maintainer utilities, repository helpers, future automation, and non-user-facing tools

## Purpose

This directory is the preferred organized home for internal helper tooling that supports maintainers but is not part of the primary user/operator script surface.

User-facing commands should generally stay in `scripts/` unless a future move map explicitly moves them with compatibility wrappers.

## Tooling boundary

| Path | Purpose |
| --- | --- |
| `scripts/` | User-facing, operator-facing, CI-facing, or documented validation scripts. |
| `tools/` | Internal maintainer utilities, local helpers, generators, analyzers, and future automation prototypes. |
| `xtask/` | Rust-based repository validation helper when implemented through the workspace. |

## Reorganization policy

Do not move scripts into `tools/` unless all of these exist:

- old path -> new path map;
- compatibility wrapper or clear replacement path;
- link updates;
- tests or syntax checks;
- rollback plan;
- maintainer approval.

When in doubt, keep user-facing scripts in `scripts/`.

## Planned categories

```text
tools/
  README.md
  docs/
  release/
  quality/
  support-ai/
  repo-maintenance/
```

Create subdirectories only when they contain real tools.

## Safety rules

Internal tools must:

- avoid destructive behavior by default;
- support dry-run or preview mode when mutation is possible;
- avoid printing secrets, tokens, private keys, recovery codes, private logs, or unrevised screenshots;
- document required environment variables;
- fail closed when required inputs are missing;
- preserve repository safety and compatibility paths.

## Validation

Before adding or changing tools, run:

```bash
sh scripts/quality-check.sh quick
```

For shell tools, ensure script syntax is checked:

```bash
sh scripts/quality-check.sh scripts
```

## Non-claims

This index does not move scripts, create maintainer automation, prove tool safety, or make internal tools user-facing.

It creates an organized destination for future internal helper tooling while preserving the current `scripts/` surface.
