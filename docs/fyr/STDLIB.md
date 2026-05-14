# Fyr standard library contract

Status: active F6 planning and fixture document  
Scope: future Phase1-owned Fyr standard library module surface  
Non-claim: these modules are not implemented as a complete standard library yet.

F6 defines the small Phase1-owned standard library surface Fyr should grow into after F3-F5 are stable enough to support it.

## Current module contract

The planned module names are:

| Module | Purpose | Current state |
| --- | --- | --- |
| `vfs` | VFS read/write helpers scoped to Phase1-owned paths. | fixture |
| `text` | Small string and text-processing helpers. | planned |
| `json-lite` | Minimal deterministic JSON-like reading/writing. | planned |
| `audit` | Structured validation and event-report helpers. | planned |
| `process` | Metadata-only process information unless a separate guarded policy is implemented. | planned |
| `package` | Fyr package manifest and layout helpers. | planned |
| `doc` | Documentation consistency and link-helper surface. | planned |

## Safety rules

F6 modules must:

- stay VFS-first;
- keep output deterministic;
- avoid host shell access by default;
- avoid network access by default;
- avoid host compiler access by default;
- include smoke tests and failure-mode tests before any module is called complete;
- keep public docs aligned with implemented behavior.

## Fixture evidence

The first F6 fixture is:

```text
docs/fyr/fixtures/stdlib-modules-ok.txt
```

It lists the planned module names and required evidence categories. It is not a module implementation.

The first module-level fixture is:

```text
docs/fyr/fixtures/stdlib-vfs-ok.txt
```

It records the planned `vfs` module operation surface and required evidence categories. It is not a module implementation.

## Completion rule

F6 can move from planned to active when module contracts, fixtures, and first module tests exist.

F6 can be marked complete only when every planned module has:

- implementation evidence;
- smoke tests;
- failure-mode tests;
- documentation examples;
- safety-boundary notes;
- language-book and roadmap alignment.
