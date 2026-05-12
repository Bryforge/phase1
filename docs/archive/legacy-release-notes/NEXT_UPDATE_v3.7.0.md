# phase1 v3.7.0 next update feature set

## Theme

**Secure Operator Persistence + Policy Gate**

v3.7.0 should build on the v3.6.0 security hardening by adding useful operator persistence and a real command-policy layer without weakening the secure-by-default posture.

Working title:

```text
phase1 v3.7.0 — Secure Operator Persistence Update
```

## Security baseline from v3.6.0

Do not regress these guarantees:

- Safe mode defaults to on.
- Host network inspection is skipped in safe mode.
- Safe mode uses a simulated loopback-only network view.
- Browser, ping, WiFi scan/connect, Python, C compiler, and plugins are blocked in safe mode.
- Runtime state files and common credential files are ignored by Git.
- Persistent VFS state stays limited to phase1-managed `/home` content.

## Primary feature set

### 1. Two-key host tools gate

Add a second explicit opt-in before host-backed execution can run.

Current behavior:

```text
safe mode on  -> host tools blocked
safe mode off -> host tools allowed
```

Target behavior:

```text
safe mode on                         -> host tools blocked
safe mode off + no host tools allow  -> host tools still blocked
safe mode off + PHASE1_ALLOW_HOST_TOOLS=1 -> host tools allowed
```

Affected commands:

- `browser`
- `ping`
- `wifi-scan`
- `wifi-connect`
- `nmcli`
- `python` / `py`
- `gcc` / `cc`
- Python plugins
- real host network inspection inside `ifconfig` / `iwconfig`

Rationale:

Turning off safe mode should not be enough by itself to run host-backed tools. The user should need a deliberate environment variable too.

Acceptance checks:

- `cargo run` blocks host tools.
- Preboot option `4` alone still blocks host tools and explains that `PHASE1_ALLOW_HOST_TOOLS=1` is required.
- `PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 cargo run` enables trusted-user host-backed testing.
- Smoke tests verify all three states.

### 2. Central command policy module

Add a real policy module that uses registry metadata instead of scattered command checks.

Suggested shape:

```rust
policy::check(command, capability, security_context) -> PolicyDecision
```

Decision fields:

```text
command
capability
result=allow|deny
reason
host_backed=true|false
```

Policy rules:

- `none`, simulated `sys.read`, `fs.read`, `hw.read`, and virtual shell commands are allowed in safe mode.
- Host-backed capabilities require both safe mode off and `PHASE1_ALLOW_HOST_TOOLS=1`.
- `net.admin` still requires `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` before real host network mutation.
- Denied commands do not run fallback host code.

Acceptance checks:

- Safe mode blocks host-backed commands through the policy module.
- Host tools are blocked without the second host-tools gate.
- Policy decisions are visible in `audit`.
- Existing safe-mode smoke tests still pass.

### 3. Structured audit events

Move policy and syscall audit entries toward deterministic key/value records.

Target examples:

```text
0004 user=root action=sys.write object=/home/demo.txt result=allow
0005 user=root action=policy.check object=browser capability=host.net result=deny reason=safe-mode
0006 user=root action=policy.check object=python capability=host.exec result=deny reason=host-tools-disabled
```

Acceptance checks:

- `audit` includes a denied safe-mode host command.
- `audit` includes a denied host-tools-disabled command.
- Existing syscall audit tests are updated to deterministic key/value strings.

### 4. Persistent shell history, privacy-first

Add disk-backed shell history only after the policy foundation is in place.

Rules:

- Default remains in-memory only.
- `PHASE1_HISTORY=off` disables disk history.
- `PHASE1_HISTORY=/path/to/file` enables disk history at that path.
- If persistent state is on and `PHASE1_HISTORY` is unset, use `.phase1_history`.
- `.phase1_history` stays ignored by Git.
- Cap history at 512 entries.
- Skip empty lines.
- Do not crash if history cannot be read or written.

Commands:

```text
history
history clear
history path
```

Security rules:

- Do not store history unless explicitly enabled or persistent state is on.
- Do not store history in `phase1.state`.
- Document that users should not type account secrets into phase1.

Acceptance checks:

- `history` still shows current session history.
- `history path` shows `off` by default.
- `PHASE1_HISTORY=off` stays disabled even with persistent state on.
- Persistent history restores across restarts only when enabled.
- `history clear` clears memory and disk-backed history.

### 5. Security status command

Add a small `security` command that prints the active runtime security posture.

Example:

```text
security mode       : safe
host tools          : disabled
host network changes: disabled
persistent state    : off
history             : off
```

Aliases:

```text
sec
policy
```

Acceptance checks:

- `security` works in default safe mode.
- `security` reflects `PHASE1_ALLOW_HOST_TOOLS=1`.
- `security` reflects `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1`.
- `capabilities` still displays registry metadata.

## Documentation updates

Update:

- `README.md`
- `SECURITY.md`
- `SECURITY_REVIEW.md`
- `docs/project/ROADMAP_DESIGNS.md`
- `docs/roadmap/operator-shell.md`
- `docs/roadmap/security-capabilities.md`
- `RELEASE_NOTES_v3.7.0.md`

## Out of scope for v3.7.0

Do not include these yet:

- Full structured pipelines.
- WASM/WASI plugin runtime.
- Full-screen TUI dashboard.
- Real host sandboxing claims.
- Real permissions/users/groups beyond the simulator model.
- Release binaries.

## Implementation order

1. Add `SecurityContext` and `PolicyDecision` types.
2. Add `PHASE1_ALLOW_HOST_TOOLS=1` gate.
3. Route host-backed command checks through policy.
4. Route host network inspection checks through policy.
5. Add policy audit records.
6. Add `security` command and registry metadata.
7. Add privacy-first persistent history.
8. Add smoke tests for default safe mode, safe-off/no-host-tools, and safe-off/host-tools-enabled.
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

Before promoting v3.7.0:

```text
fmt clean
clippy clean
unit tests passed
smoke tests passed
safe mode blocks host tools
safe-off without PHASE1_ALLOW_HOST_TOOLS still blocks host tools
safe-off with PHASE1_ALLOW_HOST_TOOLS enables trusted-user host tools
persistent history verified only when enabled
0 failed
```
