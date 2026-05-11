# Security and Trust Documentation

> **Status:** Documentation index and review entry point.
>
> **Validation:** Links to the trust model, crypto policy roadmap, claims policy, review guide, and manual roadmap added in this repository.
>
> **Non-claims:** Phase1 is not currently a finished secure OS replacement, Base1 is not currently a released bootable daily-driver image, and this documentation index does not prove a security property by itself.

This folder is the security and trust entry point for **The Phase1 Codex**.

## Security goal

Phase1 should be as secure as possible while maintaining practical usability.

Security docs should preserve safe defaults, explicit trust gates, read-only and dry-run flows, redaction, evidence-backed claims, and operator-visible controls without making legitimate learning, development, validation, recovery, or local operator workflows unnecessarily difficult.

## Start here

1. [`TRUST_MODEL.md`](TRUST_MODEL.md) — trust boundaries, safe defaults, guarded host tools, Base1 claim levels, and review checklist.
2. [`CRYPTO_POLICY_ROADMAP.md`](CRYPTO_POLICY_ROADMAP.md) — cryptographic policy registry, operator-selectable profiles, documentation requirements, and Base1 alignment.
3. [`DOCS_CLAIMS.md`](DOCS_CLAIMS.md) — allowed wording, disallowed wording, status labels, and required evidence.
4. [`REVIEW_GUIDE.md`](REVIEW_GUIDE.md) — practical reviewer checklist for safety-sensitive documentation.
5. [`../MANUAL_ROADMAP.md`](../MANUAL_ROADMAP.md) — full Codex architecture, safety model, glossary, and launch plan.

## Review order for safety-sensitive docs

Use this order when reviewing pages about host tools, Base1, recovery, rollback, hardware, installers, image writing, cryptographic policy, or security claims:

1. Confirm the page has a status block.
2. Confirm the page names the current implementation status.
3. Confirm the page separates current behavior from roadmap goals.
4. Confirm host-backed behavior names the required capability.
5. Confirm destructive workflows are not presented as default paths.
6. Confirm dry-run or read-only validation is shown first when applicable.
7. Confirm cryptographic algorithms, modes, parameters, and profiles are documented before they are presented as usable controls.
8. Confirm no page claims secure OS replacement, bootable Base1 release, installer readiness, daily-driver readiness, recovery completion, cryptographic completeness, audit completion, certification, or quantum safety without linked evidence.
9. Use [`REVIEW_GUIDE.md`](REVIEW_GUIDE.md) before approving safety-sensitive documentation.

## Security documentation rule

Security documentation must be operator-visible and testable. Prefer narrow claims such as `requires explicit confirmation`, `blocked by default`, `read-only validation`, `profile is operator-selected`, or `logged for review` over broad claims such as `safe`, `secure`, `quantum-safe`, `certified`, or `hardened`.
