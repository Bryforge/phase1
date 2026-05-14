# Optics Registry Promotion

Status: initial registry promotion
Scope: moving Optics from WASI-lite preview-only routing into Phase1 command registry, help, completion, and manual visibility.

## Purpose

Optics currently works through the read-only WASI-lite plugin route.

The initial registry promotion makes Optics discoverable from the normal Phase1 command registry without changing live UI behavior.

## Current safe routes

```text
optics preview
optics rails
optics status
```

These routes remain preview-only and run through the existing `optics` WASI-lite plugin with capability `none`.

## Registry shape

The initial registry entry uses this command shape:

```text
optics [preview|rails|status|help]
```

Properties:

- name: `optics`
- aliases: `pro`, `hudrails`
- category: `user`
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

The command map makes the Optics surface discoverable without implying live UI activation.

## Runtime boundary

Registry promotion does not activate live HUD rails.

The command keeps routing to the existing read-only preview behavior until a separate renderer-backed shell command is explicitly added and tested.

## Status surface

`optics status` reports the current preview state:

- preview-only mode;
- Rust static renderer source;
- top rail preview readiness;
- bottom rail preview readiness;
- live HUD disabled state;
- explicit activation gate requirement;
- input, history, and parser non-mutation labels;
- non-claims.

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

The initial implementation patch adds tests preserving:

- `lookup("optics")` resolves to `optics`;
- `canonical_name("pro")` resolves to `optics`;
- `canonical_name("hudrails")` resolves to `optics`;
- `completions("opt")` contains `optics`;
- `completions("pro")` contains `pro`;
- `man_page("optics")` contains `optics [preview|rails|status|help]`;
- `capabilities_report()` lists `optics` with capability `none`.

## Current status

Optics is now initially promoted into the command registry for discovery.

Live Optics PRO HUD activation remains future work.
