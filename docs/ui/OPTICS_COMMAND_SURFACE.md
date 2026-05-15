# Optics Command Surface

Status: implemented read-only command surface checkpoint
Scope: Optics PRO preview, rails, status, device previews, command discovery, and non-claims.

## Purpose

The Optics command surface defines how operators discover and inspect Optics PRO before live UI activation exists.

The current surface is complete for read-only preview work. It is not a live HUD, compositor, terminal emulator, sandbox, security boundary, crypto-enforcement layer, Base1 boot environment, or Phase movement engine.

## Current commands

```text
optics help
optics preview
optics status
optics rails
optics device mobile
optics device laptop
optics device desktop
optics device terminal
```

Registry aliases:

```text
pro
hudrails
```

The current `optics preview` and `optics help` routes display the complete read-only Optics command card. `optics status` displays the renderer/status contract. `optics rails` displays the current rail renderer. `optics device <profile>` displays the rail preview for a selected device profile.

## Current visibility

Optics now exposes the Phase universe prerequisites as visible status labels before live movement exists:

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

## Device profiles

Supported read-only device previews:

```text
mobile
laptop
desktop
terminal
```

## Discovery versus execution

Completion commands only print possible command names or aliases.

Examples:

```text
complete opt
complete pro
complete hud
```

Those are discovery checks. They do not execute the preview surface.

To execute Optics surfaces, run:

```text
optics help
optics preview
optics status
optics rails
optics device terminal
```

## Safety rules

Optics preview commands must not:

- activate live HUD rails;
- mutate shell state;
- change parser behavior;
- change command history;
- hide command output;
- claim a compositor;
- claim a terminal emulator;
- claim a sandbox;
- claim a security boundary;
- claim crypto enforcement;
- claim a system integrity guarantee;
- claim a Base1 boot environment;
- claim live Phase movement;
- claim origin mutation;
- claim safe-portal recovery execution;
- claim runtime domain mutation;
- claim host mutation;
- claim external effects.

## Completion checkpoint

The read-only Optics command surface is considered complete when these routes remain deterministic and tested:

```text
optics help
optics preview
optics status
optics rails
optics device mobile
optics device laptop
optics device desktop
optics device terminal
```

Focused validation:

```bash
cargo test -p phase1 --test optics_command_surface_complete
cargo test -p phase1 --test optics_hud_rail_renderer
cargo test -p phase1 --test optics_hud_rails_runtime_preview
```

## Future work

Future Optics work should move in this order:

1. Keep read-only command surface stable.
2. Add state fixtures for Phase path, breadcrumb, origin, safe portal, health, and lock states.
3. Add invariant tests for those fixtures.
4. Add explicit-gate experiments only after the status model is test-backed.
5. Do not promote live movement or recovery execution until evidence supports it.

Live Optics PRO HUD activation remains future work.