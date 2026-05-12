# Next Website Roadmap Implementation

Status: prepared for the next design pass after the v4.0.0 stable release.

This document defines the next practical website implementation direction for `https://bryforge.github.io/phase1/`.

## Objective

Turn the website from a strong landing page into a structured public front door for phase1 and Bryforge.

The next implementation should separate three visitor needs:

1. Understand what phase1 is.
2. Learn how to run, use, and trust phase1.
3. Understand Bryforge as the builder behind phase1 and future products.

## Current foundation

The current site already has:

- Neo Tokyo / cyberpunk visual identity.
- phase1 hero section.
- interactive browser terminal demo.
- feature cards.
- founder profile.
- Bryforge company block.
- sponsor section.
- website roadmap timeline.
- static/offline-friendly asset posture.
- mobile readability and desktop animation performance guards.
- stable v4.0.0 release metadata.

## Next implementation theme

Use a two-track site model:

```text
phase1 product track
Bryforge company track
```

The product track should answer:

- What is phase1?
- Why is it useful?
- How do I install it?
- How do I use it?
- What is safe by default?
- What is simulated versus host-backed?
- What is Base1 and how does it protect the host?
- What is the roadmap?

The company track should answer:

- What is Bryforge?
- Who is Chase Bryan?
- What does Bryforge build?
- What principles guide the company?
- How can people support or contact the project?

## Recommended information architecture

```text
/
  site/index.html                 homepage: product overview + founder + company teaser
  project.html               phase1 product page
  company.html               Bryforge company page
  roadmap.html               visual roadmap and release direction
  support.html               sponsor/support page

/wiki/
  site/index.html                 docs hub
  quick-start.html           install and first run
  commands.html              command map
  security.html              security model and safe-mode behavior
  base1.html                 Base1 secure-host overview
  releases.html              v4 stable, previous stable, validation gate
  tutorials.html             guided learning paths
```

## Page priority

### Priority 1: Project page

Build `project.html` first.

Purpose:

- Explain phase1 as a Rust virtual OS console.
- Show the feature stack without overwhelming new users.
- Explain safe mode, host trust, VFS, audit log, browser, wiki, and runtime roadmap.
- Link to GitHub quick start and docs.

Sections:

1. Product hero: `phase1 // Rust virtual OS console`.
2. Operator promise: secure, private, powerful, open.
3. System map: kernel, VFS, process table, audit log, guarded browser, wiki, runtime support.
4. Safety model: safe by default, explicit host trust, no secrets required.
5. Quick start: clone, run, first commands.
6. Base1 connection: secure host foundation for Raspberry Pi and ThinkPad X200.
7. Roadmap preview: stable v4.0.0 now, next docs/runtime/Base1 milestones.
8. Calls to action: Star GitHub, open docs, support Bryforge.

### Priority 2: Company page

Build `company.html` second.

Purpose:

- Make Bryforge feel real, focused, and trustworthy.
- Keep founder messaging clear without repeating the founder card in awkward ways.
- Present Bryforge as the company behind phase1 and future software products.

Sections:

1. Company hero: `Bryforge // software forged for builders`.
2. Founder statement: Chase Bryan, computer scientist and developer.
3. Mission: secure, polished, developer-first software.
4. Current flagship: phase1.
5. Future direction: Base1, dev tools, secure systems, automation, custom software products.
6. Principles: security, usefulness, transparency, speed, craftsmanship.
7. Support/contact CTA: GitHub, Buy Me a Coffee, project repository.

### Priority 3: Roadmap page

Build `roadmap.html` third.

Purpose:

- Turn the current timeline into a full public roadmap.
- Give contributors and supporters a clear view of what comes next.

Sections:

1. Stable release status.
2. Website roadmap: Phase A through Phase E.
3. Product roadmap: terminal UX, docs/wiki, Git/storage, runtime support, Base1.
4. Security roadmap: audit posture, dependency policy, safe host bridge, Base1 hardening.
5. Contributor path: good first issues, docs tasks, test tasks, screenshots.

## Visual direction

Keep the existing cyberpunk look, but make page-level navigation calmer than the homepage.

Rules:

- Homepage can be dramatic.
- Docs and company pages should be calmer, readable, and faster.
- Use the same dark glass/neon language, but reduce motion density on text-heavy pages.
- Avoid duplicate labels such as founder profile plus builder profile.
- Use one page title, one section label, and one clear user action per block.

## Component direction

Reuse these components:

- `glass-panel`
- `section-heading`
- `feature-card`
- `timeline`
- `button`
- `terminal`
- `metric-strip`

Add these components next:

```text
.page-hero
.page-nav
.proof-grid
.system-map
.principle-card
.release-card
.cta-band
.docs-shell
.docs-sidebar
.docs-card
```

## Performance requirements

All new pages must preserve the current static/security posture:

- no external JavaScript dependencies
- no CDN framework dependency
- no tracking scripts
- no heavy third-party fonts
- reduced-motion support
- readable without canvas animation
- GitHub Pages compatible
- mobile-first layout
- desktop animation should pause when hidden

## Quality gate

Every implementation PR should include tests that assert:

- the new page exists
- the page links from the homepage or footer
- phase1 naming remains lowercase where product name is official
- Bryforge page has company wording
- founder section has one logical label only
- no stale `Builder profile` label
- no external CDN dependency
- stable release metadata remains v4.0.0

Suggested command gate:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

## Recommended PR sequence

### PR A: Project page foundation

- Add `project.html`.
- Add homepage/footer link to `project.html`.
- Add tests.

### PR B: Company page foundation

- Add `company.html`.
- Move/expand Bryforge messaging into company page.
- Keep homepage company block as a teaser only.
- Add tests.

### PR C: Roadmap page foundation

- Add `roadmap.html`.
- Convert current roadmap timeline into a full page.
- Add product/company roadmap split.
- Add tests.

### PR D: Wiki hub expansion

- Add or improve `wiki/site/index.html`.
- Add docs cards for Quick Start, Security, Base1, Releases, Tutorials.
- Add tests.

## Decision

The next implementation should start with `project.html`, because it clarifies the product before expanding the company story. The company page should follow immediately after so Bryforge has a professional public identity separate from the phase1 technical pitch.

## Roadmap maintenance requirement

Future implementation PRs must keep roadmap and planning documents current. When a PR completes a planned item, starts a new track, or changes implementation priority, update the relevant roadmap docs in the same PR or explain why no roadmap change is required. Review docs/project/LEARNING.md, docs/project/WIKI_ROADMAP.md, docs/website/NEXT_ROADMAP_IMPLEMENTATION.md, docs/repo/EDGE.md, and docs/releases/CHANGELOG.md during milestone PRs.
