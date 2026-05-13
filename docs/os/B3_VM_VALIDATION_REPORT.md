# Base1 B3 VM validation report

Status: reviewed local VM evidence report
Scope: B3 emulator evidence collected from local proof/stage runs, B2 focused test evidence, B3 log-bundle review, X200 emulator evidence, and latest Phase1 big verification report

## Summary

This report records reviewed local B3 emulator evidence for Phase1/Base1 without claiming full boot readiness.

Current evidence state:

```text
BASE1_B3_EVIDENCE_STATE=evidence-present
BASE1_B3_EVIDENCE_SUMMARY_COUNT=4
BASE1_B3_VALIDATION_CLAIM=not_claimed
BASE1_B3_REVIEWED_STATE=reviewed_vm_evidence
```

The current evidence demonstrates that the B3 emulator scaffolds can produce local proof/stage outputs and that the local evidence bundle is internally reviewable. It does not prove Base1 is fully bootable, installer-ready, hardened, hardware-validated, recovery-complete, release-candidate ready, or daily-driver ready.

## Evidence recorded

| Evidence item | Current status | Evidence path | Claim boundary |
| --- | --- | --- | --- |
| B2 focused test suite | Pass | `build/base1-b2-test-suite/b2-test-suite-summary.env` | Focused B2 test evidence only; no bootability, VM, installer, or hardware claim. |
| B3 UEFI proof | Present locally / pass | `build/base1-b3-uefi-proof/reports/b3-summary.env` | Emulator-only UEFI proof-of-life. |
| B3 kernel/initrd handoff | Present locally / pass | `build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env` | Known-good local kernel/initrd handoff evidence; Linux kernel-start marker only. |
| B3 GNU/Linux stage | Present locally / pass | `build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env` | Hardened-profile request and Linux kernel-start evidence only. |
| B3 OpenBSD stage | Present locally / pass | `build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env` | Launch-check evidence only until serial marker routing is tuned. |
| B3 OpenBSD serial marker limitation | Documented | [`B3_OPENBSD_SERIAL_LIMITATION.md`](B3_OPENBSD_SERIAL_LIMITATION.md) | Marker-check is a known limitation until serial routing is tuned. |
| B3 log-bundle review | Pass | `build/base1-b3-vm-validation/b3-log-bundle-review.env` | Local evidence review only; no boot-ready or hardware claim. |
| X200 emulator evidence report | Present | [`B3_X200_EMULATOR_EVIDENCE_REPORT.md`](B3_X200_EMULATOR_EVIDENCE_REPORT.md) | X200-hosted emulator evidence only. |
| Reviewed VM evidence summary | Present | [`B3_REVIEWED_VM_EVIDENCE.md`](B3_REVIEWED_VM_EVIDENCE.md) | Reviewed local VM evidence; claim remains `not_claimed`. |
| Latest big verification | Pass, 15 steps, 0 failed | `docs/reports/PHASE1_BIG_VERIFY_LATEST.md` | Repository and local emulator-evidence validation only. |

## Commands represented by this report

UEFI proof evidence:

```bash
sh scripts/base1-b3-uefi-proof.sh --build --check
```

Kernel/initrd handoff evidence:

```bash
sh scripts/base1-b3-kernel-handoff.sh \
  --kernel build/linux/alpine-netboot/vmlinuz \
  --initrd build/linux/alpine-netboot/initrd.img \
  --check \
  --boot-profile hardened \
  --expect "Linux version" \
  --timeout 45
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

B3 log-bundle review:

```bash
sh scripts/base1-b3-log-bundle-review.sh --review
```

Big verification:

```bash
sh scripts/phase1-big-verify.sh --fix --full --base1 --b2 --b3 --status --wiki
```

## Kernel/initrd handoff interpretation

The generic B3 kernel/initrd handoff has staged a local kernel/initrd pair and checked for this serial marker:

```text
Linux version
```

This proves the handoff pipeline can stage a caller-provided local kernel/initrd pair and observe Linux kernel-start evidence under QEMU.

It does not prove a complete userspace boot, a Phase1 launch, installer readiness, hardware support, or hardening.

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

The OpenBSD serial marker limitation is documented in [`B3_OPENBSD_SERIAL_LIMITATION.md`](B3_OPENBSD_SERIAL_LIMITATION.md).

## Reviewed B3 result

The B3 evidence bundle is now recorded as reviewed local VM evidence when:

- B2 focused test suite passes;
- UEFI proof summary/log are present;
- kernel/initrd handoff summary/log are present;
- GNU/Linux stage summary/log are present;
- OpenBSD stage summary/log are present;
- OpenBSD serial limitation is documented;
- B3 log-bundle review passes;
- latest big verification passes with 15 steps and 0 failed;
- non-claims remain visible.

Reviewed state:

```text
reviewed_vm_evidence
```

Claim state:

```text
not_claimed
```

## B4 handoff

B4 recovery validation has been prepared as the next evidence stage:

```bash
sh scripts/base1-b4-recovery-validate.sh --prepare
```

Current B4 prepared state:

```text
BASE1_B4_RECOVERY_MODE=prepare
BASE1_B4_RECOVERY_BOOT_ARTIFACT=planned
BASE1_B4_RECOVERY_ARTIFACT=planned
BASE1_B4_RECOVERY_ROLLBACK_PATH=planned
BASE1_B4_RECOVERY_EMERGENCY_STOP=planned
BASE1_B4_RECOVERY_CLAIM=not_claimed
```

B4 does not yet claim recovery completion.

## Required before strengthening beyond reviewed VM evidence

Before B3 can become a stronger VM boot validated or boot-ready claim, the project still needs:

- explicit VM runtime/version identifiers;
- explicit boot artifact identifiers and provenance;
- Phase1 launch result beyond kernel-start or launch-check evidence;
- repeatable reviewed validation report with stable artifacts;
- recovery validation evidence;
- non-claims preserved in any strengthened report.

## Non-claims

This report does not make Base1 bootable on physical hardware, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It records local emulator evidence and preserves the current B3 boundary: proof-of-life, kernel/initrd handoff evidence, GNU/Linux kernel-start stage evidence, OpenBSD launch-check stage evidence, documented OpenBSD serial-marker limitation, reviewed log-bundle evidence, X200 emulator evidence, and validation scaffolding only.
