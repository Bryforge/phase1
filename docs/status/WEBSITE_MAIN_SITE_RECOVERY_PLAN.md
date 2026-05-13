# Phase1 main site recovery plan

Plan date: 2026-05-13
Branch: `edge/stable`
Baseline checkpoint: `342d6e4` — `Checkpoint core values presentation state`
Public site: <https://bryforge.github.io/phase1/>

## Purpose

This plan locks the website work back onto the accepted Phase1 homepage direction after the design work became too scattered. The main rule is simple: start from the checkpoint that was described as strikingly beautiful, then make only small, reversible, single-purpose improvements.

## Baseline to preserve

The accepted baseline includes:

- premium black starlight background;
- controlled moving starfield;
- official Phase1 logo in the hero stage;
- dark solid buttons with living spectrum edges;
- reduced static glow;
- refined core values module;
- fixed `core values` label clipping;
- homepage and status page visual alignment;
- no broad experimental redesign layers.

## Work rules

1. Do not stack new broad CSS experiments.
2. Do not change homepage, status, and wiki in the same pass.
3. Use one patch per visible issue.
4. Keep the accepted color, glow, and starlight language intact.
5. Prefer editing or adding a very small override to changing existing large design layers.
6. After any accepted improvement, checkpoint before continuing.
7. If a change makes the site feel worse, revert that single change immediately.

## Main-site improvement sequence

### Phase 1 — Verify restored checkpoint

- Confirm the live homepage loads from the restored checkpoint.
- Confirm hero logo, core values, buttons, and starfield match the accepted direction.
- Confirm status page still loads correctly.
- Do not modify CSS during this verification step.

### Phase 2 — Main homepage micro-polish only

Potential small fixes, one at a time:

- adjust only the most oversized headings if they still feel too large;
- fix any clipping on descenders such as `g`, `y`, `p`, `q`, and `j`;
- tune hero spacing without changing the layout identity;
- tune logo box sizing without replacing the official logo;
- keep static glow comfortable and hover glow alive.

### Phase 3 — Homepage content hierarchy

- Improve readability of hero text and CTAs.
- Make the demo/terminal area feel intentionally premium.
- Keep the feature cards consistent with the core values style.
- Do not introduce a new color theme.

### Phase 4 — Status page alignment

Only after homepage is accepted again:

- refine status cards and progress bars;
- keep the status nav dark and clean;
- preserve public status boundary/non-claims.

### Phase 5 — Wiki and docs alignment

Only after homepage and status are stable:

- align wiki with the accepted homepage style;
- avoid making wiki title headers too large;
- keep docs readable before adding decorative effects.

## Current next action

The next action is review-only: load the restored homepage and identify exactly one visible problem to fix next. The next code change should be limited to the main Phase1 homepage only.

## Non-claims

This plan is for website presentation only. It does not claim Phase1 is production-ready, installer-ready, daily-driver ready, hardware-validated, security-hardened, cryptographically complete, or a complete operating system.
