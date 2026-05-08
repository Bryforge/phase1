# Phase1 v4.1.0 Stable Release Preparation

This document prepares the `v4.1.0` stable release candidate.

## Stable tag target

| Item | Value |
| --- | --- |
| Preparation branch | `prepare-v4.1.0-stable` |
| Stable branch to create after validation | `release/v4.1.0` |
| Tag | `v4.1.0` |
| Package version | `4.1.0` |
| Previous stable | `v4.0.0` |
| Compatibility base | `v3.6.0` |
| Next edge branch | `edge/v4.2.0-dev` |

## Release highlights

- Promotes the post-`v4.0.0` Phase1 work into a stable `v4.1.0` candidate.
- Includes the current Phase1 Terminal wrapper and Gina/assistant WASI-lite offline assistant path.
- Includes CodeQL remediation for Rust build mode and logging/privacy hardening.
- Removes the unpinned Rust toolchain action path from the Rust workflow.
- Keeps safe-by-default host-tool policy, runtime guards, secret scanning, and CodeQL validation in place.

## Pre-tag validation

Run the full stable gate from `prepare-v4.1.0-stable` before creating `release/v4.1.0`:

```bash
git fetch origin
git checkout prepare-v4.1.0-stable
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
sh scripts/quality-check.sh full
git status
```

Expected result:

```text
all validation passes
working tree clean
Cargo.toml version is 4.1.0
Cargo.lock version is 4.1.0
CodeQL and security checks are green
```

## Branch and tag commands

After validation passes and the stable PR is merged:

```bash
git fetch origin
git checkout master
git pull --ff-only origin master
git branch release/v4.1.0
git tag v4.1.0
git push origin release/v4.1.0 v4.1.0
```

## Post-release edge rule

After `v4.1.0` is tagged, resume development on `edge/v4.2.0-dev` or feature branches based on that edge line. Keep release-facing documentation clear that `v4.1.0` is stable and `v4.2.0-dev` is experimental.
