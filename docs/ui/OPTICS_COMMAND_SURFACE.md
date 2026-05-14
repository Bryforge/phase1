# Optics Command Surface

Status: command surface contract
Scope: read-only Optics commands, WASI-lite preview routing, help/registry expectations, and non-claims for Optics PRO.

## Purpose

The Optics command surface defines how operators should discover and use Optics preview commands before live UI activation exists.

The current command surface is intentionally read-only.

## Current commands

The current preview commands are routed through the existing WASI-lite plugin path:

```text
optics preview
optics rails
```

The commands are available because `optics` is exposed as a WASI-lite plugin with capability `none`.

## Command behavior

`optics preview` should show the original Optics PRO static preview:

- minimal main screen;
- Phase1 edge enabled;
- PRO profile;
- bottom HUD preview;
- mutation labels;
- typing safety labels;
- fallback labels;
- non-claims.

`optics rails` should show the rail preview surface:

- top HUD rail;
- center viewport;
- bottom HUD rail;
- context, nest, portal, ghost, integrity, crypto, Base1, and Fyr summaries;
- read-only runtime status;
- non-claims.

## Help and registry expectation

Future work should promote Optics into the regular command registry so it appears in help, completions, and manuals.

The planned registry row should preserve this shape:

```text
optics [preview|rails|help]
```

Until that registry row exists, the WASI-lite route remains the safe preview path.

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
- claim a Base1 boot environment.

## Integration path

The command surface should progress in this order:

1. WASI-lite read-only preview.
2. Command surface documentation and tests.
3. Registry/help/manual visibility.
4. Renderer-backed shell preview command.
5. Explicitly gated live activation.

## Current status

This document preserves the command surface contract only.

Live Optics PRO HUD activation remains future work.
