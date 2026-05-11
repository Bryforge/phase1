# Lab-only cryptographic profile

Status: planned profile
Scope: educational, experimental, and non-production cryptographic work

## Purpose

The `lab-only` profile is the planned profile for educational and experimental cryptographic work inside Phase1.

It exists so research, demos, learning exercises, and prototype implementations can be clearly separated from real security decisions.

## Intended operator

- Researchers.
- Educators.
- Developers testing ideas.
- Advanced operators reviewing non-production behavior.
- Contributors writing documentation or examples for cryptographic concepts.

## Security goal

The `lab-only` profile supports Phase1's goal of being as secure as possible while maintaining practical usability by keeping experimentation visible, labeled, and isolated.

It must never be confused with a profile that protects real data, Base1 boot material, recovery media, operator identity, production packages, or long-term evidence.

## Allowed control points

This profile should not be enabled for production control points.

Allowed use is limited to:

| Control point | Profile behavior |
| --- | --- |
| `lab` | Educational demos, examples, and experiments only. |
| `docs` | Documentation examples that are clearly labeled non-production. |
| `tests` | Unit tests and fixtures that do not protect real data. |

## Disallowed control points

The `lab-only` profile must not protect:

- `storage`
- `transport`
- `identity`
- `base1`
- `plugins`
- `logs/evidence`
- `fyr/packages`

Any future command that attempts to bind `lab-only` to a production control point should fail unless it is explicitly a test fixture or documentation-only path.

## Registry requirements

No algorithm may enter this profile unless it is documented in:

- [`../CRYPTO_REGISTRY.md`](../CRYPTO_REGISTRY.md)
- [`../CRYPTO_ALGORITHM_TEMPLATE.md`](../CRYPTO_ALGORITHM_TEMPLATE.md)

Required status before profile use:

- `lab-only` for educational/demo entries.
- `rejected` when the entry exists only to document why it must not be used.
- `documented` when the entry is a safe educational explanation but not runtime behavior.

## Algorithm policy

The profile may include:

- educational implementations;
- toy algorithms;
- rejected algorithms used as cautionary examples;
- simplified examples for documentation;
- prototype integrations that are not wired to real protection decisions.

The profile must reject:

- protecting real data;
- protecting Base1 material;
- protecting operator credentials or identity;
- production package signing;
- production transport or storage policy;
- silent promotion into another profile;
- any claim that lab behavior is secure, audited, certified, quantum-safe, or hardened.

## Operator controls

This profile must require explicit selection and should be unavailable in normal workflows.

Future commands may include:

```text
crypto select lab-only --scope lab --confirm
crypto explain lab-only
crypto policy verify --profile lab-only
```

Any attempt to select this profile should show a strong warning.

## Warning behavior

Activating this profile should show:

- non-production warning;
- affected control point;
- statement that real data must not be protected;
- statement that Base1 material must not be protected;
- rollback path to `safe-default`;
- audit logging notice.

## Audit and logging expectations

Future audit events should show:

- active profile;
- affected control point;
- explicit lab-only warning acknowledgement;
- whether the operation was docs-only, test-only, or demo-only.

Audit output must avoid printing secrets, private keys, seed material, tokens, or raw credentials.

## Migration and rollback guidance

No real data should migrate into `lab-only`.

If a lab-only experiment creates local artifacts, docs must define:

- artifact location;
- cleanup command;
- proof that no production control point was changed;
- rollback to `safe-default`.

## Test and review requirements

Before this profile can be connected to any runtime behavior, reviewers must confirm:

- all included registry entries are documented;
- all lab-only warnings are visible;
- production control points reject this profile;
- examples do not ask users to protect real data;
- non-claims are preserved;
- usability impact is documented.

## Non-claims

This profile does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It is a planned educational and experimental profile only. It must not be used as a real security profile.
