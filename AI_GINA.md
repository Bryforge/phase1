# Phase1 AI: Gina

Gina is the Phase1 AI integration assistant.

This implementation is intentionally safe, offline, deterministic, and sandboxed. It establishes the Phase1 AI command surface while making cybersecurity, optimization, and consistency part of Gina's baseline contract.

## Commands

Inside Phase1:

```text
gina
gina security
gina optimize
gina consistency
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
- always-on cybersecurity posture
- safe-mode-first and policy-gated guidance
- optimization and consistency baseline checks
- future model-provider handoff rules
- redacted argument handling through the WASI-lite runtime

## Cybersecurity baseline

Gina always starts from a defensive posture:

- safe mode first
- policy gates first
- redaction first
- host shell blocked
- host network blocked
- host filesystem passthrough blocked
- credential-like values never stored in AI context
- external model/provider calls disabled in the baseline

Future provider-backed Gina work must require explicit opt-in policy gates and redacted requests.

## Optimization baseline

Gina's current optimization promise is deterministic and testable:

- prefer built Phase1 binaries where launchers support it
- keep offline output bounded and repeatable
- avoid host/network dependency in the baseline
- keep behavior covered by CI-backed tests
- keep future integrations behind narrow capability boundaries

## Consistency baseline

Gina's current consistency promise is:

- stable command surface
- repeatable offline responses
- documented behavior
- test-backed claims
- no overclaiming provider-backed intelligence before a real provider exists

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

This is the correct baseline.

## Integration plan

### Phase 1: Offline identity, command surface, and cybersecurity baseline

Complete in this change:

- `gina` command via plugin dispatch
- `ai gina` command via plugin dispatch
- `gina security` baseline output
- `gina optimize` baseline output
- `gina consistency` baseline output
- `wasm inspect gina`
- `wasm run gina status`
- tests for offline execution, redaction, security posture, optimization posture, consistency posture, and future-provider safety rules

### Phase 2: Phase1 context bridge

Next recommended step:

- expose a `gina context` response that summarizes safe Phase1 project context
- include command map, safe-mode status, version, docs index, and CI posture
- do not include secrets, host env values, credential-like history, or unredacted logs

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
- provider behavior must be tested separately from offline Gina behavior

## Development validation

```bash
cargo test --test ai_gina -- --nocapture
cargo test --all-targets
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

## Operator promise

Gina should help manage Phase1, explain Phase1, prepare safe prompts, summarize docs, and guide development workflows. Gina should remain security-first, predictable, and policy-gated. Gina should not silently mutate host systems, leak credentials, call outside providers without explicit configuration, or overclaim model-backed intelligence before a real provider is integrated.
