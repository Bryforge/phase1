# Publish to GitHub Wiki

![Wiki Source](https://img.shields.io/badge/wiki%20source-docs%2Fwiki-00d8ff) ![Manual](https://img.shields.io/badge/manual-v6.0.0-39ff88) ![Stable](https://img.shields.io/badge/stable-v5.0.0-39ff88) ![Base1](https://img.shields.io/badge/Base1-foundation-ff8a00)

This repository stores the Phase1 wiki source in `docs/wiki/` so the manual can be reviewed, versioned, and tested with normal code changes before it is copied to the native GitHub Wiki repository.

## Native GitHub Wiki status

The native GitHub Wiki repository uses this form:

```text
https://github.com/Bryforge/phase1.wiki.git
```

If the native wiki is enabled, publish the files from `docs/wiki/` into that wiki repository.

> [!IMPORTANT]
> If GitHub returns `Repository not found` for `Bryforge/phase1.wiki`, enable Wiki support in the repository settings first, or create the first wiki page manually through the GitHub web UI. After that, the wiki repository can be cloned and updated.

## Manual publish steps

From the main Phase1 checkout:

```bash
cd phase1
git fetch origin
git checkout edge/stable
git pull origin edge/stable
```

Clone the wiki repository beside the main checkout:

```bash
cd ..
git clone https://github.com/Bryforge/phase1.wiki.git phase1.wiki
```

Copy the wiki source files:

```bash
rsync -av --delete phase1/docs/wiki/ phase1.wiki/
```

Commit and push:

```bash
cd phase1.wiki
git status
git add .
git commit -m "Update Phase1 user manual"
git push origin master
```

## One-command publish script

The repository includes a helper script:

```bash
scripts/publish-wiki.sh
```

The script clones the native wiki repository when needed, copies `docs/wiki/` with `rsync --delete`, commits changes, and pushes to the wiki `master` branch.

Useful environment overrides:

```bash
PHASE1_REPO_SLUG=Bryforge/phase1 scripts/publish-wiki.sh
PHASE1_WIKI_WORKDIR=../phase1.wiki scripts/publish-wiki.sh
```

## Wiki file map

| File | Purpose |
| --- | --- |
| `Home.md` | Manual index, reader paths, version matrix, and public guardrails. |
| `_Sidebar.md` | GitHub Wiki sidebar. |
| `01-Quick-Start.md` | First-run tutorial. |
| `02-Version-Guide.md` | Edge, stable, compatibility, and Base1 version model. |
| `03-Boot-Modes-and-Security.md` | Boot selector and guard model. |
| `04-Command-Manual.md` | Command reference by task. |
| `05-Files-Editors-and-Pipelines.md` | VFS, AVIM, and pipelines. |
| `06-Browser-and-Networking.md` | Browser and network manual. |
| `07-Language-Runtimes.md` | Python, Rust, C, WASI-lite, and runtime support. |
| `08-Updates-Releases-and-Validation.md` | Update, validation, and release workflow. |
| `09-Troubleshooting.md` | Common problems and fixes. |
| `10-Publish-to-GitHub-Wiki.md` | This publishing guide. |
| `11-Tutorials.md` | Guided learning paths. |
| `12-In-System-Wiki.md` | Compact manual commands available from inside Phase1. |
| `13-Base1-OS-Track.md` | Base1 boot, recovery, hardware, and evidence-bound OS-track guide. |
| `14-Fyr-Native-Language.md` | Fyr native language quick guide and public example policy. |
| `12-v4-Edge-Manual.md` | Legacy v4 reference retained for compatibility. |

## Version update checklist

When promoting or bumping Phase1, update these docs and fixtures:

```text
Cargo.toml
Cargo.lock
README.md
site/site.js
docs/wiki/Home.md
docs/wiki/01-Quick-Start.md
docs/wiki/02-Version-Guide.md
docs/wiki/08-Updates-Releases-and-Validation.md
docs/wiki/10-Publish-to-GitHub-Wiki.md
docs/wiki/11-Tutorials.md
docs/wiki/12-In-System-Wiki.md
docs/wiki/13-Base1-OS-Track.md
docs/wiki/14-Fyr-Native-Language.md
plugins/wiki-version.wasi
plugins/wiki-updates.wasi
plugins/wiki-quick.wasi
tests/release_metadata.rs
```

Then validate:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
sh scripts/quality-check.sh quick
```

Optional release/security checks:

```bash
cargo audit
cargo deny check
```

## Publish safety rules

- Publish from reviewed `docs/wiki/` content.
- Do not publish secrets, tokens, private keys, account passwords, recovery codes, or private environment details.
- Keep Base1 claims evidence-bound.
- Keep Fyr examples limited to implemented language behavior.
- Keep stable and edge version labels consistent with README and `Cargo.toml`.
- Keep native Wiki publishing separate from GitHub Pages deployment.

## Color policy

GitHub Wiki pages use:

- shield badges for version and section color
- GitHub callouts such as `[!TIP]`, `[!IMPORTANT]`, `[!CAUTION]`, and `[!NOTE]`
- clear code blocks for commands

The manual intentionally avoids emojis, random symbols, and clutter.
