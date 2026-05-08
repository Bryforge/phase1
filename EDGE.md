# Phase1 Edge Development

`edge/v4.1.0-dev` is the active development line after the stable `v4.0.0` release point.

## Current identity

| Item | Value |
| --- | --- |
| Development branch | `edge/v4.1.0-dev` |
| Development package version | `4.1.0-dev` |
| Stable release point | `v4.0.0` |
| Stable branch | `release/v4.0.0` |
| Previous stable | `v3.10.9` |
| Compatibility base | `v3.6.0` |

## Development rules

- Keep the `-dev` suffix until the next stable promotion.
- Keep `v4.0.0` documentation intact for stable users.
- Label experimental work clearly in docs, tests, and command output.
- Keep safe defaults enabled while adding new capabilities.
- Run formatting, compile, Clippy, and tests before merging development work.

## Start development

```bash
git fetch origin
git checkout edge/v4.1.0-dev
cargo metadata --no-deps --format-version 1 | grep '"version"'
cargo run
```

Expected package version:

```text
4.1.0-dev
```

## Validation

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Before a future stable promotion, also run:

```bash
cargo audit
cargo deny check
```

## Future stable promotion path

1. Create a release branch from the development branch when ready.
2. Remove the `-dev` suffix from `Cargo.toml`.
3. Refresh `Cargo.lock`.
4. Update README, website demo output, wiki docs, in-system wiki fixtures, and release metadata tests.
5. Run the full stable validation gate.
6. Create the release tag only after validation passes.
