# Phase1/Base1 focus policy

Status: active focus policy  
Branch: `black-phase1` first, promote to `edge/stable` after review

## Current focus

Phase1 and Base1 are now treated as one cohesive active system:

```text
Phase1 = operator/runtime/user-facing system
Base1  = boot/hardware/runtime foundation for Phase1
```

The active project target is:

```text
Phase1/Base1: one bootable, testable, documented system path
```

## Fyr completion exception

Fyr language work is allowed only when it directly advances Phase1/Base1 completion.

Allowed Fyr work:

```text
core syntax needed for operator scripts
VFS-only runtime features
safe package/check/test/build workflows
Phase1 self-workflows
validation helpers
language-book updates that match implemented behavior
```

Still frozen for Fyr:

```text
speculative language expansion
branding-only changes
production-language claims
host shell access outside guarded Phase1 policy
new compiler claims without design evidence
```

The Fyr-to-100 and Phase1-to-100 gate is tracked in [`../status/FYR_PHASE1_100_COMPLETION_GATES.md`](../status/FYR_PHASE1_100_COMPLETION_GATES.md).

## Freeze rule

All other project areas are frozen for new feature work unless they directly support Phase1/Base1 integration.

Frozen areas include:

```text
website/branding expansion
community/support expansion
new crypto-policy expansion
new roadmap branches not required for Phase1/Base1 boot/runtime/Fyr validation
new release/public-claim work not backed by implementation and tests
```

Existing files and documentation are preserved. Nothing is deleted just because it is frozen.

Allowed changes in frozen areas:

```text
security fixes
secret/privacy cleanup
broken-link fixes
build/test fixes required by Phase1/Base1
Fyr changes that satisfy the completion exception above
short notes pointing back to this focus policy
```

## Active priorities

1. Make the Phase1/Base1 boot path cohesive.
2. Keep X200/Linux/QEMU evidence organized.
3. Stop circular test loops.
4. Build the framebuffer path into a controlled Phase1/Base1 boot option.
5. Preserve terminal and ASCII fallback paths.
6. Add SSH/transfer support only after the boot/runtime path is organized.
7. Advance Fyr only where it improves Phase1 operator automation, validation, or self-workflows.
8. Promote only known-good work from `black-phase1` to `edge/stable`.

## Non-claims

This focus policy does not claim Phase1/Base1 is stable, hardened, audited, production-ready, or a daily-driver OS.

It also does not claim Fyr is a production language or a general replacement for Rust, Python, C, or shell.

It only defines what the project is focusing on now.
