# Compatibility cryptographic profile

Status: planned profile
Scope: interoperability-focused cryptographic policy

## Purpose

The `compatibility` profile is the planned interoperability profile for Phase1 operators who need to communicate with older systems, legacy data, or external tooling.

It should make compatibility possible without weakening the default security posture or hiding risk from the operator.

## Intended operator

- Maintainers working with older data or external systems.
- Advanced operators who understand downgrade risk.
- Migration operators moving data from old formats into safer profiles.
- Developers testing interoperability behavior.

## Security goal

The `compatibility` profile should make Phase1 as secure as possible while maintaining practical usability for interoperability workflows.

It should treat weaker or older choices as explicit exceptions, not defaults.

## Allowed control points

Compatibility behavior may be considered for these control points only when clearly documented:

| Control point | Profile behavior |
| --- | --- |
| `transport` | Interoperate with external systems where safer options are unavailable. |
| `storage` | Read or migrate older protected data into safer profiles. |
| `plugins` | Verify older package or manifest formats when migration is needed. |
| `fyr/packages` | Read older package metadata or lockfile formats for migration. |
| `logs/evidence` | Verify older evidence records without creating new weak records by default. |

Base1 critical paths should avoid this profile unless a recovery-specific document explicitly justifies the exception.

## Registry requirements

No algorithm may enter this profile unless it is documented in:

- [`../CRYPTO_REGISTRY.md`](../CRYPTO_REGISTRY.md)
- [`../CRYPTO_ALGORITHM_TEMPLATE.md`](../CRYPTO_ALGORITHM_TEMPLATE.md)

Required status before profile use:

- `documented` for review.
- `compatibility-only` before compatibility use.
- `test-vector-covered` before implementation wiring.

## Algorithm policy

The profile may include compatibility-only entries only when:

- the use case is documented;
- safer alternatives are documented;
- migration guidance is documented;
- the operator sees a warning;
- the operator explicitly confirms the compatibility path;
- audit logging records the exception.

The profile should reject:

- custom security-critical primitives;
- undocumented registry entries;
- lab-only entries;
- rejected entries;
- silent downgrade behavior;
- creating new long-term data with weaker choices when a safer profile is available.

## Operator controls

This profile must require explicit selection.

Future commands may include:

```text
crypto select compatibility --scope transport --confirm
crypto explain compatibility
crypto policy verify --profile compatibility
```

## Warning behavior

Activating this profile should show:

- affected control point;
- reason for compatibility mode;
- safer profile recommendation;
- migration path back to `safe-default` or `high-security`;
- whether new data will be created under compatibility mode.

## Audit and logging expectations

Future audit events should show:

- active profile;
- affected control point;
- compatibility reason;
- whether the profile was operator-selected;
- whether data was read, verified, converted, or newly written;
- migration recommendation.

Audit output must avoid printing secrets, private keys, seed material, tokens, or raw credentials.

## Migration and rollback guidance

Compatibility workflows should prioritize migration into safer profiles.

Before this profile handles real data, docs must define:

- backup requirements;
- migration target profile;
- rollback path;
- verification command;
- failure recovery;
- warning text for data that cannot be migrated safely.

## Test and review requirements

Before this profile can be connected to runtime behavior, reviewers must confirm:

- all included registry entries are documented;
- all included entries are marked `compatibility-only` or otherwise clearly constrained;
- test vectors or equivalent compatibility tests exist;
- warnings and confirmations are documented;
- migration guidance is documented;
- non-claims are preserved;
- usability impact is documented.

## Non-claims

This profile does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It is a planned interoperability profile for future implementation and review, not a default security profile.
