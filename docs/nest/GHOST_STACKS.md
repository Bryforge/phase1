# Phase1 nest ghost stacks

Issue: #332  
Status: design contract  
Scope: operator-visible control layer for nested Phase1 machine sessions  
Non-claim: ghost stacks are not claimed to be separate physical machines, VMs, containers, or security isolation boundaries.

Ghost stacks give operators a readable control surface for nested Phase1 sessions. The goal is to make nest lineage, active level, suspended labels, recovery actions, and exit-all state visible without hiding the existing nest safety model.

## Terms

| Term | Meaning |
| --- | --- |
| root | The outermost Phase1 session. |
| nest | A Phase1 session launched from another Phase1 session. |
| active level | The currently running nest level, such as `0/1`. |
| ghost entry | A remembered or suspended nest context label, not an independent machine. |
| stack | The ordered operator view of root, active, and ghost entries. |
| exit-all | A stack-wide request to unwind all nested Phase1 sessions. |

## Proposed command surface

```text
nest stack
nest stack status
nest stack list
nest stack push <label>
nest stack pop
nest stack ghost <label>
nest stack resume <label>
nest stack prune
nest stack exit-all
```

Short aliases may be added later, but the canonical commands must remain discoverable through:

```text
nest help
help nest
help flows
```

## First implementation slice

The first runtime slice should be read-only:

```text
nest stack
nest stack status
nest stack list
```

Those commands should report the current nest level, max depth, active entry, root entry, ghost count, and whether exit-all has been requested.

Mutating commands should initially return explicit not-yet-implemented rows:

```text
nest stack push <label>
nest stack pop
nest stack ghost <label>
nest stack resume <label>
nest stack prune
nest stack exit-all
```

## Required status rows

```text
phase1 nest stack
mode          : read-only status
nest-level    : <current>/<max>
root          : active
current       : level-<n>
ghost-count   : <count>
exit-all      : clear|requested
safe-mode     : visible
trust         : visible
guardrail     : no host process spawn | no network | no isolation claim
claim-boundary: control-plane-only
```

## User and control considerations

The feature must account for:

- first-time users who only know `nest` from the prompt line;
- keyboard-only command use;
- compact laptop/mobile terminal output;
- no-color output;
- ASCII fallback;
- low-vision readability through text labels;
- clear recovery cues;
- unknown-command guidance;
- safe-mode and trust boundaries;
- copy/paste-friendly one-line commands.

## Safety boundaries

Ghost stacks must not introduce or imply:

```text
host command execution
network access
hidden process spawning
unbounded recursion
live-system writes outside documented Phase1 state/control files
autonomous stack mutation
VM isolation
container isolation
physical machine separation
security boundary hardening
```

## Runtime rules

- Read-only status commands may run in safe mode.
- Commands must report current `PHASE1_NESTED_LEVEL` and `PHASE1_NESTED_MAX` when available.
- `exit-all` must stay visible as a requested or clear control state.
- Unknown stack actions must be no-op and help-guided.
- Labels must be sanitized before future mutating support.
- Future mutation must be explicit and auditable.

## Suggested fixture output

See:

```text
docs/nest/fixtures/ghost-stack-status-ok.txt
```

## Acceptance commands

```sh
cargo test -p phase1 --test nest_ghost_stack_contract
cargo test -p phase1 --test nest
cargo test --workspace --all-targets
```

## Completion gate

This design contract is not runtime implementation. Runtime status can advance only when source commands and tests land.
