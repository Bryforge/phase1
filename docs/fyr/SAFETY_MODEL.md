# Fyr safety model

Status: active F4 boundary document  
Scope: current Fyr runtime/toolchain safety rules  
Non-claim: this does not make Fyr production-ready, hardened, audited, or a general-purpose sandbox.

Fyr is designed to become useful for Phase1 operator automation without expanding host authority. The current rule is simple: Fyr is VFS-first and host-independent by default.

## Current safety boundary

Current Fyr workflows must preserve these defaults:

- VFS-only behavior by default.
- No host shell.
- No network.
- No host compiler.
- No Cargo invocation from Fyr commands.
- Deterministic check/build/test/run output.
- Package build output reports `backend : seed/interpreted` and `host    : none`.
- Diagnostics should name Fyr files/packages without exposing host paths unless the operator explicitly chose a host-facing workflow outside Fyr.

## Current allowed operations

Current Fyr commands may:

- create `.fyr` files in the Phase1 VFS;
- create Fyr package structure in the Phase1 VFS;
- read `.fyr` files from the Phase1 VFS;
- parse and check supported source shapes;
- dry-run build supported files/packages;
- run supported print-literal seed programs;
- run package tests stored under the package `tests/` directory;
- report deterministic diagnostics.

## Current denied operations

Current Fyr commands must not:

- call the host shell;
- call Cargo;
- call rustc or another host compiler;
- access the network;
- read arbitrary host files;
- write arbitrary host files;
- promise hardened sandboxing;
- claim production language readiness.

## Current bounded-runtime behavior

Fyr does not yet expose unbounded language features such as open-ended loops, recursion, host process execution, network access, or external compiler calls.

Current bounds are intentionally conservative:

- supported execution is limited to parsed `.fyr` files stored in the Phase1 VFS;
- package tests execute only discovered `.fyr` files under the package `tests/` directory;
- build output is a deterministic dry-run artifact summary;
- unsupported or malformed source should fail during check/build/test/run instead of falling through to a host tool;
- diagnostics should remain target-scoped and deterministic.

## Current error-redaction behavior

Fyr diagnostics should identify the Phase1 VFS target, not the host workspace used to run Phase1 tests.

Expected diagnostic shape:

```text
fyr check: bad.fyr: <deterministic message>
fyr check: app: <deterministic package message>
```

Diagnostics must not require or expose:

- host temp directories;
- checkout paths;
- host shell error text;
- compiler command lines;
- network URLs;
- secrets or environment values.

## Current deterministic-output behavior

Fyr output should remain repeatable for the same VFS inputs.

Current evidence compares the Fyr-relevant output slices from repeated runs of:

- `fyr build`;
- `fyr run`;
- `fyr test`.

The comparison ignores Phase1 boot/prompt noise and focuses on Fyr command lines such as package, source, AST, backend, host, status, test, passed, and failed rows.

## F4 promotion requirements

F4 may move from planned to active/completed only after the repository includes evidence for:

- VFS read/write happy path tests.
- Guard failure tests.
- Bounded runtime tests.
- Error-redaction tests.
- Deterministic output tests.
- Documentation that matches the implemented command behavior.

## Operator wording

Use this wording while F4 is still being built:

> Fyr currently runs as a Phase1-owned, VFS-first seed language/toolchain. It avoids host shell, network, Cargo, and host compiler access by default. F4 safe runtime work is still in progress.

Do not use this wording yet:

> Fyr is a hardened sandbox.

## Validation links

Related docs:

- [`TOOLCHAIN.md`](TOOLCHAIN.md)
- [`LANGUAGE_BOOK.md`](LANGUAGE_BOOK.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`../status/FYR_PHASE1_100_COMPLETION_GATES.md`](../status/FYR_PHASE1_100_COMPLETION_GATES.md)

Related tests:

- `tests/fyr_authoring_commands.rs`
- `tests/fyr_parser_diagnostics.rs`
- `tests/fyr_f3_package_diagnostics.rs`
- `tests/fyr_f3_expression_diagnostics.rs`
- `tests/fyr_f4_runtime_safety.rs`
- `tests/fyr_f4_error_redaction.rs`
- `tests/fyr_f4_deterministic_output.rs`
