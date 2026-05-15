# Optics Rail Configuration

Status: configuration contract
Scope: Optics PRO active shell rails, command/status gap allocation, color safety, and fallback controls.

## Purpose

Optics PRO uses a rail-style shell frame to keep the operator oriented while preserving the center command area as the primary workspace.

The shell frame has four layers:

```text
A TOP RAIL
B COMMAND RAIL

C STATUS HUD
D BOTTOM HUD
```

The blank line between B and C is intentional and may grow when the operator needs more writing room.

## Default behavior

By default, Phase1 should start the active shell using Optics PRO rails.

The old shell prompt remains available through an explicit legacy escape hatch only.

## Configuration variables

```text
PHASE1_OPTICS_PRO=1
```

Enables the Optics PRO active shell frame. This is the default when the variable is not set.

```text
PHASE1_OPTICS_PRO=0
```

Disables the Optics PRO active shell frame.

```text
PHASE1_LEGACY_SHELL_UI=1
```

Forces the legacy shell prompt. This must override the default Optics PRO shell frame.

```text
PHASE1_OPTICS_COMMAND_GAP_LINES=<number>
```

Controls the number of blank lines between the command rail and the status HUD.

The supported range is:

```text
minimum: 1
maximum: 8
```

Values below the minimum clamp to 1. Values above the maximum clamp to 8.

## Layer responsibilities

### A TOP RAIL

The top rail shows stable context:

- product;
- channel;
- profile;
- root/nest/portal/ghost context;
- trust state;
- security mode.

### B COMMAND RAIL

The command rail is the active typing area.

Only typed or pasted operator input may use bright yellow.

No other label, status, warning, or rail title may use bright yellow.

### C STATUS HUD

The status HUD shows live interpretation and safety context:

- result state;
- mutation state;
- integrity state;
- crypto chain state;
- Base1 state;
- Fyr state.

### D BOTTOM HUD

The bottom HUD shows persistent operator cues:

- input state;
- command family;
- active task;
- warning state;
- copy/paste safety rule.

## Color policy

The color policy exists for readability, not decoration.

- A TOP RAIL: cyan
- B COMMAND RAIL label: blue
- typed/pasted operator input: bright yellow only
- C STATUS HUD: green
- D BOTTOM HUD: magenta
- denied/failed/unsafe states: red

Bright yellow is reserved for operator input only.

## Dynamic spacing policy

The B-to-C gap exists to make typing, pasted commands, code fragments, and future multi-line editing easier to read.

Current configuration is environment-driven.

Future live controls may use a key chord such as Shift+Enter or another terminal-supported equivalent to grow the B-to-C gap interactively.

## Non-claims

Optics rail configuration does not claim:

- compositor behavior;
- terminal emulator behavior;
- sandboxing;
- security boundary enforcement;
- crypto enforcement;
- system integrity guarantees;
- Base1 boot environment completion.

## Current status

This document preserves the Optics rail configuration contract.

Live interactive gap resizing remains future work.
