# Phase1 website core-values presentation checkpoint

Checkpoint date: 2026-05-13
Branch: `edge/stable`
Public site: <https://bryforge.github.io/phase1/>
Status page: <https://bryforge.github.io/phase1/status.html>

## Checkpoint summary

This checkpoint records the accepted public website presentation state after the core-values module was fixed and the site reached a visually strong public-facing baseline.

The latest accepted direction is described as strikingly beautiful, with the remaining immediate issue fixed: the `core values` label no longer clips off the card.

## Preserved design direction

- Premium black starlight background.
- Controlled visible starfield motion.
- Official Phase1 logo in the hero stage.
- Dark solid interface surfaces with subtle signal/circuit styling.
- Living spectral rainbow treatment on buttons, rings, and selected accents.
- Reduced static glow to avoid eye strain.
- Rainbow energy is primarily used as a moving rim, ring, hover state, or signal-node accent.
- Founder avatar ring and values-module styling are visually related.
- Homepage and status page share the public starlight/spectrum design language.
- Core-values module is retained and refined.

## Core-values fix

The `core values` label was clipped at the top edge of the values module. The fix moved the label into normal layout flow, ensured it is not absolutely clipped, and added safe responsive padding.

Related files:

- `site/phase1-core-values-fix.css`
- `site/button-fix.css`
- `.github/workflows/pages.yml`

## Recent checkpoint commits

- `733c060` — Add presentation 10 polish layer
- `e6c4220` — Load presentation 10 polish layer
- `804269d` — Deploy presentation 10 polish layer
- `08447d6` — Fix clipped core values label
- `e517431` — Load core values clipping fix
- `883aa4c` — Deploy core values clipping fix directly

## Active late-stage visual layers

The accepted public design uses the following late-stage CSS layers on top of the base site:

- `site/phase1-starlight-motion.css`
- `site/phase1-starlight-premium.css`
- `site/phase1-starlight-depth.css`
- `site/phase1-spectrum-buttons.css`
- `site/phase1-spectrum-solid.css`
- `site/phase1-computer-joy.css`
- `site/phase1-machine-harmony.css`
- `site/phase1-public-polish.css`
- `site/phase1-presentation-10.css`
- `site/phase1-core-values-fix.css`

## Next update targets

1. Continue with incremental polish only; avoid broad visual rewrites.
2. Extend the accepted visual language to the wiki and docs pages.
3. Keep homepage/status/wiki visually coherent.
4. Improve status page data hierarchy and roadmap card readability.
5. Preserve readability, dark solidity, and controlled living spectrum accents.

## Non-claims

This is a website design checkpoint only. It does not claim Phase1 is production-ready, installer-ready, daily-driver ready, hardware-validated, security-hardened, cryptographically complete, or a complete operating system.
