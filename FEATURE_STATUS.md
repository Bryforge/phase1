# Phase1 Feature Status Matrix

This document is the source of truth for what Phase1 currently implements, what is experimental, what is restricted, and what is not supported yet.

Phase1 is a **terminal-first virtual OS console**, not a full bare-metal operating system, VM, container runtime, or hardened sandbox. It combines simulated OS components with guarded host integrations for learning and local development.

## Status labels

| Label | Meaning |
| --- | --- |
| Implemented | Works inside Phase1 today and is covered by tests or smoke validation. |
| Experimental | Exists, but is still being refined and may change. Treat it as edge/development functionality. |
| Restricted | Exists only behind explicit trust, safe-mode, or host-mutation gates. |
| Planned | Intended future work, not available yet. |
| Not planned | Outside the current Phase1 scope. |

## Core system features

| Feature | Status | Notes |
| --- | --- | --- |
| Terminal operator shell | Implemented | Command parsing, prompt, help, manual pages, autocomplete, history, and boot flow exist. |
| Virtual filesystem | Implemented | Simulated VFS with `/`, `/home`, `/proc`, `/dev`, `/etc`, `/tmp`, and `/var/log`. |
| File commands | Implemented | `ls`, `cd`, `pwd`, `cat`, `mkdir`, `touch`, `rm`, `cp`, `mv`, `tree`, and `echo` operate on the VFS. |
| Text tools and pipelines | Implemented | `grep`, `wc`, `head`, `tail`, `find`, shell chaining, and structured text pipelines are implemented. |
| Simulated process table | Implemented | `ps`, `top`, `spawn`, `jobs`, `fg`, `bg`, `kill`, and `nice` model processes inside Phase1. |
| `/proc`-style inspection | Implemented | Simulated `/proc/version`, `/proc/cpuinfo`, `/proc/meminfo`, and uptime are available. |
| PCIe/CR3/CR4 model | Implemented | Architecture commands are simulated and validation-guarded; they do not modify real hardware. |
| Audit log | Implemented | Kernel and shell events are recorded in a bounded in-memory audit log. |
| Persistent state | Implemented | Optional VAULT mode persists selected `/home` content to `phase1.state`. |
| Persistent history | Implemented | Command history can persist locally with sensitive-command redaction. |
| Compact dynamic prompt | Implemented | Prompt chips show channel, safe/host state, and trust state across device modes. |
| Mobile-safe line editor | Implemented | Mobile/narrow terminals use a simple line-editor path to avoid prompt repaint artifacts. |
| Theme palettes | Implemented | Multiple terminal palettes and preview commands exist. |
| Dashboard/sysinfo | Implemented | `dash`, `sysinfo`, and related operator reports are available. |
| Local learning system | Implemented | `learn` and `phase1-learn` use local sanitized memory, rules, notes, suggestions, and history import. |
| In-shell feature status | Implemented | `status`, `status features`, and `capabilities` show a compact implemented/experimental/restricted/planned/not-planned summary plus command gates. |

## Runtime, host, and security features

| Feature | Status | Notes |
| --- | --- | --- |
| Safe mode default | Implemented | Phase1 boots with host tools blocked unless explicitly trusted. |
| Command capability metadata | Implemented | Commands have capability labels and guard descriptions. Run `capabilities` or `status features` in Phase1. |
| Guarded host runtime execution | Experimental | `lang run` can execute trusted local runtimes with explicit host trust while safe mode remains enabled. This is a guardrail layer, not VM isolation. |
| Direct `python` / `py` wrapper | Experimental | Works as host-backed execution, but should be migrated to the same runtime helper as `lang run`. |
| Direct `gcc` / `cc` wrapper | Experimental | Works as host-backed execution, but should be migrated to the same runtime helper as `lang run`. |
| `lang run` Python support | Experimental | Works with bounded stdin, timeouts, temp workspace, redaction, and audit metadata when host tools are trusted. |
| Git/storage helper | Experimental | Host-backed storage/Git workflows exist, guarded by explicit trust and validation. |
| Rust build support | Experimental | Rust workflow support exists through host tooling and remains trust-gated. |
| Browser/curl text fetch | Experimental | `browser` fetches HTTP/HTTPS text through guarded host networking. |
| Host network inspection | Restricted | Host-backed network inspection is guarded; safe mode uses simulated fallbacks. |
| Host network mutation | Restricted | Network changes require safe mode off, host trust, and explicit network-change opt-in. |
| Self-update execution | Restricted | Mutating update flows require privileged host gates and refuse unsafe local state. |
| WASI-lite plugin runtime | Implemented | WASI-lite manifests run without host shell/network passthrough; this is Phase1's sandboxed plugin path. |
| Python plugins | Experimental | Python plugins are host-backed and trust-gated. |
| Secret redaction | Implemented | History, ops log, runtime output, and storage helper paths include redaction guards. |
| Hardened VM/chroot/container sandbox | Not planned | Phase1 does not currently provide hardened OS isolation. Use a real VM/container for hostile code. |
| Full secure OS replacement | Not planned | Phase1 is not a replacement for Linux, macOS, Windows, or a hardened operating system. |

## Website, docs, and release features

| Feature | Status | Notes |
| --- | --- | --- |
| Public GitHub Pages site | Implemented | Static site with project overview, terminal demo, and mobile-first layout. |
| Browser terminal demo | Implemented | Website demo is static and illustrative; it is not a live Phase1 shell. |
| Release metadata checks | Implemented | Tests enforce stable, previous-stable, edge, and compatibility-base consistency. |
| Quality score/check scripts | Implemented | Repository quality scripts validate docs, metadata, and shell syntax. |
| Edge checkpoint docs | Implemented | `docs/archive/checkpoints/DEVELOPMENT_CHECKPOINT_EDGE_4_3_0_DEV.md` records the active edge boundary. |
| Roadmap design docs | Implemented | Existing roadmap docs describe planned design direction. |
| Explicit unimplemented-feature index | Implemented | This file is the current implementation/not-implemented matrix. |

## Planned follow-up work

| Feature | Status | Notes |
| --- | --- | --- |
| Unified legacy runtime wrappers | Planned | Move direct `python`, `py`, `gcc`, and `cc` through the same helper as `lang run`. |
| `doctor mobile` | Planned | Report terminal width, prompt mode, line-editor mode, color mode, safe mode, trust gate, and recommended launch command. |
| Named boot profiles | Planned | Save/use profiles such as phone, laptop-dev, release-demo, and trusted-runtime. |
| Broader language support | Planned | Continue expanding the guarded runtime manager for major programming languages. |

## Quick answers

- **Is Phase1 a full OS?** No. It is a terminal-first virtual OS console with simulated OS components and guarded host integrations.
- **Is Python/Git/Cargo support stable secure execution?** No. It is experimental host integration and should be labeled that way until more isolation work exists.
- **Does Phase1 run untrusted code safely?** No. Do not treat host-backed runtimes as a hardened sandbox.
- **What is implemented today?** The shell, simulated VFS/process/kernel model, docs, local learning, prompt/UI, quality checks, WASI-lite plugin path, and guarded runtime pathways described above.
- **What should users run to inspect command-level gates?** Run `status features`, `status`, or `capabilities` inside Phase1.

<!-- phase1:auto:repo-model:start -->
## Phase1 repository model

- `base/v4.2.0` is the frozen stable base.
- `edge/stable` is the active default development path.
- `checkpoint/*` branches are verified milestone snapshots.
- `feature/*` branches target `edge/stable`.

Keep the 4.2.0 image and stable base boring. Move tested work through edge/stable.
<!-- phase1:auto:repo-model:end -->

