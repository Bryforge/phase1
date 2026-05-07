# Phase1 Website + Wiki Roadmap

This roadmap turns GitHub Pages into the main public home for Phase1 and gradually brings the existing in-system wiki into the website.

## Goal

Make `https://bryforge.github.io/phase1/` the main Phase1 homepage:

- minimalist dark-space rainbow landing page
- Phase1 logo and operator-console identity
- founder profile for Chase Bryan
- Bryforge startup software development company profile
- sponsor section with Buy Me a Coffee support link
- direct links to GitHub, install steps, docs, and releases
- web version of the Phase1 wiki
- command index and tutorial hub

## Phase A — Launch homepage

Status: started.

- Add animated GitHub Pages homepage
- Use a dark space canvas background with live stars/comets
- Use moving rainbow gradients, orbital rings, and neon glass panels
- Use the Phase1 logo on the homepage
- Add Chase Bryan founder/computer scientist section
- Add Bryforge startup software development company section
- Add Buy Me a Coffee sponsor call-to-action
- Link the website from the root `README.md`
- Add a GitHub Pages deploy workflow

## Phase B — Bring existing wiki content into the site

Convert the current `docs/wiki/` content into web pages.

Initial pages:

- Quick Start
- Boot Profiles
- Command Map
- Files and VFS
- Browser and Network Safety
- Language Runtime Support
- Updates
- Troubleshooting
- Tutorials
- Sponsorship and Bryforge

Recommended layout:

```text
wiki/
  index.html
  quick-start.html
  boot.html
  commands.html
  files.html
  browser.html
  lang.html
  updates.html
  trouble.html
  tutorials.html
  sponsor.html
  bryforge.html
```

## Phase C — Add docs navigation

- Add persistent docs sidebar
- Add mobile docs drawer
- Add page-local table of contents
- Add previous/next links
- Add keyboard-friendly search
- Add command metadata cards for every Phase1 command
- Add support links in the docs footer

## Phase D — Add visual demos

- Add screenshots or terminal captures
- Add animated command previews
- Add boot-profile selector preview
- Add storage/Git/Rust workflow preview
- Add safe-mode vs trusted-host explanation
- Add founder/build-log updates for major milestones

## Phase E — Make the website the public front door

- Keep README focused on quick start and link to the website for full docs
- Add release notes and migration guides
- Add stable/edge version cards
- Add contributor guide
- Add roadmap and quality score sections
- Add links to security policy and audit notes
- Keep the sponsor and Bryforge company sections visible from the homepage

## Design principles

- Dark background first
- Rainbow motion, but accessible reduced-motion support
- Fast static files only
- No external JavaScript dependencies
- Mobile-first layout
- GitHub Pages compatible
- Docs should be readable even if animations are disabled
- Sponsor content should support the project without distracting from technical docs
