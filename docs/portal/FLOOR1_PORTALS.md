# Phase1 floor1 portals

Issue: #336  
Status: design contract  
Scope: operator portals, door-style workspace contexts, and floor1-owned policy  
Non-claim: portals are not claimed to be VMs, containers, hardened sandboxes, separate physical machines, or independent network namespaces.

## Purpose

Phase1 portals let an operator open multiple named doors into separated work areas while keeping one root control floor.

The model is:

```text
floor1
  portal/root
  portal/<name>
  split/<view>
```

`floor1` is the root control floor. It owns recovery, policy, portal registry, shared resources, network decisions, and audit. Portals are operator workspace/session contexts above `floor1`. Splits are views into portals, not separate machines.

## Terms

| Term | Meaning |
| --- | --- |
| floor1 | Root Phase1 control floor and policy owner. |
| portal | Named workspace/session context controlled by an operator. |
| door | Operator-facing metaphor for opening a portal. |
| split | A local view into a portal. A split is not a new machine. |
| local-link | Planned portal-to-portal local message/control channel. Disabled in the first slice. |
| brokered-egress | Future mode where floor1 performs explicit allowlisted external requests for a portal. Disabled in the first slice. |

## Command surface

```text
portal
portal status
portal list
portal open <name>
portal enter <name>
portal leave
portal close <name>
portal inspect <name>
portal network <name> <denied|local-only|brokered-egress>
portal split <left> <right>
portal snapshot <name>
portal restore <name>
portal clone <source> <name>
portal help

Planned later:
```

Possible future aliases:

```text
door
portals
doors
```

The canonical command is `portal`.

## First implementation slice

Start read-only:

```text
portal
portal status
portal list
portal help
```

The first slice must report the portal model, network policy, and non-claims. It must not mutate portal state.

## Required status rows

```text
phase1 portals
mode              : read-only status
floor             : floor1
active-portal     : root
open-portals      : root
portal-count      : 1
portal-layer      : workspace/session
split-mode        : local-view
local-link        : planned-disabled
network-owner     : floor1
network-mode      : denied
network-default   : denied
brokered-egress   : planned-disabled
vfs-scope         : portal-context
history-scope     : portal-context
log-scope         : labelled
host-isolation    : not-claimed
process-isolation : not-claimed
network-isolation : not-claimed
network           : blocked
claim-boundary    : workspace-context-only
```

## Network policy model

Portals may eventually request network capability, but `floor1` owns the decision.

Initial modes:

```text
denied
local-only
brokered-egress
```

### denied

Default mode. The portal has no network access.

### local-only

Future mode. A portal may communicate only through a Phase1 local portal bus. This must not silently open host sockets or host network paths.

### brokered-egress

Future mode. `floor1` performs explicit allowlisted external requests for a portal. Requests must be operator-approved, audited, and redacted.

## Rules

- Network mode defaults to `denied`.
- Portal status must show the network owner.
- Portals must not silently inherit host network access.
- future network access must be brokered through `floor1`.
- Brokered access requires allowlists, audit rows, and redaction tests.
- Portals cannot claim network isolation until separately implemented and tested.

## User and control considerations

The feature must support:

- first-time users who understand doors and portals better than sessions;
- keyboard-only operation;
- mobile and compact terminal output;
- direct command entry;
- help-first discovery;
- tab-completion expectations;
- copy/paste-safe commands;
- no-color and ASCII fallback;
- low-vision readability through text labels;
- clear recovery cues;
- unknown-command recovery;
- safe-mode and trust boundaries.

## Safety boundaries

The first implementation must not introduce or imply:

```text
hidden host process spawning
host shell execution
network access by default
autonomous portal creation
implicit host directory access
VM isolation
container isolation
security isolation
network namespace isolation
hardware separation
live-system writes outside documented portal state
```

## Runtime staging

1. Add this contract and a fixture.
2. Add read-only runtime status/list/help.
3. Add runtime tests for status/list/help and unknown-action no-op output.
4. Add `portal open`, `portal enter`, and `portal leave` as explicit local state changes.
5. Add `portal inspect` with isolation and policy rows.
6. Add `portal split <portal>` as a local view concept.
7. Add local-link design and tests before implementation.
8. Add brokered-egress design, allowlist, audit, and redaction tests before implementation.

## Acceptance commands

```sh
cargo fmt --all -- --check
cargo test -p phase1 --test portal_floor1_contract
cargo test --workspace --all-targets
```

## Completion rule

Do not close #336 until the contract, fixture, read-only runtime source, runtime tests, and manual smoke have landed. Do not claim stronger isolation until implementation and tests exist.
