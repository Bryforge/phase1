# Gina AI Roadmap

Gina is Phase1's safe operations-assistant track. The roadmap keeps the offline baseline stable before any provider-backed features are explored.

## Stage G1 — Offline baseline

Status: active.

- Gina and AI bridge WASI-lite manifests live in `plugins/`.
- `assistant` is a compatibility alias for Gina.
- Output is deterministic and local.
- Sandbox language is explicit: virtual filesystem, network disabled, host blocked.
- Launch path is `./start_phase1 --gina` or `phase1-terminal gina`.

## Stage G2 — Native command polish

- Keep plugin execution working through `gina`, `assistant`, and `ai`.
- Add focused tests for plugin discovery, inspection, execution, redaction, and safety text.
- Keep README changes minimal and link to this doc instead of rewriting release text.

## Stage G3 — Operations context bridge

- Provide a privacy-safe summary of Phase1 status.
- Never include secrets, command history payloads, API keys, cookies, SSH keys, or recovery codes.
- Prefer high-level status such as version, safe-mode state, quality gate state, and Base1 preflight availability.

## Stage G4 — Cybersecurity advisor

- Explain CodeQL findings, safe-mode posture, redaction rules, workflow pinning, Base1 boundaries, and quality gates.
- Recommend tests and docs before implementation claims.
- Never encourage unsafe host mutation or credential storage.

## Stage G5 — Terminal integration

- Keep Phase1 Terminal as a wrapper around `./start_phase1`.
- Let `phase1-terminal gina` call the existing launch path.
- Avoid a separate trust path.

## Stage G6 — Provider-backed research

Future external-provider work must be separate from the offline baseline and must require:

- explicit operator opt-in
- provider-disabled default
- request redaction
- no credential persistence in repo files
- tests for disabled-by-default behavior
- documentation of exactly what leaves the machine

Until that exists, Gina remains offline-only.
