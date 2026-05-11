# Base1 B3 OpenBSD serial marker limitation

Status: documented limitation
Scope: OpenBSD B3 stage serial-marker evidence boundary

## Purpose

This note records the current OpenBSD B3 stage limitation: a bounded OpenBSD QEMU launch can be staged and checked, but the expected OpenBSD serial marker is not yet captured in `reports/openbsd-qemu-boot.log` during marker-check mode.

This means OpenBSD currently has launch-check evidence, not serial-marker boot evidence.

## Observed behavior

The OpenBSD stage supports two check modes:

```text
marker
launch
```

`marker` mode runs QEMU and requires the expected marker, usually `OpenBSD`, to appear in the captured serial log.

`launch` mode records that QEMU accepted the local OpenBSD boot artifact and ran within a bounded timeout.

Current local evidence has shown:

```text
OpenBSD launch-check: pass
OpenBSD serial marker-check: not yet captured
```

## Current interpretation

A launch-check pass is useful B3 staging evidence, but it is weaker than marker-check evidence.

It proves only that:

- the local OpenBSD artifact path was accepted;
- the OpenBSD stage generated a guarded QEMU command;
- QEMU launched within a bounded timeout;
- the run stayed under the local build/report evidence boundary.

It does not prove that OpenBSD reached its installer, kernel, userland, or expected serial marker.

## Likely cause

The likely issue is console routing: OpenBSD boot media may be writing to VGA or bootloader console paths rather than the serial device captured by QEMU with:

```text
-serial file:build/base1-b3-openbsd-stage/reports/openbsd-qemu-boot.log
```

OpenBSD may need explicit bootloader console configuration, different QEMU display/serial wiring, an ISO boot path, or an interactive step before serial output becomes available.

## Next tuning candidates

Future OpenBSD serial-marker work should explore:

- booting an OpenBSD ISO instead of the miniroot raw image;
- using `-display cocoa` or another visible display mode for interactive bootloader inspection;
- routing serial with `-serial stdio` during manual diagnosis;
- trying `-nographic` during a dedicated serial-console experiment;
- checking whether the OpenBSD boot prompt needs `set tty com0` or an equivalent console selection;
- documenting the exact command that first captures `OpenBSD` in the serial log.

## B3 status impact

This limitation satisfies the checklist item:

```text
OpenBSD serial marker evidence is captured or documented as a limitation.
```

It does not satisfy a stronger OpenBSD boot validation claim. B3 still needs a reviewed log bundle and final validation report before the B3 claim can strengthen.

## Non-claims

This limitation note does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It records that OpenBSD launch-check evidence exists while OpenBSD serial-marker evidence remains a known limitation.
