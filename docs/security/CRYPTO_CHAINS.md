# Crypto Chains

Status: planning  
Scope: scoped cryptographic policy chaining across Phase1/Base1 floors, nests, portals, analysis workspaces, recovery workspaces, and future service contexts

## Purpose

Crypto Chains define how Phase1/Base1 can assign cryptographic profiles, providers, and services to different operating contexts without claiming that runtime cryptographic isolation is complete.

The goal is operator-selectable cryptographic separation by scope.

## Core idea

A crypto chain is a declared relationship between a parent context and a child context.

Context examples:

- floor
- nest
- portal
- analysis workspace
- recovery workspace
- Base1 service scope
- future Fyr automation scope

## Example concept

```text
floor1
  crypto-profile: compatibility/blowfish-lab
  crypto-provider: documented-provider-a
  scope: local metadata only

portal alpha
  crypto-profile: safe-default
  crypto-provider: documented-provider-b
  scope: portal-local records

nest forensic-lab
  crypto-profile: high-security
  crypto-provider: documented-provider-c
  scope: isolated analysis records
```

## Planned chain fields

Each crypto chain should eventually declare:

- parent context
- child context
- selected crypto profile
- selected crypto provider
- selected crypto service family
- allowed data scope
- denied data scope
- audit behavior
- downgrade behavior
- fail-closed behavior
- verification status

## Guardrails

Crypto Chains are planning records only until runtime enforcement exists.

A documented crypto chain must not be presented as:

- completed cryptographic isolation
- hardware-backed secrecy
- formal sandboxing
- production-grade key management
- certified cryptographic compliance
- post-quantum security
- broad system hardening

## Safety model

Crypto Chains should prefer fail-closed behavior.

If a requested provider, profile, or service is unavailable, the chain should deny the operation instead of silently falling back to a weaker mode.

## Relationship to existing crypto docs

Crypto Chains depend on:

- `CRYPTO_POLICY_ROADMAP.md`
- `CRYPTO_PROVIDER_REGISTRY.md`
- `CRYPTO_PROVIDER_SERVICE_MATRIX.md`
- crypto profile planning docs
- operator command planning docs

## Current status

This document defines the planning model only.

Runtime chain selection, provider enforcement, service binding, audit logging, and Base1/Fyr integration remain future work.
