# Phase1 User Manual

![Edge](https://img.shields.io/badge/edge-v3.10.9--dev-00d8ff) ![Stable](https://img.shields.io/badge/stable-v3.10.7-39ff88) ![Base](https://img.shields.io/badge/compatibility-v3.6.0-7f8cff)

Welcome to the Phase1 manual. This wiki source reflects the current edge build and the current stable release line.

| Track | Version | Use it for |
| --- | --- | --- |
| Edge | `v3.10.9-dev` | Active development, browser/network improvements, idle-enter guard, current docs |
| Stable | `v3.10.7` | Tagged stable release usage and general demos |
| Compatibility base | `v3.6.0` | Historical stable comparison shown by some in-app version checks |

> [!NOTE]
> GitHub renders these manual callouts with colored accents. This manual intentionally avoids emojis and uses clear command blocks instead.

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

## What Phase1 is

Phase1 is a terminal-first virtual OS and advanced operator console written in Rust. It includes a simulated kernel, virtual filesystem, process table, audit log, guarded browser, guarded network inspection, language runtime manager, storage helper, update protocol, modal editor, and Neo Tokyo style terminal interface.

## Core rules

> [!IMPORTANT]
> Phase1 starts secure by default. Host-backed tools are blocked until SHIELD is off and TRUST HOST is enabled.

> [!TIP]
> TRY THIS
>
> ```text
> help
> cat readme.txt
> wiki
> wiki-quick
> version --compare
> security
> roadmap
> ```

## Quick command index

| Goal | Command |
| --- | --- |
| Show command map | `help` |
| Read in-system guide | `cat readme.txt` |
| Open in-system wiki index | `wiki` |
| Open in-system wiki quick start | `wiki-quick` |
| Check current version | `version` |
| Compare stable and edge | `version --compare` |
| Inspect security posture | `security` |
| Show dashboard | `dash` |
| Show system info | `sysinfo` |
| Edit a file | `avim hello.py` |
| Run browser reader | `browser example.com` |
| Show language support | `lang support` |
| Show update protocol | `update protocol` |

## Tutorial paths

| Path | Best first page |
| --- | --- |
| New user | [Quick Start](01-Quick-Start.md) |
| In-system wiki commands | [In-System Wiki](12-In-System-Wiki.md) |
| Operator commands | [Command Manual](04-Command-Manual.md) |
| Python/Rust/C | [Language Runtimes](07-Language-Runtimes.md) |
| Browser/network | [Browser and Networking](06-Browser-and-Networking.md) |
| Full guided labs | [Tutorials](11-Tutorials.md) |

## Manual maintenance rule

When the Cargo package version changes, update this manual version matrix, README status, in-system `wiki-*` pages, and release workflow examples together.
