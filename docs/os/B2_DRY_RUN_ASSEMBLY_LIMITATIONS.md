# Base1 B2 dry-run assembly limitations

Status: active B2 limitations note
Scope: known limitations for `scripts/base1-b2-assembly-dry-run.sh --dry-run --profile <profile>`

## Purpose

This note documents what the first B2 dry-run assembly preview can and cannot prove.

The B2 preview is intentionally limited. It connects boot-readiness planning pieces into a no-write assembly preview, but it does not build, install, boot, validate, or harden Base1.

## Current command

```bash
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
```

## What B2 can report

The initial B2 dry-run assembly preview can report:

- `writes: no`;
- selected boot-readiness level: `B2`;
- selected profile;
- B1 detector command reference;
- basic machine and firmware hints;
- profile assumptions;
- image-builder preview status;
- boot handoff preview status;
- installer preview status;
- recovery preview status;
- rollback preview status;
- validation bundle planned path;
- known limitations;
- next validation step.

## What B2 cannot prove

The initial B2 dry-run assembly preview cannot prove:

- that a Base1 image can be built;
- that a Base1 image can boot;
- that a boot loader is configured correctly;
- that the selected profile is valid for real hardware;
- that kernel or initramfs handoff will work;
- that Phase1 autostart will work after boot;
- that emergency shell fallback is usable;
- that installer flow is safe beyond dry-run planning;
- that recovery media exists or works;
- that rollback is complete;
- that VM validation has passed;
- that hardware is validated;
- that the system is hardened;
- that the system is release-candidate ready;
- that the system is daily-driver ready.

## Known limitations

| Area | Limitation |
| --- | --- |
| Detection | B2 uses only lightweight local hints and does not consume a formal B1 report yet. |
| Profile selection | A selected profile is a planning input, not proof that the profile is correct for the target. |
| Image builder | The preview references image-builder requirements but does not build an image. |
| Boot handoff | Kernel, initramfs, and Phase1 autostart are preview-only. |
| Installer | The preview does not partition, mount, write, install, or configure a boot loader. |
| Recovery | Recovery and emergency shell status are not validated. |
| Rollback | Rollback metadata is previewed but not written or validated. |
| Validation bundle | The bundle path is planned only; no validation report is created by the initial script. |
| VM validation | The VM profile remains unvalidated until a B3 VM validation report exists. |
| Hardware | No physical hardware support claim is allowed from B2. |

## Required behavior

The B2 dry-run assembly script must continue to:

- require `--dry-run`;
- require `--profile`;
- reject unsupported profiles;
- report `writes: no`;
- report `mutation: no`;
- report `network: no`;
- avoid image writes;
- avoid boot-loader mutation;
- avoid partition mutation;
- avoid package installation;
- avoid network access;
- avoid writing to `/boot`, `/etc`, EFI variables, initramfs files, or partitions;
- keep all boot, recovery, rollback, hardening, hardware, and release-candidate claims explicitly unclaimed.

## Before B2 can be considered complete

B2 completion still requires:

- script test suite passing in CI or local validation;
- status tracker links updated after implementation;
- source review confirming no mutation paths;
- output review confirming no secret leakage;
- limitations documented in this file;
- validation report documenting review results;
- README or roadmap visibility for the B2 boundary.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`B2_DRY_RUN_ASSEMBLY_PLAN.md`](B2_DRY_RUN_ASSEMBLY_PLAN.md)
- [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md)
- [`B1_READ_ONLY_DETECTION_LIMITATIONS.md`](B1_READ_ONLY_DETECTION_LIMITATIONS.md)
- [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)

## Non-claims

This limitations note does not make Base1 bootable, installer-ready, recovery-complete, hardened, VM-validated, hardware-validated, release-candidate ready, or daily-driver ready.

It documents the current B2 dry-run assembly boundary so implementation can continue safely.
