# Phase1 Edge Development

`v6.0.0` is the active development line after the stable `v5.0.0` release point.

## Current identity

| Item | Value |
| --- | --- |
| Current package version | `6.0.0` |
| Current edge label | `v6.0.0` |
| Stable package version | `4.3.0` |
| Stable release point | `v4.3.0` |
| Previous stable | `v4.4.0` |
| Compatibility base | `v3.6.0` |

## Current checkpoint

The current edge checkpoint includes the post-`v4.3.0` development work that made Phase1 more practical from mobile and trusted-host workflows:

- Guarded host runtime execution can run with safe mode still enabled when the host trust gate is explicit.
- Privileged host mutation remains behind safe mode off plus the trust gate.
- `lang run` has bounded stdin, timeout controls, guarded temporary workspaces, promptless host-tool environment, and audit metadata.
- Mobile and narrow-terminal input uses a simple line editor path to avoid prompt repaint/NUL-padding artifacts.
- The prompt now defaults to compact dynamic status chips across all modes, for example `phase1://root ~ [edge safe trust] ⇢`.
- WASM test fixture directories are collision-safe under parallel test execution.

## Development rules

- Keep the `-dev` suffix on future development work until the next stable promotion.
- Keep `v4.3.0` documentation intact for stable users.
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

<!-- phase1:auto:repo-model:start -->
## Phase1 repository model

- `base/v4.2.0` is the frozen stable base.
- `edge/stable` is the active default development path.
- `checkpoint/*` branches are verified milestone snapshots.
- `feature/*` branches target `edge/stable`.

Keep the 4.3.0 image and stable base boring. Move tested work through edge/stable.
<!-- phase1:auto:repo-model:end -->

