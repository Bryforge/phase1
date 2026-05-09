# Phase1 operating-system track

Phase1 has a long-term operating-system track through Base1.

This does not mean the current Phase1 Rust console is already a kernel, hardened sandbox, or drop-in replacement for Linux, macOS, or Windows. The current project remains a terminal-first virtual OS console. The OS track defines how Phase1 can become the primary user environment on bootable hardware in stages.

## Definition

The Phase1 operating-system track means:

- Base1 provides the minimal trusted host foundation.
- Phase1 becomes the default operator shell and system surface.
- Boot, recovery, update, storage, network, and hardware flows are owned and documented.
- Unsafe host actions remain explicit, guarded, and recoverable.
- Claims are tied to tested boot images, audits, hardware matrices, and rollback paths.

## Non-goals for the current checkpoint

- No claim that the current terminal console replaces a kernel.
- No claim that host-backed execution is hardened against hostile code.
- No removal of recovery access to the underlying host.
- No silent privilege change.
- No unsupported hardware promises.

## Stage 0: Documentation pivot

Status: active.

Goals:

- Use a staged long-term OS track.
- Keep current limitations clear.
- Link the track from README and Base1 documentation.
- Add tests that prevent overclaiming.

## Stage 1: Base1 bootable foundation

Goal: produce a minimal bootable Base1 image that launches Phase1 as the primary shell.

Required pieces:

- Minimal Linux-based base image.
- Read-only base with writable user layer.
- Phase1 autostart path.
- Emergency shell fallback.
- First-boot setup flow.
- Hardware preflight checks.
- Reproducible image build notes.

First design slice: [`Base1 image-builder design`](BASE1_IMAGE_BUILDER.md).

## Stage 2: Installer and recovery

Goal: make installation and rollback boring, visible, and recoverable.

Required pieces:

- Non-destructive installer mode.
- Disk layout plan.
- Recovery partition or recovery USB flow.
- Snapshot and rollback path.
- Signed or verifiable update metadata.
- Clear uninstall / restore path.

## Stage 3: Daily-driver basics

Goal: cover the essentials needed for a primary computing environment.

Required pieces:

- User account flow.
- Wi-Fi and network management.
- Storage manager.
- Package and update manager.
- Browser/document escape hatch.
- Time, locale, keyboard, display, and accessibility settings.
- Low-resolution and mobile-safe UI path.

## Stage 4: Phase1-owned system surface

Goal: make Phase1 the controlling system experience rather than only a terminal app.

Required pieces:

- Phase1 login/session manager.
- Phase1 system settings panel.
- Base1 service supervision from Phase1.
- Snapshot and rollback controls from Phase1.
- Nested Phase1 as workspace and sandbox manager.
- System health dashboard.

## Stage 5: Hardware targets

Goal: ship only against explicit hardware targets.

Initial targets:

- Raspberry Pi image.
- ThinkPad X200-class image.
- Generic x86_64 image.

Each target needs:

- Boot validation.
- Network validation.
- Storage validation.
- Display/input validation.
- Recovery validation.
- Known limitations.

## Guardrails

- Keep safe mode default-on.
- Keep host tool execution guarded.
- Keep destructive/admin actions explicit.
- Keep recovery available.
- Keep security claims conservative until validated.
- Keep Base1 as the boundary between Phase1 and hardware.

## First engineering slices

1. Add this roadmap and README positioning.
2. Add the [`Base1 image-builder design`](BASE1_IMAGE_BUILDER.md).
3. Add the [`Base1 installer and recovery design`](INSTALLER_RECOVERY.md).
4. Add the [`Base1 installer dry-run design`](BASE1_INSTALLER_DRY_RUN.md).
5. Add system-surface command stubs behind safe defaults.
6. Add hardware-target checklists.


## Stage 1 recovery command design

- [`Base1 recovery command design`](BASE1_RECOVERY_COMMAND.md) defines the first read-only recovery command surface for status and planning.


## Stage 2 storage layout checker design

- [`Base1 storage layout checker design`](BASE1_STORAGE_LAYOUT_CHECKER.md) defines the first read-only disk-layout validation surface for Base1 installer planning.


## Stage 2 rollback metadata design

- [`Base1 rollback metadata design`](BASE1_ROLLBACK_METADATA.md) defines the first safe restore-record contract for Base1 installer, update, and recovery planning.


## Stage 2 installer dry-run script

- `scripts/base1-install-dry-run.sh` is the first non-destructive installer preview command. It requires `--dry-run`, requires an explicit target, and reports `writes: no`.


## Stage 2 recovery dry-run script

- `scripts/base1-recovery-dry-run.sh` is the first non-destructive recovery preview command. It requires `--dry-run`, keeps boot settings unchanged, and reports `writes: no`.


## Stage 2 storage layout dry-run script

- `scripts/base1-storage-layout-dry-run.sh` is the first non-destructive storage layout preview command. It requires `--dry-run`, requires an explicit target, and reports `writes: no`.


## Stage 2 rollback metadata dry-run script

- `scripts/base1-rollback-metadata-dry-run.sh` is the first non-destructive rollback metadata preview command. It requires `--dry-run`, stores no secrets, writes nothing, and reports `operator_confirmed: no`.


## Stage 2 dry-run command index

- [`Base1 dry-run command index`](BASE1_DRY_RUN_COMMANDS.md) lists the current non-destructive Base1 preview commands and their shared guardrails.
