# phase1 Website + Wiki Roadmap

This roadmap turns GitHub Pages into the main public home for phase1 and Bryforge.

Detailed implementation plans:

- [`docs/website/NEXT_ROADMAP_IMPLEMENTATION.md`](docs/website/NEXT_ROADMAP_IMPLEMENTATION.md)
- [`docs/website/PROJECT_AND_COMPANY_PAGES.md`](docs/website/PROJECT_AND_COMPANY_PAGES.md)

## Goal

Make `https://bryforge.github.io/phase1/` the public front door for:

1. phase1, the Rust virtual OS console and advanced operator kernel.
2. Bryforge, the startup software development company behind phase1.
3. Chase Bryan, founder, computer scientist, and developer.

The site should help visitors understand the project, try it quickly, read the docs, follow the roadmap, and support the work.

## Current stable baseline

| Track | Status |
| --- | --- |
| Stable release | `v4.0.0` |
| Previous stable | `v3.10.9` |
| Compatibility base | `v3.6.0` |
| Website state | Phase A complete, Phase B started |
| Base1 state | secure host foundation added |

## Clear separation of pages

The next website implementation should separate product and company messaging.

| Page | Purpose |
| --- | --- |
| `index.html` | dramatic homepage and routing surface |
| `project.html` | phase1 product page |
| `company.html` | Bryforge company page |
| `roadmap.html` | full public roadmap |
| `support.html` | support and sponsorship page |
| `wiki/index.html` | docs hub |

## Phase A — Launch homepage

Status: complete.

Delivered:

- animated GitHub Pages homepage
- dark space canvas background
- rainbow gradients, orbital rings, and neon glass panels
- phase1 logo on homepage
- founder profile for Chase Bryan
- Bryforge startup software company teaser
- Buy Me a Coffee sponsor call-to-action
- GitHub, docs, and wiki links
- SEO and social metadata
- interactive browser terminal demo
- mobile readability fixes
- desktop animation performance guards
- stable v4.0.0 release metadata
- single logical Founder profile label

## Phase B — Product and company page foundation

Status: next implementation.

Purpose:

Build the first deeper public pages so the site is no longer only a homepage.

### PR B1: phase1 project page

Add:

```text
project.html
```

Page sections:

- phase1 product hero
- system map
- safety model
- quick start
- stable release status
- Base1 bridge
- contribution path

Definition of done:

- linked from homepage nav/footer
- explains phase1 clearly without overclaiming
- includes clone/run command
- includes safe mode and host trust explanation
- links to Base1 docs
- includes tests

### PR B2: Bryforge company page

Add:

```text
company.html
```

Page sections:

- Bryforge company hero
- founder statement
- what Bryforge builds
- company principles
- flagship project: phase1
- support/contact CTAs

Definition of done:

- linked from homepage nav/footer
- clearly separates Bryforge from phase1
- avoids duplicate founder/builder labels
- includes tests

### PR B3: Roadmap page

Add:

```text
roadmap.html
```

Page lanes:

- website and docs
- phase1 product/runtime
- Base1 secure host foundation
- Bryforge company development

Definition of done:

- linked from homepage roadmap preview
- shows stable v4.0.0 status
- shows Phase A through Phase E website direction
- shows product/Base1/company lanes
- includes tests

## Phase C — Docs hub and navigation

Add a real docs surface under `wiki/`.

Recommended pages:

```text
wiki/
  index.html
  quick-start.html
  commands.html
  security.html
  base1.html
  releases.html
  tutorials.html
```

Features:

- docs cards
- mobile docs drawer
- page-local table of contents
- previous/next links
- keyboard-friendly search later
- command metadata cards later

## Phase D — Visual demos and release storytelling

Add richer proof and demo material.

- screenshots or terminal captures
- animated command previews
- boot-profile selector preview
- storage/Git/Rust workflow preview
- safe-mode versus trusted-host explanation
- release notes and migration guides
- stable/edge version cards if edge resumes after v4 stable

## Phase E — Full public front door

Make the website the primary public destination.

- keep README quick and practical
- keep full docs on the website
- add contributor guide
- add quality score section
- add security policy and audit notes links
- keep support and Bryforge company sections visible
- add future Bryforge product portfolio when ready

## Design principles

- Dark background first.
- Neon/rainbow identity, but readable before flashy.
- Mobile-first layout.
- Static files only.
- No external JavaScript dependencies.
- No tracking scripts.
- Reduced-motion support.
- Docs must be readable without animation.
- Company pages should feel calmer than the homepage.
- Avoid duplicate labels such as `Founder profile` plus `Builder profile`.
- Keep product, company, and founder identities distinct.

## Quality gate

Every website implementation PR should run:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

Every website implementation PR should add or update tests for:

- required page exists
- homepage/footer links exist
- no external CDN dependency
- stable release metadata stays current
- founder labels stay logical
- lowercase `phase1` product naming is preserved
- Bryforge company page remains distinct from phase1 product page

## Immediate next action

Start with **PR B1: phase1 project page**.

Reason:

The product page clarifies what phase1 is before the company page expands Bryforge. After that, build the company page so Bryforge has a professional identity separate from the phase1 technical pitch.

## Development checkpoint rule

Feature milestones must update roadmap and planned implementation docs when they complete planned work or change direction. Review LEARNING.md, WIKI_ROADMAP.md, docs/website/NEXT_ROADMAP_IMPLEMENTATION.md, EDGE.md, and CHANGELOG.md. If no roadmap update is needed, say why in the PR body.

<!-- phase1:auto:repo-model:start -->
## Phase1 repository model

- `base/v4.2.0` is the frozen stable base.
- `edge/stable` is the active default development path.
- `checkpoint/*` branches are verified milestone snapshots.
- `feature/*` branches target `edge/stable`.

Keep the 4.2.0 image and stable base boring. Move tested work through edge/stable.
<!-- phase1:auto:repo-model:end -->

