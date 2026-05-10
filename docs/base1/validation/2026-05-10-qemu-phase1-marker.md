# Base1 QEMU Phase1 Marker Validation Report — 2026-05-10

## Result

PASS: Base1 QEMU Phase1 marker boot path validated.

## Scope

This report records emulator-only evidence that:

- QEMU launched a real Linux kernel.
- A Base1-controlled init wrapper executed.
- The serial log emitted `phase1 6.0.0 ready`.
- The original Alpine init remained preserved as `/init.alpine`.

## Evidence

Command:

~~~sh
PATH="/opt/homebrew/bin:$PATH" sh scripts/base1-qemu-boot-check.sh \
  --bundle build/base1-phase1-marker-demo \
  --execute \
  --confirm launch-qemu-base1-preview \
  --timeout 60 \
  --expect "phase1 6.0.0 ready"
~~~

Observed result:

~~~text
qemu-exit-code: 124
result: pass
log: build/base1-phase1-marker-demo/reports/qemu-boot.log
summary: build/base1-phase1-marker-demo/reports/qemu-boot-summary.env
~~~

Marker evidence:

~~~text
Linux version 6.18.22-0-virt
base1 init wrapper reached
phase1 6.0.0 ready
base1 handoff: exec alpine init
~~~

## Interpretation

Exit code `124` is expected for this bounded emulator run because `gtimeout` terminates QEMU after the timeout. The validation result is PASS because the expected serial marker was observed before timeout.

## Non-claims

This report does not claim:

- Full Phase1 OS payload boot.
- Installer readiness.
- Hardware validation.
- Recovery completeness.
- Daily-driver readiness.
- A released Base1 image.
- That the marker initramfs is a production initramfs.

## Promotion rule

Base1 may only move beyond this evidence level after a real Phase1 binary is included in the boot payload and launched by the boot path, with serial evidence captured by the guarded QEMU checker.
