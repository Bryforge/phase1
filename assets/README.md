# Phase1 assets

Status: active asset index
Scope: public project imagery, symbols, word marks, banners, splash art, and branding references

## Purpose

This directory stores public-facing Phase1, Base1, Fyr, and Bryforge visual assets.

Use this index as the source of truth for current asset filenames before linking images from README files, docs, website pages, social posts, or release notes.

## Current public assets

| Asset | Path | Purpose |
| --- | --- | --- |
| Phase1/Base1/Fyr public banner | `assets/phase1_base_fyr_banner1.png` | Main public repository/banner image. |
| Phase1 boot splash | `assets/phase1-splash.svg` | SVG boot-splash style visual for Phase1. |
| Fyr symbol | `assets/fyr_symbol.png` | Current standalone Fyr visual symbol. |
| Fyr word mark | `assets/fyr_word.png` | Current Fyr word mark with symbol styling. |

## Fyr asset status

The current Fyr visual mark uses:

- [`fyr_symbol.png`](fyr_symbol.png)
- [`fyr_word.png`](fyr_word.png)

Older references to `fyr-flame.svg` are outdated and should be replaced with the current PNG assets above.

## Naming policy

Preferred future naming patterns:

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

Use lowercase names with project prefixes. Avoid random upload IDs in final public paths.

## Documentation update rule

When an asset is renamed, added, or removed, update:

- [`../README.md`](../README.md)
- [`../PHASE1_NATIVE_LANGUAGE.md`](../PHASE1_NATIVE_LANGUAGE.md), when Fyr assets change
- [`../docs/fyr/ROADMAP.md`](../docs/fyr/ROADMAP.md), when Fyr assets change
- [`../docs/website/README.md`](../docs/website/README.md)
- tests that guard public asset references

## Safety rule

Do not commit private, unrevised, accidental, credential-bearing, or sensitive screenshots.

Public assets should not contain secrets, tokens, private keys, recovery codes, personal credentials, private logs, or unrevised device/account screenshots.

## Non-claims

This asset index does not prove brand completeness, publish a website, validate marketing copy, or make Phase1, Base1, or Fyr production-ready.

It documents current public asset filenames so repository links stay accurate.
