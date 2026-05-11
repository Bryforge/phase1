# Base1 pre-move checklist

Status: active pre-move checklist
Scope: Base1 documentation, script, test, and compatibility-path moves

## Purpose

This checklist defines what must be true before any Base1 file group is moved or mirrored into a new location.

It is preservation-first. It does not authorize deletion or broad movement.

## Required before any group move

Before moving or mirroring a Base1 group, confirm:

1. The group is listed in `docs/base1/INVENTORY.md`.
2. Related tests are listed in `docs/base1/TEST_INVENTORY.md`.
3. Current and proposed paths are listed in `docs/base1/MIGRATION_TABLE.md`.
4. Stable public paths are consistent with `docs/base1/POST_REORGANIZATION_LAYOUT.md`.
5. Compatibility paths are documented in `docs/base1/ROOT_COMPATIBILITY_MAP.md` or the relevant compatibility plan.
6. Script movement, if any, follows `docs/base1/SCRIPT_COMPATIBILITY_PLAN.md`.
7. Markdown movement, if any, passes `scripts/base1-link-check.sh`.
8. The docs quality gate passes:

```bash
sh scripts/quality-check.sh base1-docs
```

## Pre-move test requirement

Every group move must have tests before the move.

At minimum, tests must prove:

- Old compatibility paths are still present or wrapped.
- New organized paths are present.
- Indexes link to the new organized paths.
- Existing public paths remain recoverable.
- Non-claims remain visible.
- Dry-run and read-only wording remains visible where relevant.

## First safe candidate group

The safest current candidate group is release/checkpoint notes because organized mirrors already exist under `docs/base1/releases/` while root compatibility paths remain present.

Even for this group, no root checkpoint file should be removed.

Required tests for this group include:

- Root compatibility map tests.
- Release/checkpoint mirror tests.
- README and documentation map link tests.
- Base1 link-check tests.
- Test-inventory verification.

## Do not move yet when

Do not move a group when:

- The reporter and test inventory disagree.
- Link checking fails.
- The migration table lacks a compatibility decision.
- A script move lacks a wrapper plan.
- A public/root path would disappear.
- Non-claims would be weakened.
- The move would require deletion to look clean.

## Validation command

Run before and after any group move:

```bash
sh scripts/quality-check.sh base1-docs
cargo test --all-targets
```

## Non-claims

This checklist does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only defines the safety conditions for future file organization.
