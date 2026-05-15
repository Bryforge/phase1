# Fyr native execution guidance

Status: active operator guidance
Scope: safe use of Fyr inline/native execution versus file-backed `.fyr` source workflows.

## Purpose

Fyr can be exercised from the Phase1 native command surface, but native inline runs should stay short and disposable.

Real Fyr work should be saved into `.fyr` source files and edited with an editor.

This keeps Fyr programs reviewable, repeatable, testable, and recoverable.

## Rule

Use native or inline Fyr execution only for:

- short parser checks;
- quick expression tests;
- smoke tests;
- command-surface verification;
- tiny examples that do not need to be saved.

Use `.fyr` files for:

- real programs;
- package work;
- reusable scripts;
- examples that should be committed;
- tests;
- automation candidates;
- anything that might need review, diffing, recovery, or repeat execution.

## Recommended workflow

Prefer this shape for real work:

```text
fyr init app
avim app/src/main.fyr
fyr check app
fyr test app
fyr run app/src/main.fyr
```

Single-file work should also be file-backed:

```text
fyr new hello.fyr
avim hello.fyr
fyr check hello.fyr
fyr run hello.fyr
```

## Native execution boundary

Native execution is useful for checking that Fyr is alive and for testing tiny snippets.

It should not become the normal place where operators write programs.

The native environment should not encourage long, unsaved source text, because long inline input is harder to:

- save;
- recover;
- diff;
- format;
- review;
- test repeatedly;
- attach to package workflows;
- inspect during failures.

## Editor expectation

Operators should use an editor for meaningful Fyr source.

Current acceptable editor paths include Phase1 editor commands such as:

```text
avim file.fyr
ned file.fyr
```

Host editors may be used only through explicit host-facing workflows outside Fyr's VFS-first safety boundary.

## Package and test expectation

Package/check/build/test/run workflows should prefer file-backed sources.

A package should keep source and tests under stable paths:

```text
fyr.toml
src/main.fyr
tests/smoke.fyr
```

## Safety relationship

This guidance supports the existing Fyr safety model:

- VFS-first by default;
- no host shell by default;
- no host compiler by default;
- no network by default;
- deterministic outputs for check/build/test/run evidence.

## Non-claims

This guidance does not claim Fyr is production-ready, hardened, audited, or a general-purpose sandbox.

It is operator workflow guidance for keeping Fyr usage evidence-bound and file-backed.
