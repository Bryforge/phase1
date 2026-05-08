# Phase1 Terminal

Phase1 Terminal is the current-master terminal entrypoint wrapper for Phase1.

It is intentionally thin. The canonical launch path remains:

```bash
./start_phase1
```

`phase1-terminal` delegates to that launcher instead of duplicating trust, build, Gina, Base1, or quality logic.

## Commands

```bash
terminal/bin/phase1-terminal help
terminal/bin/phase1-terminal doctor
terminal/bin/phase1-terminal run
terminal/bin/phase1-terminal safe
terminal/bin/phase1-terminal gina
terminal/bin/phase1-terminal quality
terminal/bin/phase1-terminal base1
terminal/bin/phase1-terminal version
terminal/bin/phase1-terminal selftest
```

## Safety model

Defaults stay conservative:

- `PHASE1_SAFE_MODE=1`
- `PHASE1_ALLOW_HOST_TOOLS=0` for the explicit `safe` command
- no host network mutation
- no external AI provider call
- no full terminal-emulator claim

The wrapper does not create a new privilege path. It only makes the already-merged `./start_phase1` flow easier to discover from Linux, macOS, and local terminal sessions.

## Gina workflow

```bash
terminal/bin/phase1-terminal gina
```

This delegates to:

```bash
./start_phase1 --gina
```

Gina remains offline by default and runs through the existing Phase1 WASI-lite plugin path.

## Quality workflow

```bash
terminal/bin/phase1-terminal selftest
sh scripts/test-phase1-terminal.sh
```

Run the full project gate before release work:

```bash
sh scripts/quality-check.sh quick
cargo test --workspace --all-targets
```

## Future work

Keep future terminal expansion staged:

1. Wrapper and tests.
2. Linux/macOS install helpers.
3. Optional `.desktop` and `.command` launchers.
4. Theme/profile helpers.
5. Native terminal application exploration only if it adds clear value without weakening Phase1 safety guarantees.
