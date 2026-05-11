# B25 supervisor orchestration plan

Status: planning scaffold

Scope: GNU/Linux-backed Phase1 runtime orchestration for concurrent system lanes.

## Purpose

B25 brings the simultaneous multiboot idea back in a safer place: inside the GNU/Linux-backed Phase1 runtime, not inside firmware or GRUB.

The physical route stays simple:

`Libreboot -> SeaBIOS -> USB GRUB -> GNU/Linux -> Phase1 supervisor`

The supervisor then coordinates plans for multiple lanes.

## Lane model

Initial lanes:

- `linux-runtime`: primary Phase1 runtime lane;
- `workspace`: Phase1 workspace and evidence lane;
- `openbsd-plan`: future OpenBSD integration lane;
- `crypto-plan`: future identity, signing, and evidence lane.

The X200 concurrent lab profile limits concurrency to two lanes because the machine is low-resource.

## Success state

`phase1_supervisor_plan_seen`

This means the GNU/Linux-backed Phase1 runtime exposed the supervisor plan, lane list, and evidence path.

## Non-claims

B25 does not automatically boot multiple operating systems.

B25 does not make Base1 hypervisor-ready, installer-ready, recovery-complete, hardened, release-candidate ready, or daily-driver ready.

B25 does not write the internal disk.
