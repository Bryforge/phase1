# Base1 boot readiness race plan

Status: active planning roadmap
Scope: staged path toward Base1 boot readiness, read-only validation, x86_64 support, recovery, rollback, and hardening goals

## Purpose

This plan organizes the fastest safe path toward boot readiness for the Phase1/Base1 operating-system track.

The goal is speed without unsafe claims: move quickly through documentation, read-only checks, dry-run tooling, VM validation, recovery validation, and hardware validation before claiming boot readiness.

Track current readiness in [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md). The first B1 coding slice is planned in [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md), implemented by `scripts/base1-x86_64-detect.sh`, and guarded by `tests/base1_x86_64_detect_script.rs`. B2 dry-run assembly starts in [`B2_DRY_RUN_ASSEMBLY_PLAN.md`](B2_DRY_RUN_ASSEMBLY_PLAN.md), is implemented initially by `scripts/base1-b2-assembly-dry-run.sh`, and is guarded by `tests/base1_b2_assembly_dry_run_script.rs`.

## Boot readiness goal

Base1 should become a bootable foundation that can launch Phase1 as the primary operator surface while keeping recovery, rollback, validation, and safe defaults visible.

Boot readiness means the repository can show evidence for:

- image build path;
- boot profile selection;
- kernel/initramfs handoff;
- Phase1 autostart path;
- emergency shell fallback;
- recovery path;
- rollback metadata;
- hardware or VM validation reports;
- known limitations;
- no unsupported hardening or daily-driver claims.

## Current boundary

Phase1 is still a terminal-first virtual OS console. Base1 boot readiness is a staged roadmap and validation track until boot images, recovery, update paths, hardware support, and review evidence exist.

Hardening is a valid roadmap goal. Current hardened-status claims require implementation, tests, validation reports, recovery evidence, and review evidence.

## Boot readiness ladder

| Level | Name | Meaning | Claim allowed |
| --- | --- | --- | --- |
| B0 | Documentation ready | Boot path is documented and bounded. | Planning only. |
| B1 | Read-only detection ready | Scripts can inspect host/target facts without writes. | Detection preview. |
| B2 | Dry-run assembly ready | Image/build/install/recovery commands preview actions with no writes. | Dry-run only. |
| B3 | VM boot validated | A VM profile boots through the expected path and records evidence. | VM validated only. |
| B4 | Recovery validated | Emergency shell, recovery media, and rollback paths are validated. | Recovery validation for named profile only. |
| B5 | Physical target validated | A named hardware class has validation reports. | Hardware-specific boot support only. |
| B6 | Release candidate | Repeatable build, validation, docs, and rollback evidence are complete for a target. | Release-candidate for named target only. |

Do not skip levels when strengthening claims.

## Finish-first gate

Before implementation resumes, keep the finish-first checklist current in [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md).

B1 coding began only after the status tracker was linked from the OS roadmap, this race plan, the x86_64 roadmap, and the README, with tests preserving the ladder and non-claims.

The first B1 implementation plan is [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md). It is implemented initially by:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

Initial B1 script tests:

```bash
cargo test -p phase1 --test base1_x86_64_detect_script
```

B2 dry-run assembly is implemented initially by:

```bash
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
```

Initial B2 script tests:

```bash
cargo test -p phase1 --test base1_b2_assembly_dry_run_script
```

B2 remains dry-run-only until limitations, validation report, review, and status updates are complete.

## Fastest safe sequence

### Sprint 1: readiness inventory

- Add this boot readiness race plan.
- Link it from the OS roadmap and README.
- Create a boot readiness checklist.
- Create a boot readiness status table: [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md).
- Create the B1 plan: [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md).
- Protect the docs with tests.

### Sprint 2: read-only detection

- Implement only the B1 detector planned in [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md).
- Initial script: `scripts/base1-x86_64-detect.sh`.
- Initial tests: `tests/base1_x86_64_detect_script.rs`.
- Add or plan read-only architecture detection.
- Add or plan firmware-mode detection.
- Add or plan boot-loader detection.
- Add or plan storage-layout detection.
- Add or plan recovery-media detection.
- Report `writes: no`.

### Sprint 3: B2 dry-run assembly

- Add the B2 dry-run assembly plan: [`B2_DRY_RUN_ASSEMBLY_PLAN.md`](B2_DRY_RUN_ASSEMBLY_PLAN.md).
- Add the B2 dry-run assembly script: `scripts/base1-b2-assembly-dry-run.sh`.
- Add B2 dry-run assembly tests: `tests/base1_b2_assembly_dry_run_script.rs`.
- Preview dry-run assembly for `x86_64-vm-validation`.
- Connect B1 detection facts to profile, image, boot handoff, recovery, rollback, and validation bundle previews.
- Keep B2 preview-only until limitations and validation report exist.

### Sprint 4: boot parameter inventory

