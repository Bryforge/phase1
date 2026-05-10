# Base1 QEMU Real Phase1 Binary Validation — 2026-05-10

## Scope

This report records guarded QEMU serial evidence that a Base1 preview boot path reached initramfs `/init`, launched the real `phase1` x86_64 Linux musl binary, and observed Phase1 output on the serial log.

## Evidence

Bundle:

- `build/base1-real-phase1-demo`

Boot inputs:

- `build/base1-real-boot/vmlinuz-virt`
- `build/base1-real-boot/initramfs-real-phase1.img`

Real Phase1 binary:

- `target/x86_64-unknown-linux-musl/release/phase1`
- `ELF 64-bit LSB pie executable, x86-64`
- `static-pie linked`
- copied into initramfs at `/opt/phase1/phase1`

Guarded QEMU checker:

~~~sh
PATH="/opt/homebrew/bin:$PATH" sh scripts/base1-qemu-boot-check.sh \
  --bundle build/base1-real-phase1-demo \
  --execute \
  --confirm launch-qemu-base1-preview \
  --timeout 60 \
  --expect "phase1.workspace"
~~~

Expected serial evidence includes:

- `Linux version`
- `base1 init wrapper reached`
- `base1 launching real Phase1 binary`
- `phase1.log`
- `phase1.workspace`
- `phase1.conf`

## Interpretation

This is stronger than marker-only evidence. The boot path launched the real Phase1 binary from initramfs and captured Phase1-owned output over QEMU serial.

Exit code `124` is expected for the bounded emulator run because `gtimeout` terminates QEMU after the timeout. A PASS means the expected Phase1 serial evidence was observed before timeout.

## Non-claims

This report does not claim:

- Full Base1 OS release readiness.
- Installer readiness.
- Hardware validation.
- Recovery completeness.
- Daily-driver readiness.
- A released Base1 image.
- A production initramfs.
- That Phase1 cleanly exited.
- That Base1 has completed full userspace handoff.

## Promotion rule

Base1 may only move beyond this evidence level after the real Phase1 binary is launched by the normal Base1 boot payload, with serial evidence, provenance, repeatable build instructions, and a non-preview release boundary.
