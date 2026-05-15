# Trilateral Phase Movement

Status: design contract and fixture-first implementation guide
Scope: Phase universe movement model, safety planes, Optics visibility, tests, and non-claims.

## Purpose

Trilateral Phase Movement is the planned movement model for the Phase universe.

It exists to make Phase1 movement real without making it reckless. Movement must eventually represent actual state transitions, but it must begin as pure state and proof before live execution exists.

## Definition

A Phase movement is trilateral when it is checked across three planes:

| Plane | Meaning | Example fields |
| --- | --- | --- |
| Spatial plane | Where the operator is moving. | `origin`, `route`, `axis`, `path` |
| State plane | How the movement is remembered and inspected. | `breadcrumb`, `trace`, `context`, `selected-domain` |
| Safety plane | Whether the movement is allowed and effect-free. | `safe-portal`, `rollback`, `health`, `risk`, `lock`, `dark_phase`, `host-effect`, `external-effect` |

A transition is accepted only when all three planes produce a consistent safe result.

## Initial state fixture

The first implementation should preserve this default shape:

```text
origin=0/0
route=ROOT
axis=ROOT
path=ROOT>0/0
breadcrumb=ROOT
trace=trace-preview
safe-portal=planned
rollback=available
health=nominal
risk=low
lock=open
dark_phase=off
host-effect=none
external-effect=none
```

This state is not live movement. It is the seed fixture for tests, Optics render output, and future transition modeling.

## Movement vocabulary

Initial movement directions should be abstract and device-independent:

```text
up
down
left
right
enter
back
root
```

Recommended meanings:

| Direction | Initial interpretation |
| --- | --- |
| `up` | Move toward parent, higher domain, or overview. |
| `down` | Move toward child, deeper domain, or selected detail. |
| `left` | Move to prior sibling, previous route, or previous axis. |
| `right` | Move to next sibling, next route, or next axis. |
| `enter` | Focus selected domain without host or external effect. |
| `back` | Return to previous breadcrumb if rollback is available. |
| `root` | Return to `origin=0/0` and `route=ROOT` in preview state. |

These meanings are intentionally broad until fixtures and route tables exist.

## Transition contract

The first transition engine should be pure.

Input:

```text
current_phase_state
movement_direction
operator_intent
```

Output:

```text
next_phase_state
transition_result
reason
```

The pure transition engine must not:

- mutate shell state;
- mutate origin outside the returned next-state value;
- execute recovery;
- touch the host filesystem;
- touch host networking;
- launch a process;
- hide command output;
- claim security enforcement;
- claim Base1 boot readiness.

## Safety rules

A movement must be denied when:

- `lock` is closed;
- `risk` is high and the movement is not read-only;
- `safe-portal` is blocked and the movement depends on recovery state;
- `rollback` is unavailable and the movement requires reversible state;
- `dark_phase` is observed, isolated, or blocked and no explicit safe action exists;
- the movement would produce any host effect before explicit gating;
- the movement would produce any external effect before explicit gating.

Every denied movement should return a visible reason.

## Optics relationship

Optics PRO is the operator lens for Trilateral Phase Movement.

Optics should show:

- current origin;
- current route;
- current axis;
- current path;
- breadcrumb;
- trace id;
- safe portal state;
- rollback state;
- health;
- risk;
- lock;
- dark phase state;
- host effect;
- external effect;
- last movement result.

Optics should not activate live movement in the first implementation.

## Fyr relationship

Fyr should eventually inspect and script Phase state, but early Fyr integration should remain read-only.

Safe early commands or examples:

```text
fyr phase inspect
fyr phase assert-root
fyr phase assert-no-host-effect
fyr phase assert-rollback-visible
```

Fyr must not promote staged candidates, mutate live Phase state, or trigger Base1 recovery from this model until explicit approval gates and tests exist.

## Base1 relationship

Base1 is where Phase movement can eventually become hardware-adjacent.

Before any Base1 movement is real, the model needs:

- VM evidence;
- read-only hardware evidence;
- recovery evidence;
- rollback evidence;
- operator confirmation flow;
- post-run validation.

Trilateral movement cannot claim Base1 boot or recovery readiness by itself.

## Test plan

Initial tests should cover:

```text
phase_state_default_fixture_has_required_labels
phase_state_default_fixture_has_no_effects
trilateral_move_preview_accepts_safe_route
trilateral_move_preview_denies_closed_lock
trilateral_move_preview_denies_missing_rollback_when_required
trilateral_move_preview_preserves_trace_or_records_new_trace
trilateral_move_preview_never_sets_host_effect_before_gate
trilateral_move_preview_never_sets_external_effect_before_gate
optics_renders_phase_state_labels
```

## First command surface

The first operator command should be preview-only:

```text
phase move preview <direction>
```

Possible output:

```text
PHASE MOVE PREVIEW
from.path=ROOT>0/0
direction=right
result=allowed-preview
next.path=ROOT>0/1
host-effect=none
external-effect=none
recovery-executed=no
origin-mutated=no
```

This command should not perform live movement.

## Promotion ladder

```text
D0 concept documented
D1 default fixture added
D2 invariant tests added
D3 Optics renders fixture
D4 pure transition model added
D5 preview command added
D6 denied movement reasons added
D7 Fyr read-only inspection added
D8 VM evidence added
D9 real-device read-only evidence added
D10 explicit-gated live experiment considered
```

## Non-claims

Trilateral Phase Movement does not currently provide live Phase movement, origin mutation, recovery execution, runtime domain mutation, host mutation, external effects, sandboxing, crypto enforcement, system integrity guarantees, or Base1 boot readiness.

This document is a development contract for moving toward those ideas safely and truthfully.
