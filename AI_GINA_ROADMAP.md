# Gina AI Roadmap

Gina is the Phase1 AI integration assistant. Her job is to keep Phase1 understandable, secure, optimized, consistent, and ready for future model-backed workflows without weakening Phase1's safety model.

This roadmap grows Gina from the current offline WASI-lite baseline into a policy-gated Phase1 management and cybersecurity advisor.

## Guiding principles

- Security-first by default.
- Offline baseline always available.
- No silent host shell access.
- No silent host network access.
- No credential storage in AI context.
- No external model calls without explicit policy gates.
- Deterministic behavior where possible.
- Every claim must be backed by tests, docs, or explicit limitations.
- Gina should assist Phase1 management, not bypass Phase1 safety controls.

## Current baseline

Implemented in the first Gina PR:

- `plugins/gina.wasm` and `plugins/gina.wasi`.
- `plugins/ai.wasm` and `plugins/ai.wasi`.
- Commands:
  - `gina`
  - `gina security`
  - `gina optimize`
  - `gina consistency`
  - `assistant`
  - `ai gina`
  - `wasm inspect gina`
  - `wasm run gina status`
- Native registry metadata for `ai`, `gina`, and `assistant`.
- `ai.offline` capability classification.
- Security-first output that states:
  - safe mode first
  - policy gates first
  - redaction first
  - host shell blocked
  - host network blocked
  - host filesystem passthrough blocked
- Tests for offline execution, sandboxing, redaction, cybersecurity posture, optimization posture, consistency posture, registry metadata, and future-provider safety rules.

## Phase 1 — Native offline Gina command

Goal: move Gina from a manifest-backed WASI-lite response into a native Phase1 module while preserving the plugin fallback.

Planned work:

- Add `src/gina.rs`.
- Route `gina` and `ai` through native command logic in `src/main.rs` or `src/commands.rs`.
- Keep `plugins/gina.*` as compatibility fallback.
- Add subcommands:
  - `gina status`
  - `gina security`
  - `gina optimize`
  - `gina consistency`
  - `gina help`
- Add deterministic output builders that are easy to unit test.
- Preserve `ai.offline` capability metadata.

Acceptance checks:

```bash
cargo test gina
cargo test --test ai_gina -- --nocapture
cargo test --all-targets
```

## Phase 2 — Safe Phase1 context bridge

Goal: let Gina summarize the Phase1 system state without exposing secrets, host environment values, or unredacted logs.

Planned work:

- Add `gina context`.
- Include safe context only:
  - Phase1 version
  - boot profile
  - safe mode status
  - command registry summary
  - capability summary
  - docs index
  - CI validation checklist
  - current VFS cwd
- Exclude unsafe context:
  - host environment variables
  - raw command history
  - credentials, tokens, keys, cookies, recovery codes
  - unredacted logs
  - browser/session data
- Add redaction tests for every context source.

Acceptance checks:

```bash
cargo test gina_context
cargo test redaction
```

## Phase 3 — Cybersecurity advisor

Goal: make Gina the first stop for Phase1 security posture review.

Planned work:

- Add `gina audit`.
- Add `gina harden`.
- Add `gina threat-model`.
- Add `gina policy`.
- Summarize:
  - safe mode
  - host tool gates
  - network mutation gates
  - history persistence
  - ops log redaction
  - plugin/WASI boundaries
  - Base1 compatibility risks
  - terminal launcher safety
- Produce prioritized recommendations:
  - critical
  - high
  - medium
  - low
- Add tests that prevent Gina from recommending unsafe defaults.

Acceptance checks:

```bash
cargo test gina_security
cargo test policy
cargo test ops_log
cargo test history
```

## Phase 4 — Optimization and consistency advisor

Goal: help keep Phase1 fast, predictable, and easy to maintain.

Planned work:

- Add `gina optimize --report`.
- Add `gina consistency --report`.
- Check for:
  - documented commands with registry metadata
  - command aliases with tests
  - stale docs references
  - missing README links
  - missing roadmap links
  - duplicated safety claims
  - untested command surfaces
