# Base1 OS Track

![Base1](https://img.shields.io/badge/Base1-foundation-ff8a00) ![Boot](https://img.shields.io/badge/boot-readiness%20tracked-00d8ff) ![Recovery](https://img.shields.io/badge/recovery-evidence%20bound-39ff88) ![Claims](https://img.shields.io/badge/claims-conservative-39ff88)

Base1 is the long-term host-foundation track for making Phase1 a bootable, recoverable, Phase1-first computing environment. It keeps host recovery, boot support, installer behavior, rollback metadata, and hardware validation separate from the Phase1 virtual OS console.

> [!IMPORTANT]
> Base1 is not currently a finished hardened operating system. It is a staged foundation. Use read-only, dry-run, and evidence-bound language until boot images, recovery validation, update validation, audit evidence, and hardware reports exist.

## What Base1 is for

| Area | Purpose |
| --- | --- |
| Boot foundation | Define how a Phase1-first environment can start safely on real hardware. |
| Recovery path | Keep emergency recovery outside Phase1 control so the host remains recoverable. |
| Installer path | Plan installation and target selection with dry-run validation before writes. |
| Storage layout | Describe target disks, partitions, mount expectations, and rollback metadata. |
| Hardware matrix | Track Raspberry Pi, x86_64 VM, X200-class, Libreboot, and generic targets honestly. |
| Supervisor path | Define future control-plane, policy, artifact, profile, and storage-tier surfaces. |
| Evidence gates | Require validation reports before stronger public claims. |

## Current safe posture

Base1 work should prefer these categories:

1. Documentation and design notes.
2. Read-only detection.
3. Dry-run assembly previews.
4. Preflight checks.
5. Recovery evidence.
6. Hardware checklists.
7. Validation reports.
8. Reviewed implementation.

Do not describe dry-run output as a completed install. Do not describe a hardware checklist as hardware support. Do not describe planned hardening as audited hardening.

## Start here

From the main repository documentation:

- `docs/os/ROADMAP.md` — Phase1 operating-system track.
- `docs/os/BOOT_READINESS_STATUS.md` — boot-readiness tracker.
- `docs/os/BOOT_READINESS_RACE_PLAN.md` — evidence-bound path toward boot readiness.
- `docs/os/X86_64_BOOT_SUPPORT_ROADMAP.md` — x86_64 boot support planning.
- `docs/os/BASE1_IMAGE_BUILDER.md` — image-builder design.
- `docs/os/INSTALLER_RECOVERY.md` — installer and recovery design.
- `docs/os/BASE1_DRY_RUN_COMMANDS.md` — Base1 dry-run command index.
- `base1/README.md` — Base1 overview.
- `base1/SECURITY_MODEL.md` — Base1 boundary and security model.
- `base1/HARDWARE_TARGETS.md` — hardware target matrix.

## B1 and B2 path

B1 focuses on read-only detection:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

B2 focuses on dry-run assembly preview:

```bash
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
```

Related validation:

```bash
cargo test -p phase1 --test base1_x86_64_detect_script
cargo test -p phase1 --test b1_read_only_detection_limitations_docs
cargo test -p phase1 --test b1_read_only_detection_validation_docs
cargo test -p phase1 --test b2_dry_run_assembly_plan_docs
cargo test -p phase1 --test base1_b2_assembly_dry_run_script
cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs
cargo test -p phase1 --test b2_dry_run_assembly_validation_docs
```

## Recovery and rollback path

Recovery and rollback work must preserve a strong host boundary. Phase1 can describe, prepare, and validate recovery artifacts, but the recovery path must not depend on a broken Phase1 runtime to recover the host.

Useful read-only or dry-run commands:

```bash
sh scripts/base1-preflight.sh
sh scripts/base1-libreboot-preflight.sh
sh scripts/base1-install-dry-run.sh --dry-run --target /dev/example
sh scripts/base1-recovery-dry-run.sh --dry-run
sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example
sh scripts/base1-rollback-metadata-dry-run.sh --dry-run
```

## Hardware target language

Use precise wording:

| Better wording | Avoid |
| --- | --- |
| `planned target` | `fully supported` |
| `read-only detected` | `installed` |
| `dry-run preview` | `assembled image` |
| `validation checklist exists` | `validated hardware` |
| `recovery design documented` | `recovery guaranteed` |
| `Libreboot profile notes` | `Libreboot certified` |

## Public claim checklist

Before making a stronger Base1 public claim, confirm:

- the implementation exists
- tests cover the claimed behavior
- docs explain limits and non-claims
- dry-run and write paths are clearly separated
- host recovery remains outside Phase1 control
- validation reports are present
- screenshots and website copy match the actual status
- README, wiki, in-system wiki, and release metadata agree

## Next pages

- [Updates, Releases, and Validation](08-Updates-Releases-and-Validation.md)
- [Boot Modes and Security](03-Boot-Modes-and-Security.md)
- [Fyr Native Language](14-Fyr-Native-Language.md)
