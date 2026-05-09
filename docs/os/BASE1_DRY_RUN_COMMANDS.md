# Base1 dry-run command index

This index collects the current non-destructive Base1 OS-track preview commands.

These commands are operator-facing safety surfaces. They are designed to preview installer, recovery, storage, and rollback behavior before any destructive Base1 action exists.

## Current dry-run commands

```text
sh scripts/base1-install-dry-run.sh --dry-run --target /dev/example
sh scripts/base1-recovery-dry-run.sh --dry-run
sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example
sh scripts/base1-rollback-metadata-dry-run.sh --dry-run
```

## Shared contract

Every Base1 dry-run command must:

- Require `--dry-run`.
- Report `writes: no`.
- Avoid destructive disk tools.
- Avoid host trust escalation.
- Avoid secret or credential storage.
- Keep recovery access visible.
- Make target identity visible when a target is required.

## Command purposes

| Command | Purpose |
| --- | --- |
| `base1-install-dry-run.sh` | Previews installer layout, boot plan, recovery, rollback, and Phase1 state paths. |
| `base1-recovery-dry-run.sh` | Previews emergency recovery status without changing boot settings. |
| `base1-storage-layout-dry-run.sh` | Previews `/boot`, `/base1`, `/state/phase1`, `/recovery`, and rollback layout. |
| `base1-rollback-metadata-dry-run.sh` | Previews rollback metadata without writing restore records or secrets. |

## Promotion rule

No dry-run command may become mutating until the matching design doc, script test, recovery path, rollback plan, hardware target checks, and final destructive-action confirmation are all present.
