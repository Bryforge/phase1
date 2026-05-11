# Base1 B1 read-only detection plan

Status: implementation planning
Scope: first B1 boot-readiness implementation slice for x86_64 read-only detection

## Purpose

This plan defines the first coding slice after the finish-first documentation gate: a read-only x86_64 detection script for Base1 boot readiness.

The script must inspect host or target facts without modifying boot configuration, partitions, filesystems, firmware settings, or kernel command lines.

## Command

Planned command:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

The command must require `--dry-run` for the initial implementation.

## Required output

The B1 detector should report:

- `writes: no`;
- architecture hints;
- firmware hints;
- boot-loader hints;
- virtualization hints;
- storage-layout hints;
- recovery availability hints;
- unknown or unsupported state warnings;
- next recommended read-only check.

## Detection categories

| Category | Examples | Required behavior |
| --- | --- | --- |
| Architecture | `uname -m`, `arch` | Detect `x86_64` where available; report unknown otherwise. |
| Firmware | `/sys/firmware/efi`, boot environment hints | Report UEFI, BIOS/legacy, Libreboot/GRUB hints, or unknown. |
| Boot loader | GRUB/systemd-boot/EFI hints | Report hints only; do not modify boot entries. |
| Kernel command line | `/proc/cmdline` | Read only; redact sensitive tokens if any appear. |
| Storage layout | `lsblk`, `/proc/mounts` hints | Report layout summary only; do not write. |
| Virtualization | common VM/container hints | Report VM/container hints without making hardware claims. |
| Recovery | recovery media or emergency shell hints | Report visible recovery evidence or unknown. |

## Fail-closed behavior

The detector should fail closed when:

- `--dry-run` is missing;
- architecture cannot be determined;
- required read-only tools are unavailable and no fallback exists;
- output would require privileged mutation;
- a command would write to disk, firmware, boot loader configuration, or partitions.

Fail-closed output should explain what was unknown and which read-only check can be run next.

## Redaction rules

The detector must not print secrets.

Redact or avoid:

- tokens;
- private keys;
- recovery codes;
- credentials;
- sensitive local paths when possible;
- private kernel command-line secrets if detected.

## Non-mutation rules

The detector must not:

- call `mount` in a mutating mode;
- call `grub-install`;
- call `efibootmgr` with write flags;
- write boot entries;
- edit `/boot`, `/etc`, EFI variables, partitions, or initramfs files;
- install packages;
- require network access.

## Initial implementation shape

A safe first script can:

1. parse arguments;
2. require `--dry-run`;
3. print a stable heading;
4. collect read-only hints using available commands;
5. print unknown when a fact cannot be determined;
6. print `writes: no`;
7. exit successfully when read-only inspection completed;
8. exit non-zero only for missing `--dry-run` or unrecoverable invocation errors.

## Test expectations

Initial tests should verify:

- script exists;
- script syntax passes `sh -n`;
- script requires `--dry-run`;
- script prints `writes: no` with `--dry-run`;
- script does not contain known mutating boot commands;
- docs link this plan;
- non-claims are preserved.

## Status tracker update rule

After this plan is linked and tested, update [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md):

- mark `B1 implementation issue/plan for read-only x86_64 detection` as complete;
- keep `B1 detection script` as not started until the script exists;
- keep `B1 detection tests` as not started until script tests exist.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)
- [`ROADMAP.md`](ROADMAP.md)

## Non-claims

This plan does not implement the detector by itself.

It does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It defines the first safe coding slice for B1 read-only x86_64 detection.
