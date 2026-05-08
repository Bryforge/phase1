# Updates, Releases, and Validation

![Edge](https://img.shields.io/badge/edge-v4.1.0--dev-00d8ff) ![Stable](https://img.shields.io/badge/stable-v4.0.0-39ff88) ![Validation](https://img.shields.io/badge/tests-required-39ff88)

Phase1 treats updates and release work as guarded operator workflows. Dry-run plans are preferred until execution is explicitly requested.

## Current release tracks

| Track | Branch | Version | Purpose |
| --- | --- | --- | --- |
| Stable tag target | `release/v4.0.0` | `v4.0.0` | Preserved stable point for the `v4.0.0` tag |
| Bleeding edge | `edge/v4.1.0-dev` | `v4.1.0-dev` | Post-v4.0.0 development and validation |
| Previous stable | historical | `v3.10.9` | Previous stable reference line |
| Compatibility base | historical | `v3.6.0` | Long-term comparison base |

## Validation checklist

Run this before every push:

> [!TIP]
> TRY THIS
>
> ```bash
> cargo fmt --all -- --check
> cargo check --all-targets
> cargo clippy --all-targets -- -D warnings
> cargo test --all-targets
> ```

Full stable release validation:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

Optional targeted validation:

```bash
cargo test --test smoke -- --nocapture
cargo test --test bleeding -- --nocapture
cargo test --test game -- --nocapture
```

## In-app update commands

| Command | Purpose |
| --- | --- |
| `update` | Show safe update plan |
| `update protocol` | Show update safety rules |
| `update latest --trust-host --check` | Check latest update with host trust gate |
| `update latest --trust-host --execute --build` | Execute guarded update and build |
| `update now --trust-host` | Run guarded update now workflow |
| `update test quick` | Show quick test plan |
| `update test full` | Show full test plan |
| `update test quick --trust-host --execute` | Execute quick test workflow with host trust |

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
- SHIELD must be off
- TRUST HOST must be on
- tracked local changes block execution instead of being overwritten
- updater output is sanitized before display

## v4.0.0 stable tag workflow

Stable releases have no `-dev` suffix and must pass validation.

Current stable release point:

```text
v4.0.0
```

Previous stable reference:

```text
v3.10.9
```

Validate and tag from the stable release branch:

```bash
git fetch origin
git checkout release/v4.0.0
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
git status
git tag v4.0.0
git push origin v4.0.0
```

## v4.1.0-dev bleeding-edge workflow

Bleeding-edge builds keep the `-dev` suffix and must not be advertised as stable.

Current edge line:

```text
v4.1.0-dev
```

Continue development from the edge branch:

```bash
git fetch origin
git checkout edge/v4.1.0-dev
cargo metadata --no-deps --format-version 1 | grep '"version"'
cargo test --all-targets
```

Expected package version on edge:

```text
4.1.0-dev
```

## Documentation release checklist

When version numbers change, update:

```text
Cargo.toml
Cargo.lock
README.md
site.js
docs/wiki/Home.md
docs/wiki/02-Version-Guide.md
docs/wiki/08-Updates-Releases-and-Validation.md
docs/wiki/10-Publish-to-GitHub-Wiki.md
plugins/wiki-version.wasi
plugins/wiki-updates.wasi
tests/release_metadata.rs
/home/readme.txt generator if command behavior changed
```

## Tutorial: Run a complete release check

> [!TIP]
> TRY THIS
>
> ```bash
> git pull origin master
> cargo fmt --all -- --check
> cargo check --all-targets
> cargo clippy --all-targets -- -D warnings
> cargo test --all-targets
> cargo audit
> cargo deny check
> git status
> ```

Expected result:

```text
all tests pass
working tree clean
```

## Handling failures

| Failure | Action |
| --- | --- |
| Formatting diff | Run `cargo fmt --all`, then re-check |
| Compile error | Fix source, then run `cargo check --all-targets` |
| Unit test failure | Fix implementation or expected behavior |
| Smoke test failure | Compare expected output with current UI text |
| Audit or dependency policy failure | Fix, update, or explicitly document the dependency decision before release |
| Local changes block pull | Commit, stash, or discard local changes before pulling |
