# Project and Company Pages Direction Plan

This document defines the direction for the next phase1 project pages and Bryforge company pages.

## Core distinction

Do not let the website confuse these two identities:

| Identity | Meaning | Primary page |
| --- | --- | --- |
| phase1 | The Rust virtual OS console and advanced operator kernel | `project.html` |
| Bryforge | The startup software development company behind phase1 | `company.html` |
| Chase Bryan | Founder, computer scientist, and developer | founder section + company page |

The homepage should introduce all three, but each deeper page should have one clear focus.

## Homepage role

The homepage should remain a dramatic front door.

It should answer quickly:

- What is phase1?
- Why should I care?
- How do I try it?
- Who built it?
- What is Bryforge?
- Where do I go next?

Homepage sections should stay concise:

1. Hero.
2. Terminal demo.
3. Feature cards.
4. Roadmap preview.
5. Founder profile.
6. Bryforge teaser.
7. Support CTA.
8. Footer navigation.

Avoid duplicate labels such as founder profile plus builder profile. Do not show both `Founder profile` and `Builder profile` in the founder card.

## phase1 project page

Filename:

```text
project.html
```

Primary message:

```text
phase1 is a Rust-built virtual OS console for operators who want control.
```

Audience:

- developers
- cybersecurity learners
- OS/systems learners
- contributors
- supporters who want to understand the product

Page sections:

### 1. Product hero

Recommended copy:

```text
phase1 // Rust virtual OS console
A terminal-first advanced operator kernel with a simulated kernel, VFS, process table, audit log, guarded browser, in-system wiki, and safe-by-default shell.
```

Primary CTA:

```text
Clone and run
```

Secondary CTA:

```text
Open docs
```

### 2. System map

Show the system as a clear layered map:

```text
operator shell
command metadata
safe mode + host trust gates
virtual filesystem
process table + audit log
guarded browser + network inspection
language/runtime roadmap
Base1 secure host foundation
```

### 3. Security model

Explain:

- safe mode is default
- host-backed tools are explicit
- no secrets are needed for normal use
- audit records explain actions
- Base1 protects the host below phase1

### 4. Quick start

Use one short block:

```bash
git clone https://github.com/Bryforge/phase1.git
cd phase1
cargo run
```

Then show first commands:

```text
help
version
security
wiki
sysinfo
roadmap
```

### 5. Stable release status

Show:

```text
Stable: v4.0.0
Previous stable: v3.10.9
Compatibility base: v3.6.0
```

### 6. Base1 bridge

Explain Base1 as:

```text
Base1 is the secure hardware host foundation planned for Raspberry Pi and ThinkPad X200-class systems. Its job is to keep the host bootable, recoverable, and protected even if phase1 is damaged, reset, or removed.
```

### 7. Contribution path

Show simple paths:

- try the system
- improve docs
- test commands
- review security model
- contribute examples
- support the roadmap

## Bryforge company page

Filename:

```text
company.html
```

Primary message:

```text
Bryforge is a startup software development company building secure, polished, developer-first systems and tools.
```

Audience:

- potential clients
- contributors
- sponsors
- recruiters/collaborators
- people who want to know what Bryforge is

Page sections:

### 1. Company hero

Recommended copy:

```text
Bryforge // software forged for builders
A startup software development company founded by Chase Bryan, focused on secure systems, terminal-first tools, developer products, automation, and practical software that gives people more control.
```

### 2. Founder statement

Keep wording direct:

```text
Founded by Chase Bryan, a computer scientist and developer building phase1 as Bryforge's flagship research-and-build project.
```

Do not use multiple competing labels. One section label is enough:

```text
Founder profile
```

### 3. What Bryforge builds

Current flagship:

- phase1
- Base1 foundation

Future categories:

- secure developer tools
- terminal-first applications
- automation systems
- systems education tools
- custom software products

### 4. Principles

Recommended company principles:

| Principle | Meaning |
| --- | --- |
| Security first | Software should avoid unnecessary trust and secrets |
| Operator control | Users should understand what the system is doing |
| Craftsmanship | Interfaces should feel polished, not thrown together |
| Open learning | Documentation should teach, not obscure |
| Practical ambition | Build bold systems in realistic, testable steps |

### 5. Support and contact

Use CTAs:

- View phase1 on GitHub
- Support on Buy Me a Coffee
- Follow the roadmap

## Roadmap page

Filename:

```text
roadmap.html
```

Purpose:

The roadmap page should show the full public direction of phase1 and Bryforge.

Separate it into four lanes:

1. Website and docs.
2. phase1 product/runtime.
3. Base1 secure host foundation.
4. Bryforge company development.

### Website/docs lane

- Phase A: homepage launched
- Phase B: wiki/docs hub
- Phase C: docs navigation/search
- Phase D: visual demos/releases
- Phase E: public front door

### phase1 product lane

- v4.0.0 stable baseline
- terminal UX improvements
- storage/Git tooling
- language/runtime support
- security/audit improvements
- examples and tutorials

### Base1 lane

- read-only preflight
- host profile hardening
- Raspberry Pi support
- X200 support
- recovery workflow
- installer only after repeated safety checks

### Bryforge lane

- company page
- support page
- project portfolio
- service/product direction
- public credibility assets

## Support page

Filename:

```text
support.html
```

Purpose:

Make sponsorship clear without overwhelming technical docs.

Sections:

1. Why support phase1.
2. What support funds.
3. Current roadmap priorities.
4. Buy Me a Coffee CTA.
5. GitHub star/watch/fork CTA.

## Copy rules

Use lowercase `phase1` as the product name in most UI text.

Use `Bryforge` for the company.

Use `Chase Bryan` for the founder.

Avoid vague claims like:

- revolutionary
- unbreakable
- military grade
- impossible to compromise

Prefer grounded claims:

- safe-by-default
- guarded host tools
- simulated kernel
- auditable behavior
- tested release gate
- static/offline-friendly website

## Design rules

- Keep neon, glass, space, and terminal identity.
- Use less animation on docs/company pages than the homepage.
- Avoid cramped mobile cards.
- Avoid huge text that breaks words awkwardly.
- Keep CTAs visible but not spammy.
- Use one role label per person/company section.
- Keep founder content professional and not repetitive.

## Implementation checklist

For each new page:

- Add semantic HTML.
- Add homepage/footer navigation.
- Add mobile-responsive layout.
- Add reduced-motion-safe effects only.
- Add tests for required text and links.
- Verify no external CDN dependencies.
- Verify stable release metadata is consistent.
- Verify no `Builder profile` label appears.

## Recommended next issue list

1. Create `project.html` phase1 product page.
2. Create `company.html` Bryforge company page.
3. Create `roadmap.html` full roadmap page.
4. Create `support.html` support page.
5. Expand `wiki/index.html` into a docs hub.
6. Add visual command/system map components.
7. Add screenshots/terminal captures after the layout is stable.

## Definition of done for the next design implementation

The next design implementation is complete when:

- homepage links to product/company/roadmap pages
- `project.html` clearly explains phase1
- `company.html` clearly explains Bryforge
- roadmap direction is visible and not buried in README text
- mobile view is readable
- desktop animation remains smooth
- tests pass
- no duplicate founder/builder labels appear
