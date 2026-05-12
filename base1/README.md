# Base1

Base1 is the planned secure hardware host for Phase1.

It is designed for machines where Phase1 should be useful, portable, and recoverable without giving Phase1 the power to destroy the underlying host. The first hardware targets are:

- Raspberry Pi systems, especially a small dedicated terminal/security node.
- ThinkPad X200-class systems, especially libreboot/coreboot-friendly legacy laptops.

Base1 is not presented as more secure than OpenBSD today. The project goal is to build toward that level of discipline: simple design, small trusted computing base, secure defaults, aggressive compartmentalization, clear code paths, excellent documentation, and repeatable audits.

## Mission

Base1 exists to keep the hardware safe while Phase1 runs above it.

If Phase1 is corrupted, crashed, misconfigured, wiped, or intentionally destroyed, Base1 should remain intact, bootable, recoverable, and able to reinstall or rebuild a known-good Phase1 environment.

## Initial design principles

1. Secure by default.
2. Minimal base install.
3. Phase1 is treated as contained application workload, not as the host authority.
4. Host mutation is denied unless the operator explicitly enters a maintenance mode.
5. Runtime state is separated from system state.
6. Recovery paths are designed before convenience features.
7. Every host integration must have a compatibility contract with Phase1.
8. Raspberry Pi and X200 support must be designed from the beginning, not bolted on later.

## Trust boundary

Base1 owns:

- Boot and recovery policy.
- Disk layout and rollback policy.
- Kernel, init, networking, firewalling, logging, and update channels.
- Host secrets and operator credentials.
- Phase1 install, update, and reset controls.

Phase1 owns:

- The terminal-first virtual OS experience.
- Simulated kernel, VFS, process table, audit log, language runtime interface, and operator console.
- Its own workspace and user data inside the controlled Phase1 storage area.

Phase1 must not own:

- Base1 boot files.
- Base1 package manager keys.
- Host SSH credentials.
- Host firewall rules.
- Host persistent secrets.
- Host recovery images.

## First implementation layer

This repository now contains the first Base1 foundation files:

- `SECURITY_MODEL.md` - threat model and security architecture.
- `HARDWARE_TARGETS.md` - Raspberry Pi and X200 target requirements.
- `PHASE1_COMPATIBILITY.md` - compatibility contract between Base1 and Phase1.
- `NETWORK_LOCKDOWN_DRY_RUN.md` - network lockdown dry-run contract and promotion guardrails.
- `config/base1-secure-profile.toml` - machine-readable secure profile draft.
- `scripts/base1-preflight.sh` - non-destructive host readiness checker.
- `scripts/base1-network-lockdown-dry-run.sh` - non-destructive network lockdown preview.
- `scripts/base1-phase1-run.sh` - hardened Phase1 launcher wrapper.
- `systemd/phase1-base1.service` - hardened systemd unit template.

## Install strategy

Base1 installation must be staged:

1. Preflight only: detect hardware, OS, storage, boot mode, and hardening capability.
2. Dry-run plan: print exactly what would change.
3. Operator confirmation: require explicit maintenance mode.
4. Build immutable/recoverable host profile.
5. Install Phase1 into a contained runtime account.
6. Verify recovery path before enabling convenience features.

No Base1 script should silently repartition, erase, or reconfigure a host.

## Security posture

Base1 should prefer:

- Read-only or rollback-capable root where practical.
- Separate writable Phase1 workspace.
- Dedicated low-privilege `phase1` runtime account.
- No passwordless host mutation from Phase1.
- Firewall default deny inbound.
- SSH disabled by default unless explicitly enabled for a managed appliance use case.
- Logs retained outside Phase1 workspace.
- Tight service sandboxing.
- Measured and documented boot chain where hardware supports it.

## Network lockdown preview

The current network work is deliberately read-only. Run:

```bash
sh scripts/base1-network-lockdown-dry-run.sh --dry-run
```

Useful profile previews:

```bash
sh scripts/base1-network-lockdown-dry-run.sh --dry-run --profile secure-default
sh scripts/base1-network-lockdown-dry-run.sh --dry-run --profile offline
sh scripts/base1-network-lockdown-dry-run.sh --dry-run --profile appliance --target raspberry-pi
sh scripts/base1-network-lockdown-dry-run.sh --dry-run --profile dev --target x200
```

The command reports `writes: no`, inspects available firewall tooling, previews inbound/outbound policy, and keeps Phase1 host-backed tools denied by default. It does not change firewall rules or service state.

## Current status

Base1 is at foundation stage. The current files establish the secure architecture, hardware constraints, first operator tooling, and read-only network lockdown preview. They do not yet constitute a destructive installer or complete operating system image.

