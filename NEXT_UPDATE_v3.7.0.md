# phase1 v3.7.0 next update feature set

## Theme

**R1 Operator Shell Completion + Policy Foundation**

v3.7.0 should finish the remaining high-value R1 operator shell items while laying the smallest useful foundation for R2/R3. The goal is to make phase1 feel more like a durable terminal OS console without jumping too far ahead into the larger kernel and TUI phases.

## Why this is next

The roadmap's immediate order already has several completed or partially completed items:

- Registry-backed alias normalization: implemented.
- `capabilities` command: implemented.
- Alias/capability smoke tests: implemented.
- Persistent boot configuration: implemented.
- Persistent `/home` VFS state: implemented.

The next missing roadmap item with the most user-facing value is persistent shell history, followed by policy/audit groundwork.

## Target version

```text
phase1 v3.7.0
```

Working title:

```text
phase1 v3.7.0 — Operator Shell Persistence Update
```

## Feature set

### 1. Persistent shell history

Add disk-backed command history for the operator shell.

Behavior:

- Load history before entering the shell.
- Save history on exit.
- Cap history at 512 entries.
- Skip empty lines.
- Do not crash if history cannot be read or written.

History path resolution:

1. `PHASE1_HISTORY=off` disables disk history.
2. `PHASE1_HISTORY=/path/to/file` uses that file.
3. If persistent state mode is enabled, prefer `.phase1_history` in the host working directory.
4. Otherwise default to volatile in-memory history only.

Commands:

```text
history
history clear
history path
```

Acceptance checks:

- `history` still prints current session history.
- A command entered in one run appears after restarting with history enabled.
- `PHASE1_HISTORY=off` disables disk history for tests and privacy.
- Failed writes only print a warning; the shell continues.

### 2. Command policy check foundation

Add a simple policy layer that reads command metadata before dispatch.

Initial behavior:

- Commands with capability `none` always allow.
- Safe mode denies host execution/network entry points through one policy helper instead of repeated manual checks.
- Denied commands write a structured audit line.
- Allowed host-backed commands write a structured audit line.

Policy helper shape:

```rust
policy::check(command, capability, boot_config, shell) -> PolicyDecision
```

Suggested decision fields:

```text
allow | deny
reason
capability
command
```

Acceptance checks:

- Safe mode denies `browser`, `ping`, `wifi-scan`, `wifi-connect`, `python`, `gcc`, and plugins.
- Audit output shows allowed and denied policy decisions.
- Existing command behavior stays stable outside safe mode.

### 3. Structured audit event formatting

Improve audit entries toward the R2 design without requiring a full typed kernel rewrite yet.

Target format:

```text
0004 user=root action=sys.write object=/home/demo.txt result=allow
0005 user=root action=policy.check object=browser result=deny reason=safe-mode
```

Acceptance checks:

- Existing audit tests continue passing or are updated to match deterministic structured strings.
- Smoke tests verify at least one allow and one deny audit entry.

### 4. History and policy smoke tests

Add smoke tests for:

- Persistent history across restarts.
- `history path` output.
- `PHASE1_HISTORY=off` behavior.
- Safe mode policy denial and audit logging.

### 5. Documentation updates

Update:

- `README.md`
- `docs/roadmap/operator-shell.md`
- `docs/roadmap/security-capabilities.md`
- `RELEASE_NOTES_v3.7.0.md`

## Stretch goals

These are optional and should not block the v3.7.0 release:

- Add `history clear` with confirmation-free behavior inside the simulator.
- Add `audit --json` as the first structured output example.
- Add a `policy` command that prints safe-mode and host-integration policy state.
- Start a small `CommandOutput` enum for future table/json/text pipelines.

## Out of scope for v3.7.0

Do not include these yet:

- Full structured pipelines.
- WASM/WASI plugin runtime.
- Full-screen TUI dashboard.
- Real mount table implementation.
- Full users/groups permission model.

## Implementation order

1. Add shell history path resolution helpers.
2. Load history during shell startup.
3. Save history on clean shell exit.
4. Extend `history` command with `clear` and `path`.
5. Add policy decision type and helper.
6. Route safe-mode host command denies through policy helper.
7. Add structured audit strings for policy decisions.
8. Add smoke tests.
9. Update README and release notes.
10. Run:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo test --test smoke -- --nocapture
```

## Release validation target

Before publishing v3.7.0, validation should show:

```text
unit tests passed
smoke tests passed
persistent history verified across restarts
safe-mode policy denial verified through audit log
0 failed
```
