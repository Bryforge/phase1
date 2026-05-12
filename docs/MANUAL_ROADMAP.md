# The Phase1 Codex

## Building a Terminal-First Operating World

**Subtitle:** Base1, Fyr, and the Road to a Complete and Programmable Computing System

This document is the canonical planning map for the Phase1 book, manual, and future public wiki. It is designed to grow into a repository documentation system first, then a website or book later.

## Non-claim boundary

Phase1 documentation must preserve these boundaries:

- Phase1 is a Rust-based terminal-first virtual OS console and advanced operator environment. It is not currently a finished operating system, kernel, hardened sandbox, or secure OS replacement.
- Base1 is a planned minimal trusted host foundation for future Phase1-first bootable environments. It must be documented as roadmap, design, dry-run, or preview unless a bootable image has been built, tested, and released.
- Fyr is the Phase1-native language and tooling track. It may be described by the features implemented in the repository and by clearly labeled roadmap goals.
- Installer, recovery, image-writing, hardware-target, and rollback language must not imply destructive readiness unless real validation exists.
- Safety claims must be narrow, testable, visible to the operator, and tied to commands, tests, docs, or release artifacts.

Use this phrase when in doubt:

> This is a current design or roadmap item unless the linked implementation, tests, and release notes prove otherwise.

## Book identity

Canonical title:

> **The Phase1 Codex: Building a Terminal-First Operating World**
>
> **Base1, Fyr, and the Road to a Self-Owned Computing System**

Alternate title options:

1. **Phase1: The Operator's Codex** — A Manual for Terminal-First Computing, Base1, and Fyr.
2. **The Phase1 Operating World** — Building a Rust Terminal Console, Base1 Foundation, and Fyr Toolchain.
3. **Phase1 from Console to Foundation** — A Technical Manual for Operators, Developers, and System Builders.
4. **The Phase1 Field Manual** — Operating, Extending, and Recovering a Terminal-First Computing Environment.
5. **Phase1, Base1, Fyr** — A Practical Roadmap Toward Self-Owned Computing.

## Recommended book structure

### Part I — Orientation

1. **What Phase1 Is**: terminal-first virtual OS console, operator environment, current scope, and non-claims.
2. **The Operating World Concept**: shell, VFS, command registry, logs, help, man pages, host boundary, nested sessions, and roadmap.
3. **Safety Before Capability**: safe mode, trust gates, guarded host tools, visible confirmation, and non-escalation.

### Part II — Phase1 Operator Manual

4. **Installing and Launching Phase1**: clone, `sh phase1`, local command install, `cargo run`, version checks.
5. **Boot Selector and Modes**: safe defaults, SHIELD, trust host, operator-visible boot state, and manual boot equivalents.
6. **The Operator Shell**: command grammar, help, completion, command registry, man-style pages, and workflows.
7. **Virtual Filesystem and System Model**: VFS paths, `/proc`, `/dev`, logs, simulated process table, and inspection commands.
8. **Storage, Git, and Rust Workflows**: guarded storage helpers, Git workflows, Cargo/Rust checks, logs, and safe failure behavior.
9. **Nested Phase1 Sessions**: metadata-only child contexts, topology, active context, and current non-runtime boundary.
10. **Operations Logs and Audit Surfaces**: ops logs, history, learning memory, redaction, and local artifacts.
11. **Troubleshooting and Recovery from the Console**: doctor, selftest, version, safe mode recovery, and common error paths.

### Part III — Base1 Foundation Manual

12. **Why Base1 Exists**: minimal host foundation, read-only base, writable Phase1 state, recovery path, and trust boundary.
13. **Base1 Architecture**: layers, host responsibilities, Phase1 responsibilities, and what is explicitly outside scope.
14. **Boot and Recovery Flow**: boot selector, recovery shell, rollback path, target identity, and emergency shell preservation.
15. **Image Provenance and Validation**: checksums, signatures or attestations when available, read-only validation bundles, and release records.
16. **Hardware Targets**: Libreboot-backed ThinkPad X200-class systems, Raspberry Pi, generic x86_64 later, and validation levels.
17. **Installer and Destructive Action Policy**: dry-run first, target verification, no destructive defaults, explicit confirmation, and rollback metadata.
18. **Base1 Roadmap and Maturity Model**: design, dry-run, emulator, hardware lab, preview image, validated release.

### Part IV — Fyr Language Book

