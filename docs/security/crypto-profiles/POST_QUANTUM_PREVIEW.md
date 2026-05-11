# Post-quantum-preview cryptographic profile

Status: planned preview profile
Scope: post-quantum cryptography planning, experimentation, and advanced operator review

## Purpose

The `post-quantum-preview` profile is the planned preview profile for post-quantum cryptographic policy exploration in Phase1.

It should make PQC-oriented options visible and reviewable without presenting them as mature defaults before implementation maturity, ecosystem compatibility, test coverage, and operational guidance are strong enough.

## Intended operator

- Advanced operators evaluating PQC readiness.
- Security reviewers tracking future cryptographic migration.
- Maintainers preparing long-term compatibility plans.
- Researchers and developers testing PQC-capable workflows.

## Security goal

The `post-quantum-preview` profile should help Phase1 become as secure as possible while maintaining practical usability during long-term cryptographic transition planning.

It should make post-quantum options inspectable without overstating quantum safety or silently replacing stable defaults.

## Allowed control points

Planned preview control points:

| Control point | Profile behavior |
| --- | --- |
| `transport` | Preview PQC-capable transport policy where ecosystem support exists. |
| `identity` | Preview PQC-capable signatures or hybrid identity approaches when documented. |
| `base1` | Preview long-term image provenance and recovery verification migration planning. |
| `logs/evidence` | Preview PQC-capable or hybrid signed evidence records. |
| `plugins` | Preview future package-signing migration options. |
| `fyr/packages` | Preview future package and lockfile signature migration options. |

Storage use should require special caution because failed migration or unsupported recovery can make data inaccessible.

## Registry requirements

No algorithm may enter this profile unless it is documented in:

- [`../CRYPTO_REGISTRY.md`](../CRYPTO_REGISTRY.md)
- [`../CRYPTO_ALGORITHM_TEMPLATE.md`](../CRYPTO_ALGORITHM_TEMPLATE.md)

Required status before profile use:

- `documented` for planning.
- `test-vector-covered` before implementation wiring.
- `profile-eligible` before operator-selectable preview use.

## Algorithm policy

The profile may include post-quantum or hybrid entries only when:

- the implementation provider is documented;
- maturity and compatibility notes are documented;
- test vectors or equivalent tests are linked;
- migration and rollback guidance is documented;
- the operator sees preview warnings;
- the profile remains explicitly selected, never silent default behavior.

The profile should reject:

- custom security-critical primitives;
- undocumented registry entries;
- lab-only entries presented as protection;
- unsupported claims of quantum safety;
- algorithms without test-vector coverage;
- silent migration of existing keys, signatures, or encrypted data;
- use for critical Base1 paths without recovery documentation.

## Operator controls

This profile must require explicit operator selection.

Future commands may include:

```text
crypto select post-quantum-preview --scope identity --confirm
crypto explain post-quantum-preview
crypto policy verify --profile post-quantum-preview
```

## Warning behavior

Activating this profile should show:

- affected control point;
- preview maturity warning;
- ecosystem compatibility warning;
- recovery and rollback warning;
- whether a hybrid mode is being used;
- safer default recommendation;
- whether new data, keys, signatures, or evidence will be created under preview policy.

## Audit and logging expectations

Future audit events should show:

- active profile;
- affected control point;
- whether the profile was operator-selected;
- implementation provider and registry entry identifiers;
- preview or hybrid status;
- migration recommendation.

Audit output must avoid printing secrets, private keys, seed material, tokens, or raw credentials.

## Migration and rollback guidance

Post-quantum migration must be deliberate.

Before this profile handles real data or signatures, docs must define:

- backup requirements;
- key migration path;
- rollback path;
- hybrid transition strategy;
- verification command;
- failure recovery;
- compatibility warning for systems that cannot verify the selected scheme.

No data, signature chain, recovery material, or Base1 evidence should be silently migrated under this profile.

## Base1 requirements

Base1 use of this profile should be preview-only until stronger evidence exists.

Any Base1 PQC preview planning should document:

- image provenance compatibility;
- recovery media verification compatibility;
- rollback metadata verification compatibility;
- signed validation report compatibility;
- operator recovery if a verifier cannot validate the selected scheme;
- exact non-claims around quantum safety, boot security, and hardware validation.

## Test and review requirements

Before this profile can be connected to runtime behavior, reviewers must confirm:

- all included registry entries are documented;
- all included entries have test vectors or equivalent tests;
- maturity and compatibility warnings are documented;
- preview status is visible in operator output;
- downgrade and rollback behavior is documented;
- non-claims are preserved;
- usability impact is documented.

## Non-claims

This profile does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It is a planned preview profile for post-quantum migration planning, not a default security profile.
