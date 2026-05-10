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
- Root compatibility map: `docs/base1/ROOT_COMPATIBILITY_MAP.md`
- Release/checkpoint mirror index: `docs/base1/releases/README.md`
- Integrity gate: `scripts/base1-doc-integrity.sh`
- Quality integration: `sh scripts/quality-check.sh base1-docs`
- Root checkpoint-note compatibility files remain present.
- Organized release/checkpoint mirrors remain present.
- Rust tests cover the root compatibility map and quality gate.

## Not ready yet

Before a full reorganization, Base1 still needs:

1. A complete inventory of Base1 docs, scripts, tests, and root compatibility files.
2. A path-by-path migration table for every planned new location.
3. A policy for generated files, legacy files, and compatibility shims.
4. Tests for every major moved group, not only release/checkpoint notes.
5. A script or test that verifies old compatibility paths still resolve.
6. A documentation update that explains the final post-reorganization layout.
7. A final integrity pass through `sh scripts/quality-check.sh base1-docs` and `cargo test --all-targets`.

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

## Recommended next work

The next safe step is inventory, not broad movement.

Create or update an inventory that groups Base1 material into:

- Core design docs.
- Recovery USB docs.
- Libreboot docs.
- Release/checkpoint notes.
- Real-device read-only docs.
- Validation reports and templates.
- Dry-run scripts.
- Tests.
- Root compatibility paths.

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
