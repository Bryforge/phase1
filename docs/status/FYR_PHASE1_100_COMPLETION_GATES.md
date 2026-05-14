# Fyr and Phase1 100% completion gates

Status: planning gate and evidence checklist  
Scope: Fyr first, then Phase1 promotion alongside Fyr  
Branch target: `edge/stable`

This document defines what "100%" means for Fyr and for Phase1 without weakening the repository's existing non-claims. The goal is to move fast, but only promote percentages when implementation, tests, docs, and release evidence all agree.

## Current public baseline

The current public status marker lists:

| Track | Current estimate | Current state | Next milestone |
| --- | ---: | --- | --- |
| Phase1 operator console | 82% | Usable edge console with guarded host access, VFS, dashboards, help UI, themes, learning, and tests. | Polish release-facing flows and keep safe defaults simple. |
| Fyr native language | 44% | Seed language and toolchain surface exist with scripts, tests, assertions, package checks, and docs. | Expand language book, package workflow, and runtime integration. |
| Overall roadmap | 65% | Active edge development with public status and evidence checkpoints. | Keep progress evidence-bound. |

The repository organization track may remain 100% as long as root cleanliness, generated-artifact exclusion, and compatibility links stay clean.

## Definition of 100%

A track reaches 100% only when all of these are true:

1. The feature is implemented in source, not only described in docs.
2. The behavior has deterministic tests that pass in CI or in the recorded validation log.
3. Public docs describe exactly what works and what does not.
4. The public status marker and release notes match the evidence.
5. Security, host-access, hardware, installer, daily-driver, and production-readiness non-claims remain explicit unless separately proven.

## Fyr path to 100%

### F0-F2: Identity, seed runner, and authoring loop

Gate status: present but should stay evidence-checked.

Required evidence:

- `fyr status`, `fyr spec`, `fyr new`, `fyr cat`, `fyr self`, and `fyr run` are covered by tests.
- README, native language spec, examples, and command help agree on `.fyr`, `fyr`, and VFS-first behavior.
- No host shell, Cargo, network, or external compiler is required for seed workflows.

Promotion target: keep this foundation stable; do not call it complete until the smoke suite is green after each parser/toolchain change.

### F3: Core syntax

Target after completion: Fyr can be described as a small, stable, Phase1-native scripting language instead of only a seed language.

Required implementation:

- Lexer/token handling for identifiers, integers, strings, punctuation, comments, and simple operators.
- Parser support for `fn`, `let`, `return`, `print`, assertions, basic arithmetic, comparisons, and predictable blocks.
- Diagnostics that name the file and explain the failure without leaking host details.
- Duplicate entry-point detection and missing-main detection.

Required tests:

- Passing tests for valid single-file programs.
- Passing tests for package `src/*.fyr` modules.
- Failing tests for invalid identifiers, malformed blocks, unsupported values, duplicate `fn main`, and missing `fn main`.

### F4: Safe runtime

Target after completion: Fyr can run useful operator scripts inside Phase1 without expanding host authority.

Required implementation:

- Runtime remains VFS-only by default.
- Read/write operations are explicit and bounded.
- Runtime output is deterministic.
- Errors are redacted, structured, and testable.
- Time, size, and recursion limits are enforced before broader language growth.

Required tests:

- VFS read/write happy path.
- Permission/guard failure path.
- Bounded runtime path.
- Error-redaction path.

### F5: Phase1 self-workflows

Target after completion: Fyr helps Phase1 inspect and maintain itself without replacing Rust.

Required implementation:

- Repository manifest reader.
- Documentation sync helper.
- Checkpoint helper.
- Public status helper or read-only public status inspector.
- `fyr self` explains current capability and limits.

Required tests:

- Self-workflow commands operate on fixture data.
- Generated or inspected outputs remain deterministic.
- The workflows refuse to alter host files unless a guarded Phase1 path explicitly permits it.

### F6: Standard library

Target after completion: Fyr has a small Phase1-owned standard library with stable names.

Required modules:

- `vfs`
- `text`
- `json-lite`
- `audit`
- `process` as metadata-only unless host execution is separately guarded
- `package`
- `doc`

Required tests:

- One smoke test per module.
- One failure-mode test per module.
- Compatibility tests for command-help and language-book examples.

### F7: Compiler or packaging path

Target after completion: the repository has a clear decision on whether Fyr remains interpreted, lowers to Rust, or targets WASI-lite.

Required evidence:

- Design note explaining the chosen path and rejected alternatives.
- Packaging contract for `.fyr` projects.
- Release compatibility policy.
- Versioned example packages.

## Fyr 100% release checklist

Fyr may be marked 100% only when:

- All F0-F7 gates are satisfied.
- `cargo test` or the documented validation group passes.
- Language-book examples match real command behavior.
- Toolchain docs match real command behavior.
- Public status moves from "seed language" wording to "Phase1-native scripting language" wording.
- Non-claims still state that Fyr is not a production replacement for Rust, Python, C, or shell unless a separate production readiness review proves that claim.

## Phase1 path alongside Fyr

Phase1 should rise with Fyr, but not by hiding unfinished OS/security work. The Phase1 percentage can move when Fyr increases the practical operator surface and when the broader console remains stable.

### P1: Release-facing operator flow

Required evidence:

- Quick start works from a clean checkout.
- Main help, Fyr help, Base1 help, storage help, and update/status docs agree.
- Public status, badge, and project status are generated from current repository state.

### P2: Guarded host workspace

Required evidence:

- Host tool execution stays opt-in.
- Git/Cargo flows, if enabled, run inside `.phase1/host-workspaces/` or another documented guarded workspace.
- Argument filtering, timeout handling, audit output, and failure redaction are tested.

### P3: Validation matrix

Required evidence:

- Unit tests pass.
- Script syntax checks pass for maintained shell scripts.
- Documentation integrity checks pass.
- Link checks pass or publish a scoped exception file.
- Public status generation is tested.

### P4: Release candidate boundary

Required evidence:

- Release notes name what is included and what is not.
- Hardware, installer, security-hardening, daily-driver, and production claims remain separate from roadmap progress.
- Recovery and rollback instructions exist for any public release candidate.

## Promotion rule

Do not edit `site/status.json` or public percentage values manually to reach 100%. Update implementation and tests first, then regenerate public status from the repository state.

Recommended order:

1. Finish Fyr F3 core syntax and tests.
2. Finish Fyr F4 safe runtime and tests.
3. Land Phase1 guarded host workspace v2 so Git/Cargo flows are useful but contained.
4. Finish F5 self-workflows.
5. Add F6 standard library smoke/failure tests.
6. Decide and document F7 packaging/compiler path.
7. Regenerate status and publish release notes only after validation passes.

## Public wording rule

Use this wording until every 100% gate is satisfied:

> Fyr is the Phase1-native language track. It has a seed runner, package bootstrap, checks, tests, and growing parser/runtime support. It is not yet a production language or a general replacement for Rust, Python, C, or shell.

Use this wording only after the 100% gate is satisfied:

> Fyr is Phase1's native scripting language for VFS-first operator automation, with documented syntax, deterministic tests, a bounded runtime, package workflow, standard library surface, and release compatibility policy.
