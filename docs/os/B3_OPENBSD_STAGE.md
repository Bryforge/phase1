# Base1 B3 OpenBSD stage

Status: implementation scaffold present
Scope: local OpenBSD ISO/image staging point for B3 emulator validation

## Purpose

This document defines an OpenBSD staging point for Base1 B3 validation work.

OpenBSD is intentionally separate from the GNU/Linux kernel/initrd handoff path. OpenBSD uses its own bootloader, kernel, ramdisk, and installer media shape, so the B3 OpenBSD stage operates on a local OpenBSD ISO or raw boot image instead of trying to reuse the Linux `vmlinuz`/`initrd.img` flow.

This stage gives Phase1/Base1 a practical bridge:

```text
local OpenBSD ISO/image -> guarded QEMU serial check -> future B3 validation report
```

This is a staging scaffold only. It does not make Base1 an OpenBSD distribution, install Base1, download OpenBSD, mutate host boot settings, validate hardware, validate recovery, validate an installer, prove hardening, or prove daily-driver readiness.

## Command

Use a local OpenBSD ISO:

```bash
sh scripts/base1-b3-openbsd-stage.sh \
  --iso /path/to/install.iso \
  --prepare
```

Use a local OpenBSD raw boot image:

```bash
sh scripts/base1-b3-openbsd-stage.sh \
  --img /path/to/install.img \
  --prepare
```

Print the guarded QEMU plan:

```bash
sh scripts/base1-b3-openbsd-stage.sh \
  --iso /path/to/install.iso \
  --dry-run
```

Run the guarded serial-marker check:

```bash
sh scripts/base1-b3-openbsd-stage.sh \
  --iso /path/to/install.iso \
  --check
```

The default expected marker is:

```text
OpenBSD
```

Override it when validating a more specific boot path:

```bash
sh scripts/base1-b3-openbsd-stage.sh \
  --iso /path/to/install.iso \
  --check \
  --expect "OpenBSD/amd64"
```

## Output bundle

Default output path:

```text
build/base1-b3-openbsd-stage/
```

Expected contents:

```text
openbsd-stage.env
reports/openbsd-qemu-boot.log       # only after --check
reports/openbsd-qemu-summary.env    # only after --check
```

## Evidence model

`--prepare` proves only that a local OpenBSD artifact path was accepted and stage metadata was written.

`--dry-run` proves that the staged artifact can be converted into a guarded QEMU command plan.

`--check` runs QEMU with serial capture and marks the run as pass only when the expected marker appears in the captured log.

A passing `--check` is evidence for a named local emulator stage only. It is not a general Base1 bootability claim.

## Safety boundary

The OpenBSD stage wrapper must not:

- download OpenBSD;
- install Base1;
- change host boot settings;
- partition disks;
- format host disks;
- write EFI variables;
- claim physical hardware validation;
- claim installer readiness;
- claim recovery readiness;
- claim hardening;
- claim daily-driver readiness.

The only intended writes are local files under `build/`.

## Related docs

- [`B3_VM_BOOT_VALIDATION_PLAN.md`](B3_VM_BOOT_VALIDATION_PLAN.md)
- [`B3_GNULINUX_STAGE.md`](B3_GNULINUX_STAGE.md)
- [`B3_KERNEL_INITRD_HANDOFF.md`](B3_KERNEL_INITRD_HANDOFF.md)
- [`B3_VM_BOOT_LOGS.md`](B3_VM_BOOT_LOGS.md)
- [`B3_VM_BOOT_VALIDATION_LIMITATIONS.md`](B3_VM_BOOT_VALIDATION_LIMITATIONS.md)
- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)

## Non-claims

This OpenBSD stage does not make Base1 an OpenBSD distribution, bootable on physical hardware, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It is a local emulator staging scaffold for future B3 validation evidence.
