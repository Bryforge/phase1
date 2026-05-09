# Base1 installer dry-run design

The Base1 installer dry-run is the first safe command surface for moving Phase1 toward a bootable OS track.

This checkpoint is still documentation-only. It defines what the future dry-run command must show before any installer script is allowed to touch disks.

## Goal

Provide a non-destructive installer preview that helps an operator understand the target system, proposed layout, recovery path, and risks before any destructive action exists.

## Command shape

```text
base1 install --dry-run --target <disk>
```

The first implementation must refuse to run without `--dry-run`.

## Required dry-run output

A dry-run report must include:

- Selected target disk.
- Hardware preflight summary.
- Proposed boot partition.
- Proposed read-only Base1 layer.
- Proposed writable Phase1 state layer.
- Proposed recovery layer.
- Phase1 auto-launch plan.
- Emergency shell fallback.
- Rollback metadata plan.
- Explicit statement that no disk writes occurred.

## Forbidden early behavior

The dry-run command must not:

- Partition disks.
- Format filesystems.
- Mount writable system targets.
- Install bootloaders.
- Enable host trust.
- Disable recovery access.
- Hide the target disk identity.

## Example output

```text
base1 installer dry-run
target  : /dev/example
writes  : no
boot    : preview only
base1   : read-only layer preview
state   : writable phase1 state preview
recover : emergency shell + rollback metadata preview
status  : dry-run complete
```

## Promotion gate

The dry-run command can only become a real installer after:

1. Target-disk confirmation exists.
2. Recovery boot path is tested.
3. Rollback metadata is tested.
4. Hardware target checks pass.
5. The user receives a final destructive-action confirmation.
