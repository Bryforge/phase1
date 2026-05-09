# Base1 image-builder design

The Base1 image builder is the first practical engineering slice of the Phase1 operating-system track.

Its job is to produce a minimal, bootable, recoverable Base1 image that launches Phase1 as the primary operator shell without pretending Phase1 is already a kernel or hardened standalone OS.

## Goal

Build a repeatable image pipeline for:

- Raspberry Pi-class targets.
- ThinkPad X200-class targets.
- Generic x86_64 test targets.

The first checkpoint is documentation-only. The next checkpoints should add scripts behind dry-run and preflight guards.

## Stage 1 boot image contract

A Base1 boot image must provide:

- Minimal Linux-based host foundation.
- Read-only base layer.
- Writable user/data layer.
- Phase1 auto-launch as the primary shell.
- Emergency shell fallback.
- Recovery boot path.
- Hardware preflight report.
- Clear version metadata.
- No silent host trust escalation.

## Proposed image layout

```text
/boot/                 bootloader and kernel assets
/base1/                read-only Base1 system layer
/phase1/               Phase1 application checkout or packaged build
/state/phase1/         writable Phase1 state, history, logs, learning data
/recovery/             fallback tools and recovery metadata
```

## First boot flow

1. Boot Base1.
2. Mount read-only base.
3. Mount writable state layer.
4. Run hardware preflight.
5. Start Phase1 in safe mode.
6. Show recovery instructions in the boot panel.
7. Write non-sensitive boot metadata to the audit log.

## Recovery requirements

Every image must include:

- Emergency shell access.
- Rollback or restore path.
- Logs that do not expose secrets.
- Clear instructions for leaving Phase1 and recovering the host.
- A way to disable Phase1 auto-launch.

## Guardrails

- Do not claim the image is a secure OS replacement until recovery, update, hardware, and audit validation exist.
- Do not remove safe mode.
- Do not enable host tools by default.
- Do not perform destructive disk actions without explicit confirmation.
- Do not ship an image target without a hardware checklist.

## Initial implementation slices

1. Add image-builder design docs.
2. Add dry-run script outline.
3. Add preflight integration.
4. Add hardware checklist docs.
5. Add recovery design docs.
6. Add first experimental image build script.
