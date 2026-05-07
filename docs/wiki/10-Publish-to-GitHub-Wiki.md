# Publish to GitHub Wiki

![Wiki Source](https://img.shields.io/badge/wiki%20source-docs%2Fwiki-00d8ff) ![Manual](https://img.shields.io/badge/manual-v3.10.9--dev-39ff88) ![Stable](https://img.shields.io/badge/stable-v3.10.7-ffcc00)

This repository stores the Phase1 wiki source in `docs/wiki/` so the manual can be reviewed, versioned, and tested with normal code changes.

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
git pull origin master
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

The repository includes a helper script path reserved for publishing:

```text
scripts/publish-wiki.sh
```

Use that script once the native wiki repository exists.

## Wiki file map

| File | Purpose |
| --- | --- |
| `Home.md` | Manual index and version matrix |
| `_Sidebar.md` | GitHub Wiki sidebar |
| `01-Quick-Start.md` | First-run tutorial |
| `02-Version-Guide.md` | Edge, stable, and base version model |
| `03-Boot-Modes-and-Security.md` | Boot selector and guard model |
| `04-Command-Manual.md` | Command reference by task |
| `05-Files-Editors-and-Pipelines.md` | VFS, AVIM, and pipelines |
| `06-Browser-and-Networking.md` | Browser and network manual |
| `07-Language-Runtimes.md` | Python, Rust, C, and runtime support |
| `08-Updates-Releases-and-Validation.md` | Update and release workflow |
| `09-Troubleshooting.md` | Common problems and fixes |
| `10-Publish-to-GitHub-Wiki.md` | This publishing guide |
| `11-Tutorials.md` | Guided learning paths |

## Version update checklist

When promoting or bumping Phase1, update these docs:

```text
README.md
docs/wiki/Home.md
docs/wiki/02-Version-Guide.md
docs/wiki/08-Updates-Releases-and-Validation.md
docs/wiki/10-Publish-to-GitHub-Wiki.md
docs/wiki/11-Tutorials.md
```

Then validate:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```

## Color policy

GitHub Wiki pages use:

- shield badges for version and section color
- GitHub callouts such as `[!TIP]`, `[!IMPORTANT]`, `[!CAUTION]`, and `[!NOTE]`
- clear code blocks for commands

The manual intentionally avoids emojis, random symbols, and clutter.
