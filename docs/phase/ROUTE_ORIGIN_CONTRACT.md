# Phase Route and Origin Contract

## Purpose

This contract defines the first prerequisite for the Phase universe model: compact route labels, the `0/0` origin marker, and safe origin return / origin planting behavior.

This is documentation and command-contract evidence only. It does not implement live Phase movement, destructive state changes, host mutation, or external effects.

## Route model

`ROOT` is the center anchor.

`0/0` is the distinguished origin marker.

Directional routes use compact labels:

```text
              u/NUM
                ^
                |
L/NUM  <----  ROOT  ---->  R/NUM
                |
                v
              d/NUM
```

Meaning:

```text
u/NUM upper route
d/NUM lower route
L/NUM left route
R/NUM right route
0/0   origin marker
ROOT  center anchor and recovery reference
```

## Canonical path examples

```text
ROOT
0/0
ROOT>0/0
ROOT>u/1
ROOT>d/1
ROOT>L/2
ROOT>R/3
ROOT>R/3>u/1
ROOT>L/2>d/1
```

## Origin behavior

The origin marker is the stable reference point for the current Phase universe scope.

Required behavior:

- `0/0` identifies the current origin;
- `ROOT` remains the center anchor even if a new origin is planted;
- returning to origin must be explicit;
- planting a new origin must be explicit;
- previous origins must remain traceable;
- rollback target must be recorded before origin changes.

## Planned command vocabulary

Return to origin:

```text
phase origin
phase back-origin
phase return 0/0
phase move 0/0
```

Plant a new origin:

```text
phase origin plant
phase origin plant <name>
phase plant-origin
phase set-origin 0/0
```

Display route/origin state:

```text
phase route status
phase origin status
phase whereami
phase compass
phase path
```

## Required origin status fields

A future status surface should preserve these fields:

```text
root-anchor      : ROOT
origin           : 0/0
origin-name      : <name|none>
current-route    : <route|0/0>
current-axis     : u|d|L|R|ROOT
path             : ROOT>R/3>u/1
previous-origin  : <origin-id|none>
rollback-target  : <route|0/0|ROOT>
trace-id         : <trace-id>
operator-intent  : explicit
safe-mode        : enforced
claim-boundary   : internal-phase-universe-only
```

## Plant-origin safeguards

Planting a new origin must require:

- explicit operator intent;
- current route visibility;
- trace-id creation;
- previous-origin preservation;
- rollback-target recording;
- safe portal readiness check where available;
- Optics visibility before runtime promotion.

Required planning output:

```text
action           : plant-origin
current-route    : <route>
previous-origin  : <origin-id>
new-origin       : 0/0
rollback-target  : <previous-origin-id>
operator-intent  : confirmation-required
mutation         : planned-only
```

## Back-to-origin safeguards

Returning to origin must preserve the route that was left.

Required planning output:

```text
action           : return-origin
origin           : 0/0
from-route       : <route>
to-route         : 0/0
rollback-target  : <from-route>
transition       : reversible
```

## Optics PRO contract

Optics should render the compact route map and origin state in text labels:

```text
ROOT DIRECTION MAP
layout=center-root u-d-L-R
active-route=R/0
              u/NUM
                ^
                |
L/NUM  <----  ROOT  ---->  R/NUM
                |
                v
              d/NUM
origin=0/0
rule=root-remains-anchor
```

Optics should later expose origin state in rails:

```text
A TOP RAIL     origin=0/0 route=R/3 axis=R
C STATUS HUD   path=ROOT>R/3 rollback=0/0 trace=<id>
D BOTTOM HUD   origin-return=ready root-anchor=preserved
```

## Hard boundaries

This contract must not imply:

- real network movement;
- host compromise;
- privilege escalation;
- credential collection;
- uncontrolled execution;
- destructive runtime state;
- bypass of safe-mode or audit;
- external side effects.

Initial work remains contract-only until runtime state, rollback behavior, trace storage, safe portal readiness, and Optics visibility are tested.
