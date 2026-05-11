# Base1 B1 read-only detection limitations

Status: active B1 limitations note
Scope: known limitations for `scripts/base1-x86_64-detect.sh --dry-run`

## Purpose

This note documents what the first B1 x86_64 read-only detector can and cannot prove.

The detector is intentionally limited. It gathers hints only and writes nothing.

## Current command

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

## What B1 can report

The initial detector can report read-only hints for:

- machine architecture;
- UEFI directory presence;
- EFI variable directory presence;
- limited GRUB/systemd-boot directory hints;
- redacted kernel command-line availability;
- virtualization/container hints when visible;
- storage layout hints through `lsblk` or `/proc/mounts`;
- basic recovery/emergency-mode hints;
- unknown fields and next planned read-only check;
- `writes: no`.

## What B1 cannot prove

The initial detector cannot prove:

- that Base1 is bootable;
- that the current machine can safely install Base1;
- that a boot loader is configured correctly;
- that a kernel command line is complete or correct;
- that secure boot is supported or enabled correctly;
- that measured boot, TPM, or lockdown modes are available or safe;
- that recovery media exists or is usable;
- that rollback is complete;
- that hardware is validated;
- that the system is hardened;
- that the system is daily-driver ready.

## Known limitations

| Area | Limitation |
| --- | --- |
| Architecture | Uses common read-only commands such as `uname -m`; unknown output remains unknown. |
| Firmware | `/sys/firmware/efi` is a useful UEFI hint on Linux but not a complete firmware proof. |
| Boot loader | Directory hints do not prove active boot-loader configuration. |
| Kernel command line | `/proc/cmdline` may not exist or may not represent target media. Sensitive-looking values are redacted conservatively. |
| Storage | `lsblk` or `/proc/mounts` output is a hint, not an install layout decision. |
| Virtualization | VM/container detection is best-effort and may miss some environments. |
| Recovery | Recovery directory or cmdline hints do not prove recovery usability. |
| Cross-platform hosts | Non-Linux hosts may produce more unknown values. |

## Required behavior

The detector must continue to:

- require `--dry-run`;
- report `writes: no`;
- avoid network access;
- avoid boot-loader mutation;
- avoid partition mutation;
- avoid package installation;
- avoid writing to `/boot`, `/etc`, EFI variables, initramfs files, or partitions;
- keep unknown states visible.

## Before B1 can be considered complete

B1 completion still requires:

- script test suite passing in CI or local validation;
- status tracker links updated after implementation;
- secret-redaction review;
- non-mutation source review;
- limitations documented in this file;
- README or roadmap visibility for the B1 boundary.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)

## Non-claims

This limitations note does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It documents the current B1 detector boundary so implementation can continue safely.