- Connect to [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md).
- Define required boot parameters per boot profile.
- Record unknown/unsupported parameters.
- Fail closed on unknown boot states.

### Sprint 5: image and initrd path

- Verify Base1 image-builder docs.
- Verify initramfs/initrd preview docs.
- Document Phase1 autostart handoff.
- Document emergency shell fallback.
- Keep mutation disabled until dry-run and recovery evidence exist.

### Sprint 6: VM validation

- Define VM validation profile.
- Capture boot log expectations.
- Validate Phase1 launch path.
- Record known limitations.
- Do not generalize VM success to physical hardware.

### Sprint 7: recovery and rollback

- Validate recovery shell path.
- Validate recovery media docs.
- Validate rollback metadata docs.
- Validate read-only recovery reports.
- Keep restore path visible before installer claims are strengthened.

### Sprint 8: hardware validation

- Validate one named hardware target at a time.
- Start with explicit target docs.
- Require validation reports before support claims.
- Keep known limitations visible.

### Sprint 9: release-candidate evidence

- Create target-specific release checklist.
- Collect build, boot, recovery, rollback, and docs validation evidence.
- Keep hardening claims evidence-bound.

## Required boot readiness artifacts

| Artifact | Path or planned path |
| --- | --- |
| OS roadmap | [`ROADMAP.md`](ROADMAP.md) |
| Boot readiness status tracker | [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md) |
| B1 read-only detection plan | [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md) |
| B1 read-only detector | `scripts/base1-x86_64-detect.sh` |
| B1 detector tests | `tests/base1_x86_64_detect_script.rs` |
| B2 dry-run assembly plan | [`B2_DRY_RUN_ASSEMBLY_PLAN.md`](B2_DRY_RUN_ASSEMBLY_PLAN.md) |
| B2 dry-run assembly script | `scripts/base1-b2-assembly-dry-run.sh` |
| B2 dry-run assembly tests | `tests/base1_b2_assembly_dry_run_script.rs` |
| x86_64 boot support roadmap | [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md) |
| Base1 image-builder design | [`BASE1_IMAGE_BUILDER.md`](BASE1_IMAGE_BUILDER.md) |
| Installer/recovery design | [`INSTALLER_RECOVERY.md`](INSTALLER_RECOVERY.md) |
| Dry-run command index | [`BASE1_DRY_RUN_COMMANDS.md`](BASE1_DRY_RUN_COMMANDS.md) |
| Recovery command design | [`BASE1_RECOVERY_COMMAND.md`](BASE1_RECOVERY_COMMAND.md) |
| Storage layout checker | [`BASE1_STORAGE_LAYOUT_CHECKER.md`](BASE1_STORAGE_LAYOUT_CHECKER.md) |
| Rollback metadata | [`BASE1_ROLLBACK_METADATA.md`](BASE1_ROLLBACK_METADATA.md) |
| Hardware targets | [`../../base1/HARDWARE_TARGETS.md`](../../base1/HARDWARE_TARGETS.md) |
| Libreboot profile | [`../../base1/LIBREBOOT_PROFILE.md`](../../base1/LIBREBOOT_PROFILE.md) |

## Boot readiness checklist

Before claiming boot readiness for any target, confirm:

- [ ] target name is explicit;
- [ ] architecture is explicit;
- [ ] firmware mode is explicit;
- [ ] boot loader path is explicit;
- [ ] boot parameters are documented;
- [ ] image build path is documented;
- [ ] Phase1 autostart path is documented;
- [ ] emergency shell fallback is documented;
- [ ] recovery path is documented;
- [ ] rollback path is documented;
- [ ] validation commands are documented;
- [ ] validation report exists;
- [ ] known limitations are documented;
- [ ] no unsupported hardened, secure boot, measured boot, hardware-validated, installer-ready, or daily-driver claim is made.

## Safety rules

- Read-only before dry-run.
- Dry-run before mutation.
- VM validation before broad hardware claims.
- Hardware-specific validation before target support claims.
- Recovery before installer claims.
- Rollback before update claims.
- Evidence before hardening claims.

## Validation commands

General validation:

```bash
sh scripts/quality-check.sh quick
```

Base1 documentation validation:

```bash
sh scripts/quality-check.sh base1-docs
```

Broad Base1 reorganization/readiness validation:

```bash
sh scripts/quality-check.sh base1-reorg
```

B1 detector validation:

```bash
cargo test -p phase1 --test base1_x86_64_detect_script
```

B2 planning validation:

```bash
cargo test -p phase1 --test b2_dry_run_assembly_plan_docs
```

B2 dry-run assembly validation:

```bash
cargo test -p phase1 --test base1_b2_assembly_dry_run_script
```

## Non-claims

This boot readiness race plan does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, or daily-driver ready.

It defines a fast, evidence-bound path toward boot readiness without weakening safety, recovery, rollback, or usability.
