# Base1 and Phase1 Compatibility Contract

Base1 and Phase1 are separate layers with a deliberate trust boundary.

Base1 provides a secure hardware host. Phase1 provides the terminal-first virtual operating-system experience. Base1 should preserve Phase1 compatibility while preventing Phase1 from becoming host authority.

## Contract version

```text
BASE1_PHASE1_CONTRACT=0.1
```

This first contract is intentionally small. Future versions can add features without breaking safe defaults.

## Required environment variables

Base1 may provide these variables when launching Phase1:

```text
BASE1_PROFILE=secure-default
BASE1_HARDWARE_TARGET=raspberry-pi | x200 | generic
BASE1_PHASE1_CONTRACT=0.1
PHASE1_STORAGE_ROOT=/var/lib/phase1/workspace
PHASE1_SAFE_MODE=1
PHASE1_ALLOW_HOST_TOOLS=0
```

## Default compatibility posture

By default:

- Phase1 safe mode stays enabled.
- Host-backed tools stay disabled.
- Phase1 storage is placed under a Base1-owned workspace path.
- Phase1 runs as a dedicated unprivileged user.
- Phase1 does not receive host secrets.
- Phase1 does not control Base1 boot, packages, services, keys, or firewall policy.

## Maintenance posture

A future Base1 maintenance mode may temporarily allow host-backed Phase1 helper flows. That mode must be separate from normal boot.

Maintenance mode requirements:

- Operator explicitly enters maintenance mode.
- Base1 records an audit entry outside the Phase1 workspace.
- Host-backed tools receive the minimum required permission.
- The session ends with a clear return to secure-default mode.
- Phase1 cannot silently persist maintenance mode.

## Stable paths

Base1 should provide these stable paths:

```text
/opt/phase1            Read-only Phase1 executable and assets
/var/lib/phase1        Phase1 persistent state and workspace
/var/log/phase1-host   Host-side Phase1 service logs
/run/phase1            Ephemeral runtime directory
/base1                 Base1 local policy and metadata
```

Phase1 must treat `/opt/phase1`, `/base1`, and host log paths as read-only.

## Runtime account

Preferred account:

```text
user:  phase1
group: phase1
home:  /var/lib/phase1
shell: nologin for service mode, normal shell only for explicit interactive mode
```

The `phase1` user should not have passwordless administrator rights.

## Host tool gating

The existing Phase1 host-tool gate remains part of the contract:

```text
PHASE1_SAFE_MODE=1
PHASE1_ALLOW_HOST_TOOLS=0
```

Base1 must not invert this default. Any host tool enablement should be temporary, logged, and operator-approved.

## Hardware hints

Phase1 may use `BASE1_HARDWARE_TARGET` to tune UI and runtime behavior:

### raspberry-pi

- Prefer conservative color output.
- Avoid heavy default workloads.
- Keep serial-console friendly output.
- Prefer low-memory docs and commands.

### x200

- Prefer legacy terminal compatibility.
- Avoid assuming modern GPU or high-DPI display features.
- Keep keyboard-first workflows.
- Keep offline-first docs available.

### generic

- Use normal Phase1 defaults.

## Failure behavior

If Phase1 state is damaged:

- Base1 should still boot.
- Host logs should remain readable.
- The Phase1 workspace can be reset without changing Base1 boot or system files.
- Known-good Phase1 assets under `/opt/phase1` should remain protected.

If Base1 detects unsafe Phase1 launch conditions:

- Refuse to launch as root unless explicitly configured for development.
- Warn when safe mode is disabled.
- Warn when host tools are allowed.
- Warn when the workspace path is missing or world-writable.

## Phase1 development compatibility

Phase1 should continue to work without Base1. Base1 variables are optional hints, not mandatory dependencies. When Base1 variables are absent, Phase1 should use normal development defaults.

## Base1 development compatibility

Base1 should not require Phase1 internals beyond documented launch and storage behavior. This keeps Base1 useful even as Phase1 evolves.

## Compatibility test requirements

Future tests should verify:

- Phase1 runs with Base1 environment variables set.
- Phase1 safe mode remains enabled by default.
- `phase1-storage storage status` works without host-tool permissions.
- Mutating storage, Git, and Rust actions remain blocked unless the explicit trust gate is set.
- Base1 preflight never changes the host.
- Base1 launcher refuses unsafe root execution unless development override is set.
