# Phase1 Launch Experience

Phase1 now has a simple integrated launch command:

```bash
./phase1
```

If your checkout does not preserve executable bits, use:

```bash
sh phase1
```

For first-time setup:

```bash
sh scripts/configure-phase1.sh
./phase1
```

## What the launcher configures

The launch experience is intentionally simple and cybersecurity-conscious. It configures:

- Phase1 home path
- local `.phase1/` config directory
- safe-mode defaults
- cyber visual theme defaults
- Gina offline operations assistant
- Base1 preflight hook
- quality-system hook
- local terminal wrapper at `.phase1/bin/phase1-terminal`

## Commands

```bash
./phase1              # Phase1 operator launch
./phase1 --configure  # run full local configuration first
./phase1 --doctor     # show launch readiness
./phase1 --base1      # run Base1 preflight before launch
./phase1 --quality    # run lightweight quality checks before launch
./phase1 --gina       # launch with Gina command intent
./phase1 --no-build   # require an existing built binary
```

## Gina operations

Gina is configured as an offline Phase1 operations assistant. Inside Phase1, use:

```text
gina
ai gina
wasm inspect gina
wasm run gina status
```

Gina's baseline is local and deterministic. It is designed to guide secure operations, Base1 readiness, quality checks, update workflow, terminal setup, and Phase1 system usage without enabling external providers by default.

## Base1

Base1 support is included through the existing read-only preflight script:

```bash
./phase1 --base1
```

or:

```bash
sh scripts/base1-preflight.sh
```

## Quality

Run lightweight quality readiness before launch:

```bash
./phase1 --quality
```

or directly:

```bash
sh scripts/quality-score.sh
sh scripts/quality-check.sh quick
```

## Test the launch system

```bash
sh scripts/test-phase1-launch.sh
cargo test --test phase1_launch
```

## Safety notes

The launch experience keeps safe defaults. Gina is offline by default. Base1 preflight is read-only. The configuration script supports `--dry-run` so the planned setup can be inspected before writing files.
