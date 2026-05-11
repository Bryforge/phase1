# Phase1 crypto implementation plan

Status: planning design
Scope: future cryptographic policy engine, provider integration, validation, and operator controls

## Purpose

This document turns the cryptographic policy roadmap into an implementation sequence.

The goal is to implement cryptographic capability safely: policy first, registry second, provider integration third, and real protection only after documentation, tests, review, and validation are in place.

## Security goal

Phase1 should be as secure as possible while maintaining practical usability.

For implementation work, this means:

- do not invent custom security-critical primitives;
- prefer reviewed open-source libraries and established algorithms;
- keep `safe-default` as the normal profile;
- make advanced profile choices explicit and auditable;
- fail closed on unknown profiles, scopes, algorithms, and providers;
- keep lab-only behavior isolated from real protection decisions;
- preserve rollback and recovery paths before protecting real data.

## Implementation order

Implementation should proceed in this order:

1. Documentation and registry surface.
2. Read-only command surface.
3. Config parser and validator.
4. Provider abstraction.
5. Test-vector harness.
6. Profile policy engine.
7. Scoped integration points.
8. Migration and rollback tooling.
9. External review and audit preparation.

No runtime control point should use cryptographic policy until the earlier phases are complete for that scope.

## Phase 1: documentation and registry surface

Required docs:

- [`CRYPTO_POLICY_ROADMAP.md`](CRYPTO_POLICY_ROADMAP.md)
- [`CRYPTO_REGISTRY.md`](CRYPTO_REGISTRY.md)
- [`CRYPTO_PROVIDER_REGISTRY.md`](CRYPTO_PROVIDER_REGISTRY.md)
- [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md)
- [`CRYPTO_OPERATOR_COMMANDS.md`](CRYPTO_OPERATOR_COMMANDS.md)
- [`CRYPTO_CONFIG_SCHEMA.md`](CRYPTO_CONFIG_SCHEMA.md)
- [`crypto-profiles/README.md`](crypto-profiles/README.md)

Exit criteria:

- all docs are linked from the security index;
- all docs are checked by `scripts/security-crypto-doc-integrity.sh`;
- profile drafts preserve non-claims;
- no algorithm is approved for production claims by documentation alone;
- no provider is approved for production claims by documentation alone.

## Phase 2: read-only command surface

Implement read-only commands first:

```text
crypto status
crypto profiles
crypto explain <profile-or-algorithm>
crypto policy verify
```

Initial behavior should be documentation-backed and non-mutating.

Exit criteria:

- commands do not change config or data;
- unknown profiles fail closed;
- unknown scopes fail closed;
- lab-only restrictions are visible;
- output links or names the relevant docs;
- tests cover help text, profile listing, and non-claims.

## Phase 3: config parser and validator

Implement parsing for the schema in [`CRYPTO_CONFIG_SCHEMA.md`](CRYPTO_CONFIG_SCHEMA.md).

Validation must reject:

- unknown profiles;
- unknown scopes;
- `lab-only` outside `lab`, `docs`, or `tests`;
- `compatibility` without a reason;
- `post-quantum-preview` without a reason;
- production scopes using deprecated, rejected, or lab-only entries;
- missing profile docs;
- missing registry entries.

Exit criteria:

- parser is read-only by default;
- invalid config fails closed;
- warnings are visible;
- tests cover accepted and rejected examples;
- no cryptographic operation is performed yet.

## Phase 4: provider abstraction

Add a provider layer that can map registry entries to reviewed implementation providers.

Provider metadata and review requirements are tracked in [`CRYPTO_PROVIDER_REGISTRY.md`](CRYPTO_PROVIDER_REGISTRY.md).

Provider entries should include:

- provider name;
- library or crate name;
- version or source pinning plan;
- license;
- supported algorithms;
- supported platforms;
- audit/review status;
- test-vector source;
- feature flags, if any;
- failure behavior.

