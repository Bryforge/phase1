# Base1 B3 kernel/initrd handoff

Status: implementation scaffold present
Scope: local QEMU kernel/initrd handoff using existing emulator preview and boot-check scripts

## Purpose

This document defines the next B3 bridge after the UEFI proof-of-life path.

The UEFI proof confirms that a local GRUB/OVMF image can reach a Phase1 readiness marker in QEMU. The kernel/initrd handoff moves the next step closer to a real boot path by staging a supplied kernel and initrd into the existing Base1 emulator preview bundle and then using the guarded QEMU boot checker.

This is still emulator-only evidence. It does not build or download a kernel, install Base1, mutate disks, validate hardware, validate recovery, validate an installer, prove hardening, or prove daily-driver readiness.

## Command

Prepare a handoff bundle from local kernel/initrd inputs:

```bash
sh scripts/base1-b3-kernel-handoff.sh \
  --kernel /path/to/vmlinuz \
  --initrd /path/to/initrd.img \
  --prepare
```

Create the bundle and print the guarded QEMU handoff plan:

```bash
sh scripts/base1-b3-kernel-handoff.sh \
  --kernel /path/to/vmlinuz \
  --initrd /path/to/initrd.img \
  --dry-run
```

Create the bundle and run the guarded QEMU serial-marker check:

```bash
sh scripts/base1-b3-kernel-handoff.sh \
  --kernel /path/to/vmlinuz \
  --initrd /path/to/initrd.img \
  --check
```

The default marker is:

```text
phase1 6.0.0 ready
```

Override the marker when testing a different init path:

```bash
sh scripts/base1-b3-kernel-handoff.sh \
  --kernel /path/to/vmlinuz \
  --initrd /path/to/initrd.img \
  --check \
  --expect "custom marker text"
```

## Output bundle

Default output path:

```text
build/base1-b3-kernel-handoff/
```

Expected contents:

```text
manifest.env
staging/boot/vmlinuz
staging/boot/initrd.img
base1-rootfs-preview.tar
base1-sandbox.raw
run-qemu-bundle.sh
reports/qemu-boot.log          # only after --check
reports/qemu-boot-summary.env  # only after --check
```

The wrapper calls:

```text
scripts/base1-emulator-preview.sh
scripts/base1-qemu-boot-check.sh
```

This keeps the handoff path aligned with the existing emulator preview stack instead of introducing a second QEMU pipeline.

## Evidence model

`--prepare` proves only that the local bundle was staged.

`--dry-run` proves that the bundle has enough structure for the guarded QEMU checker to describe the handoff plan.

`--check` runs QEMU with serial capture and marks the run as pass only when the expected marker appears in the captured log.

A passing `--check` is evidence for a named local emulator run only. It is not a general Base1 bootability claim.

## Required inputs

The command requires local files:

```text
--kernel /path/to/vmlinuz
--initrd /path/to/initrd.img
```

Those files must already be available and safe to run in QEMU. The handoff wrapper does not build, fetch, sign, verify, or trust them by itself.

## Relationship to B3

This handoff path is narrower than full B3 validation.

It supplies these B3 evidence ingredients:

- explicit VM profile;
- explicit VM runtime path through QEMU;
- explicit kernel/initrd artifact staging;
- captured serial log path when `--check` is used;
- expected marker check;
- explicit non-claims.

It still needs a validation report before B3 can be claimed.

## Troubleshooting

### Missing kernel or initrd

The wrapper fails closed if either input is missing:

```text
--kernel is required for B3 kernel handoff
--initrd is required for B3 kernel handoff
kernel not found
initrd not found
```

### Missing timeout on macOS

The guarded QEMU checker requires `timeout` or `gtimeout` for bounded execution. On macOS, install GNU coreutils when needed:

```bash
brew install coreutils
```

### Marker missing

If `--check` fails, inspect:

```text
build/base1-b3-kernel-handoff/reports/qemu-boot.log
```

A missing marker usually means the supplied initrd did not emit the expected text, the kernel command line is not wired for `ttyS0`, or the kernel/initrd pair did not boot far enough.

## Safety boundary

The handoff wrapper must not:

- install Base1;
- write host boot settings;
- partition disks;
- format host disks;
- write EFI variables;
- download kernels or initrds;
- claim hardware validation;
- claim installer readiness;
- claim recovery readiness;
- claim hardening;
- claim daily-driver readiness.

The only intended writes are local files under `build/`.

## Related docs

- [`B3_VM_BOOT_VALIDATION_PLAN.md`](B3_VM_BOOT_VALIDATION_PLAN.md)
- [`B3_VM_BOOT_LOGS.md`](B3_VM_BOOT_LOGS.md)
- [`B3_VM_BOOT_VALIDATION_LIMITATIONS.md`](B3_VM_BOOT_VALIDATION_LIMITATIONS.md)
- [`QEMU_VISUAL_BOOT_PREVIEW.md`](QEMU_VISUAL_BOOT_PREVIEW.md)
- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)

## Non-claims

This kernel/initrd handoff does not make Base1 bootable on physical hardware, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It is a local emulator handoff scaffold that can generate evidence for a future B3 validation report.
