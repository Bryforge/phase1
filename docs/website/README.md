# Phase1 website documentation

Status: active website documentation index
Scope: public website docs, branding notes, content structure, accessibility, mobile fit, and launch/support links

## Purpose

This directory is the preferred organized home for public website planning and documentation.

The website is the public face of Phase1, Base1, Fyr, and Bryforge project coordination. Website changes should stay accurate, readable, mobile-friendly, and aligned with repository evidence.

## Website goals

The website should help visitors quickly find:

- what Phase1 is;
- how to start safely;
- what is implemented now;
- what is roadmap work;
- where Base1 fits;
- where Fyr fits;
- how to contribute;
- how to get support;
- how to report security-sensitive issues safely.

## Reorganization policy

Website documentation reorganization is preservation-first.

Rules:

- Keep existing public links working where possible.
- Prefer indexes and redirects before moving pages.
- Keep branding assets in `assets/` unless a future asset map says otherwise.
- Use [`../../assets/README.md`](../../assets/README.md) as the current public asset filename index.
- Keep public claims aligned with `README.md`, `FEATURE_STATUS.md`, `SECURITY.md`, and `QUALITY.md`.
- Do not claim production readiness, hardened status, hardware validation, audit status, certification, or quantum safety without linked evidence.

## Planned structure

```text
docs/website/
  README.md
  CONTENT_MAP.md
  BRANDING.md
  ACCESSIBILITY.md
  RELEASE_CHECKLIST.md
```

Create additional files only when they have real content and tests or validation paths.

## Public content checklist

Website-facing content should include clear links to:

- [`../../README.md`](../../README.md)
- [`../../FEATURE_STATUS.md`](../../FEATURE_STATUS.md)
- [`../../CONTRIBUTING.md`](../../CONTRIBUTING.md)
- [`../../SECURITY.md`](../../SECURITY.md)
- [`../../QUALITY.md`](../../QUALITY.md)
- [`../../assets/README.md`](../../assets/README.md)
- [`../REPOSITORY_NAVIGATION.md`](../REPOSITORY_NAVIGATION.md)
- [`../community/README.md`](../community/README.md)

## Branding and asset policy

Public visuals should be stored under:

```text
assets/
```

Current public asset filenames are documented in [`../../assets/README.md`](../../assets/README.md).

Current key assets:

- [`../../assets/phase1_base_fyr_banner1.png`](../../assets/phase1_base_fyr_banner1.png) — public Phase1/Base1/Fyr banner.
- [`../../assets/phase1-splash.png`](../../assets/phase1-splash.png) — current Phase1 boot splash PNG.
- [`../../assets/fyr_symbol.png`](../../assets/fyr_symbol.png) — current standalone Fyr symbol.
- [`../../assets/fyr_word.png`](../../assets/fyr_word.png) — current Fyr word mark.

Older references to `assets/phase1-splash.svg` are outdated for the public README boot splash component. Older references to `assets/fyr-flame.svg` are outdated and should not be used for current Fyr branding.

Use consistent naming for Phase1, Base1, and Fyr assets.

Suggested naming patterns:

```text
phase1_*.png
phase1_*.svg
base1_*.png
base1_*.svg
fyr_*.png
fyr_*.svg
bryforge_*.png
bryforge_*.svg
```

Do not commit private, unrevised, or accidental screenshots.

## Validation

Before and after website documentation changes, run:

```bash
sh scripts/quality-check.sh quick
```

If website tests are involved, also run:

```bash
sh scripts/test-website.sh
```

## Non-claims

This index does not publish the website, prove marketing accuracy, create a release, or make Phase1, Base1, or Fyr production-ready.

It creates an organized destination for future website documentation work.