Exit criteria:

- provider metadata is inspectable;
- unsupported providers fail closed;
- providers are not silently selected;
- tests verify provider metadata and rejection behavior;
- no provider is used for production claims until registry status allows it.

## Phase 5: test-vector harness

Before any algorithm protects real data, add a test-vector harness.

The harness should verify:

- standard vectors where available;
- encode/decode behavior;
- signature verification behavior;
- failure cases;
- invalid input rejection;
- config/profile compatibility;
- deterministic report output for CI.

Exit criteria:

- `test-vector-covered` registry status is backed by tests;
- failures are clear and non-secret-leaking;
- CI can run tests without special credentials;
- docs link the relevant test file.

## Phase 6: profile policy engine

Implement a policy engine that resolves:

```text
control point + configured profile + registry entries + provider support
```

The engine should return either:

- an allowed policy decision with documented provider metadata; or
- a fail-closed error with remediation guidance.

Exit criteria:

- default profile resolves to `safe-default`;
- Base1 scope can resolve to `high-security` where configured;
- compatibility and preview profiles show warnings;
- lab-only is blocked outside lab/docs/tests;
- audit events are planned before mutation is enabled.

## Phase 7: scoped integration points

Integrate only one scope at a time.

Suggested order:

1. `logs/evidence` — lowest-risk signed report planning.
2. `plugins` — manifest/package verification planning.
3. `fyr/packages` — package and lockfile integrity planning.
4. `base1` — image provenance and recovery verification planning.
5. `storage` — only after backup, rollback, and migration docs exist.
6. `transport` — only after provider and compatibility behavior are documented.
7. `identity` — only after key lifecycle docs exist.

Exit criteria for each scope:

- scope-specific docs exist;
- tests exist;
- migration/rollback guidance exists;
- non-claims are preserved;
- no unsupported security claim is introduced.

## Phase 8: profile selection and mutation

Only after read-only validation is reliable, implement mutating profile selection:

```text
crypto select <profile> --scope <control-point> --confirm
```

Required behavior:

- show old profile;
- show new profile;
- show scope;
- show risk class;
- require confirmation for downgrade, compatibility, preview, or lab-only choices;
- write a redacted audit event;
- provide rollback guidance.

Exit criteria:

- mutating commands are tested;
- config writes are atomic or recoverable;
- invalid changes fail closed;
- audit output does not leak secrets.

## Phase 9: review and audit preparation

Before production security claims, prepare:

- algorithm inventory report;
- provider inventory report;
- test-vector coverage report;
- dependency review;
- threat model update;
- external audit checklist;
- known limitations document.

## Initial implementation boundaries

Do not implement these until their prerequisites exist:

- real encryption of persistent user data;
- production signing of packages;
- Base1 boot trust claims;
- identity key management;
- automatic migration of protected data;
- post-quantum default behavior;
- compatibility downgrade defaults.

## Related docs

- [`CRYPTO_POLICY_ROADMAP.md`](CRYPTO_POLICY_ROADMAP.md)
- [`CRYPTO_REGISTRY.md`](CRYPTO_REGISTRY.md)
- [`CRYPTO_PROVIDER_REGISTRY.md`](CRYPTO_PROVIDER_REGISTRY.md)
- [`CRYPTO_OPERATOR_COMMANDS.md`](CRYPTO_OPERATOR_COMMANDS.md)
- [`CRYPTO_CONFIG_SCHEMA.md`](CRYPTO_CONFIG_SCHEMA.md)
- [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md)
- [`crypto-profiles/README.md`](crypto-profiles/README.md)
- [`TRUST_MODEL.md`](TRUST_MODEL.md)

## Non-claims

This implementation plan does not make Phase1 or Base1 cryptographically complete, audited, certified, quantum-safe, hardware-validated, installer-ready, or daily-driver ready.

It defines a safe implementation sequence for future cryptographic policy work.
