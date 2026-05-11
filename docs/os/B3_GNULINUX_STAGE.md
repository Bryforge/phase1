# Base1 B3 GNU/Linux stage

Status: implementation scaffold present
Scope: local GNU/Linux kernel/initrd staging point for B3 emulator validation

## Purpose

This document defines a GNU/Linux staging point for Base1 B3 validation work.

The stage uses an existing local GNU/Linux kernel/initrd pair as a known boot payload before Base1 attempts to own a complete bootable OS image. This gives Phase1/Base1 a practical bridge:

```text
local GNU/Linux kernel/initrd -> B3 kernel handoff -> guarded QEMU serial check -> future B3 validation report
```

This is a staging scaffold only. It does not make Base1 a GNU/Linux distribution, install Base1, download a distribution, mutate host boot settings, validate hardware, validate recovery, validate an installer, prove hardening, or prove daily-driver readiness.

## Command

Use explicit local kernel/initrd files:

```bash
sh scripts/base1-b3-gnulinux-stage.sh \
  --kernel /path/to/vmlinuz \
  --initrd /path/to/initrd.img \
  --prepare
```

Detect from a local GNU/Linux root tree:

```bash
sh scripts/base1-b3-gnulinux-stage.sh \
  --root /path/to/linux-root \
  --prepare
```

Detect from a local GNU/Linux boot directory:

```bash
sh scripts/base1-b3-gnulinux-stage.sh \
  --boot /path/to/boot \
  --prepare
```

Print the QEMU handoff plan:

```bash
sh scripts/base1-b3-gnulinux-stage.sh \
  --boot /path/to/boot \
  --dry-run
```

Run the guarded serial-marker check:

```bash
sh scripts/base1-b3-gnulinux-stage.sh \
  --boot /path/to/boot \
  --check
```

The default expected marker is:

```text
phase1 6.0.0 ready
```

If the GNU/Linux init path emits a different marker, override it:

```bash
sh scripts/base1-b3-gnulinux-stage.sh \
  --boot /path/to/boot \
  --check \
  --expect "custom marker text"
```

## Detection behavior

When `--root` is provided, the script uses:

```text
<root>/boot
```

When `--boot` is provided, the script searches that directory directly.

Kernel candidates include:

```text
vmlinuz
vmlinuz-*
bzImage
kernel
Image
```

Initrd/initramfs candidates include:

```text
initrd.img
initrd.img-*
initramfs.img
initramfs-*
initrd
initramfs
```

Explicit `--kernel` and `--initrd` paths override detection.

## Output bundle

Default output path:

```text
build/base1-b3-gnulinux-stage/
```

Expected contents are produced by the existing B3 kernel/initrd handoff wrapper:

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

## Relationship to existing B3 tooling

The GNU/Linux stage wrapper calls:

```text
scripts/base1-b3-kernel-handoff.sh
```

The kernel handoff wrapper then calls:

```text
scripts/base1-emulator-preview.sh
scripts/base1-qemu-boot-check.sh
```

This keeps the GNU/Linux stage aligned with the existing B3 handoff and guarded QEMU evidence paths.

## Evidence model

`--prepare` proves only that a local GNU/Linux kernel/initrd pair was staged into the B3 handoff bundle.

`--dry-run` proves that the staged bundle can be handed to the guarded QEMU checker as a plan.

`--check` runs the guarded serial-marker check and produces a log/summary under `build/`.

A passing `--check` is evidence for a named local emulator stage only. It is not a general Base1 bootability claim.

## Safety boundary

The GNU/Linux stage wrapper must not:

- download a GNU/Linux distribution;
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

- [`B3_KERNEL_INITRD_HANDOFF.md`](B3_KERNEL_INITRD_HANDOFF.md)
- [`B3_VM_BOOT_VALIDATION_PLAN.md`](B3_VM_BOOT_VALIDATION_PLAN.md)
- [`B3_VM_BOOT_LOGS.md`](B3_VM_BOOT_LOGS.md)
- [`B3_VM_BOOT_VALIDATION_LIMITATIONS.md`](B3_VM_BOOT_VALIDATION_LIMITATIONS.md)
- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)

## Non-claims

This GNU/Linux stage does not make Base1 a GNU/Linux distribution, bootable on physical hardware, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It is a local emulator staging scaffold for future B3 validation evidence.
