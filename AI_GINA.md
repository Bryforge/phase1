# Phase1 AI: Gina

Gina is the Phase1 AI integration assistant.

This first implementation is intentionally safe, offline, and sandboxed. It establishes the command surface and project identity for Phase1 AI without enabling host shell access, network access, credential access, or external model calls.

## Commands

Inside Phase1:

```text
gina
ai gina
wasm run gina status
wasm inspect gina
wasm run ai gina
wasm inspect ai
```

## What Gina does now

Gina currently runs as a Phase1 WASI-lite plugin:

```text
plugins/gina.wasm
plugins/gina.wasi
plugins/ai.wasm
plugins/ai.wasi
```

The baseline plugin reports:

- Gina identity and role
- offline AI integration status
- safe integration boundaries
- future model-provider handoff path
- redacted argument handling through the WASI-lite runtime

## Safety model

Gina v1 is offline by default.

It does not enable:

- host shell execution
- host network calls
- host filesystem passthrough
- external model/provider calls
- credential storage
- token access
- browser cookie access

The WASI-lite runtime displays:

```text
sandbox: fs=virtual net=disabled host=blocked
```

This is the correct baseline. Future model-backed Gina work must stay behind explicit capability metadata and policy gates.

## Integration plan

### Phase 1: Offline identity and command surface

Complete in this change:

- `gina` command via plugin dispatch
- `ai gina` command via plugin dispatch
- `wasm inspect gina`
- `wasm run gina status`
- tests for offline execution and redaction

### Phase 2: Phase1 context bridge

Next recommended step:

- expose a `gina context` response that summarizes safe Phase1 project context
- include command map, safe-mode status, version, and docs index
- do not include secrets, host env values, or credential-like history

### Phase 3: Native command metadata

Recommended after the context bridge:

- add `ai` to `src/registry.rs`
- add aliases `gina` and `assistant`
- classify under a future `ai` or `dev` category
- capability: `ai.offline`
- update help/man/completion tests

### Phase 4: Optional external model provider

Only after explicit policy design:

- provider config must be opt-in
- API keys must never be stored in Phase1 VFS by default
- requests must redact sensitive values
- network use must require an explicit AI/network capability gate
- local/offline mode must remain the default

## Development validation

```bash
cargo test --test ai_gina -- --nocapture
cargo test --all-targets
```

## Operator promise

Gina should help manage Phase1, explain Phase1, prepare safe prompts, summarize docs, and guide development workflows. Gina should not silently mutate host systems, leak credentials, or overclaim model-backed intelligence before a real provider is integrated.