## Next milestones

- Add a reproducible Base1 image build path.
- Add Raspberry Pi image profile.
- Add X200 installer profile.
- Add immutable/rollback storage layout documentation.
- Add signed Phase1 release installation flow.
- Promote network lockdown from dry-run only after recovery, rollback, hardware, and operator-confirmation tests exist.
- Add offline-mode profile validation.
- Add recovery media and reset workflow.
- Add Base1 integration tests in CI.
- [`base1/RECOVERY_USB_TARGET_SELECTION.md`](RECOVERY_USB_TARGET_SELECTION.md) — Recovery USB target selection design.

## Base1 public surface index

- [`HARDWARE_TARGETS.md`](HARDWARE_TARGETS.md) — Hardware target matrix.
- [`LIBREBOOT_COMMAND_INDEX.md`](LIBREBOOT_COMMAND_INDEX.md) — Libreboot command index.
- [`LIBREBOOT_DOCS_SUMMARY.md`](LIBREBOOT_DOCS_SUMMARY.md) — Libreboot docs summary.
- [`LIBREBOOT_GRUB_RECOVERY.md`](LIBREBOOT_GRUB_RECOVERY.md) — Libreboot GRUB recovery notes.
- [`LIBREBOOT_MILESTONE.md`](LIBREBOOT_MILESTONE.md) — Libreboot milestone checkpoint.
- [`LIBREBOOT_OPERATOR_CHECKLIST.md`](LIBREBOOT_OPERATOR_CHECKLIST.md) — Libreboot operator checklist.
- [`LIBREBOOT_PREFLIGHT.md`](LIBREBOOT_PREFLIGHT.md) — Libreboot preflight notes.
- [`LIBREBOOT_PROFILE.md`](LIBREBOOT_PROFILE.md) — Libreboot profile.
- [`LIBREBOOT_QUICKSTART.md`](LIBREBOOT_QUICKSTART.md) — Libreboot quickstart.
- [`LIBREBOOT_VALIDATION_REPORT.md`](LIBREBOOT_VALIDATION_REPORT.md) — Libreboot validation report.
- [`NETWORK_LOCKDOWN_DRY_RUN.md`](NETWORK_LOCKDOWN_DRY_RUN.md) — Network lockdown dry-run.
- [`PHASE1_COMPATIBILITY.md`](PHASE1_COMPATIBILITY.md) — Phase1 compatibility contract.
- [`RECOVERY_USB_COMMAND_INDEX.md`](RECOVERY_USB_COMMAND_INDEX.md) — Recovery USB command index.
- [`RECOVERY_USB_DESIGN.md`](RECOVERY_USB_DESIGN.md) — Recovery USB design.
- [`RECOVERY_USB_EMERGENCY_SHELL.md`](RECOVERY_USB_EMERGENCY_SHELL.md) — Recovery USB emergency shell design.
- [`RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md`](RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md) — Recovery USB emergency shell command index.
- [`RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md`](RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md) — Recovery USB emergency shell summary.
- [`RECOVERY_USB_HARDWARE_CHECKLIST.md`](RECOVERY_USB_HARDWARE_CHECKLIST.md) — Recovery USB hardware checklist.
- [`RECOVERY_USB_HARDWARE_SUMMARY.md`](RECOVERY_USB_HARDWARE_SUMMARY.md) — Recovery USB hardware summary.
- [`RECOVERY_USB_IMAGE_COMMAND_INDEX.md`](RECOVERY_USB_IMAGE_COMMAND_INDEX.md) — Recovery USB image command index.
- [`RECOVERY_USB_IMAGE_PROVENANCE.md`](RECOVERY_USB_IMAGE_PROVENANCE.md) — Recovery USB image provenance.
- [`RECOVERY_USB_IMAGE_SUMMARY.md`](RECOVERY_USB_IMAGE_SUMMARY.md) — Recovery USB image summary.
- [`RECOVERY_USB_TARGET_COMMAND_INDEX.md`](RECOVERY_USB_TARGET_COMMAND_INDEX.md) — Recovery USB target command index.
- [`RECOVERY_USB_TARGET_SUMMARY.md`](RECOVERY_USB_TARGET_SUMMARY.md) — Recovery USB target summary.
- [`RECOVERY_USB_VALIDATION_REPORT.md`](RECOVERY_USB_VALIDATION_REPORT.md) — Recovery USB validation report.
- [`ROADMAP.md`](ROADMAP.md) — Base1 roadmap.
- [`SECURITY_MODEL.md`](SECURITY_MODEL.md) — Base1 security model.