19. **Why Fyr Exists**: Phase1-native automation, self-construction, VFS workflows, and command scripting.
20. **First Fyr Program**: `.fyr` files, check/build/test/run workflow, and minimal examples.
21. **Syntax and Semantics**: functions, returns, bindings, expressions, assertions, conditionals, and package structure.
22. **Fyr Toolchain Commands**: checker, runner, package tests, diagnostics, and planned build outputs.
23. **Using Fyr with Phase1**: VFS automation, command integration, operator scripts, and safe execution boundaries.
24. **Fyr Roadmap**: standard library, workspace model, deeper Phase1 integration, and eventual self-hosting goals.

### Part V — Developer, Security, and Contributor Guide

25. **Repository Architecture**: crates, binaries, scripts, docs, tests, and release files.
26. **Adding Commands Safely**: command metadata, help text, capability labeling, host boundaries, and tests.
27. **Documentation Contribution Rules**: style, status tags, claims policy, examples, and PR checklist.
28. **Security Review Guide**: trust boundaries, host tools, secret redaction, validation bundles, and threat model.
29. **Release and Website Publishing**: docs-to-site path, versioned docs, public wiki, and release gates.

### Appendices

A. Command matrix.
B. Capability and trust-gate table.
C. Roadmap maturity table.
D. Glossary.
E. Recovery checklists.
F. Hardware validation forms.
G. Documentation PR checklist.
H. Non-claim language reference.

## Repository documentation architecture

```text
docs/
  README.md
  MANUAL_ROADMAP.md
  phase1/
    README.md
    OPERATOR_MANUAL.md
    BOOT_AND_MODES.md
    COMMAND_REFERENCE.md
    VFS_AND_SYSTEM_MODEL.md
    docs/runtime/STORAGE_GIT_RUST.md
    NESTED_PHASE1.md
    OPS_LOGS.md
    TROUBLESHOOTING.md
  base1/
    README.md
    FOUNDATION_MANUAL.md
    BOOT_FLOW.md
    RECOVERY_MODEL.md
    IMAGE_PROVENANCE.md
    HARDWARE_TARGETS.md
    INSTALLER_POLICY.md
    ROLLBACK.md
    MATURITY_MODEL.md
  fyr/
    README.md
    LANGUAGE_BOOK.md
    GETTING_STARTED.md
    SYNTAX.md
    TOOLCHAIN.md
    PACKAGES.md
    PHASE1_INTEGRATION.md
    ROADMAP.md
  operators/
    README.md
    FIRST_RUN.md
    DAILY_WORKFLOWS.md
    SAFE_HOST_TOOLS.md
    INCIDENT_NOTES.md
  developers/
    README.md
    ARCHITECTURE.md
    ADDING_COMMANDS.md
    TESTING.md
    DOCS_CONTRIBUTING.md
  recovery/
    README.md
    SAFE_RECOVERY.md
    RECOVERY_USB.md
    TARGET_IDENTITY.md
    HARDWARE_CHECKLISTS.md
  security/
    README.md
    TRUST_MODEL.md
    DOCS_CLAIMS.md
    THREAT_MODEL.md
    REVIEW_GUIDE.md
    SECRET_HANDLING.md
```

Each folder should start as an index that links to existing docs before copying or moving content. Avoid duplicate truth sources until the manual structure is stable.

## Reader paths

| Audience | Start here | Then read | Goal |
| --- | --- | --- | --- |
| First-time user | `docs/README.md` | `docs/operators/FIRST_RUN.md`, `docs/phase1/OPERATOR_MANUAL.md` | Launch Phase1 safely and understand current scope. |
| Operator | `docs/operators/README.md` | `docs/phase1/BOOT_AND_MODES.md`, `docs/operators/SAFE_HOST_TOOLS.md` | Use Phase1 without crossing host boundaries accidentally. |
| Developer | `docs/developers/README.md` | `docs/developers/ARCHITECTURE.md`, `docs/developers/ADDING_COMMANDS.md` | Add features with tests, help, and capability metadata. |
| Security reviewer | `docs/security/TRUST_MODEL.md` | `docs/security/DOCS_CLAIMS.md`, `docs/security/REVIEW_GUIDE.md` | Review narrow claims, trust boundaries, and validation evidence. |
| Recovery/hardware operator | `docs/recovery/README.md` | `docs/base1/RECOVERY_MODEL.md`, `docs/base1/HARDWARE_TARGETS.md` | Follow dry-run-first recovery planning without destructive assumptions. |
| Language/toolchain contributor | `docs/fyr/README.md` | `docs/fyr/LANGUAGE_BOOK.md`, `docs/fyr/TOOLCHAIN.md` | Extend Fyr while preserving Phase1 integration boundaries. |

