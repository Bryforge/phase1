# Phase1 cryptographic policy roadmap

Status: planning roadmap
Scope: cryptographic policy, documentation, implementation choices, and advanced operator configuration

## Purpose

Phase1 should grow toward a documented cryptographic policy layer where advanced operators can inspect and intentionally choose approved cryptographic profiles at each relevant point of control.

The goal is broad cryptographic capability with safe defaults, not unsafe custom cryptography.

## Security goal

Phase1 should be as secure as possible while maintaining practical usability.

For cryptography, this means:

- use reviewed open-source libraries and established algorithms where possible;
- keep safe defaults for normal users;
- give advanced operators explicit profile controls;
- document every supported algorithm, mode, parameter, and tradeoff;
- keep deprecated or legacy algorithms out of default profiles;
- require tests, vectors, and review before any cryptographic claim is strengthened.

## Non-goal: inventing primitives

Phase1 should not invent new cryptographic primitives for security-critical use.

Educational examples may exist only when clearly labeled as lab-only and separated from real protection decisions.

## Cryptographic capability registry

Phase1 should maintain a registry of supported cryptographic capabilities. The initial registry index is [`CRYPTO_REGISTRY.md`](CRYPTO_REGISTRY.md).

Each registry entry should include:

- algorithm or design name;
- implementation provider;
- license and source location;
- security status;
- parameter set;
- allowed use cases;
- disallowed use cases;
- test vectors;
- documentation references;
- default, advanced, compatibility, deprecated, or lab-only classification;
- migration guidance.

Use [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md) for each algorithm, design, profile component, or implementation provider added to the registry.

## Operator-selectable profiles

Phase1 should support named cryptographic profiles. The profile planning index is [`crypto-profiles/README.md`](crypto-profiles/README.md).

| Profile | Purpose |
| --- | --- |
| `safe-default` | Conservative default choices for normal use. |
| `high-security` | Stricter choices for advanced operators. |
| `compatibility` | Interoperability with clear warnings. |
| `post-quantum-preview` | Experimental or early PQC-capable options with maturity notes. |
| `lab-only` | Educational experiments barred from production use. |

## Operator command surface

The future operator command surface is planned in [`CRYPTO_OPERATOR_COMMANDS.md`](CRYPTO_OPERATOR_COMMANDS.md).

Future operator commands may include:

```text
crypto profiles
crypto status
crypto select safe-default
crypto select high-security --scope storage
crypto select post-quantum-preview --scope transport --confirm
crypto explain <algorithm>
crypto policy export
crypto policy verify
```

## Points of control

Operators should be able to choose policy by control point.

| Control point | Examples |
| --- | --- |
| Storage | encrypted state, signed metadata, rollback records |
| Transport | future sync, update checks, API clients |
| Identity | operator keys, device identity, signatures |
| Boot/Base1 | image signatures, checksums, recovery media verification |
| Plugins | package signatures, manifests, capability grants |
| Logs/evidence | signed reports and tamper-evident records |
| Fyr/packages | package signing, lockfiles, dependency integrity |

## Algorithm families to inventory

The registry should cover these families over time:

- random number generation and entropy handling;
- cryptographic hashes;
- message authentication;
- key derivation and password hashing;
- symmetric encryption and authenticated encryption;
- public-key signatures;
- public-key encryption and key encapsulation;
- key agreement;
- post-quantum cryptography;
- threshold and multisignature designs;
- authenticated data structures and Merkle proofs;
- non-cryptographic checksums where security is not claimed.

## Documentation requirement

Every supported algorithm page should use [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md) and document:

1. Status.
2. Purpose.
3. Implementation provider.
4. Parameters and limits.
5. Security notes.
6. Usability notes.
7. Test vectors.
8. Migration and rotation guidance.
9. Non-claims.

Every profile should follow the structure in [`crypto-profiles/README.md`](crypto-profiles/README.md) before it is connected to runtime behavior.

## Configuration model

Future configuration should use explicit scopes.

```toml
[crypto]
default_profile = "safe-default"

[crypto.scope.storage]
profile = "high-security"

[crypto.scope.transport]
profile = "safe-default"

[crypto.scope.base1]
profile = "high-security"
```

The active policy should be visible from inside Phase1:

```text
crypto status
```

## Safety gates

Crypto profile changes should:

- show current and proposed profile;
- show affected control points;
- warn for compatibility or lab-only profiles;
- require confirmation for downgrades;
- log a redacted audit event;
- provide rollback guidance;
- avoid silent data migration without backup and recovery documentation.

## Phase plan

### Phase 1: registry and docs

- Create registry format: [`CRYPTO_REGISTRY.md`](CRYPTO_REGISTRY.md).
- Create algorithm documentation template: [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md).
- Create profile planning index: [`crypto-profiles/README.md`](crypto-profiles/README.md).
- Create operator command plan: [`CRYPTO_OPERATOR_COMMANDS.md`](CRYPTO_OPERATOR_COMMANDS.md).
- Draft safe-default, high-security, compatibility, post-quantum-preview, and lab-only profiles.
- Add tests that preserve non-claims.

### Phase 2: policy engine

- Add `crypto status` and `crypto profiles` commands.
- Add `crypto explain` for profiles and registry entries.
- Parse crypto profile config.
- Reject unknown algorithms and scopes.
- Emit audit events for profile changes.

### Phase 3: integration points

- Storage policy.
- Update and release verification policy.
- Plugin and package signature policy.
- Base1 image provenance and recovery verification policy.

### Phase 4: advanced controls

- Scoped profile selection.
- Policy export and verification.
- Migration and rotation commands.

### Phase 5: review readiness

- Algorithm inventory reports.
- Test vector coverage.
- Dependency review.
- External audit checklist.

## Base1 alignment

Base1 cryptographic policy should be stricter than learning or demo flows.

Base1 crypto planning should prioritize image provenance, recovery media verification, rollback metadata integrity, signed validation reports, and clear recovery paths when keys or signatures fail.

## Non-claims

This roadmap does not make Phase1 or Base1 cryptographically complete, audited, quantum-safe, certified, hardware-validated, installer-ready, or daily-driver ready.

It defines a plan for safer cryptographic capability, documentation, and operator-selectable policy.
