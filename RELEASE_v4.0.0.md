# Phase1 v4.0.0 Release Tag Preparation

This document is the stable release checklist for the `v4.0.0` tag.

## Stable tag target

| Item | Value |
| --- | --- |
| Stable branch | `release/v4.0.0` |
| Tag | `v4.0.0` |
| Package version | `4.0.0` |
| Previous stable | `v3.10.9` |
| Compatibility base | `v3.6.0` |
| Next development line | `edge/v4.1.0-dev` |

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

## Branch rule

Do not advance `release/v4.0.0` with post-stable development work. New development after this point belongs on `edge/v4.1.0-dev` or a feature branch based on that edge line.

## Documentation rule

Stable release-facing docs show `v4.0.0` as stable. Development docs may mention `v4.1.0-dev` only when it is clearly labeled as the next development line.
