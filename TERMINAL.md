# Phase1 Terminal

Phase1 now has two safe local launch surfaces:

```bash
sh phase1
./phase1
```

The root `phase1` command is the simplest operator entrypoint. It delegates to `./start_phase1`, so Phase1 keeps one source launcher for trust, build, Gina, Base1, and quality logic.

The lower-level terminal wrapper remains available for explicit terminal workflows:

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

## Recommended startup

Fresh clone:

```bash
git clone https://github.com/Bryforge/phase1.git
cd phase1
sh phase1
```

After executable bits are set:

```bash
./phase1
```

Install a local command on macOS/Linux:

```bash
sh scripts/install-phase1-command.sh
phase1
```

Useful checks:

```bash
sh phase1 version
sh phase1 doctor
sh phase1 selftest
```

The original launcher remains valid:

```bash
./start_phase1
```

## Safety model

Defaults stay conservative:

- `PHASE1_SAFE_MODE=1`
- `PHASE1_ALLOW_HOST_TOOLS=0` for the explicit `safe` command
- no host network mutation
- no external AI provider call
- no full terminal-emulator claim

The simple launcher and terminal wrapper do not create new privilege paths. They only make the already-merged `./start_phase1` flow easier to discover from Linux, macOS, and local terminal sessions.

## Gina workflow

```bash
sh phase1 gina
```

This delegates to:

```bash
./start_phase1 --gina
```

Gina remains offline by default and runs through the existing Phase1 WASI-lite plugin path.

## Quality workflow

```bash
sh phase1 selftest
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

1. Simple command and tests.
2. Linux/macOS install helpers.
3. Optional `.desktop` and `.command` launchers.
4. Theme/profile helpers.
5. Native terminal application exploration only if it adds clear value without weakening Phase1 safety guarantees.
