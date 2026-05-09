# Phase1 Dev Dock

`dev` lets Phase1 work on itself from inside Phase1.

## Inside Phase1 workflow

```text
dev status
dev sync
dev docs
dev quick
dev test
dev checkpoint Docs guard edge stable
dev branch feature/example
dev commit Add example feature
dev push
dev pr Add example feature
dev merge 123
dev close 123
dev doctor
```

## Purpose

- Work on Phase1 from inside Phase1.
- Run docs sync without opening an editor.
- Create checkpoint PRs only when there are actual changes.
- Keep `base/v4.2.0` frozen as the stable base.
- Keep `edge/stable` as the active development path.

## Safety

- Uses guarded host tools.
- Safe mode can stay enabled.
- Avoids staging runtime files like `phase1.history`, `phase1.state`, `phase1.log`, and `phase1.learn`.
