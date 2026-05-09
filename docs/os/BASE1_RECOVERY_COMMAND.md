# Base1 recovery command design

The Base1 recovery command is the first safe recovery-control surface for the Phase1 OS track.

This checkpoint is documentation-only. It defines the future command contract before any recovery script changes boot settings, mounts disks, or modifies system state.

## Goal

Provide a visible, non-destructive recovery preview that lets an operator understand how to disable Phase1 auto-launch, access emergency tools, inspect logs, and recover user data without weakening the host boundary.

## Command shape

```text
base1 recovery --dry-run
base1 recovery status
base1 recovery plan
```

Early implementations must be read-only and must not require host trust.

## Required recovery status output

A recovery status report must include:

- Current boot target.
- Whether Phase1 auto-launch is enabled.
- Emergency shell availability.
- Recovery partition or recovery directory status.
- Rollback metadata status.
- Writable state layer status.
- Safe-mode and host-trust status.
- Explicit statement that no recovery changes were made.

## Required recovery plan output

A recovery plan must preview:

- How to disable Phase1 auto-launch.
- How to enter emergency shell mode.
- How to read recovery logs.
- How to export `/state/phase1`.
- How to restore the previous boot target.
- How to verify rollback metadata.
- What manual confirmation would be required before any change.

## Forbidden early behavior

The recovery command must not:

- Modify bootloader entries.
- Disable Phase1 auto-launch.
- Delete state.
- Mount writable system targets.
- Enable host trust.
- Hide recovery failures.
- Claim recovery is validated before hardware tests exist.

## Example output

```text
base1 recovery status
boot target : phase1
auto-launch : enabled
shell       : emergency fallback available
state       : /state/phase1 preview only
rollback    : metadata preview only
writes      : no
status      : recovery check complete
```

## Promotion gate

The recovery command can only gain mutating behavior after:

1. Dry-run status exists.
2. Dry-run plan exists.
3. Recovery shell path is tested.
4. Rollback metadata is tested.
5. State export is tested.
6. Final confirmation exists for every destructive or boot-changing action.
