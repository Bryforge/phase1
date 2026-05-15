# Phase Navigation Contract

## Purpose

Phase Navigation is a planned Phase1 operator navigation track for moving between explicit Phase1-controlled contexts such as floors, portals, domains, and existing nest surfaces.

This contract starts the feature as documentation and command-contract evidence only. It does not add live runtime mutation yet.

## Scope

The initial scope is an internal navigation contract for Phase1 spaces:

- floors represent named or numbered operator spaces;
- domains represent bounded Phase1-controlled context groups;
- portals represent explicit links between approved Phase1 contexts;
- root remains the reversible base context;
- all movement is operator-commanded, visible, and auditable.

## Planned command surface

```text
phase status
phase floor create <num|name>
phase floor enter <num|name>
phase floor main <num|name>
phase portal link <from> <to>
phase domain establish <name>
phase back
phase root
```

## Required status fields

The first runtime status surface should preserve these fields when the command promotes beyond this contract:

```text
phase-navigation : planned
execution-state  : not-executed
active-floor     : root
active-domain    : none
active-portal    : none
path             : root
transition       : none
phase-safe       : on
host-tools       : gated
claim-boundary   : internal-context-navigation-only
```

## Optics A/B/C/D rail contract

Phase Navigation must be visible in Optics rails before any live runtime transition is promoted.

Target shape:

```text
A TOP RAIL     floor=<num|root> domain=<name|none> portal=<active|none> nest=<level/max>
B COMMAND      phase floor enter 2

C STATUS HUD   active-main=floor/2 transition=ok path=root>floor/2
D BOTTOM HUD   back=root phase-safe=on host-tools=gated
```

## Safety boundaries

This feature is internal Phase1 context navigation only.

It must not claim or imply:

- network lateral movement;
- host compromise;
- privilege escalation;
- hidden persistence;
- uncontrolled execution;
- sandbox hardening;
- malware safety;
- production isolation.

Every phase transition must be:

- explicit;
- visible;
- logged;
- reversible;
- bounded by current safe-mode and host-tool policy;
- compatible with existing nest and portal concepts;
- represented through text labels, not color or icons alone.

## Promotion ladder

```text
planned -> documented -> command-contract tested -> runtime stub -> local-state runtime -> Optics-integrated runtime -> reviewed -> release eligible
```

No runtime command should promote until shell state, nested context restoration, rollback behavior, and Optics rail visibility are clear.

## First implementation rule

The first PR for this track is documentation-only. It may add tests that preserve this contract, but it must not create live `phase` runtime behavior yet.
