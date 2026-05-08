# Phase1 Edge Development

`v4.3.0-dev` is the next development line after the stable `v4.2.0` release point.

## Current identity

| Item | Value |
| --- | --- |
| Stable package version | `4.2.0` |
| Stable release point | `v4.2.0` |
| Previous stable | `v4.1.0` |
| Compatibility base | `v3.6.0` |
| Next development package version | `4.3.0-dev` |

## Development rules

- Keep the `-dev` suffix on future development work until the next stable promotion.
- Keep `v4.2.0` documentation intact for stable users.
- Label experimental work clearly in docs, tests, and command output.
- Keep safe defaults enabled while adding new capabilities.
- Run formatting, compile, Clippy, tests, quality, release metadata, and website validation before merging.

## Validation

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
sh scripts/quality-score.sh
sh scripts/test-release-metadata.sh
sh scripts/test-website.sh
```