## Technical style guide

### Tone

Write like an engineering manual. Prefer direct verbs, explicit status, examples, and validation steps. Avoid hype words such as revolutionary, unbreakable, military-grade, fully secure, complete OS, or production-ready unless backed by releases and audit evidence.

### Terminology

Use these status labels consistently:

- **Implemented**: code exists, tests exist, and docs explain how to run it.
- **Experimental**: code exists but boundary, UX, or hardening is incomplete.
- **Design**: architecture exists in docs but implementation is not complete.
- **Dry-run**: command or script validates intent without destructive mutation.
- **Preview**: feature is usable by early testers with known limitations.
- **Roadmap**: planned future work.
- **Not claimed**: explicitly outside the current guarantee.

### Safety language

Use narrow statements:

- Good: `The preflight script is intended to be read-only and should not write to block devices.`
- Bad: `The installer is safe.`
- Good: `This command requires explicit operator confirmation before host mutation.`
- Bad: `Phase1 prevents dangerous actions.`

### Current, preview, roadmap, and not claimed

Every major page must include a status block:

```md
> **Status:** Implemented | Experimental | Design | Dry-run | Preview | Roadmap | Not claimed
> **Validation:** tests, scripts, release notes, or manual verification path
> **Non-claims:** what this page does not guarantee
```

## Chapter outlines

### Phase1 Operator Manual

1. **Scope and mental model**: current virtual OS console, not a kernel, host boundary, safe defaults.
2. **Install and launch**: clone, run scripts, cargo run, version checks, local command install.
3. **Boot selector**: safe shield, trust host, boot choices, failure states, and recovery posture.
4. **Shell basics**: commands, help UI, command registry, man pages, autocomplete, history, and navigation.
5. **VFS and system inspection**: files, directories, `/proc`, `/dev`, process table, sysinfo, dashboards.
6. **Host tools**: what guarded host tools are, when prompts appear, what is blocked by default, how logs record use.
7. **Storage and development workflows**: storage helper, Git, Rust/Cargo, package checks, and expected errors.
8. **Nested Phase1**: metadata contexts, spawn/enter/exit/tree, what nesting does not yet execute.
9. **Fyr for operators**: run/check/test `.fyr` scripts, examples, and operator automation boundaries.
10. **Logs, learning, and local artifacts**: ops logs, sanitized memory, git-ignored files, and cleanup.
11. **Troubleshooting**: doctor, selftest, permissions, missing tools, safe-mode recovery, and issue reporting.
12. **Operator checklist**: before host tools, before Git, before scripts, before release work.

### Base1 Recovery and OS Foundation Manual

1. **Scope and non-claims**: planned foundation, not currently a finished secure OS replacement.
2. **Base1 principles**: minimal host, read-only base, writable Phase1 state, explicit recovery path.
3. **Layer model**: firmware, bootloader, Base1 base, Phase1 state, operator shell, host tools.
4. **Boot flow**: normal boot, recovery boot, rollback, target identity checks, and failure handling.
5. **Recovery shell**: preservation requirements, allowed actions, blocked actions, emergency workflows.
6. **Recovery USB planning**: read-only validation, image provenance, target selection, and hardware checklists.
7. **Image provenance**: source, build recipe, checksum, signature/attestation when available, release note tie-in.
8. **Rollback metadata**: state snapshot references, previous image references, operator-readable rollback notes.
9. **Installer policy**: dry-run first, target verification, no destructive default, confirmation prompts, logs.
10. **Hardware targets**: X200-class Libreboot, Raspberry Pi, generic x86_64, and validation maturity.
11. **Validation reports**: emulator pass, hardware pass, recovery pass, rollback pass, known gaps.
12. **Roadmap gates**: what must be true before preview image, beta hardware image, daily-driver claim, or security claim.

### Fyr Language Book

1. **Purpose and status**: Phase1-native track, current implementation, non-production claims.
2. **Getting started**: create `.fyr`, run checker, run test, run program.
3. **Program structure**: files, functions, main, return values, packages.
4. **Expressions and values**: numbers, strings, booleans, grouped expressions, comparisons.
5. **Bindings and control flow**: let bindings, if statements, assertions, return statements.
6. **Diagnostics**: syntax errors, checker output, line/column expectations, test cases.
7. **Toolchain commands**: check, build, test, run, package validation, planned outputs.
8. **Phase1 integration**: VFS automation, shell commands, command scripts, ops visibility.
9. **Safety model**: Fyr execution boundaries, no hidden host escalation, trust gates for host-backed behavior.
10. **Examples**: hello, command helper, VFS workflow, package test, failure example.
11. **Contributor guide**: parser, checker, runtime, tests, docs, compatibility expectations.
12. **Roadmap**: standard library, workspace layout, self-construction, deeper operator integration.

