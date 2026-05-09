# Phase1 Repository Channel Model

Phase1 now uses a stability-first repository layout.

## Branches

- `base/v4.2.0` is the frozen stable base system.
- `edge/stable` is the active default development and checkpoint path.
- `feature/*` branches should target `edge/stable`.
- `checkpoint/*` branches record verified milestone snapshots.

## Rules

- Do not rewrite `base/v4.2.0`.
- Keep `edge/stable` tested before merging.
- Use pull requests for all feature and checkpoint work.
- Run `cargo test --workspace --all-targets` before merge.
