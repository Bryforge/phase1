# Base1 storage layout checker design

The Base1 storage layout checker is the first safe storage-validation surface for the Phase1 OS track.

This checkpoint is documentation-only. It defines a future read-only checker before any command is allowed to partition, format, mount, or modify disks.

## Goal

Provide a non-destructive storage report that shows whether a target system can support a Phase1-first Base1 layout.

The checker must explain the proposed storage model, recovery path, writable state layer, and rollback expectations without changing the host.

## Command shape

```text
base1 storage check --dry-run --target <disk>
base1 storage check --target <disk>
base1 storage plan --target <disk>
```

All initial command shapes are read-only previews.

## Required report fields

A storage layout check must report:

- Target disk identity.
- Disk size.
- Existing partition summary.
- Proposed `/boot` layout.
- Proposed read-only `/base1` layer.
- Proposed writable `/state/phase1` layer.
- Proposed `/recovery` area.
- Whether rollback metadata can be stored.
- Whether destructive writes are disabled.
- Whether final confirmation would be required.

## Example output

```text
base1 storage layout check
target   : /dev/example
writes   : no
boot     : preview only
base1    : read-only layer preview
state    : writable phase1 state preview
recovery : recovery area preview
rollback : metadata preview only
status   : storage check complete
```

## Guardrails

- Do not partition disks.
- Do not format disks.
- Do not mount writable filesystems.
- Do not hide existing partitions.
- Do not erase or overwrite data.
- Do not imply install readiness from a dry-run report.
- Do not proceed without explicit target disk selection.

## Promotion gate

The checker can only become part of an installer path after:

1. Read-only reporting exists.
2. Target disk detection is tested.
3. Existing partitions are displayed clearly.
4. Recovery layout is documented.
5. Rollback metadata is documented.
6. Final destructive-action confirmation exists.
