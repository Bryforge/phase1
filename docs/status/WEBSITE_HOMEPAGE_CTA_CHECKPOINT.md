# Phase1 homepage CTA checkpoint

Checkpoint date: 2026-05-13
Branch: `edge/stable`
Public site: <https://bryforge.github.io/phase1/>

## Checkpoint summary

This checkpoint records the homepage state after the no-grid cleanup and the CTA hierarchy pass.

The homepage now preserves the cleaned no-grid visual system and improves the first-view call-to-action hierarchy so `Clone & run` and `Try the console` are treated as the primary actions, while Wiki, Status, and Support are secondary actions.

## Preserved direction

- Premium black starlight homepage direction.
- Direct legacy matrix/grid styles removed from the homepage.
- Final no-grid animated background and dot-only starfield direction preserved.
- Official Phase1 mark chamber preserved.
- `base1 | Fyr` caption remains removed from the main homepage hero chamber.
- CTA hierarchy clarified without redesigning the page.
- Main homepage should continue with small targeted fixes only.

## Active files at checkpoint

- `site/index.html`
- `site/button-fix.css`
- `site/phase1-final-background.css`
- `site/phase1-homepage-cta.css`

## Key recent commits

- `7ae42e7` — Remove matrix loaders from homepage CSS
- `d6725b5` — Force final starfield without grid
- `2f046ae` — Load final no-grid animated background
- `7ea5518` — Deploy final no-grid animated background
- `26367a5` — Remove direct legacy matrix styles from homepage
- `401e475` — Refine homepage CTA hierarchy
- `6dafd02` — Load homepage CTA hierarchy polish
- `9754dc4` — Deploy homepage CTA hierarchy polish

## Next-safe fix

The visible side command rail text (`BUILD`, `EXECUTE`, `STORE`, `VERIFY`, `CONNECT`) does not fit well and should be hidden or converted into a non-clipping detail. The next patch should target that issue only.

## Non-claims

This checkpoint is a website presentation checkpoint only. It does not claim Phase1 is production-ready, installer-ready, daily-driver ready, hardware-validated, security-hardened, cryptographically complete, or a complete operating system.
