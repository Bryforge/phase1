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
- Base1 test inventory reporter: `scripts/base1-test-inventory.sh`
- Base1 test inventory verifier: `scripts/base1-test-inventory-verify.sh`
- Base1 reorganization verifier: `scripts/base1-reorganization-verify.sh`
- Base1 reorganization quality mode: `sh scripts/quality-check.sh base1-reorg`
- Base1 reorganization quality alias: `sh scripts/quality-check.sh base1-reorganization`
- Path-by-path migration planning table: `docs/base1/MIGRATION_TABLE.md`
- Script compatibility plan: `docs/base1/SCRIPT_COMPATIBILITY_PLAN.md`
- Link-check strategy: `docs/base1/LINK_CHECK_STRATEGY.md`
- Local Markdown link checker: `scripts/base1-link-check.sh`
- Proposed post-reorganization layout: `docs/base1/POST_REORGANIZATION_LAYOUT.md`
- Pre-move checklist: `docs/base1/PRE_MOVE_CHECKLIST.md`
- Release/checkpoint pre-move checks: `docs/base1/releases/PRE_MOVE_CHECKS.md`
- Root compatibility map: `docs/base1/ROOT_COMPATIBILITY_MAP.md`
- Release/checkpoint mirror index: `docs/base1/releases/README.md`
- Integrity gate: `scripts/base1-doc-integrity.sh`
- Quality integration: `sh scripts/quality-check.sh base1-docs`
- Root checkpoint-note compatibility files remain present.
- Organized release/checkpoint mirrors remain present.
- Current script paths remain the stable operator interface.
- Rust tests cover the root compatibility map, quality gate, inventory docs, manual verification rule, migration table docs, script compatibility plan docs, link-check strategy docs, link-check script behavior, test inventory reporter and verifier, reorganization verifier, post-reorganization layout docs, pre-move checklist docs, release/checkpoint pre-move checks, and readiness docs.

## Not ready yet

Before a full reorganization, Base1 still needs:

1. A successful verified comparison of reporter output against `docs/base1/TEST_INVENTORY.md`.
2. A successful final run of `sh scripts/quality-check.sh base1-reorg` on a Rust-capable host.
3. No deletion of compatibility paths unless explicitly approved in a future change.

## First candidate group status

The release/checkpoint note group is the first safe candidate group for small, preservation-first organization work.

Current status:

- Organized mirrors exist under `docs/base1/releases/`.
- Root checkpoint-note compatibility files remain present.
- `docs/base1/releases/PRE_MOVE_CHECKS.md` defines group-specific pre-move checks.
- Release/checkpoint pre-move tests exist.
- No root checkpoint file may be removed.

## Full reorganization readiness criteria

Base1 can be considered ready for a full reorganization only when all of these are true:

- Every file group has an owner category.
- Every old path has a compatibility decision.
- Every new path is listed in the documentation map.
- Every inbound link is accounted for.
- Every release/checkpoint note remains recoverable.
- Every read-only and dry-run guardrail remains visible.
- Tests cover the organized layout and compatibility paths.
- The quality gate includes the Base1 integrity gate, link checker, and test inventory verifier.
- No compatibility path is removed without explicit future approval.
- Any script relocation has a wrapper or compatibility command plan.
- Markdown path movement is protected by a local, read-only link checker or equivalent validation.
- The post-reorganization layout names the stable public paths.
- The pre-move checklist is satisfied for the first group to move.
- The reorganization quality mode passes on a Rust-capable host.

## Recommended next work

The next safe step is not broad movement yet.

Recommended order:

1. Run `sh scripts/base1-test-inventory-verify.sh` and update `docs/base1/TEST_INVENTORY.md` if any reported tests are missing.
2. Run `sh scripts/quality-check.sh base1-reorg` on a Rust-capable host.
3. Move or mirror only one small doc group at a time, preserving compatibility paths.
4. Keep the first candidate group limited to release/checkpoint note organization; do not remove root checkpoint files.

## Operator command

Run this before and after every Base1 organization change:

```bash
sh scripts/quality-check.sh base1-docs
```

Run this before claiming broad readiness:

```bash
sh scripts/quality-check.sh base1-reorg
```

Alias:

```bash
sh scripts/quality-check.sh base1-reorganization
```

## Non-claims

This readiness checklist does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only defines when repository organization is safe enough to proceed more broadly.