- Add output groups:
  - performance
  - reliability
  - docs
  - tests
  - release readiness
- Add snapshot-style tests for stable sections.

Acceptance checks:

```bash
cargo test gina_optimize
cargo test gina_consistency
```

## Phase 5 — ChatGPT project-management workflow

Goal: make Gina a Phase1-facing partner for ChatGPT-managed project work.

Planned work:

- Add `gina plan <topic>`.
- Add `gina pr-checklist`.
- Add `gina release-checklist`.
- Add `gina terminal-roadmap`.
- Add `gina base1-roadmap`.
- Add `gina prompts` to reference safe AI prompt templates.
- Make Gina output copy-pasteable task prompts for ChatGPT.
- Keep all generated prompts safety-bound and redaction-aware.

Acceptance checks:

```bash
cargo test gina_project_management
```

## Phase 6 — Local model integration exploration

Goal: explore optional local/offline model integrations before any external provider support.

Planned work:

- Research local model runtimes that can be started explicitly by the user.
- Add provider-neutral config schema draft.
- Add `gina provider status`.
- Add `gina provider doctor`.
- Require explicit opt-in for any model process.
- Keep offline deterministic Gina available without a provider.
- Do not store provider secrets in Phase1 VFS.

Decision gate:

Local model support must not become a hidden dependency for core Phase1 commands.

Acceptance checks:

```bash
cargo test gina_provider_config
cargo test gina_offline_fallback
```

## Phase 7 — External provider support behind policy gates

Goal: add optional external AI provider support only after Phase1 has strong policy and redaction controls.

Planned work:

- Add explicit provider policy gates.
- Add clear user consent flow.
- Add request redaction layer.
- Add response labeling that distinguishes external AI from deterministic Gina.
- Add provider disabled-by-default behavior.
- Add audit events for provider use without storing prompts or secrets.
- Add tests proving provider calls do not occur by default.

Required gates:

- `PHASE1_AI_PROVIDER=...`
- `PHASE1_AI_ALLOW_NETWORK=1`
- explicit boot/profile or command-level confirmation
- redaction enabled
- audit enabled

Acceptance checks:

```bash
cargo test gina_provider_disabled_by_default
cargo test gina_provider_redaction
cargo test gina_provider_audit
```

## Phase 8 — Gina + Phase1 Terminal integration

Goal: make Gina discoverable from Phase1 Terminal while keeping Terminal safe by default.

Planned work:

- Add terminal doctor check for Gina plugin/native command availability.
- Add `phase1-terminal gina`.
- Add `phase1-terminal security` to guide users toward `gina security`.
- Add terminal roadmap cross-links.
- Add docs for using Gina from Linux/macOS terminal installs.

Acceptance checks:

```bash
sh scripts/test-phase1-terminal.sh
phase1-terminal doctor
phase1-terminal gina
```

## Testing strategy

Every Gina change should include at least one of:

- unit tests for deterministic output
- integration tests for command/plugin execution
- redaction tests
- policy-gate tests
- docs updates
- registry metadata tests
- CI workflow validation

Core validation commands:

```bash
cargo test --test ai_gina -- --nocapture
cargo test --all-targets
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

## Security requirements

Gina must never silently enable:

- host shell execution
- host network access
- host filesystem passthrough
- credential storage
- external model calls
- browser cookie access
- unredacted history export
- unredacted log export
- mutation of Base1 host state

Gina may recommend higher-trust workflows only when the recommendation is explicit, policy-gated, and documented.

## Success criteria

Gina is successful when:

- `gina security` gives a trustworthy system posture summary.
- `gina optimize` gives useful performance and reliability guidance.
- `gina consistency` helps keep docs, commands, tests, and roadmaps aligned.
- offline deterministic Gina remains useful without any provider.
- future provider-backed mode is opt-in, redacted, audited, and test-protected.
- ChatGPT project-management workflows are easier and safer because Gina gives consistent Phase1 context.
