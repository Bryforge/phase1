# Base1 x86_64 boot support roadmap

Status: planning roadmap
Scope: automatic x86_64 support, boot parameter discovery, firmware mode handling, kernel command-line policy, and hardware validation

## Purpose

This roadmap defines how Base1 should grow toward automatic support for generic x86_64 systems while preserving recovery, safe defaults, and conservative security claims.

The goal is to make x86_64 boot support boring and predictable: detect the system, choose the correct boot profile, expose required boot parameters, validate before mutation, and keep rollback/recovery available.

Current boot readiness is tracked in [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md). Do not start B1 implementation until that tracker says the finish-first gate is complete.

## Security and hardening goal

Base1 and Phase1 should become as hardened as practical over time while maintaining usability.

`Hardened` is allowed as a roadmap goal or planned security posture when clearly labeled as planned, design, preview, or validation-dependent.

Do not claim that a current image, installer, boot path, or x86_64 target is hardened until the repository contains implementation, tests, validation reports, recovery evidence, and review evidence for that claim.

## Target systems

Initial x86_64 planning should cover:

- generic UEFI x86_64 laptops and desktops;
- legacy BIOS x86_64 systems where supported;
- ThinkPad X200-class Libreboot/GRUB systems;
- virtual machines used for validation;
- removable recovery media boot paths.

## Firmware and boot modes

Base1 should detect and document these boot modes:

| Mode | Planned handling |
| --- | --- |
| UEFI | Prefer explicit EFI System Partition detection and signed/verifiable boot metadata when available. |
| Legacy BIOS | Support only with clear limitations and rollback guidance. |
| Libreboot/GRUB | Preserve GRUB-first recovery guidance and operator-visible boot entries. |
| Virtual machine | Use as a repeatable validation target before physical hardware claims. |
| Recovery USB | Keep emergency shell and recovery media available before installer mutation. |

## Automatic detection goals

Future tooling should detect:

- CPU architecture: `x86_64`;
- firmware mode: UEFI, BIOS, or Libreboot/GRUB path;
- boot loader: GRUB, systemd-boot, EFI stub, or unknown;
- kernel command-line source;
- root device strategy;
- initramfs availability;
- storage layout;
- display/input availability;
- network availability;
- recovery media availability;
- virtualization status;
- secure boot status where applicable;
- TPM presence where applicable.

Unknown or unsupported states should fail closed with a readable explanation.

## Boot parameter inventory

Base1 should maintain an explicit inventory of boot parameters needed for each supported boot profile.

Required categories:

| Category | Example parameters or data |
| --- | --- |
| Root filesystem | `root=`, `rootflags=`, `rootfstype=`, UUID/PARTUUID strategy. |
| Init process | `init=`, Phase1 autostart handoff, emergency shell fallback. |
| Initramfs | initrd path, module requirements, fallback initramfs. |
| Console | `console=`, serial console where needed, keyboard/display fallback. |
| Graphics | `nomodeset`, framebuffer policy, GPU driver mode notes. |
| Storage | controller modules, encryption planning, read-only base, writable layer. |
| Network | predictable interface naming policy, recovery networking defaults. |
| Recovery | emergency shell flag, rollback selector, recovery media discovery. |
| Security posture | lockdown mode planning, safe defaults, audit/logging flags. |
| Debugging | verbose boot flag, panic timeout, safe diagnostic mode. |

This inventory should be documentation-first until implementation and validation exist.

## Boot profile model

Future Base1 x86_64 support should use named boot profiles:

```text
x86_64-uefi-generic
x86_64-bios-generic
x86_64-libreboot-grub
x86_64-vm-validation
x86_64-recovery-usb
```

Each profile should define:

- supported firmware mode;
- boot loader assumptions;
- required partitions;
- required boot parameters;
- kernel/initramfs requirements;
- recovery fallback;
- validation commands;
- known limitations;
- non-claims.

## Proposed read-only commands

