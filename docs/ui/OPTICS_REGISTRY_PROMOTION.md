# Optics Registry Promotion

Status: promotion plan
Scope: moving Optics from WASI-lite preview-only routing into Phase1 command registry, help, completion, and manual visibility.

## Purpose

Optics currently works through the read-only WASI-lite plugin route.

The next promotion step is to make Optics discoverable from the normal Phase1 command registry without changing live UI behavior.

## Current safe routes

```text
optics preview
optics rails
```

These routes remain preview-only and run through the existing `optics` WASI-lite plugin with capability `none`.

## Target registry shape

The future registry entry should use this command shape:

```text
optics [preview|rails|help]
```

Expected properties:

- name: `optics`
- aliases: `pro`, `hudrails`
- category: `user` or `misc`
- capability: `none`
- description: read-only Optics PRO preview and HUD rail preview surface

## Required discovery behavior

After promotion, these should work:

```text
help optics
man optics
complete opt
complete pro
complete hud
capabilities
```

The command map should make the Optics surface discoverable without implying live UI activation.

## Runtime boundary

Registry promotion must not activate live HUD rails.

The command should keep routing to the existing read-only preview behavior until a separate renderer-backed shell command is explicitly added and tested.

## Safety rules

Registry promotion must not:

- enable live HUD rails;
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

## Acceptance target

A future implementation patch should add tests preserving:

- `lookup("optics")` resolves to `optics`;
- `canonical_name("pro")` resolves to `optics`;
- `canonical_name("hudrails")` resolves to `optics`;
- `completions("opt")` contains `optics`;
- `completions("pro")` contains `pro`;
- `man_page("optics")` contains `optics [preview|rails|help]`;
- `capabilities_report()` lists `optics` with capability `none`.

## Current status

This document defines the registry-promotion contract only.

Code-level registry promotion remains future work.
