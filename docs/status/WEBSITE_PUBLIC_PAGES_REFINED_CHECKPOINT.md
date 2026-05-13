# Phase1 public pages refined checkpoint

Checkpoint date: 2026-05-13
Branch: `edge/stable`
Public site: <https://bryforge.github.io/phase1/>
Status page: <https://bryforge.github.io/phase1/status.html>
Wiki hub: <https://bryforge.github.io/phase1/wiki/>

## Checkpoint summary

This checkpoint records the current refined public website state after:

- the main Phase1 homepage was rated excellent on mobile;
- the main page received a conservative refinement sweep;
- the status and wiki pages were aligned visually and then refined for consistency.

This is the current preferred baseline before deeper wiki content expansion begins.

## Preserved public design direction

- Premium black starlight visual system.
- Official Phase1 logo used consistently.
- Controlled living-spectrum accents.
- Dark, solid buttons with comfortable static glow and lively hover/focus states.
- Glass-like panels with readable contrast.
- Homepage, status page, and wiki hub now share the same public design language.
- Status and wiki navigation link back to the main Phase1 homepage and to each other.
- Main homepage should remain protected from broad redesigns.

## Current late-stage visual layers

### Main Phase1 homepage

- `site/phase1-main-site-finish.css`
- `site/phase1-main-site-last-mile.css`
- `site/phase1-main-site-95plus.css`
- `site/phase1-main-site-refine.css`

### Status and wiki pages

- `site/phase1-public-pages-align.css`
- `site/phase1-public-pages-refine.css`

## Recent related commits

Main site refinement:

- `bd1862b` — Add main site refinement sweep
- `e692c7e` — Load main site refinement sweep
- `8a05409` — Deploy main site refinement sweep

Status and wiki alignment/refinement:

- `175ff5b` — Add shared status and wiki alignment layer
- `a9e07ba` — Align status page with public site and wiki
- `3d954bc` — Align wiki with status page and public style
- `3380492` — Deploy aligned status and wiki pages
- `f39f082` — Refine status and wiki public pages
- `1084708` — Load status public pages refinement
- `620e76b` — Load wiki public pages refinement

## Next-safe work sequence

1. Freeze the homepage except for visible bug fixes.
2. Begin wiki content expansion from this refined baseline.
3. Add one wiki page at a time, starting with command reference.
4. Keep status page content grounded in `status.json` and non-claims.
5. Add checkpoints after each accepted wiki page or major content improvement.

## Non-claims

This checkpoint is a website presentation checkpoint only. It does not claim Phase1 is production-ready, installer-ready, daily-driver ready, hardware-validated, security-hardened, cryptographically complete, or a complete operating system.
