# Phase1 crypto provider registry

Status: planning registry
Scope: future cryptographic implementation providers, libraries, crates, and platform capabilities

## Purpose

This registry defines the documentation and review structure for cryptographic implementation providers that Phase1 may use in the future.

It is documentation-first. Listing a provider here does not approve it for production security claims.

## Security goal

Phase1 should be as secure as possible while maintaining practical usability.

For providers, this means:

- prefer reviewed open-source implementations;
- document provider source, versioning, license, and maintenance status;
- fail closed when a provider is unknown or unsupported;
- avoid silent provider substitution;
- require test-vector coverage before provider-backed behavior protects real data;
- keep operator-facing behavior clear and auditable.

## Provider entry requirements

Every provider entry must use [`CRYPTO_PROVIDER_TEMPLATE.md`](CRYPTO_PROVIDER_TEMPLATE.md) before it can be considered for a Phase1 crypto profile or runtime provider path.

Every provider entry must document:

- provider name;
- library, crate, or system API;
- upstream source;
- license;
- version or source pinning plan;
- maintenance status;
- supported platforms;
- supported algorithm families;
- supported profiles;
- supported control points;
- audit or review status;
- test-vector source;
- feature flags;
- failure behavior;
- known limitations;
- migration guidance;
- non-claims.

## Provider status labels

| Status | Meaning |
| --- | --- |
| `candidate` | Under documentation review only. |
| `documented` | Provider metadata is documented. |
| `test-vector-covered` | Provider behavior has test-vector coverage. |
| `profile-eligible` | Provider may be used by a non-default profile after review. |
| `default-eligible` | Provider may be used by default paths after stronger review. |
| `compatibility-only` | Provider may be used only for compatibility workflows. |
| `lab-only` | Provider is educational or experimental only. |
| `rejected` | Provider is documented as disallowed. |

## Provider selection rules

A future provider selector should:

- reject unknown providers;
- reject providers without registry entries;
- reject providers whose status is incompatible with the requested profile;
- reject providers whose platform support does not match the host;
- reject providers whose algorithms are not allowed for the requested control point;
- report why a provider was rejected;
- avoid silently falling back to a weaker provider;
- log provider choice in a redacted audit event when runtime behavior is connected.

## Provider metadata example

```toml
[provider.example]
name = "example-provider"
status = "candidate"
source = "upstream project URL or package name"
license = "document before use"
version_pin = "document before use"
profiles = ["safe-default"]
control_points = ["logs-evidence"]
algorithm_families = ["cryptographic hashes"]
test_vectors = "required before implementation"
```

This example is not an approval of any provider.

## Review checklist

Before a provider can be used by any profile, reviewers must confirm:

- provider metadata is complete;
- provider entry uses `CRYPTO_PROVIDER_TEMPLATE.md`;
- license is compatible;
- version/source pinning is defined;
- maintenance status is acceptable;
- supported algorithms are documented in `CRYPTO_REGISTRY.md`;
- algorithm docs use `CRYPTO_ALGORITHM_TEMPLATE.md`;
- test vectors or equivalent tests exist;
- failure behavior is fail-closed;
- provider fallback behavior is explicit;
- non-claims are preserved.

## Initial registry state

No provider is currently approved by this registry for new production security claims.

The first implementation work should create documentation-only provider entries and tests before connecting providers to runtime behavior.

## Related docs

- [`CRYPTO_IMPLEMENTATION_PLAN.md`](CRYPTO_IMPLEMENTATION_PLAN.md)
- [`CRYPTO_POLICY_ROADMAP.md`](CRYPTO_POLICY_ROADMAP.md)
- [`CRYPTO_REGISTRY.md`](CRYPTO_REGISTRY.md)
- [`CRYPTO_CONFIG_SCHEMA.md`](CRYPTO_CONFIG_SCHEMA.md)
- [`CRYPTO_OPERATOR_COMMANDS.md`](CRYPTO_OPERATOR_COMMANDS.md)
- [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md)
- [`CRYPTO_PROVIDER_TEMPLATE.md`](CRYPTO_PROVIDER_TEMPLATE.md)
- [`crypto-profiles/README.md`](crypto-profiles/README.md)

## Non-claims

This provider registry does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It defines the planned provider metadata and review structure for future cryptographic implementation work.

## Related provider/service planning

- [`CRYPTO_PROVIDER_SERVICE_MATRIX.md`](CRYPTO_PROVIDER_SERVICE_MATRIX.md) — maps provider categories to planned Phase1/Base1 crypto service families and operator-selectable profile boundaries.
