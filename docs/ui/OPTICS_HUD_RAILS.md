# Optics HUD Rails

Status: design contract
Scope: PRO screen real estate model, top HUD rail, bottom HUD rail, command viewport, and feature-density rules for Phase1, Base1, Fyr, nests, portals, ghosts, crypto chains, and integrity status.

## Purpose

Optics HUD Rails define how the PRO interface should use screen space without returning to a large boot-card or banner-heavy layout.

The goal is more live information per screen while preserving a clean professional operator surface.

## Core decision

Optics PRO should use two persistent HUD rails:

- a top HUD rail for global system state;
- a bottom HUD rail for command input, mutation state, and active operation feedback.

The center of the screen should remain the command/output viewport.

## Static rail preview fixture

The first static rail preview fixture is [`fixtures/optics-hud-rails-preview.txt`](fixtures/optics-hud-rails-preview.txt).

The fixture shows the intended top rail, center viewport, bottom rail, feature groups, device rules, accessibility labels, and non-claims.

The fixture is read-only design evidence. It is not runtime UI wiring.

## Top HUD rail

The top rail should carry stable context that operators need at all times.

Planned top rail fields:

- product: Phase1, Base1, or Fyr;
- channel: edge or stable;
- profile: PRO;
- context: root, nest, portal, ghost, analysis, recovery, Base1, or Fyr package;
- trust: safe, armed, denied, or host-gated;
- integrity: ok, changed, missing, not-checked, or planned;
- crypto chain: active, planned, denied, or not-checked;
- device: mobile, laptop, desktop, or terminal;
- evidence level when Base1 validation is in view.

The top rail should be compact, one or two text rows, and never consume the primary workspace.

## Bottom HUD rail

The bottom rail should carry live input and command activity.

Planned bottom rail fields:

- raw command input;
- command family;
- mutation state;
- guarded-operation warning;
- active task state;
- last result state;
- copy-safe command echo;
- no-color fallback labels.

The bottom rail is where bright-blue PRO HUD styling should be strongest.

## Center viewport

The center viewport is reserved for command output, reports, diagnostics, analysis, Fyr output, portal output, nest output, Base1 evidence, and integrity reports.

The center viewport should not be crowded with permanent chrome.

Persistent information belongs in the top or bottom rail unless it is directly part of command output.

## Feature density model

Optics PRO needs to surface many Phase1 features without flooding the screen.

Feature state should be grouped by family:

| Family | Examples | Rail placement |
| --- | --- | --- |
| context | root, nest, portal, ghost, analysis, recovery | top rail |
| security | safe mode, host gate, trust state, denial state | top and bottom rail |
| integrity | manifest status, file status, evidence status | top rail summary, center report detail |
| crypto | crypto profile, chain status, provider/service status | top rail summary, center report detail |
| Base1 | hardware, recovery, validation, artifact evidence | top rail summary, center report detail |
| Fyr | package, check, build, test, run, automation | top rail summary, bottom active task |
| command | typed command, command family, mutation color | bottom rail |
| result | ok, changed, denied, failed, warning | bottom rail and center detail |

## Nests, portals, and ghosts

Nests, portals, and ghosts should appear as context indicators, not decorative panels.

Examples:

```text
TOP  Phase1 edge | PRO | ctx=root > nest:0/1 | portal:none | ghost:none | trust=safe
BOT  ready | input=active | mutation=none | command=none
```

When a portal, nest, or ghost becomes active, the top rail should update the context path.

The bottom rail should only change when the operator is typing, executing, or receiving a result.

## Crypto chains and integrity

Crypto chain and integrity indicators should be compact by default.

Examples:

```text
crypto=chain-planned integrity=not-checked
crypto=denied integrity=changed
crypto=active integrity=ok
```

A rail indicator must not imply runtime enforcement unless the backing implementation and tests exist.

Detailed crypto or integrity output belongs in the center viewport.

## Mobile, laptop, and desktop behavior

Optics HUD Rails must adapt to device profile.

- mobile: use one-line top rail and one-line bottom rail;
- laptop: use one compact top rail and one or two bottom rows;
- desktop: allow a two-row top rail and two-row bottom rail;
- ASCII/no-color: preserve all labels without color.

The center viewport remains the largest area on all device profiles.

## Color model

The default rail color is bright blue.

Mutation colors are allowed only as visible state cues:

- cyan for typing;
- violet for command-family recognition;
- amber for guarded or confirmation-needed operations;
- red for denied, failed, unsafe, invalid, or changed integrity state;
- green for successful completion;
- gray for inactive, disabled, unavailable, or not checked.

Every color cue must also have a text label.

## Safety rules

HUD rails must not:

- hide command output;
- rewrite command input;
- change parser behavior;
- change command history;
- hide dangerous commands;
- imply security hardening by visual style;
- imply crypto enforcement from a planned crypto chain;
- imply total system integrity from a hash check;
- imply Base1 boot readiness without evidence.

## Runtime integration phases

### Phase 1: HUD rail contract

- Add this document.
- Add tests for screen real estate, rail responsibilities, feature grouping, and non-claims.

### Phase 2: static rail preview

- Add a static top/bottom rail fixture.
- Keep output deterministic and read-only.

### Phase 3: renderer module

- Add a small renderer that can produce no-color, ASCII-safe top and bottom rail strings.
- Keep it test-only or explicitly gated.

### Phase 4: shell preview command

- Add a preview command that prints the rail layout without changing the active shell UI.

### Phase 5: live activation gate

- Gate live PRO rail activation behind explicit config or environment flags.
- Keep default behavior unchanged until the rail renderer has coverage across Phase1, Base1, and Fyr flows.

## Non-claims

Optics HUD Rails are not a compositor, terminal emulator, sandbox, security boundary, cryptographic enforcement layer, system integrity guarantee, or Base1 boot environment.

They are a screen real estate and rendering contract for the Optics PRO interface.
