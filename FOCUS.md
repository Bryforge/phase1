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

## Freeze rule

All other project areas are frozen for new feature work unless they directly support Phase1/Base1 integration.

Frozen areas include:

```text
Fyr language work
website/branding expansion
community/support expansion
new crypto-policy expansion
new roadmap branches not required for Phase1/Base1 boot/runtime
new release/public-claim work
```

Existing files and documentation are preserved. Nothing is deleted just because it is frozen.

Allowed changes in frozen areas:

```text
security fixes
secret/privacy cleanup
broken-link fixes
build/test fixes required by Phase1/Base1
short notes pointing back to this focus policy
```

## Active priorities

1. Make the Phase1/Base1 boot path cohesive.
2. Keep X200/Linux/QEMU evidence organized.
3. Stop circular test loops.
4. Build the framebuffer path into a controlled Phase1/Base1 boot option.
5. Preserve terminal and ASCII fallback paths.
6. Add SSH/transfer support only after the boot/runtime path is organized.
7. Promote only known-good work from `black-phase1` to `edge/stable`.

## Non-claims

This focus policy does not claim Phase1/Base1 is stable, hardened, audited, production-ready, or a daily-driver OS.

It only defines what the project is focusing on now.
