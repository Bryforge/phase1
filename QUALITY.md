# Phase1 Quality Management System

Phase1 quality is managed as a repeatable system, not a one-time review.

This document defines the quality gates, ownership areas, scorecard, and release expectations for the Phase1 codebase.

## Quality mission

Phase1 should be:

- reliable to build
- predictable to run
- safe by default
- clear to operate
- documented enough for users and future AI-assisted project management
- tested wherever behavior is claimed
- easy to inspect through small scripts and CI

## Quality pillars

| Pillar | Purpose | Required evidence |
| --- | --- | --- |
| Build | Phase1 compiles cleanly | `cargo check --all-targets` |
| Format | Code stays mechanically consistent | `cargo fmt --all -- --check` |
| Lint | Rust mistakes are caught early | `cargo clippy --all-targets -- -D warnings` |
| Test | Behavior is backed by tests | `cargo test --all-targets` |
| Scripts | Shell tooling stays valid | `sh -n` checks and script-specific tests |
| Docs | User-facing claims stay discoverable | required docs and links exist |
| Release | Version and release metadata stay aligned | release metadata checks |
| Safety | Risky behavior remains explicit and guarded | safety docs and policy checks |
| Dependency | Dependency risk is reviewed | `cargo audit`, `cargo deny check` when tools are available |

## Quality gates

### Required for every PR

```bash
sh scripts/quality-check.sh quick
```

The quick gate validates formatting, compilation, Clippy, tests, script syntax, required docs, and quality score generation.

### Required before release

```bash
sh scripts/quality-check.sh full
```

The full gate runs the quick gate and then attempts dependency checks with `cargo audit` and `cargo deny` when installed.

### Scorecard

```bash
sh scripts/quality-score.sh
```

The scorecard gives a deterministic repository health snapshot. It is intentionally simple and CI-safe.

## Quality score model

The current scorecard totals 100 points:

| Area | Points |
| --- | ---: |
| Required docs present | 20 |
| Required scripts present | 20 |
| Rust source present | 10 |
| Tests present | 15 |
| CI workflows present | 15 |
| Safety docs present | 10 |
| Release docs present | 10 |

The score is not a replacement for tests. It is a quick health indicator that makes missing quality infrastructure visible.

## Required files

Quality-critical docs:

```text
README.md
SECURITY.md
SECURITY_REVIEW.md
UPDATE_PROTOCOL.md
QUALITY.md
QUALITY_SCORECARD.md
```

Quality-critical scripts:

```text
scripts/quality-check.sh
scripts/quality-score.sh
scripts/base1-preflight.sh
scripts/test-release-metadata.sh
scripts/test-website.sh
```

## PR review checklist

Every PR should answer:

- What changed?
- What user behavior changed?
- What files were touched?
- What validation was run?
- What risks remain?
- Are docs updated?
- Are tests updated?
- Does the change preserve safe defaults?

## Quality ownership areas

| Area | Files |
| --- | --- |
| Core shell | `src/main.rs`, `src/commands.rs`, `src/registry.rs` |
| Virtual system | `src/kernel.rs`, `src/policy.rs`, `src/ops_log.rs` |
| Text and editors | `src/text.rs`, `src/ned.rs`, `src/avim.rs` |
| Host-facing helpers | `src/bin/phase1-storage.rs`, `scripts/` |
| Base1 | `base1/`, `scripts/base1-*` |
| Website/wiki | `index.html`, `docs/wiki/`, `WIKI_ROADMAP.md` |
| Release/update | `UPDATE_PROTOCOL.md`, `CHANGELOG.md`, `Cargo.toml` |
| Quality system | `QUALITY.md`, `QUALITY_SCORECARD.md`, `scripts/quality-*`, `.github/workflows/quality.yml` |

## Safety baseline

Quality work must preserve these defaults:

- safe mode remains the normal posture
- host-backed behavior stays explicit
- network-changing behavior stays opt-in
- credential-like values are not printed intentionally
- docs do not overclaim unimplemented features
- tests are preferred over claims

## Local workflow

```bash
sh scripts/quality-score.sh
sh scripts/quality-check.sh quick
```

Before release:

```bash
sh scripts/quality-check.sh full
```

## CI workflow

The `Quality` workflow runs the quality scripts on pushes and pull requests. The existing Rust CI remains the deeper compiler/test/dependency gate.

## Continuous improvement

When a quality problem appears more than once, add one of:

- a test
- a quality-check rule
- a scorecard item
- a documentation requirement
- a CI workflow step
