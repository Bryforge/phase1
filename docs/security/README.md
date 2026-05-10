# Security and Trust Documentation

> **Status:** Documentation index and review entry point.
>
> **Validation:** Links to the trust model, claims policy, and manual roadmap added in this repository.
>
> **Non-claims:** Phase1 is not currently a finished secure OS replacement, Base1 is not currently a released bootable daily-driver image, and this documentation index does not prove a security property by itself.

This folder is the security and trust entry point for **The Phase1 Codex**.

## Start here

1. [`TRUST_MODEL.md`](TRUST_MODEL.md) — trust boundaries, safe defaults, guarded host tools, Base1 claim levels, and review checklist.
2. [`DOCS_CLAIMS.md`](DOCS_CLAIMS.md) — allowed wording, disallowed wording, status labels, and required evidence.
3. [`../MANUAL_ROADMAP.md`](../MANUAL_ROADMAP.md) — full Codex architecture, safety model, glossary, and launch plan.

## Review order for safety-sensitive docs

Use this order when reviewing pages about host tools, Base1, recovery, rollback, hardware, installers, image writing, or security claims:

1. Confirm the page has a status block.
2. Confirm the page names the current implementation status.
3. Confirm the page separates current behavior from roadmap goals.
4. Confirm host-backed behavior names the required capability.
5. Confirm destructive workflows are not presented as default paths.
6. Confirm dry-run or read-only validation is shown first when applicable.
7. Confirm no page claims secure OS replacement, bootable Base1 release, installer readiness, daily-driver readiness, or recovery completion without linked evidence.

## Security documentation rule

Security documentation must be operator-visible and testable. Prefer narrow claims such as `requires explicit confirmation`, `blocked by default`, `read-only validation`, or `logged for review` over broad claims such as `safe`, `secure`, or `hardened`.
