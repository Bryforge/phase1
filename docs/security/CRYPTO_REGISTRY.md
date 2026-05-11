# Phase1 cryptographic capability registry

Status: planning registry
Scope: cryptographic algorithms, designs, providers, profiles, and operator-selectable policy

## Purpose

This registry is the future source-of-truth index for cryptographic capabilities that Phase1 may expose through operator-selectable policy.

It is documentation-first. It does not make any algorithm production-ready by listing it here.

## Registry rule

Every registry entry must use [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md) before it can be considered for a Phase1 crypto profile.

Every entry must document:

- status;
- allowed use cases;
- disallowed use cases;
- implementation provider;
- parameters and limits;
- test vectors;
- migration guidance;
- usability impact;
- non-claims.

## Profile classes

| Profile | Status | Purpose |
| --- | --- | --- |
| `safe-default` | planned | Conservative defaults for normal operators. |
| `high-security` | planned | Stricter choices for advanced operators. |
| `compatibility` | planned | Interoperability with warnings and explicit consent. |
| `post-quantum-preview` | planned | PQC-oriented options with maturity warnings. |
| `lab-only` | planned | Educational or experimental entries barred from production use. |

## Control-point registry

| Control point | Default profile | Status | Notes |
| --- | --- | --- | --- |
| `storage` | `safe-default` | planned | Local state, metadata, rollback records, future vaults. |
| `transport` | `safe-default` | planned | Future sync, update checks, and API clients. |
| `identity` | `safe-default` | planned | Operator/device keys, signatures, attestations. |
| `base1` | `high-security` | planned | Image provenance, recovery media, rollback metadata. |
| `plugins` | `safe-default` | planned | Package signatures, manifests, capability grants. |
| `logs/evidence` | `safe-default` | planned | Signed reports and tamper-evident records. |
| `fyr/packages` | `safe-default` | planned | Package signing, lockfiles, dependency integrity. |

## Algorithm family registry

Entries should be added under these families over time.

| Family | Status | Notes |
| --- | --- | --- |
| Entropy and random generation | planned | Must document OS/provider assumptions. |
| Cryptographic hashes | planned | Must distinguish security hashes from checksums. |
| Message authentication | planned | Must document key management requirements. |
| Key derivation and password hashing | planned | Must document parameters and rotation guidance. |
| Symmetric encryption and AEAD | planned | Must document nonce, tag, and message limits. |
| Public-key signatures | planned | Must document key sizes, formats, and verification rules. |
| Public-key encryption and KEMs | planned | Must document maturity and use boundaries. |
| Key agreement | planned | Must document authentication requirements. |
| Post-quantum cryptography | planned | Must document maturity and fallback strategy. |
| Threshold and multisignature designs | planned | Must document recovery and operator complexity. |
| Authenticated data structures | planned | Must document proof format and validation rules. |
| Non-cryptographic checksums | planned | Must not be described as security primitives. |

## Entry status labels

| Status | Meaning |
| --- | --- |
| `candidate` | Under documentation review only. |
| `documented` | Template complete, not integrated. |
| `test-vector-covered` | Standard vectors or equivalent tests are linked. |
| `profile-eligible` | Eligible for a non-default profile after review. |
| `default-eligible` | Eligible for default use after stronger review. |
| `compatibility-only` | Allowed only for interoperability with warnings. |
| `deprecated` | Should not be used for new data. |
| `rejected` | Documented as disallowed. |
| `lab-only` | Educational/demo use only. |

## Required review before profile use

Before an entry can be used by any profile, reviewers must confirm:

- implementation source is known;
- license is compatible;
- status label is accurate;
- allowed and disallowed uses are clear;
- parameters are documented;
- standard test vectors or equivalent tests exist;
- usability impact is documented;
- migration and rollback are documented;
- non-claims are preserved.

## Initial registry state

No algorithm is currently approved by this registry for new production security claims.

The first implementation work should create documentation-only entries and tests before connecting entries to runtime behavior.

## Non-claims

This registry does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It is a planning index for safer cryptographic documentation and future operator-selectable policy.
