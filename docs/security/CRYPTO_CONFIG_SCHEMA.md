# Phase1 crypto policy configuration schema

Status: planning schema
Scope: future cryptographic profile configuration and validation

## Purpose

This document defines the planned configuration model for Phase1 cryptographic policy.

It is documentation-first. It does not implement runtime cryptographic behavior yet.

## Security goal

The configuration model should make Phase1 as secure as possible while maintaining practical usability.

Normal operators should inherit safe defaults. Advanced operators should be able to configure scoped crypto profiles intentionally, with validation, warnings, and rollback guidance.

## Default posture

The default posture should be:

```toml
[crypto]
default_profile = "safe-default"
```

If no crypto configuration exists, Phase1 should behave as though `safe-default` is active for normal scopes.

No configuration should silently enable compatibility, post-quantum-preview, or lab-only behavior.

## Planned TOML schema

```toml
[crypto]
default_profile = "safe-default"
policy_version = 1

[crypto.scope.storage]
profile = "safe-default"
reason = "default local state policy"

[crypto.scope.transport]
profile = "safe-default"
reason = "default transport policy"

[crypto.scope.identity]
profile = "safe-default"
reason = "default identity policy"

[crypto.scope.base1]
profile = "high-security"
reason = "stricter Base1 provenance and recovery policy"

[crypto.scope.plugins]
profile = "safe-default"
reason = "default plugin package policy"

[crypto.scope.logs-evidence]
profile = "safe-default"
reason = "default signed evidence policy"

[crypto.scope.fyr-packages]
profile = "safe-default"
reason = "default Fyr package integrity policy"
```

## Allowed profile values

Allowed values come from [`crypto-profiles/README.md`](crypto-profiles/README.md):

```text
safe-default
high-security
compatibility
post-quantum-preview
lab-only
```

Unknown profiles must fail closed.

## Allowed scope values

Allowed production scope values come from [`CRYPTO_REGISTRY.md`](CRYPTO_REGISTRY.md):

```text
storage
transport
identity
base1
plugins
logs-evidence
fyr-packages
```

Allowed non-production scopes are:

```text
lab
docs
tests
```

Unknown scopes must fail closed.

## Scope naming rule

Configuration keys should use TOML-friendly names:

| Registry name | Config key |
| --- | --- |
| `logs/evidence` | `logs-evidence` |
| `fyr/packages` | `fyr-packages` |

Operator output may display the original registry names, but config should avoid slash-delimited keys.

## Validation rules

A future config validator should reject:

- unknown profiles;
- unknown scopes;
- `lab-only` outside `lab`, `docs`, or `tests`;
- `compatibility` without a reason;
- `post-quantum-preview` without a reason;
- production scopes using deprecated, rejected, or lab-only registry entries;
- missing profile docs;
- profile docs that do not link the registry and algorithm template;
- algorithm docs that do not use [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md).

## Warning rules

The validator should warn when:

- a scope is configured away from `safe-default`;
- `compatibility` is selected;
- `post-quantum-preview` is selected;
- a Base1 scope is configured below `high-security`;
- profile migration may require backup, rollback, or key rotation.

Warnings should be visible before any mutating profile change.

## Audit expectations

Future config changes should produce a redacted audit event containing:

- old profile;
- new profile;
- scope;
- reason;
- whether the change is an upgrade, downgrade, compatibility exception, preview selection, or lab-only selection;
- confirmation status;
- verification result.

Audit output must not include secrets, private keys, seed material, tokens, or raw credentials.

## Example safe configuration

```toml
[crypto]
default_profile = "safe-default"
policy_version = 1

[crypto.scope.base1]
profile = "high-security"
reason = "Base1 provenance and recovery policy"
```

## Example rejected configuration

```toml
[crypto.scope.storage]
profile = "lab-only"
reason = "testing"
```

This should be rejected because `lab-only` must not protect production storage.

## Related docs

- [`CRYPTO_POLICY_ROADMAP.md`](CRYPTO_POLICY_ROADMAP.md)
- [`CRYPTO_REGISTRY.md`](CRYPTO_REGISTRY.md)
- [`CRYPTO_OPERATOR_COMMANDS.md`](CRYPTO_OPERATOR_COMMANDS.md)
- [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md)
- [`crypto-profiles/README.md`](crypto-profiles/README.md)

## Non-claims

This schema does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It defines the planned configuration structure for future cryptographic policy validation.
