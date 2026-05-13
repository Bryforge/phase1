# In-System Wiki

![In System](https://img.shields.io/badge/in--system-wiki-00d8ff) ![Sandbox](https://img.shields.io/badge/runtime-WASI--lite-39ff88) ![Host Tools](https://img.shields.io/badge/host%20tools-not%20required-ffcc00) ![Manual](https://img.shields.io/badge/manual-v6.0.0-39ff88)

Phase1 includes a compact manual that is readable directly from inside the Phase1 shell.

The in-system wiki is provided as sandboxed WASI-lite manual pages in `plugins/`. It does not require SHIELD off, TRUST HOST, browser access, Python, or any host tool.

## Commands

Run these from the Phase1 prompt:

```text
wiki
wiki-quick
wiki-version
wiki-boot
wiki-commands
wiki-files
wiki-browse
wiki-lang
wiki-updates
wiki-trouble
wiki-tutorials
```

## First page

```text
wiki
```

The index prints the available wiki sections and version targets.

## Quick start page

```text
wiki-quick
```

This page shows host install, first Phase1 commands, runtime enablement, and safe defaults.

## Version page

```text
wiki-version
```

This page explains the edge, stable, previous stable, compatibility, and Base1 foundation tracks.

## Boot and security page

```text
wiki-boot
```

This page explains SHIELD, TRUST HOST, VAULT, and network mutation gates.

## Command manual page

```text
wiki-commands
```

This page provides a compact command reference by task.

## Files and editor page

```text
wiki-files
```

This page explains the VFS, AVIM, pipelines, and persistence.

## Browser and network page

```text
wiki-browse
```

This page explains the guarded browser reader and network commands.

## Language runtime page

```text
wiki-lang
```

This page covers Python, Rust, C, WASI-lite, and runtime gates.

For Phase1-native scripting, read the full source wiki page:

```text
docs/wiki/14-Fyr-Native-Language.md
```

## Updates page

```text
wiki-updates
```

This page covers validation, update safety, and release rules.

For Base1 boot, recovery, storage, rollback, and evidence-bound OS-track language, read the full source wiki page:

```text
docs/wiki/13-Base1-OS-Track.md
```

## Troubleshooting page

```text
wiki-trouble
```

This page lists common failures and fixes, including stale wiki surfaces, Base1 dry-run wording, and Fyr script issues.

## Tutorials page

```text
wiki-tutorials
```

This page contains guided labs for boot, VFS, Fyr, Python, browser, network, Base1 dry-runs, validation, and wiki publishing.

## Design notes

The full manual source remains in `docs/wiki/` for GitHub Wiki publishing. The in-system pages are smaller terminal-readable summaries that work inside Phase1 without host access.

When adding a new full wiki page, add a matching compact in-system page when it helps users at the prompt. If no compact command exists yet, link from this page to the full source wiki page so users still have a clear path.
