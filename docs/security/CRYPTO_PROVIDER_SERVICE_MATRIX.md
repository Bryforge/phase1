# Phase1/Base1 crypto provider service matrix

Status: planning matrix
Scope: open-source cryptographic providers, algorithm families, security services, operator-selectable profiles, and future Phase1/Base1 control points

## Purpose

Phase1 and Base1 should support major reviewed open-source cryptographic systems, providers, algorithms, modes, and profile families through an operator-selectable policy layer.

The goal is broad native crypto capability with safe defaults, explicit provider choice, explicit profile choice, test-vector evidence, and no unsupported security claims.

## Boundary

This matrix is documentation-first.

It does not approve runtime cryptographic protection, production hardening, compliance posture, national-security suitability, or secure deployment claims.

No provider, algorithm, profile, or service is runtime-approved until documentation, tests, vectors, failure behavior, and review are complete.

## Standing rules

- Prefer reviewed open-source providers.
- Reject custom security-critical primitives.
- Reject unknown providers.
- Reject undocumented providers.
- Fail closed on unsupported profile/provider/scope combinations.
- Avoid silent provider substitution.
- Require test vectors before provider-backed runtime behavior protects real data.
- Require explicit operator selection for advanced profiles.
- Preserve non-claims until evidence supports promotion.

## Security service families

| Service family | Purpose | Default posture |
| --- | --- | --- |
| identity | identity, signing, attestation, and trust records | planned |
| storage | file, VFS, artifact, and backup protection | planned |
| transport | local and future network transport protection | planned |
| update | release metadata, update verification, and provenance | planned |
| logs-evidence | audit logs, evidence bundles, and integrity records | planned |
| recovery | Base1 recovery bundle verification and rollback evidence | planned |
| analysis | metadata-only analysis records and report signing | planned |
| developer | build, test, and package verification | planned |

## Provider categories

| Provider category | Examples | Status |
| --- | --- | --- |
| RustCrypto ecosystem | hashes, AEADs, signatures, KDFs where appropriate | candidate |
| ring/aws-lc family | TLS-adjacent and primitive-backed provider options | candidate |
| OpenSSL/LibreSSL/BoringSSL family | compatibility and platform-provider options | candidate |
| libsodium family | modern high-level crypto APIs | candidate |
| age/minisign/signify family | file encryption and signing workflows | candidate |
| TPM/platform keystore | hardware-backed key storage where available | candidate |
| kernel/platform crypto APIs | platform capability provider | candidate |
| post-quantum provider family | ML-KEM, ML-DSA, SLH-DSA capable providers after review | candidate |

These examples are not approvals.

## Non-claims

This document does not claim that Phase1 or Base1 currently provides finished cryptographic hardening, approved national-security crypto, compliance posture, secure communications, secure storage, or production-ready cryptographic enforcement.
