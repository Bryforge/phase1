# Security and Trust Documentation

> **Status:** Documentation index and review entry point.
>
> **Validation:** Links to the trust model, crypto policy roadmap, crypto registry, crypto profiles index, crypto operator command plan, crypto config schema, crypto algorithm template, claims policy, review guide, and manual roadmap added in this repository.
>
> **Non-claims:** Phase1 is not currently a finished secure OS replacement, Base1 is not currently a released bootable daily-driver image, and this documentation index does not prove a security property by itself.

This folder is the security and trust entry point for **The Phase1 Codex**.

## Security goal

Phase1 should be as secure as possible while maintaining practical usability.

Security docs should preserve safe defaults, explicit trust gates, read-only and dry-run flows, redaction, evidence-backed claims, and operator-visible controls without making legitimate learning, development, validation, recovery, or local operator workflows unnecessarily difficult.

## Start here

1. [`TRUST_MODEL.md`](TRUST_MODEL.md) — trust boundaries, safe defaults, guarded host tools, Base1 claim levels, and review checklist.
2. [`CRYPTO_POLICY_ROADMAP.md`](CRYPTO_POLICY_ROADMAP.md) — cryptographic policy registry, operator-selectable profiles, documentation requirements, and Base1 alignment.
3. [`CRYPTO_REGISTRY.md`](CRYPTO_REGISTRY.md) — planning registry for crypto profiles, control points, algorithm families, status labels, and review requirements.
4. [`crypto-profiles/README.md`](crypto-profiles/README.md) — planning index for operator-selectable cryptographic profiles.
5. [`CRYPTO_OPERATOR_COMMANDS.md`](CRYPTO_OPERATOR_COMMANDS.md) — planning design for future operator-facing crypto policy commands.
6. [`CRYPTO_CONFIG_SCHEMA.md`](CRYPTO_CONFIG_SCHEMA.md) — planning schema for future scoped crypto policy configuration.
7. [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md) — required template for documenting cryptographic algorithms, designs, providers, and profile components.
8. [`DOCS_CLAIMS.md`](DOCS_CLAIMS.md) — allowed wording, disallowed wording, status labels, and required evidence.
9. [`REVIEW_GUIDE.md`](REVIEW_GUIDE.md) — practical reviewer checklist for safety-sensitive documentation.
10. [`../MANUAL_ROADMAP.md`](../MANUAL_ROADMAP.md) — full Codex architecture, safety model, glossary, and launch plan.

## Review order for safety-sensitive docs

Use this order when reviewing pages about host tools, Base1, recovery, rollback, hardware, installers, image writing, cryptographic policy, cryptographic algorithms, cryptographic profiles, crypto operator commands, crypto configuration, or security claims:

1. Confirm the page has a status block.
2. Confirm the page names the current implementation status.
3. Confirm the page separates current behavior from roadmap goals.
4. Confirm host-backed behavior names the required capability.
5. Confirm destructive workflows are not presented as default paths.
6. Confirm dry-run or read-only validation is shown first when applicable.
7. Confirm cryptographic algorithms, modes, parameters, profiles, providers, limits, and migration guidance are documented before they are presented as usable controls.
8. Confirm algorithm pages use [`CRYPTO_ALGORITHM_TEMPLATE.md`](CRYPTO_ALGORITHM_TEMPLATE.md).
9. Confirm registry entries are listed or planned through [`CRYPTO_REGISTRY.md`](CRYPTO_REGISTRY.md) before they are connected to profiles.
10. Confirm profile docs follow [`crypto-profiles/README.md`](crypto-profiles/README.md) before they are connected to runtime behavior.
11. Confirm crypto operator commands follow [`CRYPTO_OPERATOR_COMMANDS.md`](CRYPTO_OPERATOR_COMMANDS.md) before they are connected to runtime behavior.
12. Confirm crypto configuration follows [`CRYPTO_CONFIG_SCHEMA.md`](CRYPTO_CONFIG_SCHEMA.md) before it is connected to runtime behavior.
13. Confirm no page claims secure OS replacement, bootable Base1 release, installer readiness, daily-driver readiness, recovery completion, cryptographic completeness, audit completion, certification, or quantum safety without linked evidence.
14. Use [`REVIEW_GUIDE.md`](REVIEW_GUIDE.md) before approving safety-sensitive documentation.

## Security documentation rule

Security documentation must be operator-visible and testable. Prefer narrow claims such as `requires explicit confirmation`, `blocked by default`, `read-only validation`, `profile is operator-selected`, `uses documented parameters`, `fails closed`, or `logged for review` over broad claims such as `safe`, `secure`, `quantum-safe`, `certified`, or `hardened`.
