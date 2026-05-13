# Phase1 homepage no-grid final checkpoint

Checkpoint date: 2026-05-13
Branch: `edge/stable`
Public site: <https://bryforge.github.io/phase1/>

## Checkpoint summary

This checkpoint records the current Phase1 homepage state after the homepage background and first-view system were corrected to remove visible grid/matrix styling and preserve the cleaner animated starfield direction.

The homepage now uses the official Phase1 mark chamber, a cleaned CSS loading path, and a final no-grid animated background layer.

## Preserved direction

- Main homepage remains the accepted premium black starlight design.
- Official Phase1 logo remains the first-view hero identity.
- The base1/Fyr caption was removed from the homepage hero chamber.
- Visible matrix/grid lines were removed from the homepage presentation.
- Legacy matrix styles were removed from the direct homepage HTML links.
- Matrix loader imports were removed from `site/button-fix.css`.
- Background now favors smoky radial atmosphere and dot-only animated stars.
- Status and wiki refinement layers remain intact.
- Main site should remain protected from broad redesigns after this point.

## Active homepage files at checkpoint

- `site/index.html`
- `site/button-fix.css`
- `site/phase1-hero-stage-focus.css`
- `site/phase1-first-view-mobile.css`
- `site/phase1-first-view-balance.css`
- `site/phase1-atmospheric-field.css`
- `site/phase1-no-grid-lines.css`
- `site/phase1-grid-kill.css`
- `site/phase1-final-background.css`

## Key recent commits

- `27730de` — Focus hero logo stage on Phase1 mark
- `7b9eea2` — Load focused hero logo stage
- `3b04156` — Deploy focused hero logo stage
- `e0ffc31` — Refine mobile first-view hero
- `0cb986b` — Load mobile first-view hero polish
- `c992419` — Deploy mobile first-view hero polish
- `87a8282` — Balance mobile first-view hierarchy
- `e92844f` — Load first-view hierarchy balance
- `9037e8b` — Deploy first-view hierarchy balance
- `d0743c3` — Add atmospheric grid and animated starfield
- `d90376c` — Load atmospheric field polish
- `d03ce28` — Deploy atmospheric field polish
- `10376de` — Remove grid lines and keep animated stars
- `e605843` — Load no-grid animated starfield polish
- `3b07b7e` — Deploy no-grid animated starfield polish
- `7ae42e7` — Remove matrix loaders from homepage CSS
- `d6725b5` — Force final starfield without grid
- `2f046ae` — Load final no-grid animated background
- `7ea5518` — Deploy final no-grid animated background
- `26367a5` — Remove direct legacy matrix styles from homepage

## Next-safe work sequence

1. Do not reintroduce matrix/grid layers on the homepage.
2. Continue only with small visible fixes.
3. Keep the starfield dot-only and secondary to the hero content.
4. Preserve the official Phase1 mark chamber.
5. Begin or continue wiki content expansion only after the homepage is visually accepted.

## Non-claims

This checkpoint is a website presentation checkpoint only. It does not claim Phase1 is production-ready, installer-ready, daily-driver ready, hardware-validated, security-hardened, cryptographically complete, or a complete operating system.
