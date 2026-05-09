# Base1 installer and recovery design

The Base1 installer and recovery design defines how Phase1 can move toward a bootable, daily-driver-capable OS track without risking user data or pretending the current console is already a full operating system.

## Goal

Create a boring, visible, recoverable installation path for a Phase1-first Base1 system.

The first checkpoint is documentation-only. Later checkpoints should add dry-run scripts before any disk-changing behavior exists.

## Installer contract

A Base1 installer must provide:

- Non-destructive dry-run mode by default.
- Explicit target-disk selection.
- Hardware preflight check.
- Storage layout preview.
- Recovery path preview.
- Rollback explanation.
- Clear final confirmation before destructive actions.
- No host trust escalation by default.

## Proposed storage layout

```text
/boot/                 bootloader and kernel assets
/base1/                read-only Base1 system layer
/state/phase1/         writable Phase1 state and user data
/recovery/             recovery tools, rollback metadata, and restore notes
```

## Recovery contract

A Base1 recovery path must provide:

- Emergency shell access.
- Disable Phase1 auto-launch.
- Restore previous boot target.
- Read logs without exposing secrets.
- Roll back system updates.
- Export user data from the writable layer.
- Verify image and update metadata.

## First-boot recovery message

Every Base1 boot image should show the operator:

```text
Recovery:
  hold recovery key during boot, or run base1 recovery from Phase1
  safe mode remains default-on
  host tools remain off unless explicitly enabled
```

## Guardrails

- Do not run destructive disk commands in early checkpoints.
- Do not hide the target disk.
- Do not overwrite a user system without explicit confirmation.
- Do not remove recovery shell access.
- Do not claim daily-driver readiness until install, rollback, update, and hardware checks pass.

## Initial implementation slices

1. Add installer and recovery design docs.
2. Add dry-run installer command design.
3. Add recovery command design.
4. Add storage layout checker.
5. Add rollback metadata design.
6. Add first non-destructive installer dry-run script.
