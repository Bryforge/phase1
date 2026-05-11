# Base1 B3 VM validation report

Status: initial local-evidence report scaffold
Scope: B3 emulator evidence collected from local proof/stage runs

## Summary

This report records local B3 emulator evidence for Phase1/Base1 without claiming full boot readiness.

Current evidence state:

```text
BASE1_B3_EVIDENCE_STATE=evidence-present
BASE1_B3_VALIDATION_CLAIM=not_claimed
```

The current evidence demonstrates that the B3 emulator scaffolds can produce local proof/stage outputs. It does not prove Base1 is fully bootable, installer-ready, hardened, hardware-validated, recovery-complete, release-candidate ready, or daily-driver ready.

## Evidence recorded

| Evidence item | Current status | Evidence path | Claim boundary |
| --- | --- | --- | --- |
| B3 UEFI proof | Present locally | `build/base1-b3-uefi-proof/reports/b3-summary.env` | Emulator-only UEFI proof-of-life. |
| B3 GNU/Linux stage | Present locally | `build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env` | Hardened-profile request and Linux kernel-start evidence only. |
| B3 OpenBSD stage | Present locally | `build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env` | Launch-check evidence only until serial marker routing is tuned. |
| B3 kernel/initrd handoff | Not yet present locally | `build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env` | Requires a known-good local kernel/initrd check. |

## Commands represented by this report

UEFI proof evidence:

```bash
sh scripts/base1-b3-uefi-proof.sh --build --check
```

GNU/Linux hardened-profile stage evidence:

```bash
sh scripts/base1-b3-gnulinux-stage.sh \
  --kernel build/linux/alpine-netboot/vmlinuz \
  --initrd build/linux/alpine-netboot/initrd.img \
  --check \
  --timeout 45
```

OpenBSD launch-check stage evidence:

```bash
sh scripts/base1-b3-openbsd-stage.sh \
  --img build/openbsd/7.8/miniroot78.img \
  --check \
  --check-mode launch \
  --timeout 20
```

B3 validation scaffold refresh:

```bash
sh scripts/base1-b3-vm-validate.sh \
  --dry-run \
  --profile x86_64-vm-validation \
  --write-report
```

## GNU/Linux stage interpretation

The GNU/Linux stage currently checks for this marker:

```text
Linux version
```

That marker proves that the Linux kernel started and emitted early serial boot output under the requested hardened profile.

It does not prove a complete GNU/Linux userspace boot. The observed Alpine netboot path may continue into initramfs logic and fail to mount boot media, which is outside the current kernel-start evidence boundary.

The hardened profile is request-only. It asks QEMU/Linux to use hardening-oriented kernel parameters, but the report does not claim hardening is proven.

## OpenBSD stage interpretation

The OpenBSD stage currently has two modes:

```text
marker
launch
```

`marker` mode requires the expected serial marker, usually `OpenBSD`, to appear in the captured serial log.

`launch` mode records bounded QEMU launch evidence when OpenBSD console output is not yet routed to serial. A launch-check pass does not prove OpenBSD booted to installer or userland. It proves the OpenBSD artifact was accepted by the bounded QEMU stage and the run stayed inside the local evidence boundary.

## Required before strengthening B3 claims

Before B3 can become “VM boot validated,” the project still needs:

- B2 focused test suite pass record;
- known-good local kernel/initrd handoff check;
- OpenBSD serial marker routing or a documented limitation;
- reviewed B3 log bundle;
- explicit VM profile;
- explicit VM runtime;
- explicit boot artifact identifiers;
- Phase1 launch result;
- non-claims preserved in the final report.

## Non-claims

This report does not make Base1 bootable on physical hardware, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It records local emulator evidence and preserves the current B3 boundary: proof-of-life, GNU/Linux kernel-start stage evidence, OpenBSD launch-check stage evidence, and validation scaffolding only.
