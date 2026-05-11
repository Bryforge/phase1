# Safe-default cryptographic profile

Status: planned profile
Scope: normal operator cryptographic defaults

## Purpose

The `safe-default` profile is the planned default cryptographic profile for normal Phase1 operators.

It should provide conservative, reviewed, broadly compatible choices without requiring the operator to understand every cryptographic detail before using Phase1 safely.

## Intended operator

- Normal operators.
- First-time users.
- Local development users.
- Users who want secure defaults without advanced tuning.

## Security goal

The `safe-default` profile should make Phase1 as secure as possible while maintaining practical usability.

It should minimize risky choices, avoid legacy algorithms, and keep security decisions visible without making normal operation confusing or fragile.

## Allowed control points

Planned default control points:

| Control point | Profile behavior |
| --- | --- |
| `storage` | Conservative encryption/signing choices when storage crypto exists. |
| `transport` | Conservative defaults for future update checks, sync, and API clients. |
| `identity` | Conservative defaults for operator/device signing. |
| `plugins` | Conservative defaults for package and manifest verification. |
| `logs/evidence` | Conservative defaults for signed reports and tamper-evident records. |
| `fyr/packages` | Conservative defaults for package integrity and lockfiles. |

Base1 may use `high-security` defaults where stricter verification is appropriate.

## Registry requirements

No algorithm may enter this profile unless it is documented in:

- [`../CRYPTO_REGISTRY.md`](../CRYPTO_REGISTRY.md)
- [`../CRYPTO_ALGORITHM_TEMPLATE.md`](../CRYPTO_ALGORITHM_TEMPLATE.md)

Required status before profile use:

- `documented` minimum for planning.
- `test-vector-covered` before implementation wiring.
- `default-eligible` before default use.

## Default algorithm policy

The profile should prefer:

- well-reviewed open-source implementations;
- established algorithms with documented parameters;
- misuse-resistant modes when available;
- clear key/nonce/parameter limits;
- documented migration and rotation paths;
- algorithms with standard test vectors;
- algorithms that do not require unusual operator decisions.

The profile should reject:

- custom security-critical primitives;
- undocumented registry entries;
- deprecated entries;
- compatibility-only entries unless explicitly selected outside this profile;
- lab-only entries;
- algorithms without test-vector coverage for production-facing behavior.

## Operator controls

Normal users should not need to configure this profile manually.

Future commands may include:

```text
crypto status
crypto profiles
crypto explain safe-default
```

Profile switching away from `safe-default` should be explicit and auditable.

## Downgrade behavior

Any move from `safe-default` to `compatibility` or `lab-only` must require:

- a warning;
- affected control-point display;
- explicit confirmation;
- audit logging;
- rollback guidance.

## Audit and logging expectations

Profile use should be inspectable.

Future audit events should show:

- active profile;
- affected control point;
- whether the profile was default or operator-selected;
- whether a downgrade or compatibility exception occurred.

Audit output must avoid printing secrets, private keys, seed material, tokens, or raw credentials.

## Migration and rollback guidance

Before any data is migrated under this profile, docs must define:

- backup requirements;
- rollback path;
- key rotation path;
- failure recovery;
- compatibility warnings.

No encrypted data should be silently migrated without operator-visible recovery guidance.

## Test and review requirements

Before this profile can protect real data, reviewers must confirm:

- all included registry entries are documented;
- all included registry entries have test vectors or equivalent tests;
- unsafe, deprecated, compatibility-only, and lab-only entries are excluded;
- downgrade behavior is guarded;
- non-claims are preserved;
- usability impact is documented.

## Non-claims

This profile does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It is a planned default policy profile for future implementation and review.
