# Nested Phase1 checkpoint

Nested Phase1 is now a documented metadata-control surface for recursive Phase1 operator contexts.

This checkpoint captures the first safe layer of the idea: no real inner process execution yet, no host escalation, and no child kernel runtime. The current implementation models nested contexts as isolated metadata children inside the outer Phase1 session.

## Current checkpoint

Implemented commands:

```text
nest status
nest spawn <name>
nest list
nest enter <name>
nest exit
nest destroy <name>
nest rm <name>
nest inspect <name>
nest info <name>
nest tree
```

## What works now

- Root and non-root nest status reporting.
- Named nested child metadata contexts.
- Duplicate-name and invalid-name guards.
- Depth cap enforcement through `PHASE1_NESTED_MAX`.
- Active child context switching.
- Safe return to root context.
- Child destruction with active-context reset.
- Child inspection.
- Tree/topology output.
- Help coverage for the nested command surface.

## Safety posture

Nested Phase1 is metadata-only at this checkpoint. It does not bypass the existing Phase1 host boundary. Child contexts inherit the safe default posture and remain operator-visible from the outer shell.

## Validation

```bash
cargo test -p phase1 --test nest_status
cargo test -p phase1 --test nest_spawn
cargo test -p phase1 --test nest_enter
cargo test -p phase1 --test nest_destroy
cargo test -p phase1 --test nest_inspect
cargo test -p phase1 --test nest_tree
cargo test --workspace --all-targets
```

## Next roadmap slices

- `nest snapshot <name>` metadata snapshots.
- `nest restore <name>` metadata restoration.
- `/proc/nests` observability.
- Per-child audit metadata.
- Optional later runtime-backed inner Phase1 process experiments.
