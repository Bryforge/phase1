# Base1 network lockdown dry-run

Base1 network lockdown is the staged plan for keeping Phase1 useful while preventing the Phase1 workload from becoming the network authority of the host.

This checkpoint is read-only. It defines the operator-facing network preview command and the promotion guardrails required before any firewall mutation exists.

## Command

```bash
sh scripts/base1-network-lockdown-dry-run.sh --dry-run
```

Optional profiles:

```bash
sh scripts/base1-network-lockdown-dry-run.sh --dry-run --profile secure-default
sh scripts/base1-network-lockdown-dry-run.sh --dry-run --profile offline
sh scripts/base1-network-lockdown-dry-run.sh --dry-run --profile appliance --target raspberry-pi
sh scripts/base1-network-lockdown-dry-run.sh --dry-run --profile dev --target x200
```

## Read-only guarantee

The dry-run command must not:

- Change firewall rules.
- Enable or disable services.
- Modify DNS, routes, package state, SSH, users, groups, or credentials.
- Store secrets.
- Open inbound ports.
- Persist maintenance mode.
- Change Phase1 host-tool gates.

It must report:

```text
mode: dry-run
writes: no
```

## Profiles

| Profile | Purpose | Default posture |
| --- | --- | --- |
| `secure-default` | Normal Base1 host mode. | Deny unexpected inbound, allow only explicitly configured outbound needs. |
| `offline` | Local-first recovery or sealed appliance mode. | Deny non-loopback network after required artifacts are present. |
| `appliance` | Managed Raspberry Pi or dedicated node mode. | Deny inbound except a separately approved management path. |
| `dev` | Development-only visibility mode. | Report current posture without claiming release security. |

## Planned secure-default behavior

The future mutating implementation should converge on this posture:

- Loopback is allowed.
- Inbound traffic is denied by default.
- SSH is disabled by default.
- SSH, when explicitly enabled for appliance mode, must be key-only, logged, rate-limited where possible, and documented as an operator decision.
- Outbound traffic is limited to the minimum required for DNS, DHCP, time sync, package mirrors, and signed Phase1/Base1 updates.
- Phase1 host-backed tools remain denied during normal boot.
- Network policy logs live outside the Phase1 workspace.

## Hardware notes

### Raspberry Pi

Raspberry Pi appliance deployments must preserve a local recovery path before any network lockout is promoted. Serial console, local keyboard/display access, or a known-good recovery image should be available before SSH is disabled.

### ThinkPad X200-class systems

X200-class deployments should remain keyboard-first and offline-friendly. Local documentation and recovery steps must remain available when the network is locked down.

## Promotion requirements

This dry-run checkpoint may not become a mutating lockdown command until these artifacts exist:

1. Network lockdown design review.
2. Recovery path that does not require working networking.
3. Rollback path for firewall mistakes.
4. Hardware target notes for Raspberry Pi and X200-class systems.
5. Tests proving `--dry-run` is read-only.
6. Tests proving the command refuses to run without explicit destructive-mode confirmation.
7. Audit log location outside the Phase1 workspace.
8. Operator-facing warning that network lockout can break remote access.

## Current status

This is a planning and dry-run surface only. It is safe to run for inspection, but it is not a firewall installer and does not harden a host by itself.
