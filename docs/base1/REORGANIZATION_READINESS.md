# Base1 reorganization readiness

Status: active readiness checklist
Scope: Base1 documentation and script organization

## Current state

Base1 is not ready for a full reorganization yet.

The repository is ready for safe incremental organization only: add indexes, add mirrors, update references, preserve root compatibility paths, and run integrity checks.

## Ready now

The following safeguards are in place:

- Documentation map: `docs/base1/DOCUMENTATION_MAP.md`
- Organization plan: `docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md`
- Base1 inventory seed: `docs/base1/INVENTORY.md`
- Base1 test inventory seed: `docs/base1/TEST_INVENTORY.md`
- Path-by-path migration planning table: `docs/base1/MIGRATION_TABLE.md`
- Script compatibility plan: `docs/base1/SCRIPT_COMPATIBILITY_PLAN.md`
- Link-check strategy: `docs/base1/LINK_CHECK_STRATEGY.md`
- Root compatibility map: `docs/base1/ROOT_COMPATIBILITY_MAP.md`
- Release/checkpoint mirror index: `docs/base1/releases/README.md`
- Integrity gate: `scripts/base1-doc-integrity.sh`
- Quality integration: `sh scripts/quality-check.sh base1-docs`
- Root checkpoint-note compatibility files remain present.
- Organized release/checkpoint mirrors remain present.
- Current script paths remain the stable operator interface.
- Rust tests cover the root compatibility map, quality gate, inventory docs, migration table docs, script compatibility plan docs, and link-check strategy docs.

## Not ready yet

Before a full reorganization, Base1 still needs:

1. A complete repository-wide test listing, not only the current seed inventory.
2. An implemented Markdown link checker or equivalent validation command, not only the current strategy document.
3. Tests for every major moved group before that group is moved.
4. A final post-reorganization layout document that names the stable public paths.
5. A final integrity pass through `sh scripts/quality-check.sh base1-docs` and `cargo test --all-targets`.

## Full reorganization readiness criteria

Base1 can be considered ready for a full reorganization only when all of these are true:

- Every file group has an owner category.
- Every old path has a compatibility decision.
- Every new path is listed in the documentation map.
- Every inbound link is accounted for.
- Every release/checkpoint note remains recoverable.
- Every read-only and dry-run guardrail remains visible.
- Tests cover the organized layout and compatibility paths.
- The quality gate includes the Base1 integrity gate.
- No compatibility path is removed without explicit future approval.
- Any script relocation has a wrapper or compatibility command plan.
- Markdown path movement is protected by a local, read-only link checker or equivalent validation.

## Recommended next work

The next safe step is not broad movement yet.

Recommended order:

1. Expand the test inventory into a repository-wide Base1 test listing.
2. Implement or add an equivalent local link-check command for Markdown moves.
3. Add the final post-reorganization layout document.
4. Only then move one small doc group at a time, preserving compatibility paths.

## Operator command

Run this before and after every Base1 organization change:

```bash
sh scripts/quality-check.sh base1-docs
```

Run this before claiming broad readiness:

```bash
sh scripts/quality-check.sh quick
```

## Non-claims

This readiness checklist does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only defines when repository organization is safe enough to proceed more broadly.
