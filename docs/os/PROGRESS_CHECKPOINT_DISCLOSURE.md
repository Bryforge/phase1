# Phase1/Base1 progress checkpoint disclosure

Status: checkpoint disclosure
Scope: current repository state, local evidence state, claim boundaries, and next blockers

## Plain-language state

Phase1/Base1 is not a finished operating system, installer, recovery system, hardened release, hardware-validated system, release candidate, or daily-driver system.

The project has made meaningful boot-readiness progress through B1, B2, and B3 scaffolding:

- B1 has a read-only x86_64 detection script and tests.
- B2 has dry-run assembly planning, scripts, docs, tests, output review, and a local focused-suite pass record.
- B3 has VM boot validation planning, UEFI proof-of-life scaffolding, kernel/initrd handoff scaffolding, GNU/Linux staging, OpenBSD staging, log-capture notes, an OpenBSD serial-marker limitation note, a VM validation scaffold, and an initial VM validation report scaffold.

Current readiness should still be described as evidence-bound boot-readiness scaffolding, not as a boot-ready product.

## Current readiness level

Current level remains below a full B3 claim.

```text
Current level: B2 local dry-run validation evidence present
Target next level: B3 VM boot validated
B3 claim: not_claimed
```

B3 cannot be claimed as complete until the B3 validation report is reviewed against captured logs and the remaining non-claims stay preserved.

## Evidence currently present

| Area | Current state | Evidence path or command | Boundary |
| --- | --- | --- | --- |
| B1 detection | Implemented and tested at script/doc level | `scripts/base1-x86_64-detect.sh` | Read-only detection only. |
| B2 focused suite | Passed locally | `build/base1-b2-test-suite/b2-test-suite-summary.env` | Local B2 focused test evidence only. |
| B3 UEFI proof | Present locally | `build/base1-b3-uefi-proof/reports/b3-summary.env` | UEFI proof-of-life only. |
| B3 kernel/initrd handoff | Present locally | `build/base1-b3-kernel-handoff/reports/qemu-boot-summary.env` | Linux kernel-start marker only. |
| B3 GNU/Linux stage | Present locally | `build/base1-b3-gnulinux-stage/reports/qemu-boot-summary.env` | Hardened-profile request and Linux kernel-start marker only. |
| B3 OpenBSD stage | Present locally | `build/base1-b3-openbsd-stage/reports/openbsd-qemu-summary.env` | Launch-check evidence only. |
| OpenBSD serial marker | Documented limitation | `docs/os/B3_OPENBSD_SERIAL_LIMITATION.md` | Marker output not yet captured. |
| B3 VM validation | Scaffold/report present | `docs/os/B3_VM_VALIDATION_REPORT.md` | Not a reviewed final B3 claim. |

## Local commands that have produced evidence

B2 focused suite:

```bash
sh scripts/base1-b2-test-suite-check.sh --check --write-report
```

B3 kernel/initrd handoff:

```bash
sh scripts/base1-b3-kernel-handoff.sh \
  --kernel build/linux/alpine-netboot/vmlinuz \
  --initrd build/linux/alpine-netboot/initrd.img \
  --check \
  --boot-profile hardened \
  --expect "Linux version" \
  --timeout 45
```

B3 GNU/Linux stage:

```bash
sh scripts/base1-b3-gnulinux-stage.sh \
  --kernel build/linux/alpine-netboot/vmlinuz \
  --initrd build/linux/alpine-netboot/initrd.img \
  --check \
  --timeout 45
```

B3 OpenBSD launch-check stage:

```bash
sh scripts/base1-b3-openbsd-stage.sh \
  --img build/openbsd/7.8/miniroot78.img \
  --check \
  --check-mode launch \
  --timeout 20
```

B3 scaffold refresh:

```bash
sh scripts/base1-b3-vm-validate.sh \
  --dry-run \
  --profile x86_64-vm-validation \
  --write-report
```

## What is proven

The evidence supports these narrow statements:

- the B1 detector exists and is bounded to read-only detection behavior;
- the focused B2 test suite can pass locally;
- the B3 UEFI proof path can produce local proof-of-life evidence;
- the B3 kernel/initrd handoff path can stage a local kernel/initrd pair and detect `Linux version` in serial output;
- the B3 GNU/Linux stage can request a hardened boot profile and detect Linux kernel-start output;
- the B3 OpenBSD stage can launch a local OpenBSD boot artifact in a bounded QEMU check;
- the OpenBSD serial-marker issue is now documented as a limitation;
- non-claims remain explicitly preserved in the B3 report and status tracker.

## What is not proven

The evidence does not prove:

- Base1 boots as a complete operating system;
- Phase1 launches inside a complete Base1 boot path;
- GNU/Linux reaches a complete userspace boot;
- OpenBSD reaches installer or userland;
- hardened mode is actually enforced or verified;
- recovery works;
- an installer works;
- physical hardware works;
- this is a release candidate;
- this is safe as a daily-driver system.

## Known limitations

### GNU/Linux stage limitation

The GNU/Linux evidence currently proves Linux kernel-start output, not a complete userspace boot. The Alpine netboot path may enter initramfs flow and fail to mount boot media. That is outside the current B3 kernel-start evidence boundary.

### OpenBSD stage limitation

The OpenBSD launch-check path can pass, but serial-marker mode has not yet captured the expected `OpenBSD` marker in `reports/openbsd-qemu-boot.log`. The limitation is documented in `B3_OPENBSD_SERIAL_LIMITATION.md`.

### Hardening limitation

The `hardened` boot profile is request-only evidence. It requests hardening-oriented kernel parameters, but the project has not yet proven enforcement, coverage, or policy correctness.

### B3 validation limitation

The B3 validation report is still a scaffold. It must be reviewed against captured logs before B3 can be strengthened.

## Remaining blockers before a stronger B3 claim

- B3 validation report reviewed against captured logs;
- VM profile explicitly recorded in final report;
- VM runtime explicitly recorded in final report;
- boot artifacts explicitly identified in final report;
- boot logs captured, reviewed, and referenced;
- Phase1 launch result recorded;
- non-claims preserved in the final reviewed report.

## Recommended next checkpoint

The next good checkpoint is a reviewed B3 log bundle:

```text
build/base1-b3-vm-validation/
```

It should collect or reference:

- B2 focused-suite summary/log;
- UEFI proof summary/log;
- kernel/initrd handoff summary/log;
- GNU/Linux stage summary/log;
- OpenBSD stage summary/log;
- OpenBSD serial-marker limitation note;
- a reviewed report stating exactly what passed and what did not.

## Non-claims

This checkpoint does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It is a disclosure checkpoint for local B1/B2/B3 progress, evidence, limitations, and remaining blockers.
