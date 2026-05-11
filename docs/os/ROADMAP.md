# Phase1 operating-system track

Phase1 has a long-term operating-system track through Base1.

This does not mean the current Phase1 Rust console is already a kernel, hardened sandbox, or drop-in replacement for Linux, macOS, or Windows. The current project remains a terminal-first virtual OS console. The OS track defines how Phase1 can become the primary user environment on bootable hardware in stages.

## Definition

The Phase1 operating-system track means:

- Base1 provides the minimal trusted host foundation.
- Phase1 becomes the default operator shell and system surface.
- Boot, recovery, update, storage, network, and hardware flows are owned and documented.
- Unsafe host actions remain explicit, guarded, and recoverable.
- Claims are tied to tested boot images, audits, hardware matrices, and rollback paths.

## Non-goals for the current checkpoint

- No claim that the current terminal console replaces a kernel.
- No claim that host-backed execution is hardened against hostile code.
- No removal of recovery access to the underlying host.
- No silent privilege change.
- No unsupported hardware promises.

## Boot readiness status

[`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md) tracks the current readiness level, finish-before-coding checklist, B1 coding-start gate, evidence map, and hardening status.

Current status: B1 read-only detection has an initial script and tests. The next target is completing B1 validation before moving to B2 dry-run assembly.

The B1 implementation plan is [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md). It defines the first coding slice and keeps the initial detector read-only, dry-run-only, and non-mutating.

Initial B1 script:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

Initial B1 test:

```bash
cargo test -p phase1 --test base1_x86_64_detect_script
```

## Boot readiness race

[`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md) defines the fastest safe path toward Base1 boot readiness.

The boot readiness ladder is:

| Level | Name | Claim boundary |
| --- | --- | --- |
| B0 | Documentation ready | Planning only. |
| B1 | Read-only detection ready | Detection preview. |
| B2 | Dry-run assembly ready | Dry-run only. |
| B3 | VM boot validated | VM validated only. |
| B4 | Recovery validated | Recovery validation for named profile only. |
| B5 | Physical target validated | Hardware-specific boot support only. |
| B6 | Release candidate | Release-candidate for named target only. |

Do not skip levels when strengthening boot, hardware, recovery, installer, daily-driver, or hardened-status claims.

## x86_64 boot support

