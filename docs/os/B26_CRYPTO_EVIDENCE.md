# B26 crypto evidence plan

Status: planning scaffold

Scope: Phase1 runtime evidence hashing and identity preparation.

## Purpose

B26 begins the cryptography layer in a practical way: evidence manifests first, signing later.

This fits the current order:

1. B23 GNU/Linux runtime shell;
2. B24 workspace and evidence paths;
3. B25 supervisor lane plan;
4. B26 evidence hashing and identity groundwork.

## Initial cryptography model

B26 starts with deterministic SHA-256 manifests for Phase1 evidence paths.

The first useful artifact is:

`phase1-evidence-manifest.sha256`

This hashes files under a selected evidence directory and records runtime metadata.

## Why hash before signing

Hashing is immediately useful in the initramfs runtime and does not require persistent private key storage.

Signing should wait until Phase1 has a safe key-storage decision:

- ephemeral runtime identity;
- external USB identity file;
- operator-provided key;
- later hardware-backed identity.

## Success states

`phase1_evidence_hash_manifest_seen`

The runtime created a SHA-256 evidence manifest.

`phase1_crypto_evidence_seen`

The runtime exposed the crypto/evidence workflow and produced the expected manifest and metadata.

## Non-claims

B26 does not claim production key management, secure boot, measured boot, hardening, release-candidate readiness, or daily-driver readiness.
