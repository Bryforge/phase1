# Base1 boot readiness race plan

Status: active planning roadmap
Scope: staged path toward Base1 boot readiness, read-only validation, x86_64 support, recovery, rollback, and hardening goals

## Purpose

This plan organizes the fastest safe path toward boot readiness for the Phase1/Base1 operating-system track.

The goal is speed without unsafe claims: move quickly through documentation, read-only checks, dry-run tooling, VM validation, recovery validation, and hardware validation before claiming boot readiness.

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

## Fastest safe sequence

### Sprint 1: readiness inventory

- Add this boot readiness race plan.
- Link it from the OS roadmap and README.
- Create a boot readiness checklist.
- Create a boot readiness status table.
- Protect the docs with tests.

### Sprint 2: read-only detection

- Add or plan read-only architecture detection.
- Add or plan firmware-mode detection.
- Add or plan boot-loader detection.
- Add or plan storage-layout detection.
- Add or plan recovery-media detection.
- Report `writes: no`.

### Sprint 3: boot parameter inventory

- Connect to [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md).
- Define required boot parameters per boot profile.
- Record unknown/unsupported parameters.
- Fail closed on unknown boot states.

### Sprint 4: image and initrd path

- Verify Base1 image-builder docs.
- Verify initramfs/initrd preview docs.
- Document Phase1 autostart handoff.
- Document emergency shell fallback.
- Keep mutation disabled until dry-run and recovery evidence exist.

### Sprint 5: VM validation

- Define VM validation profile.
- Capture boot log expectations.
- Validate Phase1 launch path.
- Record known limitations.
- Do not generalize VM success to physical hardware.

### Sprint 6: recovery and rollback

- Validate recovery shell path.
- Validate recovery media docs.
- Validate rollback metadata docs.
- Validate read-only recovery reports.
- Keep restore path visible before installer claims are strengthened.

### Sprint 7: hardware validation

- Validate one named hardware target at a time.
- Start with explicit target docs.
- Require validation reports before support claims.
- Keep known limitations visible.

### Sprint 8: release-candidate evidence

- Create target-specific release checklist.
- Collect build, boot, recovery, rollback, and docs validation evidence.
- Keep hardening claims evidence-bound.

## Required boot readiness artifacts

| Artifact | Path or planned path |
| --- | --- |
| OS roadmap | [`ROADMAP.md`](ROADMAP.md) |
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

## Non-claims

This boot readiness race plan does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, or daily-driver ready.

It defines a fast, evidence-bound path toward boot readiness without weakening safety, recovery, rollback, or usability.
