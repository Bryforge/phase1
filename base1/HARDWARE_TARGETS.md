# Base1 Hardware Targets

Base1 starts with two target classes: Raspberry Pi and ThinkPad X200-class hardware. The goal is to support small, recoverable security appliances and durable laptop-style operator consoles.

## Target A: Raspberry Pi

### Intended role

- Dedicated Phase1 terminal node.
- Portable security lab host.
- Low-cost recovery and monitoring box.
- Offline or limited-network Phase1 appliance.

### Security goals

- Bootable from known-good removable media.
- Easy rebuild and recovery.
- Default-deny inbound network policy.
- No host secrets inside Phase1 workspace.
- Phase1 runs as an unprivileged contained service or interactive app.
- Logs stored outside Phase1 workspace.

### Compatibility requirements

- ARM Linux profile.
- Conservative terminal color profile.
- Low-memory safe mode.
- No assumptions about desktop environment.
- Serial console friendly output.
- Works with keyboard-only operation.

### Storage guidance

Preferred first layout:

```text
boot media      OS boot files
root filesystem Base1 host
/var/lib/phase1 Phase1 workspace
/var/log/base1  Base1 host logs
```

Future hardened layout:

```text
read-only root
writable /var
separate Phase1 workspace
optional recovery image
```

### Risks to handle

- Power loss and removable-media corruption.
- Inconsistent USB power supplies.
- Removable media replacement.
- Network exposure when used headless.
- Operator accidentally running Phase1 with host privileges.

## Target B: ThinkPad X200-class systems

### Intended role

- Durable Phase1 operator laptop.
- Offline-first local system.
- Security-focused development host.
- Legacy hardware with predictable terminal performance.

### Security goals

- Support firmware-aware documentation for libreboot or coreboot-friendly use cases.
- Use the dedicated [Libreboot profile](LIBREBOOT_PROFILE.md) for Libreboot-backed X200-class systems.
- Keep Phase1 isolated from host package, boot, and recovery controls.
- Prefer minimal desktop or terminal-only install.
- Prefer storage encryption where practical.
- Keep logs and recovery tools outside Phase1 workspace.

### Compatibility requirements

- x86_64 Linux profile where available.
- Legacy terminal and limited color handling.
- Low-resolution display friendly layouts.
- Keyboard-first operation.
- Offline install path.
- No requirement for modern GPU features.

### Storage guidance

Preferred first layout:

```text
/boot              host boot files
/                  Base1 root
/home/phase1       phase1 user home, minimal
/var/lib/phase1    Phase1 workspace
/var/log/base1     Base1 logs
```

Future hardened layout:

```text
encrypted root
read-only base profile
separate Phase1 data volume
snapshot or rollback support
external recovery USB
```

### Risks to handle

- Legacy wireless firmware availability.
- Battery-backed clock drift.
- Older CPU mitigations and performance constraints.
- Legacy display size and color handling.
- Operator mixing host shell and Phase1 shell authority.

## Common hardening requirements

Both hardware classes need:

- Dedicated `phase1` runtime user.
- No passwordless host mutation from Phase1.
- No direct Phase1 write access to boot or package files.
- Optional offline-only profile.
- Firewall default deny inbound.
- Explicit maintenance mode for host changes.
- Recovery path tested before field use.
- Clear warning when running as root.
- Clear warning when host tools are enabled.

## Compatibility mode names

Base1 should expose stable target names:

```text
BASE1_HARDWARE_TARGET=raspberry-pi
BASE1_HARDWARE_TARGET=x200
BASE1_HARDWARE_TARGET=generic
```

The Phase1 UI can use this variable for color, terminal, runtime, and documentation hints without needing host-specific probing inside Phase1.

## First validation checklist

For each target, Base1 should validate:

- CPU architecture.
- Kernel name and version.
- Available RAM.
- Available disk space.
- Presence of `git` and `cargo` only when needed.
- Writable Phase1 workspace.
- Non-root Phase1 runtime account.
- Inbound network exposure.
- Recovery notes location.
- Host log path.

## Foundation-stage limits

The first Base1 foundation does not perform destructive setup. It only documents the target model and provides non-destructive readiness checks. Firmware updates, storage migration, SSH enablement, and network exposure remain explicit operator-controlled tasks.
