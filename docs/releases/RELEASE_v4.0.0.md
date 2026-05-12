# Phase1 v4.0.0 Release Tag Preparation

This document preserves the release checklist for the `v4.0.0` stable tag while `edge/v4.1.0-dev` continues post-stable development.

## Stable tag target

| Item | Value |
| --- | --- |
| Stable branch | `release/v4.0.0` |
| Tag | `v4.0.0` |
| Package version | `4.0.0` |
| Previous stable | `v3.10.9` |
| Compatibility base | `v3.6.0` |
| Edge branch after tag | `edge/v4.1.0-dev` |

## Pre-tag validation

Run the full stable gate from `release/v4.0.0`:

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
```

Expected result:

```text
all validation passes
working tree clean
Cargo.toml version is 4.0.0
Cargo.lock version is 4.0.0
```

## Tag command

After validation passes:

```bash
git tag v4.0.0
git push origin v4.0.0
```

## Release branch protection rule

Do not advance `release/v4.0.0` with bleeding-edge work. New work after this point belongs on `edge/v4.1.0-dev` or a feature branch based on that edge line.

## Documentation rule

Stable release-facing docs should show `v4.0.0` as stable and must not advertise `v4.1.0-dev` as stable. Edge docs may mention both tracks when clearly labeled.