Initial command work should be non-destructive:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
sh scripts/base1-x86_64-boot-params.sh --dry-run
sh scripts/base1-x86_64-profile-report.sh --dry-run
sh scripts/base1-x86_64-validate.sh --dry-run
```

These commands should report `writes: no` until installer mutation is separately designed and validated.

The first B1 implementation slice should be only:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

Expected B1 output categories:

- architecture hints;
- firmware hints;
- boot-loader hints;
- virtualization hints;
- storage-layout hints;
- recovery availability hints;
- unknown/unsupported state warnings;
- `writes: no`.

## Validation requirements

Before x86_64 support is claimed, the repository should contain:

- read-only detection report;
- boot parameter report;
- boot profile report;
- VM validation report;
- at least one physical hardware validation report per supported hardware class;
- recovery boot validation;
- rollback validation;
- known limitations;
- tests for documentation and script syntax;
- no unsupported hardening or security claims.

## Hardening roadmap items

Hardening is a valid goal for this track.

Planned hardening work may include:

- read-only base image;
- writable user/data layer separation;
- signed or verifiable boot metadata;
- measured boot planning where hardware supports it;
- recovery-first rollback paths;
- secret redaction in reports;
- explicit trust gates for host mutation;
- kernel lockdown planning where compatible;
- minimal boot services;
- service supervision and audit logging;
- secure defaults with documented usability tradeoffs.

Each hardening item must be labeled by status and validation level before being described as implemented.

## Implementation phases

### Phase 1: documentation and inventory

- Create this roadmap.
- Define boot profile names.
- Define required boot parameter categories.
- Link from the OS roadmap and Base1 docs.
- Link from [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md).
- Add docs tests that preserve non-claims.

### Phase 2: read-only detection

- Add dry-run architecture and firmware detection.
- Report x86_64, UEFI/BIOS/Libreboot hints, storage layout, boot loader hints, and recovery availability.
- Write no disk state.
- Keep current status visible in [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md).

### Phase 3: boot parameter report

- Generate a read-only boot parameter report.
- Explain required, optional, unknown, and unsupported parameters.
- Preserve recovery and rollback guidance.

### Phase 4: VM validation

- Validate generic x86_64 boot profile in a VM.
- Capture logs and known limitations.
- Do not generalize VM success to hardware claims.

### Phase 5: physical hardware validation

- Validate specific hardware classes one at a time.
- Start with explicitly documented targets.
- Require validation reports before hardware support claims.

### Phase 6: installer integration

- Only after detection, boot parameter reporting, recovery, and rollback validation exist.
- Keep mutation explicit and recoverable.
- Require confirmation for boot-loader or partition changes.

## Safety rules

- No silent boot-loader mutation.
- No silent partition mutation.
- No automatic destructive install path.
- No unsupported hardening claims.
- No hardware support claim without validation report.
- No secure boot, measured boot, TPM, or lockdown claim without evidence.
- Recovery shell must remain available before install claims are strengthened.

## Related docs

- [`ROADMAP.md`](ROADMAP.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`BASE1_IMAGE_BUILDER.md`](BASE1_IMAGE_BUILDER.md)
- [`INSTALLER_RECOVERY.md`](INSTALLER_RECOVERY.md)
- [`BASE1_DRY_RUN_COMMANDS.md`](BASE1_DRY_RUN_COMMANDS.md)
- [`../../base1/HARDWARE_TARGETS.md`](../../base1/HARDWARE_TARGETS.md)
- [`../../base1/LIBREBOOT_PROFILE.md`](../../base1/LIBREBOOT_PROFILE.md)
- [`../security/TRUST_MODEL.md`](../security/TRUST_MODEL.md)

## Non-claims

This roadmap does not make Base1 bootable on all x86_64 systems.

It does not claim hardened boot, secure boot support, measured boot support, installer readiness, recovery completion, hardware validation, or daily-driver readiness.

It defines the plan for automatic x86_64 detection, boot parameter inventory, validation, and future hardening work.
