# Phase1 crypto operator command plan

Status: planning design
Scope: future Phase1 crypto policy command surface

## Purpose

This document defines the planned operator-facing command surface for cryptographic policy inspection, profile selection, explanation, export, and verification.

It is documentation-first. It does not implement runtime cryptographic behavior yet.

## Security goal

The crypto command surface should make Phase1 as secure as possible while maintaining practical usability.

Normal users should be able to inspect safe defaults without making risky choices. Advanced operators should be able to intentionally select and verify scoped cryptographic profiles with warnings, confirmations, audit events, and rollback guidance.

## Planned commands

```text
crypto status
crypto profiles
crypto explain <profile-or-algorithm>
crypto select <profile> --scope <control-point> --confirm
crypto policy export
crypto policy verify
crypto policy verify --profile <profile>
crypto policy verify --scope <control-point>
```

## Command behavior

| Command | Purpose | Mutation |
| --- | --- | --- |
| `crypto status` | Show active profiles by control point. | No |
| `crypto profiles` | List available profile docs and status labels. | No |
| `crypto explain <profile-or-algorithm>` | Explain a profile, registry entry, or algorithm doc. | No |
| `crypto select <profile> --scope <control-point> --confirm` | Select a scoped profile after warnings and confirmation. | Yes, future only |
| `crypto policy export` | Export current policy for review or backup. | No |
| `crypto policy verify` | Verify configured profiles against registry and docs. | No |

## Control-point scopes

Allowed scope names should come from [`CRYPTO_REGISTRY.md`](CRYPTO_REGISTRY.md):

```text
storage
transport
identity
base1
plugins
logs/evidence
fyr/packages
lab
docs
tests
```

Unknown scopes should fail closed.

## Profile names

Allowed profile names should come from [`crypto-profiles/README.md`](crypto-profiles/README.md):

```text
safe-default
high-security
compatibility
post-quantum-preview
lab-only
```

Unknown profiles should fail closed.

## Default behavior

The default runtime posture should be:

```text
crypto default profile: safe-default
crypto profile changes: explicit only
crypto downgrade behavior: warning + confirmation + audit event
crypto lab-only behavior: blocked outside lab/docs/tests scopes
```

## Example future output

```text
crypto status

storage        safe-default           default
transport      safe-default           default
identity       safe-default           default
base1          high-security          planned stricter default
plugins        safe-default           default
logs/evidence  safe-default           default
fyr/packages   safe-default           default
```

## Selection safety gates

Any future profile selection must show:

- current profile;
- requested profile;
- affected control point;
- whether the change is an upgrade, downgrade, compatibility exception, preview selection, or lab-only selection;
- warning text when appropriate;
- rollback command or guidance;
- audit/logging notice.

Downgrades and compatibility exceptions must require `--confirm`.

Lab-only selections must fail outside `lab`, `docs`, or `tests` scopes.

## Verification rules

`crypto policy verify` should check:

- every configured profile exists;
- every configured scope exists;
- profiles are allowed for the selected scope;
- profile docs exist;
- registry entries exist;
- algorithm docs use [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md);
- deprecated, rejected, or lab-only entries are not used in production scopes;
- non-claims are preserved.

## Audit expectations

Future mutation commands should emit redacted audit events containing:

- command name;
- old profile;
- new profile;
- scope;
- reason, when provided;
- whether confirmation was supplied;
- verification result;
- rollback guidance reference.

Audit output must not print secrets, private keys, seed material, tokens, or raw credentials.

## Configuration relationship

The command surface should read and validate the future scoped config model from [`CRYPTO_POLICY_ROADMAP.md`](CRYPTO_POLICY_ROADMAP.md).

Example:

```toml
[crypto]
default_profile = "safe-default"

[crypto.scope.base1]
profile = "high-security"
```

## Implementation phases

1. Documentation-only command plan.
2. Read-only `crypto status`, `crypto profiles`, and `crypto explain` commands.
3. Read-only `crypto policy verify` against docs and registry.
4. Config parsing with fail-closed validation.
5. Explicit profile selection with audit events.
6. Scoped integration with storage, transport, identity, Base1, plugins, logs/evidence, and Fyr/package flows.

## Non-claims

This command plan does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It defines the intended operator command surface for future cryptographic policy work.
