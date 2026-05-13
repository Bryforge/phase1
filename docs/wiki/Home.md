# Phase1 User Manual

![Edge](https://img.shields.io/badge/edge-v6.0.0-00d8ff) ![Stable](https://img.shields.io/badge/stable-v5.0.0-39ff88) ![Previous Stable](https://img.shields.io/badge/previous%20stable-v4.4.0-7f8cff) ![Compatibility](https://img.shields.io/badge/compatibility-v3.6.0-7f8cff) ![Base1](https://img.shields.io/badge/Base1-foundation-ff8a00) ![Fyr](https://img.shields.io/badge/Fyr-native%20language-ff5a00)

Welcome to the Phase1 manual. This wiki source tracks the current `edge/stable` development line, the current stable base, the Base1 host-foundation track, and the Fyr native-language track.

| Track | Current value | Use it for |
| --- | --- | --- |
| Edge | `v6.0.0` | Active development, v6 UI/help polish, wiki updates, Base1 planning, Fyr growth, and validation work. |
| Stable | `v5.0.0` | Release-qualified demos, public-facing stable references, and safer checkpoint work. |
| Previous stable | `v4.4.0` | Compatibility comparison against the prior stable line. |
| Compatibility base | `v3.6.0` | Historical comparison point shown by older in-app version checks. |
| Base1 | `foundation` | Long-term secure host layer for boot, recovery, installer, storage, rollback, and hardware validation. |
| Fyr | `native language` | Phase1-owned scripting and automation language surface. |

> [!IMPORTANT]
> Phase1 is a terminal-first virtual operating-system console. Base1 is the staged host-foundation path toward real boot and recovery work. This wiki must not describe Phase1 as a hardened drop-in replacement for Linux, macOS, or Windows until boot images, recovery evidence, update paths, audits, and hardware validation exist.

## Start here

1. [Quick Start](01-Quick-Start.md)
2. [Version Guide](02-Version-Guide.md)
3. [Boot Modes and Security](03-Boot-Modes-and-Security.md)
4. [Command Manual](04-Command-Manual.md)
5. [Files, Editors, and Pipelines](05-Files-Editors-and-Pipelines.md)
6. [Browser and Networking](06-Browser-and-Networking.md)
7. [Language Runtimes](07-Language-Runtimes.md)
8. [Updates, Releases, and Validation](08-Updates-Releases-and-Validation.md)
9. [Troubleshooting](09-Troubleshooting.md)
10. [Publish to GitHub Wiki](10-Publish-to-GitHub-Wiki.md)
11. [Tutorials](11-Tutorials.md)
12. [In-System Wiki](12-In-System-Wiki.md)
13. [Base1 OS Track](13-Base1-OS-Track.md)
14. [Fyr Native Language](14-Fyr-Native-Language.md)
15. [Legacy v4 Reference](12-v4-Edge-Manual.md)

## What Phase1 is

Phase1 is a Rust-built, terminal-first virtual OS console created by Chase Bryan / Bryforge. It combines a futuristic operator surface with practical systems ideas: a simulated kernel, virtual filesystem, process table, audit log, command metadata, guarded host access, storage helpers, local learning, the Fyr native language, Nested Phase1 metadata-control, and a long-term operating-system track through Base1.

Phase1 is useful for learning, experimentation, terminal workflows, documentation, and staged OS design. It is not a current hardened sandbox, kernel, or daily-driver operating-system replacement.

## Core rules

> [!IMPORTANT]
> Phase1 starts secure by default. Host-backed tools require explicit trust gates. Do not enter tokens, private keys, account passwords, recovery codes, or other secrets into demos, issues, logs, wiki examples, or screenshots.

> [!TIP]
> TRY THIS INSIDE PHASE1
>
> ```text
> help
> help ui
> help flows
> wiki
> wiki-quick
> version
> version --compare
> security
> capabilities
> sysinfo
> nest status
> nest tree
> roadmap
> ```

## Quick command index

| Goal | Command |
| --- | --- |
| Show command map | `help` |
| Show modern launch pad | `help ui` |
| Show workflow deck | `help flows` |
| Read in-system guide | `cat readme.txt` |
| Open in-system wiki index | `wiki` |
| Open in-system wiki quick start | `wiki-quick` |
| Check current version | `version` |
| Compare release lines | `version --compare` |
| Inspect security posture | `security` |
| Inspect command gates | `capabilities` |
| Show dashboard | `dash` |
| Show system info | `sysinfo` |
| Edit quickly | `ned notes.txt` |
| Edit with AVIM | `avim hello.fyr` |
| Run Fyr script | `fyr run hello.fyr` |
| Inspect nested contexts | `nest status` |
| Show nested topology | `nest tree` |
| Show update protocol | `update protocol` |

## Reader paths

| Path | Best first page |
| --- | --- |
| New user | [Quick Start](01-Quick-Start.md) |
| Current release model | [Version Guide](02-Version-Guide.md) |
| In-system manual commands | [In-System Wiki](12-In-System-Wiki.md) |
| Operator command reference | [Command Manual](04-Command-Manual.md) |
| Python/Rust/C/WASI-lite | [Language Runtimes](07-Language-Runtimes.md) |
| Browser and network tools | [Browser and Networking](06-Browser-and-Networking.md) |
| Base1 boot/recovery path | [Base1 OS Track](13-Base1-OS-Track.md) |
| Fyr scripting path | [Fyr Native Language](14-Fyr-Native-Language.md) |
| Full guided labs | [Tutorials](11-Tutorials.md) |
| Native GitHub Wiki publish flow | [Publish to GitHub Wiki](10-Publish-to-GitHub-Wiki.md) |

## Public wiki source rule

The reviewable wiki source lives in `docs/wiki/`. Publish those files to the native GitHub Wiki only after the source changes are reviewed and validation-sensitive docs stay aligned with the repository README, in-system `wiki-*` pages, release metadata, and website links.

## Manual maintenance rule

When the Cargo package version, stable base, branch model, public asset path, command behavior, or Base1/Fyr status changes, update the README, this manual, in-system `wiki-*` pages, website demo output, release workflow examples, and release metadata tests together. Stable release branches must avoid unsupported edge claims; edge docs must label experimental or host-backed behavior clearly.
