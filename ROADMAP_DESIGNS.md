# phase1 roadmap designs

This document is the design index for turning phase1 into a more legitimate terminal-first virtual operating-system console.

## Design goals

phase1 should feel like a real operator environment while staying safe as a Rust userspace simulator.

Core principles:

- Keep the simulator fast, understandable, and terminal-native.
- Prefer explicit kernel boundaries over direct state mutation.
- Make every command observable and testable.
- Keep host integrations guarded with validation, dry-runs, and timeouts.
- Make mobile terminal output compact but still premium.
- Build extensibility through policy, packages, and sandboxed plugins.

## Next update

Prepared next feature set:

```text
NEXT_UPDATE_v3.7.0.md
```

Theme:

```text
R1 Operator Shell Completion + Policy Foundation
```

Primary targets:

- persistent shell history
- history command extensions
- command policy check foundation
- structured audit event formatting
- smoke tests for history and policy behavior

## Roadmap phases

### Phase R1 — Operator shell foundation

Status: in progress.

Design file: `docs/roadmap/operator-shell.md`

Targets:

- registry-backed alias dispatch
- persistent shell history
- command completion design
- command metadata as source of truth
- shell smoke tests
- consistent command error style

### Phase R2 — Virtual kernel boundary

Design file: `docs/roadmap/virtual-kernel.md`

Targets:

- syscall layer for file/process/network operations
- process lifecycle model
- mount table
- procfs/devfs/logfs behavior
- structured audit events

### Phase R3 — Security and capability model

Design file: `docs/roadmap/security-capabilities.md`

Targets:

- capability checks from command metadata
- simulated users/groups
- command allow/deny policy
- host integration policy
- audit trails for allowed and denied operations

### Phase R4 — Structured pipelines

Design file: `docs/roadmap/structured-pipelines.md`

Targets:

- command output model
- table/json/text outputs
- `where`, `sort`, `get`, `table`, and `json`
- pipeline tests

### Phase R5 — Package and plugin runtime

Design file: `docs/roadmap/package-plugin-runtime.md`

Targets:

- package manifest format
- plugin command registration
- WASM/WASI runtime design
- plugin capabilities
- package trust and signatures

### Phase R6 — TUI dashboard

Design file: `docs/roadmap/tui-dashboard.md`

Targets:

- fullscreen dashboard mode
- process monitor
- network monitor
- audit viewer
- VFS browser
- mobile fallback mode

## Immediate implementation order

1. Add persistent command history with a disable flag for tests.
2. Add a policy check function and log allow/deny decisions to audit.
3. Expand smoke tests around history and safe-mode policy behavior.
4. Start the structured output enum behind command handlers.
5. Continue virtual kernel boundary cleanup.

## Definition of done

Each roadmap phase should include:

- design document
- implementation code
- smoke tests
- README update
- system test report update
- no new Clippy warnings
