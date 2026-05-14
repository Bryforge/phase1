# Optics PRO UI overhaul

Status: design contract
Scope: Phase1 PRO operator interface, top and bottom HUD rail model, input mutation visuals, and integration boundaries for Phase1, Base1, and Fyr.

## Codename

Optics is the codename for the PRO user-interface overhaul.

PRO is the operator-facing interface profile.

## Purpose

Optics PRO defines a minimal professional advanced-operator interface for Phase1.

The interface should start clean with Phase1 edge enabled and keep the main screen quiet until the operator issues a command.

The primary persistent visual elements should be top and bottom HUD rails that preserve the center command/output viewport.

## Visual intent

Optics PRO should feel like a professional hacker console and advanced operator workstation without visual noise.

The default screen should be minimal:

- no large boot card by default;
- no wide decorative banners by default;
- no constant full-screen animation by default;
- no command-output distortion;
- no unnecessary prompt clutter;
- only compact top and bottom HUD rails remain persistent.

## Starting state

The initial PRO start state should show Phase1 edge enabled.

The first visual state should communicate:

- Phase1 edge active;
- safe operator mode visible;
- current context visible;
- command input active;
- top HUD rail online;
- bottom HUD rail online;
- no broad security or hardening claim.

## HUD rails

The HUD rail screen real estate model is defined in [`OPTICS_HUD_RAILS.md`](OPTICS_HUD_RAILS.md).

Optics PRO should use:

- a top HUD rail for global system state, context, trust, integrity, crypto chain, Base1, Fyr, nest, portal, and ghost summaries;
- a center viewport for command output and detailed reports;
- a bottom HUD rail for command input, mutation state, command family, active task, guarded-operation warnings, and last result state.

The bottom HUD rail should be bright blue by default and text-first for accessibility.

The HUD should eventually show:

- active product: Phase1, Base1, or Fyr;
- active context: root, portal, nest, ghost, analysis workspace, recovery workspace, or Fyr package;
- edge/stable channel;
- safe mode / host tools status;
- current command family;
- input mutation state;
- process or task state when available;
- integrity status when the system integrity layer is available;
- crypto chain status when that surface exists;
- error or denial state in plain text.

## Mutation colors

Optics PRO may use mutation colors for active command activity.

Mutation colors are visual state cues, not claims.

Planned state examples:

| State | Meaning |
| --- | --- |
| bright blue | normal PRO HUD / ready state |
| cyan pulse | command is being typed |
| violet | command family detected |
| amber | guarded operation or confirmation needed |
| red | denied, failed, unsafe, or invalid |
| green | completed successfully |
| gray | inactive, disabled, or unavailable |

All color states must have text labels for no-color and accessibility modes.

## Typed-command visuals

Optics PRO should make the actively typed command feel alive without changing command meaning.

Allowed effects:

- cursor-area highlight;
- command-family label;
- token category labels;
- guarded-operation warning;
- inline validation hints;
- safe-mode or host-tool cue;
- no-color text fallback.

Disallowed effects:

- changing the command text before execution;
- hiding dangerous command text;
- adding hidden characters;
- writing different history than what the operator typed;
- changing parser behavior;
- affecting copied commands.

## Scheme integration

Optics PRO must integrate with the current Phase1, Base1, and Fyr schemes.

Phase1 integration:

- shell prompt and HUD rails share context state;
- portal, nest, ghost, analysis, and security contexts remain visible;
- safe mode and host-tool gates stay explicit;
- existing commands remain readable.

Base1 integration:

- Base1 state appears as readiness, recovery, validation, artifact, hardware, or evidence context;
- Base1 non-claims remain visible;
- recovery and validation surfaces remain read-only unless explicitly promoted by separate work;
- integrity status can show manifest validation when available.

Fyr integration:

- Fyr package, check, build, test, and run phases can light the HUD;
- Fyr diagnostics remain text-first;
- Fyr automation must not receive hidden UI-only state;
- Fyr command output remains deterministic in tests.

## Static preview fixture

The first static preview fixture is [`fixtures/optics-pro-preview.txt`](fixtures/optics-pro-preview.txt).

The fixture shows the intended minimal start state, bottom HUD rows, mutation-state labels, typing-safety boundaries, accessibility fallback labels, and non-claims.

The fixture is read-only design evidence. It is not runtime UI wiring.

## Accessibility and fallback rules

Optics PRO must preserve readable behavior when any of these are active:

```text
PHASE1_NO_COLOR=1
PHASE1_ASCII=1
PHASE1_TEST_MODE=1
PHASE1_COOKED_INPUT=1
```

Fallback must retain all state labels without relying on color.

## Implementation phases

### Phase 1: contract and tests

- Add this Optics PRO design contract.
- Add tests that preserve the UI boundaries and integration requirements.
- Do not change runtime UI yet.

### Phase 2: static PRO preview

- Add a read-only PRO preview command or fixture.
- Show bottom HUD rows and fallback rows.
- Keep output deterministic.

### Phase 3: HUD rails and renderer

- Add the [`OPTICS_HUD_RAILS.md`](OPTICS_HUD_RAILS.md) design contract.
- Add a small rendering adapter for top and bottom HUD rails.
- Keep existing prompt modes available.
- Gate activation behind explicit config or environment flags.

### Phase 4: typed-command state

- Add command-family detection for display only.
- Add mutation-state labels and color hints.
- Preserve raw command input and history.

### Phase 5: Phase1/Base1/Fyr integration

- Connect portal/nest/ghost/analysis context labels.
- Connect Base1 readiness and integrity labels.
- Connect Fyr package/check/build/test/run labels.
- Keep all integrations evidence-bound and testable.

## Non-claims

Optics PRO is not a new kernel, compositor, graphical desktop, terminal emulator, sandbox, security boundary, cryptographic enforcement layer, or Base1 boot environment.

It is a Phase1 user-interface profile and rendering model.

## Acceptance commands

```sh
cargo fmt --all -- --check
cargo test -p phase1 --test optics_pro_ui_plan_docs
cargo test -p phase1 --test optics_pro_preview_fixture_docs
cargo test -p phase1 --test optics_hud_rails_docs
```
