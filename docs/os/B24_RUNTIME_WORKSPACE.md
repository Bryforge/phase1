# B24 runtime workspace plan

Status: planning scaffold

Scope: GNU/Linux-backed Phase1 runtime workspace on external USB.

## Purpose

B24 builds on B23 by making the Phase1 GNU/Linux runtime useful after it boots.

B23 target:

`Libreboot -> SeaBIOS -> USB GRUB -> GNU/Linux -> phase1>`

B24 target:

`Libreboot -> SeaBIOS -> USB GRUB -> GNU/Linux -> phase1> + workspace + evidence`

## Workspace model

B24 should expose:

- `/phase1/workspace` for operator work;
- `/phase1/evidence` for runtime evidence;
- `/phase1/state` for runtime state;
- `workspace` command;
- `evidence` command;
- `export-help` command;
- no automatic internal disk mounting;
- no installer behavior.

## Persistence model

The safe default is tmpfs-only runtime workspace. This proves the runtime interface without touching internal storage.

The next persistence step should be an explicit external USB persistence partition, never automatic internal disk use.

## Success states

`phase1_runtime_workspace_seen`

The GNU/Linux runtime exposed `/phase1/workspace`, `/phase1/evidence`, and `/phase1/state`.

`phase1_persistent_workspace_seen`

A later external persistence layer was mounted and visibly available.

## Non-claims

B24 does not make Base1 installer-ready, recovery-complete, hardened, release-candidate ready, or daily-driver ready.

B24 does not write the internal disk and does not claim a completed production operating system boot.