[`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md) defines the automatic x86_64 support plan.

The plan covers:

- generic UEFI x86_64 systems;
- legacy BIOS x86_64 systems where supported;
- ThinkPad X200-class Libreboot/GRUB systems;
- VM validation profiles;
- recovery USB boot paths;
- boot profile names;
- boot parameter inventory;
- read-only detection before mutation;
- future hardening work with evidence-bound claims.

## Stage 0: Documentation pivot

Status: active.

Goals:

- Use a staged long-term OS track.
- Keep current limitations clear.
- Link the track from README and Base1 documentation.
- Add tests that prevent overclaiming.
- Maintain boot readiness, x86_64, and hardening plans without overclaiming current status.
- Keep [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md) current before implementation resumes.

## Stage 1: Base1 bootable foundation

Goal: produce a minimal bootable Base1 image that launches Phase1 as the primary shell.

Required pieces:

- Minimal Linux-based base image.
- Read-only base with writable user layer.
- Phase1 autostart path.
- Emergency shell fallback.
- First-boot setup flow.
- Hardware preflight checks.
- Reproducible image build notes.
- Boot readiness ladder alignment.

First design slice: [`Base1 image-builder design`](BASE1_IMAGE_BUILDER.md).

## Stage 2: Installer and recovery

Goal: make installation and rollback boring, visible, and recoverable.

Required pieces:

- Non-destructive installer mode.
- Disk layout plan.
- Recovery partition or recovery USB flow.
- Snapshot and rollback path.
- Signed or verifiable update metadata.
- Clear uninstall / restore path.
- Recovery validation before installer claims are strengthened.

## Stage 3: Daily-driver basics

Goal: cover the essentials needed for a primary computing environment.

Required pieces:

- User account flow.
- Wi-Fi and network management.
- Storage manager.
- Package and update manager.
- Browser/document escape hatch.
- Time, locale, keyboard, display, and accessibility settings.
- Low-resolution and mobile-safe UI path.

## Stage 4: Phase1-owned system surface

Goal: make Phase1 the controlling system experience rather than only a terminal app.

Required pieces:

- Phase1 login/session manager.
- Phase1 system settings panel.
- Base1 service supervision from Phase1.
- Snapshot and rollback controls from Phase1.
- Nested Phase1 as workspace and sandbox manager.
- System health dashboard.

## Stage 5: Hardware targets

Goal: ship only against explicit hardware targets.

Initial targets:

- Raspberry Pi image.
- ThinkPad X200-class image.
- Generic x86_64 image.

Each target needs:

- Boot validation.
- Network validation.
- Storage validation.
- Display/input validation.
- Recovery validation.
- Known limitations.
- Target-specific boot readiness level.

## Stage 6: Hardening track

Goal: move toward a hardened-by-design Base1 posture where evidence supports the claim.

Planned hardening areas:

- read-only base image;
- writable user/data layer separation;
- signed or verifiable boot metadata;
- recovery-first rollback paths;
- secret redaction in reports;
- explicit trust gates for host mutation;
- minimal boot services;
- service supervision and audit logging;
- secure defaults with documented usability tradeoffs.

Hardening is a valid roadmap goal. Current hardened-status claims require implementation, tests, validation reports, recovery evidence, and review evidence.

## Guardrails

- Keep safe mode default-on.
- Keep host tool execution guarded.
- Keep destructive/admin actions explicit.
- Keep recovery available.
- Keep security claims conservative until validated.
- Keep Base1 as the boundary between Phase1 and hardware.
- Keep boot readiness claims tied to the named readiness ladder.
- Keep hardening claims evidence-bound.
- Keep boot readiness status current before implementation resumes.

## First engineering slices

1. Add this roadmap and README positioning.
2. Add the [`Base1 boot readiness race plan`](BOOT_READINESS_RACE_PLAN.md).
3. Add the [`Base1 boot readiness status tracker`](BOOT_READINESS_STATUS.md).
4. Add the [`Base1 B1 read-only detection plan`](B1_READ_ONLY_DETECTION_PLAN.md).
5. Add the B1 read-only detector: `scripts/base1-x86_64-detect.sh`.
6. Add B1 detector tests: `tests/base1_x86_64_detect_script.rs`.
7. Add the [`Base1 x86_64 boot support roadmap`](X86_64_BOOT_SUPPORT_ROADMAP.md).
8. Add the [`Base1 image-builder design`](BASE1_IMAGE_BUILDER.md).
9. Add the [`Base1 installer and recovery design`](INSTALLER_RECOVERY.md).
10. Add the [`Base1 installer dry-run design`](BASE1_INSTALLER_DRY_RUN.md).
11. Add system-surface command stubs behind safe defaults.
12. Add hardware-target checklists.
13. Add read-only boot readiness status reporting.


## Stage 1 recovery command design

- [`Base1 recovery command design`](BASE1_RECOVERY_COMMAND.md) defines the first read-only recovery command surface for status and planning.


## Stage 2 storage layout checker design

- [`Base1 storage layout checker design`](BASE1_STORAGE_LAYOUT_CHECKER.md) defines the first read-only disk-layout validation surface for Base1 installer planning.


## Stage 2 rollback metadata design

- [`Base1 rollback metadata design`](BASE1_ROLLBACK_METADATA.md) defines the first safe restore-record contract for Base1 installer, update, and recovery planning.


## Stage 2 installer dry-run script

- `scripts/base1-install-dry-run.sh` is the first non-destructive installer preview command. It requires `--dry-run`, requires an explicit target, and reports `writes: no`.


## Stage 2 recovery dry-run script

- `scripts/base1-recovery-dry-run.sh` is the first non-destructive recovery preview command. It requires `--dry-run`, keeps boot settings unchanged, and reports `writes: no`.


## Stage 2 storage layout dry-run script

- `scripts/base1-storage-layout-dry-run.sh` is the first non-destructive storage layout preview command. It requires `--dry-run`, requires an explicit target, and reports `writes: no`.


## Stage 2 rollback metadata dry-run script

- `scripts/base1-rollback-metadata-dry-run.sh` is the first non-destructive rollback metadata preview command. It requires `--dry-run`, stores no secrets, writes nothing, and reports `operator_confirmed: no`.


## Stage 2 dry-run command index

- [`Base1 dry-run command index`](BASE1_DRY_RUN_COMMANDS.md) lists the current non-destructive Base1 preview commands and their shared guardrails.


## Stage 2 recovery USB design

- [`Base1 recovery USB design`](../../base1/RECOVERY_USB_DESIGN.md) defines the first read-only recovery-media plan for Libreboot/X200-class systems without writing USB media or changing boot settings.

## Non-claims

This roadmap does not make Phase1 or Base1 a finished OS, hardened system, installer-ready system, hardware-validated system, recovery-complete system, or daily-driver replacement.

It defines the staged path and claim boundaries for future boot readiness work.
