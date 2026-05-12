# Phase1/Base1 unification plan

Status: active organization plan  
Branch: `black-phase1` first  
Purpose: reduce project sprawl and focus on one cohesive bootable system

## Decision

Phase1 and Base1 should be organized as one active system path.

```text
Phase1/Base1
  = Phase1 operator/runtime
  + Base1 boot/hardware foundation
  + test evidence
  + QEMU/X200 build paths
```

Other project components remain preserved, but they are frozen for new feature work unless they directly help Phase1/Base1.

## Why this is needed

Recent work proved that the project can move quickly, but it also exposed organizational drift:

- many scripts with historical B-numbers;
- multiple boot experiments still visible at the same level;
- old docs/tests from broader areas competing for attention;
- dirty working trees caused by repeated patch/test cycles;
- unclear separation between active path, archived path, and future work.

The next improvement is not another boot experiment. It is project focus.

## New active lane

The active lane is:

```text
black-phase1 rapid branch
  -> QEMU validation
  -> X200 validation
  -> promote clean known-good commits to edge/stable
```

The active technical target is:

```text
Phase1/Base1 boot path
  -> stable safe terminal fallback
  -> framebuffer boot card path
  -> manual config card path
  -> SSH transfer later
```

## Proposed repository organization

Use existing files for now. Do not mass-move everything in one risky commit.

Start by introducing clear indexes and focus docs:

```text
FOCUS.md
README.md
base1/
docs/os/
scripts/
tests/
```

### Active scripts

Active scripts should eventually be wrapped by simple names:

```text
scripts/phase1-base1-preflight.sh
scripts/phase1-base1-build.sh
scripts/phase1-base1-qemu.sh
scripts/phase1-base1-x200-usb.sh
scripts/phase1-base1-record-result.sh
```

Historical scripts can remain available but should be treated as implementation history.

### Active docs

Active docs should point to:

```text
docs/os/PHASE1_BASE1_UNIFICATION_PLAN.md
docs/os/B47_QEMU_FRAMEBUFFER_EVIDENCE.md
docs/os/B47_X200_FRAMEBUFFER_BOOT_ENTRY.md
docs/os/PHASE1_ASSISTANT_WORKING_REFERENCE.md
```

## Freeze policy

Frozen but preserved:

```text
Fyr docs/work
website expansion
community expansion
crypto-policy expansion
extra roadmap work
public-claim expansion
```

Allowed frozen-area changes:

```text
fix secrets/privacy
fix broken links
fix tests blocking Phase1/Base1
add pointer back to FOCUS.md
```

## Immediate implementation sequence

1. Add `FOCUS.md`.
2. Add this unification plan.
3. Add a single Phase1/Base1 router script.
4. Add a doctor script that catches dirty trees before work starts.
5. Add README pointers to the focus policy.
6. Only then continue framebuffer/X200 work.

## Success criteria

The repo should make it easy to answer:

```text
What are we building now?
How do I test it?
What is frozen?
What is safe to promote?
What is only experimental?
```

## Current answer

```text
We are building Phase1/Base1: a bootable Phase1 operator/runtime on a Base1 hardware/boot foundation.
We test rapidly on black-phase1, verify in QEMU/X200, then promote clean work to edge/stable.
Everything else is preserved but frozen unless it supports this path.
```
