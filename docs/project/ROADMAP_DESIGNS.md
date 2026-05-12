# phase1 roadmap designs

This document is the design index for turning phase1 into a more legitimate terminal-first virtual operating-system console.

## Design goals

phase1 should feel like a real operator environment while staying safe as a Rust userspace simulator.

Core principles:

- Keep the simulator fast, understandable, and terminal-native.
- Prefer explicit kernel boundaries over direct state mutation.
- Make every command observable and testable.
- Keep host integrations guarded with validation, dry-runs, timeouts, safe mode, and explicit host-tools opt-in.
- Make mobile terminal output compact but still premium.
- Build extensibility through policy, packages, and sandboxed plugins.
- Keep future pipelines internal to the phase1 simulator.
- Keep developer storage, cloned repositories, generated projects, and build artifacts isolated under a phase1-managed workspace.
- Grow language runtime support behind the same capability metadata, timeout, redaction, and workspace controls used by existing guarded host tools.

## Prepared update queue

### Next implementation target

```text
docs/archive/legacy-release-notes/NEXT_UPDATE_v3.7.0.md
```

Theme:

```text
Secure Operator Persistence + Policy Gate
```

Primary targets:

- two-key host tools gate with `PHASE1_ALLOW_HOST_TOOLS=1`
- central command policy module
- structured policy/syscall audit formatting
- privacy-first persistent shell history
- `security` / `sec` / `policy` status command
- smoke tests for default safe mode, safe-off without host tools, and trusted-user host tools mode

### Follow-up implementation target

```text
docs/archive/legacy-release-notes/NEXT_UPDATE_v3.8.0.md
```

Theme:

```text
Structured Output + Pipeline Foundation
```

Primary targets:

- `CommandOutput` and `TableOutput` models
- `--text`, `--table`, and `--json` output flags
- phase1-only internal pipeline parsing
- `where`, `sort`, `get`, `table`, `json`, and `count` pipeline stages
- secret-conscious `audit --json`
- smoke tests for structured output and pipeline behavior

### Developer workspace implementation target

```text
docs/runtime/STORAGE_GIT_RUST.md
```

Theme:

```text
Guarded Storage + Git Clone + Rust Runner
```

Primary targets:

- local `phase1.workspace` storage root
- guarded `git clone`, `git status`, and `git pull` helper paths
- Rust toolchain checks, single-file compile/run, generated Cargo projects, and Cargo check/build/test/run helpers
- non-interactive Git prompts and conservative output redaction
- generated workspace artifact handling

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

### Phase R7 — Storage, Git, and language runtime support

Design file: `docs/roadmap/language-runtime-support.md`

User-facing workflow: `docs/runtime/STORAGE_GIT_RUST.md`

Targets:

- isolated storage workspace for cloned repositories and generated projects
- guarded Git clone/status/pull workflows
- Rust compile/run and Cargo project support
- staged roadmap for Python, C/C++, JavaScript/TypeScript, Go, Java/Kotlin, .NET, Swift, PHP, Ruby, data/science languages, systems languages, BEAM/JVM/functional ecosystems, Dart, and WASM/WASI
- consistent runtime controls: safe-mode gate, host-tools opt-in, timeout limits, bounded output, redaction, workspace isolation, docs, and smoke tests

## Immediate implementation order

1. Implement v3.7.0 secure operator persistence and policy gate.
2. Validate v3.7.0 locally with fmt, Clippy, unit tests, and smoke tests.
3. Promote v3.7.0 only after security behavior is confirmed.
4. Implement v3.8.0 structured output and internal pipelines.
5. Validate v3.8.0 locally with fmt, Clippy, unit tests, and smoke tests.
6. Stabilize `phase1-storage` as the first developer workspace helper.
7. Add shell-facing wrappers for `storage`, `git`, `rust`, and `lang` after helper behavior is validated.
8. Continue virtual kernel boundary cleanup and package/plugin runtime work.

## Definition of done

Each roadmap phase should include:

- design document
- implementation code
- smoke tests
- README update
- system test report update
- no new Clippy warnings
