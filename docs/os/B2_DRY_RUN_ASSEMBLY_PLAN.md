# Base1 B2 dry-run assembly plan

Status: implementation planning
Scope: B2 boot-readiness dry-run assembly path for Base1 image, boot profile, installer preview, recovery preview, rollback preview, and validation bundle

## Purpose

This plan defines the next boot-readiness target after B1 read-only detection: B2 dry-run assembly.

B2 does not install Base1, write boot entries, partition disks, modify firmware, generate a release image, or claim boot readiness. It previews the assembly path and records what would be required for a future bootable Base1 target.

## B2 goal

B2 means Base1 can preview a complete boot-readiness assembly flow without writing host or target state.

A B2 preview should connect:

- B1 detection facts;
- boot profile selection;
- image-builder preview;
- kernel/initramfs handoff preview;
- Phase1 autostart preview;
- emergency shell fallback preview;
- installer dry-run preview;
- recovery dry-run preview;
- rollback metadata dry-run preview;
- validation bundle summary;
- known limitations.

## Proposed command

Planned B2 command:

```bash
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
```

The command must require `--dry-run` and a named profile.

## Required output

The B2 dry-run assembly command should report:

- `writes: no`;
- selected boot-readiness level: `B2`;
- selected profile;
- architecture and firmware facts from B1 or explicit unknowns;
- boot profile assumptions;
- image-builder preview status;
- kernel/initramfs handoff preview;
- Phase1 autostart preview;
- emergency shell fallback preview;
- installer preview status;
- recovery preview status;
- rollback metadata preview status;
- validation bundle path or planned path;
- known limitations;
- next recommended validation step.

## Input profiles

Initial B2 profile names should align with the x86_64 roadmap:

```text
x86_64-uefi-generic
x86_64-bios-generic
x86_64-libreboot-grub
x86_64-vm-validation
x86_64-recovery-usb
```

The first implementation should prefer `x86_64-vm-validation` because VM validation is safer before physical hardware claims.

## B2 dry-run assembly stages

| Stage | Preview only? | Required output |
| --- | --- | --- |
| Detect | Yes | B1 detector summary or unknowns. |
| Profile | Yes | Selected profile and assumptions. |
| Image | Yes | Image-builder inputs and missing requirements. |
| Boot handoff | Yes | Kernel/initramfs/Phase1 autostart preview. |
| Recovery | Yes | Emergency shell and recovery path preview. |
| Rollback | Yes | Rollback metadata preview. |
| Validation bundle | Yes | Planned logs/reports and next checks. |

## Non-mutation rules

The B2 command must not:

- write images;
- write partitions;
- format disks;
- mount filesystems read-write;
- edit `/boot`, `/etc`, EFI variables, initramfs files, or partitions;
- call mutating boot-loader commands;
- install packages;
- require network access;
- mark hardware as validated;
- claim a bootable release candidate.

## Required integration points

B2 should integrate or reference these existing pieces:

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md)
- [`B1_READ_ONLY_DETECTION_LIMITATIONS.md`](B1_READ_ONLY_DETECTION_LIMITATIONS.md)
- [`B1_READ_ONLY_DETECTION_VALIDATION.md`](B1_READ_ONLY_DETECTION_VALIDATION.md)
- [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)
- [`BASE1_IMAGE_BUILDER.md`](BASE1_IMAGE_BUILDER.md)
- [`INSTALLER_RECOVERY.md`](INSTALLER_RECOVERY.md)
- [`BASE1_DRY_RUN_COMMANDS.md`](BASE1_DRY_RUN_COMMANDS.md)
- [`BASE1_ROLLBACK_METADATA.md`](BASE1_ROLLBACK_METADATA.md)

## Test expectations

B2 tests should verify:

- B2 plan exists;
- B2 command is documented;
- B2 requires `--dry-run`;
- B2 requires a named profile;
- B2 reports `writes: no`;
- B2 reports profile, image, boot handoff, recovery, rollback, and validation bundle sections;
- B2 source avoids mutating boot, disk, package, and network commands;
- B2 does not claim boot readiness, hardware validation, hardening, installer readiness, or release-candidate readiness.

## Completion checklist

B2 can be considered complete only after:

- [ ] B2 dry-run assembly script exists;
- [ ] B2 script tests exist;
- [ ] B2 plan tests exist;
- [ ] B2 status tracker is updated;
- [ ] B2 output is reviewed for secret redaction;
- [ ] B2 source is reviewed for non-mutation;
- [ ] B2 known limitations are documented;
- [ ] B2 validation report exists;
- [ ] README and OS roadmap reflect the B2 boundary.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)

## Non-claims

This B2 plan does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It defines the next dry-run assembly target after B1 read-only detection.
