# High-security cryptographic profile

Status: planned profile
Scope: advanced operator cryptographic policy

## Purpose

The `high-security` profile is the planned stricter cryptographic profile for advanced Phase1 and Base1 operators.

It should prioritize stronger review requirements, stricter defaults, clearer failure behavior, and conservative compatibility decisions while preserving practical usability for operators who understand the tradeoffs.

## Intended operator

- Advanced operators.
- Security reviewers.
- Base1 recovery and image-provenance operators.
- Maintainers preparing signed releases, validation bundles, or evidence reports.

## Security goal

The `high-security` profile should make Phase1 and Base1 cryptographic decisions as secure as possible while maintaining practical usability for advanced operators.

It should increase assurance without turning normal operation into guesswork or blocking recovery paths.

## Allowed control points

Planned high-security control points:

| Control point | Profile behavior |
| --- | --- |
| `storage` | Stricter encryption/signing choices when storage crypto exists. |
| `identity` | Stricter signing and key-handling requirements. |
| `base1` | Stricter image provenance, recovery media, rollback, and validation report policy. |
| `logs/evidence` | Stronger signed-report and tamper-evidence expectations. |
| `plugins` | Stricter package, manifest, and capability verification. |
| `fyr/packages` | Stricter package signing and dependency-integrity expectations. |

The `transport` control point may use this profile when compatibility is not the primary concern.

## Registry requirements

No algorithm may enter this profile unless it is documented in:

- [`../CRYPTO_REGISTRY.md`](../CRYPTO_REGISTRY.md)
- [`../CRYPTO_ALGORITHM_TEMPLATE.md`](../CRYPTO_ALGORITHM_TEMPLATE.md)

Required status before profile use:

- `test-vector-covered` before implementation wiring.
- `profile-eligible` before advanced profile use.
- `default-eligible` before this profile is used automatically for Base1-critical paths.

## Algorithm policy

The profile should prefer:

- reviewed open-source implementations;
- stronger parameter sets where practical;
- conservative algorithm lifetimes;
- explicit key rotation and migration guidance;
- clear failure behavior;
- standard test vectors;
- documented implementation provenance;
- stricter rejection of ambiguous or legacy choices.

The profile should reject:

- custom security-critical primitives;
- undocumented registry entries;
- deprecated entries;
- compatibility-only entries unless explicitly overridden outside this profile;
- lab-only entries;
- algorithms without test-vector coverage;
- silent downgrade behavior;
- any algorithm whose operational recovery path is undocumented.

## Operator controls

This profile should require explicit operator selection unless a Base1 control point declares it as the planned stricter default.

Future commands may include:

```text
crypto select high-security --scope base1 --confirm
crypto explain high-security
crypto policy verify --profile high-security
```

## Downgrade behavior

Any move from `high-security` to `safe-default`, `compatibility`, or `lab-only` must require:

- a warning;
- affected control-point display;
- explicit confirmation;
- audit logging;
- rollback guidance;
- a reason field in any future policy report.

## Audit and logging expectations

Future audit events should show:

- active profile;
- affected control point;
- whether the profile was operator-selected or default for that scope;
- whether a downgrade, compatibility exception, or lab-only exception occurred;
- verification status when available.

Audit output must avoid printing secrets, private keys, seed material, tokens, or raw credentials.

## Migration and rollback guidance

Before this profile protects real data, docs must define:

- backup requirements;
- rollback path;
- key rotation path;
- failed-verification recovery behavior;
- compatibility warnings;
- minimum evidence required before migration.

No encrypted or signed data should be silently migrated without operator-visible recovery guidance.

## Base1 requirements

Base1 use of this profile should require stricter evidence than general Phase1 use.

Base1 profile decisions should document:

- image provenance requirements;
- recovery media verification requirements;
- rollback metadata integrity requirements;
- signed validation report requirements;
- operator recovery if keys or signatures fail;
- exact non-claims around boot security and hardware validation.

## Test and review requirements

Before this profile can protect real data, reviewers must confirm:

- all included registry entries are documented;
- all included registry entries have test vectors or equivalent tests;
- unsafe, deprecated, compatibility-only, and lab-only entries are excluded;
- downgrade behavior is guarded;
- recovery behavior is documented;
- non-claims are preserved;
- usability impact is documented.

## Non-claims

This profile does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It is a planned advanced-operator policy profile for future implementation and review.
