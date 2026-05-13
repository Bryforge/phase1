# Updates, Releases, and Validation

![Edge](https://img.shields.io/badge/edge-v6.0.0-00d8ff) ![Stable](https://img.shields.io/badge/stable-v5.0.0-39ff88) ![Validation](https://img.shields.io/badge/tests-required-39ff88) ![Safe Defaults](https://img.shields.io/badge/safe%20defaults-required-39ff88)

Phase1 treats updates and release work as guarded operator workflows. Dry-run plans are preferred until execution is explicitly requested.

## Current release tracks

| Track | Branch or reference | Version | Purpose |
| --- | --- | --- | --- |
| Edge | `edge/stable` | `v6.0.0` | Active development, v6 docs, UI/help polish, Base1/Fyr work, and validation. |
| Stable base | `base/v5.0.0` | `v5.0.0` | Current stable base for release-qualified work. |
| Previous stable | historical | `v4.4.0` | Previous stable comparison line. |
| Compatibility base | historical | `v3.6.0` | Long-term historical comparison base. |
| Base1 | foundation track | `foundation` | Boot, recovery, installer, rollback, storage, and hardware-validation planning. |

## Validation checklist

Run this before ordinary pushes:

> [!TIP]
> TRY THIS
>
> ```bash
> sh scripts/quality-check.sh quick
> ```

Run Rust-specific validation when changing code:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Run the full validation gate before release work:

```bash
sh scripts/quality-check.sh full
```

Focused gates:

```bash
sh scripts/quality-check.sh base1-docs
sh scripts/quality-check.sh base1-reorg
sh scripts/quality-check.sh security-crypto-docs
```

Optional security tooling:

```bash
cargo install cargo-audit --locked
cargo install cargo-deny --locked
cargo audit
cargo deny check
```

## In-app update commands

| Command | Purpose |
| --- | --- |
| `update` | Show safe update plan. |
| `update protocol` | Show update safety rules. |
| `update latest --trust-host --check` | Check latest update with host trust gate. |
| `update latest --trust-host --execute --build` | Execute guarded update and build. |
| `update now --trust-host` | Run guarded update workflow. |
| `update test quick` | Show quick test plan. |
| `update test full` | Show full test plan. |
| `update test quick --trust-host --execute` | Execute quick test workflow with host trust. |

> [!TIP]
> TRY THIS INSIDE PHASE1
>
> ```text
> update
> update protocol
> update test quick
> update test full
> ```

## Update safety rules

> [!IMPORTANT]
> Update execution is host-backed and guarded. The operator must explicitly request execution.

Rules:

- `update` defaults to a dry-run plan
- `--execute` is required before file mutation
- `--trust-host` is required for host-backed execution paths
- SHIELD must be off for host-backed execution
- TRUST HOST must be on for host-backed execution
- tracked local changes block execution instead of being overwritten
- updater output is sanitized before display
- secrets must never be placed in issue text, wiki examples, logs, screenshots, or demo commands

## Edge workflow

Continue active development from `edge/stable`:

```bash
git fetch origin
git checkout edge/stable
git pull origin edge/stable
cargo metadata --no-deps --format-version 1 | grep '"version"'
sh scripts/quality-check.sh quick
```

Expected package version on the current edge line:

```text
6.0.0
```

## Stable base workflow

Stable references should point at the stable base line and must not include unsupported edge-only claims.

Current stable base:

```text
v5.0.0
```

Before release-facing work, verify the target branch, package version, and documentation status. Use release notes, README status, wiki pages, website demo output, and in-system wiki pages consistently.

## Base1 validation workflow

Base1 work must stay evidence-bound. Read-only and dry-run commands are preferred until implementation and review justify stronger claims.

Common safe checks:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
sh scripts/base1-preflight.sh
sh scripts/base1-libreboot-preflight.sh
sh scripts/base1-install-dry-run.sh --dry-run --target /dev/example
sh scripts/base1-recovery-dry-run.sh --dry-run
sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example
sh scripts/base1-rollback-metadata-dry-run.sh --dry-run
```

Focused Base1 tests may include:

```bash
cargo test -p phase1 --test base1_x86_64_detect_script
cargo test -p phase1 --test base1_b2_assembly_dry_run_script
cargo test -p phase1 --test b1_read_only_detection_validation_docs
cargo test -p phase1 --test b2_dry_run_assembly_validation_docs
```

## Documentation release checklist

When version numbers, public claims, wiki navigation, Base1 status, Fyr behavior, or command output changes, update:

```text
Cargo.toml
Cargo.lock
README.md
site/site.js
docs/wiki/Home.md
docs/wiki/01-Quick-Start.md
docs/wiki/02-Version-Guide.md
docs/wiki/08-Updates-Releases-and-Validation.md
docs/wiki/10-Publish-to-GitHub-Wiki.md
docs/wiki/11-Tutorials.md
docs/wiki/12-In-System-Wiki.md
docs/wiki/13-Base1-OS-Track.md
docs/wiki/14-Fyr-Native-Language.md
plugins/wiki-version.wasi
plugins/wiki-updates.wasi
plugins/wiki-quick.wasi
tests/release_metadata.rs
/home/readme.txt generator if command behavior changed
```

## Tutorial: Run a complete release check

```bash
git fetch origin
git status
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
sh scripts/quality-check.sh full
git status
```

Expected result:

```text
format passes
compile passes
clippy passes
tests pass
quality gate passes
working tree clean
```

## Handling failures

| Failure | Action |
| --- | --- |
| Formatting diff | Run `cargo fmt --all`, then re-check. |
| Compile error | Fix source, then run `cargo check --all-targets`. |
| Unit test failure | Fix implementation or expected behavior. |
| Smoke test failure | Compare expected output with current UI text. |
| Quality gate failure | Read the focused gate output and update docs/tests together. |
| Audit or dependency policy failure | Fix, update, or explicitly document the dependency decision before release. |
| Local changes block pull | Commit, stash, or discard local changes before pulling. |
| Base1 claim is ahead of evidence | Reword as roadmap/dry-run/read-only status until validation exists. |
