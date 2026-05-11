# Phase1 cryptographic profiles

Status: planning index
Scope: operator-selectable cryptographic policy profiles

## Purpose

This directory will hold the documented cryptographic policy profiles that Phase1 may expose to operators in the future.

Profiles are documentation-first until implementation, tests, review, and validation exist.

## Security goal

Phase1 should be as secure as possible while maintaining practical usability.

Crypto profiles should provide safe defaults for normal users and explicit, reviewable choices for advanced operators without hiding risk or encouraging unsupported security claims.

## Profile index

| Profile | Status | Purpose |
| --- | --- | --- |
| [`safe-default`](SAFE_DEFAULT.md) | planned | Conservative default choices for normal operators. |
| [`high-security`](HIGH_SECURITY.md) | planned | Stricter choices for advanced operators. |
| [`compatibility`](COMPATIBILITY.md) | planned | Interoperability with warnings and explicit consent. |
| [`post-quantum-preview`](POST_QUANTUM_PREVIEW.md) | planned | PQC-oriented options with maturity warnings. |
| [`lab-only`](LAB_ONLY.md) | planned | Educational or experimental entries barred from production use. |

## Required profile documentation

Every profile document should define:

- intended operator;
- allowed control points;
- default algorithms or registry entries;
- unavailable or rejected algorithms;
- downgrade and compatibility warnings;
- confirmation requirements;
- audit/logging expectations;
- migration and rollback guidance;
- test and review requirements;
- non-claims.

## Profile safety rules

- Safe defaults should require the least operator decision-making.
- Advanced profiles should require explicit selection.
- Compatibility profiles should warn before weaker choices are used.
- Lab-only profiles must not protect real data or production workflows.
- No profile may claim audit completion, certification, quantum safety, or production hardening without evidence.
- No profile may use registry entries that lack documentation through `docs/security/CRYPTO_ALGORITHM_TEMPLATE.md`.

## Current profile drafts

- [`SAFE_DEFAULT.md`](SAFE_DEFAULT.md) — planned normal-operator default profile.
- [`HIGH_SECURITY.md`](HIGH_SECURITY.md) — planned advanced-operator and Base1-stricter profile.
- [`COMPATIBILITY.md`](COMPATIBILITY.md) — planned interoperability profile with warnings and explicit consent.
- [`POST_QUANTUM_PREVIEW.md`](POST_QUANTUM_PREVIEW.md) — planned post-quantum migration preview profile with maturity warnings.
- [`LAB_ONLY.md`](LAB_ONLY.md) — planned educational and experimental profile barred from production protection.

## Related docs

- [`../CRYPTO_POLICY_ROADMAP.md`](../CRYPTO_POLICY_ROADMAP.md)
- [`../CRYPTO_REGISTRY.md`](../CRYPTO_REGISTRY.md)
- [`../CRYPTO_ALGORITHM_TEMPLATE.md`](../CRYPTO_ALGORITHM_TEMPLATE.md)
- [`../TRUST_MODEL.md`](../TRUST_MODEL.md)

## Non-claims

This profile index does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It only defines the documentation structure for future operator-selectable cryptographic profiles.
