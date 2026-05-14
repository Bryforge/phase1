# Phase1 System Integrity Layer

Status: planning
Scope: native file integrity records, SHA-256 manifests, read-only validation, and operator-visible integrity reports

## Purpose

The system integrity layer defines how Phase1/Base1 can record and validate known file checksums for important project, runtime, documentation, fixture, and release surfaces.

The goal is evidence-bound integrity reporting, not a broad claim that the entire system is secure.

## Core model

The planned integrity layer should use explicit manifests that list files, expected SHA-256 values, size information, source path, generation time, and validation status.

A manifest can support:

- release artifact review;
- docs and fixture integrity checks;
- Base1 recovery bundle review;
- portal fixture review;
- crypto policy document review;
- future signed integrity reports.

## Planned operator commands

Future commands may include:

```text
integrity status
integrity manifest create --dry-run
integrity manifest verify --dry-run
integrity file <path>
integrity report
```

The first safe implementation should be read-only validation and report generation.

## Validation behavior

Integrity validation should report:

- file path;
- expected SHA-256;
- observed SHA-256;
- expected byte length;
- observed byte length;
- result label;
- manifest source;
- validation scope;
- non-claims.

Result labels should include:

- ok;
- changed;
- missing;
- extra;
- unreadable;
- manifest-invalid;
- not-checked.

## Safety rules

- Prefer SHA-256 or stronger reviewed digest algorithms.
- Do not use non-cryptographic checksums for security claims.
- Do not silently repair changed files.
- Do not delete extra files.
- Do not rewrite manifests unless the operator explicitly requests a generation flow.
- Keep validation read-only by default.
- Fail closed when a required manifest is missing or malformed.

## Relationship to Crypto Chains

Crypto Chain status should eventually identify which integrity manifest was used to verify policy files, provider/service matrix files, fixtures, and chain records.

A valid digest report does not prove runtime cryptographic enforcement.

## Non-claims

This document does not claim total system integrity, tamper-proof execution, hardware-backed measurement, secure boot, production hardening, certification, or complete compromise detection.

It defines a planned native integrity reporting layer based on explicit manifests and SHA-256 validation.
