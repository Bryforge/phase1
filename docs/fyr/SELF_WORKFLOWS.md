# Fyr inspection workflows

Status: active F5 planning and evidence document  
Scope: using Fyr to help Phase1 inspect and validate Phase1-owned fixtures  
Non-claim: this does not make Fyr production-ready or a replacement for Rust.

F5 is the point where Fyr starts helping Phase1 inspect its own project-shaped data. The work must stay evidence-bound, deterministic, and VFS-first.

## Current state

Current Fyr inspection surface:

- `fyr self` exists as the current status/capability command.
- Fyr package/file workflows are VFS-first.
- F3 parser diagnostics and F4 runtime-safety evidence are active.
- Repository inspection workflows are still planned and must not be described as complete.
- The first repository-manifest fixture lives at [`fixtures/repo-manifest-ok.txt`](fixtures/repo-manifest-ok.txt).

## F5 target workflows

F5 should add or document these workflows only after implementation and tests exist:

1. Repository manifest reader.
2. Documentation consistency helper.
3. Checkpoint metadata helper.
4. Public status reader or documented deferral.
5. Fixture-based validation helper.

## Repository manifest fixture

The first fixture is intentionally small and static:

```text
docs/fyr/fixtures/repo-manifest-ok.txt
```

It records a project-shaped manifest with a name, root, kind, file list, and check list. It is test evidence for the future manifest reader; it is not yet a full reader command.

## Safety rules

F5 inspection workflows must:

- operate on VFS fixtures first;
- keep output deterministic;
- avoid host shell access;
- avoid network access;
- avoid host compiler access;
- avoid arbitrary host file reads or writes;
- keep generated outputs scoped and reviewable;
- preserve all production-readiness, hardening, installer, and daily-driver non-claims.

## Promotion evidence

F5 can move from planned to active only when the repository includes evidence for:

- `fyr self` current behavior test coverage;
- fixture repository manifest reader tests;
- fixture documentation consistency tests;
- fixture checkpoint metadata tests;
- public status reader tests or a documented deferral;
- language-book and roadmap updates that match implemented behavior.

F5 can be marked complete only when those workflows are implemented, tested, documented, and connected to the 100% completion gate.

## Current wording

Use this wording now:

> Fyr has a `fyr self` capability/status surface and a planned F5 path for Phase1 inspection workflows. Repository manifest, documentation consistency, checkpoint metadata, and status reader workflows are not complete yet.

Do not use this wording yet:

> Fyr can fully maintain Phase1 on its own.

## Validation links

Related docs:

- [`ROADMAP.md`](ROADMAP.md)
- [`LANGUAGE_BOOK.md`](LANGUAGE_BOOK.md)
- [`SAFETY_MODEL.md`](SAFETY_MODEL.md)
- [`../status/FYR_PHASE1_100_COMPLETION_GATES.md`](../status/FYR_PHASE1_100_COMPLETION_GATES.md)

Related fixtures:

- [`fixtures/repo-manifest-ok.txt`](fixtures/repo-manifest-ok.txt)

Related tests:

- `tests/fyr_authoring_commands.rs`
- `tests/fyr_f5_self_workflows.rs`
- `tests/fyr_f5_manifest_fixtures.rs`
