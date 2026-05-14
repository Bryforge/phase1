# Gina AI for Phase1

Gina is Phase1's offline operations assistant baseline.

Current-master Gina is intentionally conservative:

- offline by default
- deterministic WASI-lite plugin output
- no host shell access
- no host network access
- no host filesystem passthrough
- no external provider calls
- no token, API key, cookie, password, or recovery-code storage

The current command surface is plugin-driven and works through the existing Phase1 WASI-lite runtime:

```text
gina
assistant
ai
wasm inspect gina
wasm run gina status
wasm inspect assistant
wasm run assistant status
wasm inspect ai
wasm run ai gina
```

The `assistant` plugin is a compatibility alias for Gina. It keeps user-facing language natural without adding a separate trust path.

## Launch workflow

The operator launch path remains:

```bash
./phase1 --gina
```

The terminal wrapper delegates to the same flow:

```bash
terminal/bin/phase1-terminal gina
```

## Security contract

Gina can explain Phase1 operations, launch posture, Base1 preflight, quality checks, update workflow, and terminal setup. Gina must not request credentials or connect to an external model provider unless a future, separately reviewed provider integration adds explicit policy gates, request redaction, and tests.

## Implementation notes

The Gina baseline uses `.wasm` artifacts with `.wasi` manifests under `plugins/`. The manifests provide deterministic output for Phase1's `phase1-wasi-lite` executor.

This keeps the first AI layer auditable and safe while leaving room for later provider-backed research behind strong policy controls.
