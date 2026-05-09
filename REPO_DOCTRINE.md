# Phase1 Repository Doctrine

Phase1 now uses a stability-first repository model.

## Channels

- `base/v4.2.0` is the frozen stable base.
- `edge/stable` is the active default development path.
- `checkpoint/*` branches are verified milestone snapshots.
- `feature/*` branches target `edge/stable`.

## Inside Phase1

```text
repo status
repo base
repo edge
repo checkpoint
```

## Rule

Keep the base boring. Move tested work through edge/stable. Cut checkpoints after validated milestones.

<!-- phase1:auto:repo-model:start -->
## Phase1 repository model

- `base/v4.2.0` is the frozen stable base.
- `edge/stable` is the active default development path.
- `checkpoint/*` branches are verified milestone snapshots.
- `feature/*` branches target `edge/stable`.

Keep the 4.2.0 image and stable base boring. Move tested work through edge/stable.
<!-- phase1:auto:repo-model:end -->
