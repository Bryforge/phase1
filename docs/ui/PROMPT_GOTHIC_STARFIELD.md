# Prompt gothic and starfield visual mode

Status: design contract  
Scope: prompt-area visual treatment for `black_arts` and portal contexts only.  
Non-claim: this is not a terminal-wide font switch, graphical compositor, animated background engine, or readability change for command output.

## Goal

Add an optional Phase1 visual mode where special operator contexts can make the prompt feel more magical without making the shell harder to read.

The visual treatment is prompt-only:

```text
✦ 𝔭𝔥𝔞𝔰𝔢1://black_arts ~ ❯
✧ 𝔭𝔥𝔞𝔰𝔢1://portal/alpha ~ ❯
```

Everything outside the prompt remains the normal readable Phase1 output.

## Activation

The first implementation must be opt-in and environment-gated:

```text
PHASE1_PROMPT_GOTHIC=1
PHASE1_STARFIELD=1
```

`PHASE1_PROMPT_GOTHIC=1` enables the gothic prompt wordmark.  
`PHASE1_STARFIELD=1` enables subtle prompt-area starlight glyphs.

The mode may activate contextually when the current prompt context is `black_arts` or a portal path, but it must not affect general command output, boot cards, help text, tests, docs, or logs.

## Prompt-only gothic rule

Terminal fonts are global in most environments, so Phase1 must not claim or attempt a real terminal font switch.

The gothic prompt is a Unicode text skin for the prompt label only. For example:

```text
phase1 -> 𝔭𝔥𝔞𝔰𝔢1
```

The path, command output, status rows, diagnostics, logs, and help text remain plain readable text.

## Starfield rule

Starfield mode is ambient prompt decoration only. It should use one short deterministic row or inline prefix, not a full scrolling background.

Allowed examples:

```text
· ✦ ·  ✧   · ✦
𝔭𝔥𝔞𝔰𝔢1://portal/alpha ~ ❯
```

```text
✦ 𝔭𝔥𝔞𝔰𝔢1://black_arts ~ ❯
```

Disallowed examples:

```text
full-screen animated background
random output before every command result
wide visual noise that breaks mobile terminals
stars inside diagnostics, logs, or copied command text
```

## Required fallbacks

The feature must fall back to the existing readable prompt when any of these are set:

```text
PHASE1_NO_COLOR=1
PHASE1_ASCII=1
PHASE1_TEST_MODE=1
PHASE1_COOKED_INPUT=1
```

Fallback example:

```text
phase1://black_arts ~ >
phase1://portal/alpha ~ >
```

## Safety and usability boundaries

- Prompt visuals must be decorative only.
- The feature must not change parser behavior.
- The feature must not affect command history contents.
- The feature must not alter copied command input.
- The feature must not add network, host, process, VM, container, or hardware claims.
- The feature must preserve low-vision readability by keeping all state names text-first.
- The feature must preserve mobile readability with a short prompt and no wide star rows.
- The feature must be deterministic or disabled in tests.

## First implementation slice

1. Add this contract and tests.
2. Add a small prompt-skin helper near the prompt renderer.
3. Gate gothic and starfield rendering by environment variables.
4. Disable the visual skin for no-color, ASCII, test, and cooked-input modes.
5. Add runtime smoke tests that prove normal output remains readable.

## Acceptance commands

```sh
cargo fmt --all -- --check
cargo test -p phase1 --test prompt_gothic_starfield_contract
cargo test --workspace --all-targets
```