## Diagrams and tables to include

- Phase1 architecture diagram: operator shell, command registry, VFS, logs, host boundary, Fyr, nested sessions.
- Base1 layer diagram: firmware, bootloader, read-only base, writable state, recovery shell, Phase1.
- Boot flow diagram: normal boot, safe mode, trust host off/on, recovery path.
- Trust boundary table: Phase1 internal, host shell, filesystem, network, block devices, secrets.
- Guarded host tool flow: command request, capability metadata, shield check, trust gate, confirmation, log.
- VFS map table: path, purpose, implementation status, persistence status.
- Command matrix: command, category, implemented status, host access, confirmation required, tests.
- Base1 recovery flow: detect failure, preserve shell, identify target, verify provenance, rollback or inspect.
- Image provenance table: artifact, source, checksum, signer, build recipe, release note, validation date.
- Roadmap maturity table: design, docs, dry-run, emulator, hardware lab, preview, validated release.
- Fyr toolchain table: file type, command, current behavior, planned behavior.
- Documentation claim matrix: phrase, allowed status, required proof.

## Safety and trust model summary

### Safe defaults

Phase1 should default to behavior that keeps host mutation off unless the operator makes a visible choice. Safe defaults must be documented as defaults, not as absolute protection.

### Guarded host tools

Host-backed commands must declare what host capability they need. The manual should explain the capability before instructing operators to enable it.

### No host trust escalation

Phase1 documentation must not imply that Phase1 can turn an untrusted host into a trusted host. Trust gates only control Phase1's own willingness to call host-backed tools.

### Read-only validation bundles

Validation scripts and reports should prefer read-only checks. If a script writes anything, the docs must say what it writes, where, and how to inspect or remove it.

### Explicit operator confirmation

Commands that mutate host state, write images, change devices, or touch credentials must require explicit operator confirmation. The docs must show the confirmation path and expected log.

### Recovery shell preservation

Base1 recovery planning must preserve a shell path even when Phase1 fails. Documentation should treat recovery shell access as a requirement, not a nice-to-have.

### Image provenance and checksum rules

Base1 image docs must require source, build recipe, checksum, release identifier, and validation notes before an image is presented as usable. Without those, the image must be called a design artifact, dry-run artifact, or preview artifact.

## Glossary

**Phase1**: A Rust-based terminal-first virtual OS console and advanced operator environment. Current Phase1 runs on a host system and is not itself a kernel or finished OS replacement.

**Base1**: The planned minimal trusted host foundation for future Phase1-first bootable environments. Base1 is expected to start Linux-based with a read-only base, writable Phase1 state layer, recovery shell, and rollback path.

**Fyr**: The Phase1-native language and toolchain track for `.fyr` scripts, VFS automation, command workflows, package structure, checking, testing, running, and eventual deeper Phase1 integration.

**VFS**: The Phase1 virtual filesystem model used to expose files, simulated system paths, logs, and operator-visible state inside the Phase1 environment.

**Operator shell**: The interactive Phase1 command surface used to inspect state, run commands, read help/man pages, and operate workflows.

**Guarded host tools**: Host-backed commands or scripts that require explicit capability handling, trust gates, safe-mode posture, and visible operator intent.

**Safe shield**: The default protective posture that keeps higher-risk host-backed behavior disabled unless explicitly changed by the operator.

**Trust gate**: A visible decision point before Phase1 invokes host-backed behavior. A trust gate is not a guarantee that the host is trustworthy.

**Recovery USB**: A planned Base1 recovery medium for inspection, recovery shell access, target identity validation, provenance checks, and rollback support. It must remain roadmap or preview until validated on real hardware.

**Image provenance**: The documented origin, build recipe, checksum, release identity, and validation evidence for a Base1 or recovery image.

**Rollback metadata**: Operator-readable metadata describing the previous known-good image or state, the current candidate, and how rollback should be performed or verified.

**Nested Phase1**: A current metadata-only recursive operator-context feature. It tracks child contexts and topology but does not yet imply runtime-backed inner kernels.

## First implementation slice

### Files to add first

