# Base1 Recovery and OS Foundation Manual

> **Status:** Roadmap and design index.
>
> **Validation:** Links to current Base1 design docs, dry-run scripts, readiness matrix, validation report template, validation reports archive, and future validation reports.
>
> **Non-claims:** Base1 is not currently documented here as a released bootable daily-driver image, finished secure OS replacement, or destructive installer-ready system.

Base1 is the planned minimal host foundation for future Phase1-first bootable environments. The first practical documentation goal is to keep Base1 precise: read-only base, writable Phase1 state, recovery shell, rollback planning, image provenance, target identity verification, and explicit operator confirmation.

## Planned chapters

1. Scope and non-claims.
2. Base1 principles.
3. Layer model.
4. Boot flow.
5. Recovery shell.
6. Recovery USB planning.
7. Image provenance.
8. Rollback metadata.
9. Installer policy.
10. Hardware targets.
11. Validation reports.
12. Roadmap gates.

## Source-of-truth links

- [`READINESS_MATRIX.md`](READINESS_MATRIX.md)
- [`VALIDATION_REPORT_TEMPLATE.md`](VALIDATION_REPORT_TEMPLATE.md)
- [`validation/README.md`](validation/README.md)
- [`../MANUAL_ROADMAP.md`](../MANUAL_ROADMAP.md)
- [`../security/TRUST_MODEL.md`](../security/TRUST_MODEL.md)
- [`../../base1/README.md`](../../base1/README.md)
- [`../../base1/SECURITY_MODEL.md`](../../base1/SECURITY_MODEL.md)
- [`../../base1/HARDWARE_TARGETS.md`](../../base1/HARDWARE_TARGETS.md)
- [`../../base1/ROADMAP.md`](../../base1/ROADMAP.md)
- [`../os/ROADMAP.md`](../os/ROADMAP.md)

## Base1 wording rule

Use `planned`, `design`, `dry-run`, `preview`, or `validated` according to evidence. Do not call Base1 bootable, daily-driver ready, recovery-complete, or installer-ready without release artifacts and validation reports.

## Promotion rule

Use [`READINESS_MATRIX.md`](READINESS_MATRIX.md) before strengthening Base1 wording. A page may only move from roadmap to design, dry-run, preview, or validated when the linked evidence supports that level.

## Report rule

Use [`VALIDATION_REPORT_TEMPLATE.md`](VALIDATION_REPORT_TEMPLATE.md) when recording Base1 evidence so reports name the scope, target, commands, result, observations, evidence links, boundaries, and promotion recommendation.

Store future Base1 reports under [`validation/`](validation/) so evidence remains discoverable and reviewable.
