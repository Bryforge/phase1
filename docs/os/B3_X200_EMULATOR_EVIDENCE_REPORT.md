# B3 X200 emulator evidence report

Status: local X200 emulator evidence report
Generated UTC: 2026-05-13T23:30:41Z
Source branch: edge/stable
Source commit: ab77a4e824acef9c0b3d640b899d9be0d5424fa2
Host: X200
Host kernel: Linux X200 6.8.0-110-generic #110trisquel35 SMP PREEMPT_DYNAMIC Wed Apr 15 21:32:36 UTC 2026 x86_64 x86_64 x86_64 GNU/Linux

## Summary

This report records local X200 B3 emulator evidence generated from repository build outputs.

The report is intentionally conservative. It records emulator evidence only and keeps the B3 validation claim as not_claimed until reviewed release-facing validation is complete.

| Evidence item | Present | Result | Exit code | Marker / expectation |
| --- | --- | --- | --- | --- |
| B3 VM scaffold | yes | evidence-present | n/a | claim: not_claimed |
| B3 UEFI proof | yes | pass | 124 | phase1 6.0.0 ready |
| B3 kernel/initrd handoff | yes | pass | 124 | Linux version |
| B3 GNU/Linux stage | yes | pass | 124 | Linux version |
| B3 OpenBSD stage | yes | optional/not used | n/a | optional stage |

Evidence summary count: 4

## Local evidence paths

- VM validation scaffold: build/base1-b3-vm-validation/b3-validation-scaffold.env
- UEFI proof summary: build/base1-b3-uefi-proof/reports/b3-summary.env
- UEFI proof log: build/base1-b3-uefi-proof/reports/b3-serial.log
- Kernel/initrd handoff summary: build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env
- Kernel/initrd handoff log: build/base1-b3-kernel-handoff/reports/qemu-boot.log
- GNU/Linux stage summary: build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env
- GNU/Linux stage log: build/base1-b3-gnulinux-stage/reports/qemu-boot.log

Raw build logs remain under build/ and are not committed by this report.

## Interpretation boundary

The evidence above supports this limited statement:

- B3 emulator evidence for GNU/Linux local kernel/initrd staging, UEFI proof, and kernel/initrd handoff is present on the X200 test host.

It does not support these claims:

- Base1 is installed.
- Base1 is hardware-validated.
- Base1 recovery is validated.
- Base1 hardening is proven.
- Base1 is release-candidate ready.
- Base1 is daily-driver ready.
- Phase1/Base1 is a production operating system.
