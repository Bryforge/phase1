# phase1 v3.8.0 next update feature set

## Theme

**Structured Output + Pipeline Foundation**

v3.8.0 should follow the v3.7.0 security/persistence work by making command output more useful, queryable, and scriptable without weakening the secure-by-default host model.

Working title:

```text
phase1 v3.8.0 — Structured Pipeline Foundation
```

## Why this comes after v3.7.0

v3.7.0 is planned to add:

- two-key host tools gate
- central policy decisions
- structured policy audit events
- privacy-first persistent shell history
- `security` status command

After that, phase1 will have enough metadata and audit structure to safely start R4: structured command output and pipelines.

## Security baseline

Do not regress these guarantees:

- Safe mode defaults to on.
- Host-backed tools require intentional opt-in.
- Host network mutation remains separately gated.
- Structured output must not expose host secrets.
- Pipeline commands operate on phase1 command output, not arbitrary host shell pipelines.
- No command should invoke `/bin/sh`, `bash`, `zsh`, or host shell expansion for pipelines.

## Primary feature set

### 1. CommandOutput enum

Introduce a command output model behind selected command handlers.

Suggested shape:

```rust
enum CommandOutput {
    Text(String),
    Table(TableOutput),
    Json(JsonValue),
    Empty,
}
```

Initial table model:

```rust
struct TableOutput {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}
```

Start with commands that are already simulated and safe:

- `ps`
- `jobs`
- `ls -l`
- `lspci`
- `audit`
- `capabilities`
- `security`

Acceptance checks:

- Existing text output stays stable by default.
- Structured data is generated internally for selected commands.
- No host-backed commands are required for structured output tests.

### 2. Output format flags

Add explicit format flags for supported structured commands:

```text
--text
--table
--json
```

Default remains human-readable text.

Examples:

```text
ps --json
lspci --table
audit --json
capabilities --table
security --json
```

Acceptance checks:

- `ps --json` emits deterministic JSON-like output.
- `audit --json` emits structured records without leaking command arguments that may contain secrets.
- Invalid format flags produce clear usage errors.

### 3. Safe phase1-only pipelines

Add minimal internal pipelines using `|` without invoking the host shell.

Initial pipeline operators:

```text
where <column> <value>
sort <column>
get <column>
table
json
count
```

Examples:

```text
ps --table | where STATE run | table
capabilities --table | where guard safe-mode | table
lspci --table | get name
history | count
```

Acceptance checks:

- Pipelines are parsed internally by phase1.
- No host shell is spawned for pipeline evaluation.
- Pipelines work only with phase1 `CommandOutput` values.
- Unsupported pipeline stages return clear errors.

### 4. Secret-conscious audit JSON

Add `audit --json` with redaction rules.

Rules:

- Policy records may include command name, capability, result, and reason.
- Do not store or output full command arguments for host-backed commands.
- If an event field contains likely secrets, show `[redacted]`.

Suggested redaction patterns:

```text
token=
password=
secret=
key=
ghp_
github_pat_
BEGIN PRIVATE KEY
```

Acceptance checks:

- `audit --json` works with existing audit entries.
- Test fixtures prove token/password-looking fields are redacted.
- Human-readable `audit` output still works.

### 5. Pipeline smoke tests

Add smoke tests for:

- `ps --json`
- `lspci --table`
- `capabilities --table | where capability host.exec | table`
- `audit --json`
- invalid pipeline stage error
- proof that pipelines do not run host shell commands

## Documentation updates

Update:

- `README.md`
- `SECURITY.md`
- `SECURITY_REVIEW.md`
- `docs/project/ROADMAP_DESIGNS.md`
- `docs/roadmap/structured-pipelines.md`
- `RELEASE_NOTES_v3.8.0.md`

## Out of scope for v3.8.0

Do not include these yet:

- Full POSIX shell pipeline compatibility.
- Host shell execution for pipelines.
- WASM/WASI plugin runtime.
- Full query language.
- External JSON parser dependency unless absolutely necessary.
- Full-screen TUI dashboard.

## Implementation order

1. Add `output.rs` with `CommandOutput`, `TableOutput`, and render helpers.
2. Convert `ps`, `lspci`, `capabilities`, `audit`, and `security` to produce structured output internally.
3. Add `--json`, `--table`, and `--text` render flags.
4. Add internal pipeline parser for `|`.
5. Add pipeline stages: `where`, `sort`, `get`, `table`, `json`, `count`.
6. Add redaction helper for audit JSON.
7. Add unit tests for renderers, redaction, and pipeline stages.
8. Add smoke tests for structured output and pipelines.
9. Update docs and release notes.
10. Validate locally.

## Required local validation

```bash
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo test --test smoke -- --nocapture
```

## Release validation target

Before promoting v3.8.0:

```text
fmt clean
clippy clean
unit tests passed
smoke tests passed
structured output verified
safe internal pipelines verified
no host shell pipeline execution
secret-conscious audit JSON verified
0 failed
```