- `docs/README.md`: reader entry point and status legend.
- `docs/MANUAL_ROADMAP.md`: canonical Codex roadmap and documentation architecture.
- `docs/security/TRUST_MODEL.md`: consolidated trust model and safety claims rules.
- `docs/security/DOCS_CLAIMS.md`: allowed phrases, disallowed phrases, proof requirements.
- `docs/phase1/README.md`: Phase1 operator manual index.
- `docs/base1/README.md`: Base1 foundation manual index.
- `docs/fyr/README.md`: Fyr language book index.
- `docs/operators/README.md`: operator reader path.
- `docs/developers/README.md`: developer reader path.
- `docs/recovery/README.md`: recovery/hardware reader path.
- `tests/manual_roadmap_docs.rs`: guard test for required pages and non-claim phrases.

### Tests to add

The first test should verify that:

- `docs/MANUAL_ROADMAP.md` exists.
- `docs/security/TRUST_MODEL.md` exists.
- The docs include the title `The Phase1 Codex`.
- The docs include the exact non-claim that Phase1 is not a finished secure OS replacement.
- Base1 pages include roadmap or planned-foundation language.
- Recovery pages do not contain destructive installer claims without dry-run wording.

### README links to update

Add one top-level README link:

```md
[Codex](MANUAL_ROADMAP.md)
```

Add or update the docs section so users can find:

- Phase1 Operator Manual
- Base1 Foundation Manual
- Fyr Language Book
- Security and Trust Model
- Recovery and Hardware Guide

### Acceptance criteria

- Documentation builds as plain Markdown.
- Existing quick-start instructions remain unchanged.
- New docs are repo-local and do not require a website generator.
- Non-claim language is present in roadmap, Base1, recovery, and security docs.
- The docs guard test passes with `cargo test -p phase1 --test manual_roadmap_docs`.
- No page claims Phase1 is a finished OS, Base1 is a released bootable image, or recovery is validated on real hardware unless linked evidence exists.

### Non-claims to preserve

- Phase1 is not currently a secure OS replacement.
- Phase1 is not currently a kernel.
- Base1 is not currently a released bootable daily-driver image.
- Recovery USB and destructive installer workflows are not complete unless validated.
- Guarded host tools do not make untrusted host actions safe by themselves.
- Safe mode and trust gates reduce accidental action inside Phase1; they do not replace a VM, container, audit, or real operating-system hardening.

## Documentation PR quality checklist

Every documentation PR should answer:

- Does the page identify its status: implemented, experimental, design, dry-run, preview, roadmap, or not claimed?
- Does every safety statement name the mechanism and validation path?
- Does the PR avoid saying secure, safe, hardened, bootable, daily-driver, installer-ready, or recovery-complete without evidence?
- Are commands copy-pasteable and scoped to the right platform?
- Are destructive or host-mutating commands marked clearly?
- Are dry-run examples shown before real mutation examples?
- Are Phase1, Base1, and Fyr terms used consistently?
- Are current implementation details separated from roadmap goals?
- Are tests, scripts, or release notes linked where claims are made?
- Does the PR avoid duplicating canonical docs without linking back to the source of truth?
- Does the PR include or update a docs guard test when it adds safety-critical language?

## Launch plan for public book/wiki

1. **Repo-first Codex**: land the folder architecture and guard tests.
2. **Manual skeleton**: add chapter stubs with status blocks and links to existing docs.
3. **Content migration**: move or index existing wiki, Base1, Fyr, nested, and OS-track docs without losing history.
4. **Validation pass**: add docs tests for non-claims, broken links, command snippets, and required status blocks.
5. **Website preview**: generate a static site from `docs/` while keeping Markdown readable in GitHub.
6. **Versioned docs**: split stable and edge docs once releases need different manual versions.
7. **Public Codex launch**: publish the site as the official technical manual, not as a marketing page.
8. **Hardware/recovery appendix release**: publish hardware pages only after validation reports exist.
9. **Book export**: convert the stable manual to PDF/ePub after the repo docs have stabilized.
10. **Ongoing governance**: make docs claim review part of every release checklist.

## Base1 public compatibility path

- [`docs/base1/ROOT_COMPATIBILITY_MAP.md`](ROOT_COMPATIBILITY_MAP.md) — Public path for the Base1 root compatibility map.

## Root compatibility map public path

- [`ROOT_COMPATIBILITY_MAP.md`](ROOT_COMPATIBILITY_MAP.md) — Base1 root compatibility map.
- `docs/base1/ROOT_COMPATIBILITY_MAP.md` — Public compatibility-map path.
- [`docs/base1/ROOT_COMPATIBILITY_MAP.md`](base1/ROOT_COMPATIBILITY_MAP.md) — Repository-root public path for the Base1 root compatibility map.
